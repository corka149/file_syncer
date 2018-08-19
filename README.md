# file_syncer

This is a simple cli tool to keep the content of two different folders in sync. It is similar to netcat in a way that file_sync acts as client on one side as client and on the other as server. Those two folders could be on different systems.

The final goal is to make it embeddedable in python scripts. Hence it should allow to react on new/change files with commands or python code on both sides.

## TODO

Parts that need to be impelmented
- ~~File register: Check which files avaible and determine if they are new or changed~~
    - ~~Hash and store them~~
    - ~~Check if file is open then do not register it~~
- Send/Receive files and replace/create them
    - Execute command before sending files
    - Execute command after receiving files
- CLI-interface
    - Client or server parameters
    - Filter possibility
- Python module-interface
    - Implement event or hook point for python 
    - Easy installation
- Standalone mod
- Made time interval configurable
- Implement recursive file checking
