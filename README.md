# Intro 
[WIP] Experimental disk utility CLI that can shred files, clone disks, securely wipe drives.


# Project Structure
This application is organized into separate modules:

*App*: Contains application logic, data and state is stored and/or updated.
*UI*: UI draws the visual representation of the app, text, graphics, colors.
*Disk*: Contains methods to fetch system information such as drive type, mount point, model, etc.
*Core*: Where the magic lives. Will contain the various algorithms used in disk destruction or disk cloning.