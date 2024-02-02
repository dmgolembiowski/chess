#!/usr/bin/env bash

editor_address=$(hyprctl activewindow -j | jq -r .address) # or jq

socat -u UNIX-CONNECT:/tmp/hypr/"$HYPRLAND_INSTANCE_SIGNATURE"/.socket2.sock - | \
while read -r line; do
    if [[ ${line:0:10} == openwindow ]]; then
        found=$(echo $line | grep funky-chess | grep -v grep | wc -c)
        if [[ "$found" != "0" ]]; then
            hyprctl dispatch focuswindow address:"$editor_address"
        fi
        # window=${line:27}
        # class=${window%,*}
        # title=${window#*,}

        #if [[ $window == "floating,funky-chess" ]]; then
        #    # or use class/title separately
        #    hyprctl dispatch focuswindow address:"$editor_address"
        #fi
    fi
done
