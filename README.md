# file_syncer

This is a simple cli tool to keep the content of two different folders in sync. It is similar to netcat in a way that file_sync acts as client on one side as client and on the other as server. Those two folders could be on different systems.

The final goal is to make it embeddedable in python scripts. Hence it should allow to react on new/change files with commands or python code on both sides.