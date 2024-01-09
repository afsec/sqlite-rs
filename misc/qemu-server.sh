#!/bin/bash
# Create a new tmux session
tmux new-session -s 'app-under-test-session' -d -x 140 -y 35
# Create a new tmux windows, where the app is running
tmux new-window -t 'app-under-test-session' -n 'app-window' -d './app-to-launch.sh | tee app-log.log'
# Do the test with the app.
sleep 1
exit=0
while [ $exit -lt 1 ]
do
    clear
    while IFS= read -r line
    do 
        echo "SERVER: RECEIVED [$line]"
        
        echo "$line" | grep '^login:' > /dev/null && tmux send  -t 'app-under-test-session':'app-window' root Enter
        echo "$line" | grep '^\$ ' > /dev/null && tmux send -t 'app-under-test-session'.0 exit Enter
        echo "$line" | grep '^Exiting' > /dev/null && exit=1
    done <<< $(tail -n 2 ./app-log.log)
    sleep 1
    echo
done
echo 'SERVER: Finished'
#sleep 10
# Bonus, stopping the tmux session cleans up sub processes etc as well
tmux kill-session -t 'app-under-test-session'