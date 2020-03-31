# arm-none-eabi-objcopy --remove-section .tracebuffer target/thumbv7em-none-eabihf/debug/stm32mp1-hal
ssh root@192.168.1.110 'mount -o remount,rw /'
ssh root@192.168.1.110 'mkdir -p /lib/firmware'
scp target/thumbv7em-none-eabihf/debug/stm32mp1-hal root@192.168.1.110:/lib/firmware/firmware.elf
ssh root@192.168.1.110 'echo stop > /sys/class/remoteproc/remoteproc0/state'
ssh root@192.168.1.110 'echo firmware.elf > /sys/class/remoteproc/remoteproc0/firmware'
ssh root@192.168.1.110 'echo start > /sys/class/remoteproc/remoteproc0/state'
ssh root@192.168.1.110 'cat /sys/class/remoteproc/remoteproc0/state'