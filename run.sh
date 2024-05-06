#!/usr/bin/sh
set -e

cargo run

sed -i 's/Promise.all.*/const withLogging = (callback, name) => (...args) => { let result = callback(...args); console.log(`Function [[${name}]] called with`, ...args, `returned`, result); return result; };/' ./out/component.js

sed -i -E "s/'\\[resource-drop\\]employee': (.*?),/'[resource-drop]employee': withLogging(\\1, 'DROP'),/g" ./out/component.js
sed -i -E "s/'\\[resource-new\\]employee': (.*?),/'[resource-new]employee': withLogging(\\1, 'NEW'),/g" ./out/component.js
sed -i -E "s/'\\[resource-rep\\]employee': (.*?),/'[resource-rep]employee': withLogging(\\1, 'REP'),/g" ./out/component.js
