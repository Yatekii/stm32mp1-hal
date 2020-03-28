ssh root@192.168.178.63 'mkdir -p /lib/firmware'
scp target/thumbv7em-none-eabihf/debug/stm32mp1-hal root@192.168.178.63:/lib/firmware/firmware.elf
ssh root@192.168.178.63 'echo stop > /sys/class/remoteproc/remoteproc0/state'
ssh root@192.168.178.63 'echo firmware.elf > /sys/class/remoteproc/remoteproc0/firmware'
ssh root@192.168.178.63 'echo start > /sys/class/remoteproc/remoteproc0/state'
ssh root@192.168.178.63 'cat /sys/class/remoteproc/remoteproc0/state'