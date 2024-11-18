# MP4 Scrubber

This is my tool for prepping MP4s for using on websites. 
It removes the audio and any black bars on the video. I 
set this up because I make a lot of clips in DaVinci 
Resolve and it's a pain to resize the output in it 
when I crop in. 

## Notes

- The process starts in the ``source_dir``

- It searches recursively for any ``.mp4`` files

- If it finds an ``.mp4`` and that file is in 
a directory named ``raw`` it checks to see if
there's a corresponding ``.mp4`` with the same
name in a ``cropped`` directory (that's a 
sibling of ``raw``)

- If there's no ``cropped/FILENAME.mp4`` file 
it makes it

## Example

A crop for a video might start out like this:

<img width="2032" alt="starting-with-black-bar" src="https://github.com/user-attachments/assets/ba75b678-913c-4954-b3ee-4168e269f058">

After running it through the process, this is the clip:

![example-for-github](https://github.com/user-attachments/assets/55fe6275-f5a7-4cf5-b0c9-57d7b1e606cf)

## Todo

- Create the ``cropped`` directory if it doesn't
already exist




