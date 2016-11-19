# Remote player

Remote player is a small command line application that plays audio.
It is written in Rust and uses GStreamer as a portable backend. As it is using GStreamer it should be able to play almost everything, needs testing though.

Run it with `remote-player` and send it commands.

## Usage

```
LOAD <track_path> -- Load and begin to play a new track 
PAUSE -- Pause and Resume playback
VOLUME <volume> -- set volume, from 0.0 to 1.0
SEEK <seek_parameter> -- seek to the desired position in the track.

```

Three formats are accepted for seeking, percentage, relative and absolute with durations given in seconds and accepting decimals for greater precision.
Relative and absolute seeking is given in seconds. Examples:
```SEEK +10```,
```SEEK -5.55```,
```SEEK %55```,
```SEEK 22```,

## Todo

* Sane error handeling
* Normalize volume (ReplayGain)
* Crossfade tracks?



