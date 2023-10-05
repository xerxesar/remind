#!/usr/bin/bash
rm $HOME/.config/systemd/user/remind-daemon.service
systemctl --user stop remind-daemon
systemctl --user disable remind-daemon
systemctl --user daemon-reload
exit 0