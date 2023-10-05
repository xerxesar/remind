#!/usr/bin/bash

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

SERVICE_NAME="remind-daemon"
DESCRIPTION="Remind daemon service"
BIN_FILE="/usr/local/bin/remind"

SERVICE_UNIT_PATH=$HOME/.config/systemd/user/${SERVICE_NAME//'"'/}.service

# check if service is active
IS_ACTIVE=$(systemctl --user is-active $SERVICE_NAME)
echo $IS_ACTIVE
if [ "$IS_ACTIVE" == "active" ]; then
    # restart the service
    echo "Service is running"
    echo "Restarting service"
    systemctl --user restart $SERVICE_NAME
    echo "Service restarted"
else
    # create service file
    echo "Creating service file"
    cat > $SERVICE_UNIT_PATH << EOF
[Unit]
Description=$DESCRIPTION

[Service]
ExecStart=$BIN_FILE
Restart=on-failure

[Install]
WantedBy=graphical-session.target
EOF
    # restart daemon, enable and start service
    echo "Reloading daemon and enabling service"
    systemctl --user daemon-reload 
    systemctl --user enable ${SERVICE_NAME//'.service'/} # remove the extension
    systemctl --user start ${SERVICE_NAME//'.service'/}
    echo "Service Started"
fi

exit 0