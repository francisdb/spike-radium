# radium
Stern SPIKE 2 radium file rust library

## Getting started

Make an image of your orginal SD card before doing anything else. Keep that image and preferably the original SD card safe.

Mount the image on you linux computer. (currently we only use the main game partition)

Create a symlink on the root of this project to the lcd directory

```
 lcd -> /media/you/partition_id/star_wars_le/assets/lcd
```

Now start the test

```
cargo run
```

**I'm currently testing a dump of Star Wars Premium**

## More SPIKE 2 resources

See some of the following how you can configure your machine for remote-access:

- https://pastebin.com/raw/RryUb8iC
- https://missionpinball.org/latest/hardware/spike/mpf-spike-bridge/
- https://missionpinball.org/latest/hardware/spike/connection/
- https://github.com/missionpinball/mpf-spike
- https://github.com/missionpinball/mpf/blob/dev/mpf/platforms/spike/spike.py
- https://github.com/JayFoxRox/stern-spike-dumper
- https://github.com/JayFoxRox/spk-tools
- https://github.com/bdash/spike-spk
- https://archive.org/details/stern-pinball-sd-card-images
- https://archive.org/details/stern-pinball-game-code
- https://vpuniverse.com/forums/topic/5497-the-munsters-le-wip-vpx/page/3/
