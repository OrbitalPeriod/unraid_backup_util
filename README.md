# Unraid_backup_util

A quick rust program that allows easy backups using rsync.

Uses the unraid notification system to send you notification about the status of backups.

## Requirements:
For the base of the program:
- The device needs to be able to rsync into the backup side using key authentication.
- Needs to have read/write priviliges to the appropiate folders.

For Wol to work (Optional):
- WoL needs to be enabled on the target device, and you need to add the mac_address to the config file. The mac address is in decimal.

For shutdown to work (Optional): 
- The user needs to be able to perform the shutdown command without password.

## Configuration:
The default file used is config.json, this file has certain fields:
- mac_address (optional): wether to use WoL. 
- Shutdown: Wether for the program shuts down the remote afterwards. 
- User: The user to rsync into the remote as.
- hostname: The address of the remote.
- Options: Any options to run rsync with.
- Jobs: An array of sets of sources and destinations. Do not include the hostname and user in this, the program does this automatically.

## Usage:
simply run the executable from the terminal.
By default is uses config.json as config file, but a different file name can be passed as an argument.
