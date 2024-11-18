# MP4 Scrubber

This is my tool for prepping MP4s for using on websites. 
It removes the audio and any black bars on the video. I 
set this up because I make a lot of clips in DaVinci 
Resolve and it's a pain to resize the output in it 
when I crop in. 

## Notes

- This is a Rust script that you should be able to 
run from the command line (I'm not sure if you have
to have "nightly" installed anymore or not, but
you might). Do a ``chmod u+x scrub_mp4s.rs`` on
it to make it executable with ``./scurb_mp4s.rs``

- The process starts in the ``source_dir``. Update
it to point to the directory you want to process

- It searches recursively for any ``.mp4`` files
in the ``soruce_dir`` recursively. 

- If it finds an ``.mp4`` and that file is in 
a directory named ``raw`` it checks to see if
there's a corresponding ``.mp4`` with the same
name in a ``cropped`` directory (that's a 
sibling of ``raw``)

That is, if it finds:

~/some/videos/example/raw/FILENAME.mp4

it looks for

~/some/videos/example/cropped/FILENAME.mp4

- If there's no ``cropped/FILENAME.mp4`` file 
it makes it

## Example

A crop for a video might start out like this:

<img width="2032" alt="Screen capture of a video player with the center portion showing an image of a man looking at the camera. He's surrounded by all sides by black bars that are the result of cropping in on the image" src="https://github.com/user-attachments/assets/ba75b678-913c-4954-b3ee-4168e269f058">

After running it through the process, this is the clip:

![example-for-github](https://github.com/user-attachments/assets/55fe6275-f5a7-4cf5-b0c9-57d7b1e606cf)

## Todo

- Create the ``cropped`` directory if it doesn't
already exist




