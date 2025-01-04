#!/usr/bin/env sh

FILES=/dev/hidraw*
found_hidraw=""

for f in $FILES
do
  FILE=${f##*/}
  DEVICE="$(cat /sys/class/hidraw/${FILE}/device/uevent | grep HID_NAME | cut -d '=' -f2)"
  printf "%s \t %s\n" $FILE "$DEVICE"

  # Check if the device name contains "Maschine"
  if [[ "$DEVICE" == *"Maschine"* ]]; then
    echo "Found Maschine device: $FILE"
    # Extract the number from the FILE variable
    found_hidraw=${FILE#hidraw}
    break
  fi
done

# Check if a Maschine device was found
if [ -n "$found_hidraw" ]; then
  hidraw=$found_hidraw
  echo "Setting hidraw to: $hidraw"
else
  echo "No Maschine device found. Please enter the hidraw NUMBER manually."
  read hidraw
fi

echo starting MK2

echo Do you want to write on the screen? Y/N
read confirm

if [ "$confirm" = "n" ] || [ "$confirm" = "N" ]
then ./target/release/maschine /dev/hidraw$hidraw any
else ./target/release/maschine /dev/hidraw$hidraw
fi
