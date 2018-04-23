class DelayCmd < Formula
  desc "Delay a command"
  homepage "https://github.com/kjstouffer/delay-cmd"
  version "0.1.0"
  url "#{homepage}/releases/download/#{version}/delay-cmd-#{version}-x86_64-apple-darwin.tar.gz"
  sha256 "ee4b5d179f4a367c816a8768a878813ae40773048794ec3183f6c2aa146e4b7e"
  head "https://github.com/kjstouffer/delay-cmd.git"

  option "with-logging", "Redirect stdout and stderr to log files"

  def install
    (var/"log/delay-cmd").mkpath
    bin.install "delay-cmd"

    bash_completion.install "delay-cmd.bash"
    fish_completion.install "delay-cmd.fish"
    zsh_completion.install "_delay-cmd"
  end

  def caveats; <<~EOS
    If the formula has been built with --with-logging, logs will be found in
      #{var}/log/delay-cmd/delay-cmd.[out|err].log
    EOS
  end

  if build.with? "logging"
      def plist; <<~EOS
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
          <key>Label</key>
          <string>#{plist_name}</string>
          <key>ProgramArguments</key>
          <array>
            <string>#{opt_bin}/delay-cmd</string>
            <string>--server</string>
          </array>
          <key>EnvironmentVariables</key>
          <dict>
            <key>PATH</key>
            <string>#{HOMEBREW_PREFIX}/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
          </dict>
          <key>RunAtLoad</key>
          <true/>
          <key>KeepAlive</key>
          <true/>
          <key>StandardOutPath</key>
          <string>#{var}/log/delay-cmd/delay-cmd.out.log</string>
          <key>StandardErrorPath</key>
          <string>#{var}/log/delay-cmd/delay-cmd.err.log</string>
        </dict>
        </plist>
        EOS
      end
  else
      def plist; <<~EOS
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
        <dict>
          <key>Label</key>
          <string>#{plist_name}</string>
          <key>ProgramArguments</key>
          <array>
            <string>#{opt_bin}/delay-cmd</string>
            <string>--server</string>
          </array>
          <key>EnvironmentVariables</key>
          <dict>
            <key>PATH</key>
            <string>#{HOMEBREW_PREFIX}/bin:/usr/bin:/bin:/usr/sbin:/sbin</string>
          </dict>
          <key>RunAtLoad</key>
          <true/>
          <key>KeepAlive</key>
          <true/>
        </dict>
        </plist>
        EOS
      end
  end

  test do
    assert_match "delay-cmd #{version}", shell_output("#{bin}/delay-cmd --version")
  end
end
