#!/bin/bash

URL=$1
ID=$2

current_workspace_num=$(xprop -root -notype _NET_CURRENT_DESKTOP | awk '{print $3}')
exist_browser=$(wmctrl -l | grep "\s$current_workspace_num\s" | grep Brave | wc -l)

# workspace가 1번만 존재할 경우 xprop -root -notype _NET_CURRENT_DESKTOP은 "not found." 가 된다
# 위 경우 "$current_workspace_num" 는 "found." 가 되고 이때는 무조건 --new-tab으로 open 하면 된다
if (( exist_browser > 0 )) || [[ "$current_workspace_num" == "found." ]] ; then
    brave $1
else
    brave --new-window $1
fi

# 최근 방문한 사이트 정보를 남긴다
echo "$ID | $(date +%s)" >> ~/.cache/illef-findex-plugin/access_log
