# Snapshoot
Snapshoot recursively copy a source folder to a destination folder.
The destination folder must exist and should be used only with the same source folder.
For each day snapshot, the system will compare with the yesterday snapshot and avoid a full copy for the files that did not change.

# Usage
- Source folder and destination folder must exist
```
Usage: snapshoot <COMMAND>

Commands:
  shoot  Shoot process
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```
Usage: snapshoot shoot --source <SOURCE> --destination <DESTINATION>

Options:
      --source <SOURCE>            Source folder (required)
      --destination <DESTINATION>  Destination folder (required)
  -h, --help                       Print help
```
