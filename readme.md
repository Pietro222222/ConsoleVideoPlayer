# how it works

```
video.mp4
   |
   | -> Convert To Gif -> Play through Viu
   |
   | -> Convert to mp4 -> Play audio

```

cache: $HOME/.console_player/cache/{video-hash}/
                                    -> video.mp3
                                    -> video.gif
# How to use

./consoleplayer video.mp4


# thanks to
thanks to [yxqsnz](https://github.com/yxqsnz)
   this project was made by him, but i thought i could improve a lot the code, and the algorithm, so i made this one (btw this one works way better lol)


# know issues
unfortunately the audio isnt something good, when it comes to displaying videos on terminal. the audio doesnt play in the moment the video does.

so what do i do? if youre using the kitty terminal emulator, keep it the default, otherwise, change the line 69 in the file src/utils.

```rs
std::thread::sleep(time::Duration::from_millis(2700));
```
then compile it again

# attention

if the video is longer than 30 seconds, this might kill your computer lmao. `viu` is quite dangerous
