PIDS=($(ps aux | grep 'lsyncd' | grep "$USER" | awk '{print $2}'))
PID_KILLED=0
if [ ${#PIDS[@]} -gt 0 ];then
    for i in "${PIDS[@]}"
        do
           echo "Killing process with PID - $i"
           kill -9 $i
           PID_KILLED=1
        done
fi
