#!/bin/bash

CACHE_HOME="$HOME/.cache/illef-findex-plugin"
CACHE_FILE="$CACHE_HOME/raindrop.cache.json"

# JSON 파일 읽기 및 파싱
items=$(jq -c '.[]' "$CACHE_FILE")

# 각 항목에 대해 반복
echo "$items" | while read -r item; do
    # link와 id 추출
    link=$(echo "$item" | jq -r '.link')
    id=$(echo "$item" | jq -r '._id')

    if [ -f "$CACHE_HOME/favicons/${id}.ico" ]; then
        continue
    fi

    # favicon URL 구성 및 다운로드
    favicon_url="https://t1.gstatic.com/faviconV2\?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=$link&size=64"
    echo $favicon_url
    curl -L -s "$favicon_url" -o "$CACHE_HOME/favicons/${id}.ico" --http1.1

    size=$(identify "$HOME/.cache/raindrop/favicons/${id}.ico" | awk '{print $3}')

    if [ "$size" = "16x16" ]; then
        favicon_url="https://t1.gstatic.com/faviconV2\?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url=https://raindrop.io&size=64"
        curl -s -L "$favicon_url" -o "$CACHE_HOME/favicons/${id}.ico" --http1.1
    fi

    echo "Downloaded favicon for $link as ${id}.ico"
done


