on run {daemon_file, agent_file, user}

  set prefs_dir to "/Users/" & user & "/Library/Preferences/com.corevia.CoreviaDesk/"
  set prefs_toml to quoted form of (prefs_dir & "Corevia Desk.toml")
  set prefs2_toml to quoted form of (prefs_dir & "Corevia Desk2.toml")

  set sh1 to "echo " & quoted form of daemon_file & " > /Library/LaunchDaemons/com.corevia.CoreviaDesk_service.plist && chown root:wheel /Library/LaunchDaemons/com.corevia.CoreviaDesk_service.plist;"

  set sh2 to "echo " & quoted form of agent_file & " > /Library/LaunchAgents/com.corevia.CoreviaDesk_server.plist && chown root:wheel /Library/LaunchAgents/com.corevia.CoreviaDesk_server.plist;"

  set sh3 to "cp -rf " & prefs_toml & " /var/root/Library/Preferences/com.corevia.CoreviaDesk/;"

  set sh4 to "cp -rf " & prefs2_toml & " /var/root/Library/Preferences/com.corevia.CoreviaDesk/;"

  set sh5 to "launchctl bootout system/com.corevia.CoreviaDesk_service 2>/dev/null || launchctl unload -w /Library/LaunchDaemons/com.corevia.CoreviaDesk_service.plist 2>/dev/null || true; launchctl bootstrap system /Library/LaunchDaemons/com.corevia.CoreviaDesk_service.plist 2>/dev/null || launchctl load -w /Library/LaunchDaemons/com.corevia.CoreviaDesk_service.plist;"

  set sh to sh1 & sh2 & sh3 & sh4 & sh5

  do shell script sh with prompt "Corevia Desk wants to install daemon and agent" with administrator privileges
end run
