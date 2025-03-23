

# BlackBox Background (bb_bg)

First off, please be kind. This is my first bit of code in Rust. I'm totally game for comments but there are somethings I'm not going to pay attention to. Those include...

* Comments on my choice of using snake_case for MY code. I'll use other stuff for an employer, but not here. 
* On style. Yes! I prefer the [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) or [Allman](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) style over K&R. If you 
don't like that, here is some [help](https://www.therapyden.com/).


## So what is BlackBox Background? 

This is something I originally wrote a long time ago ('02 I think) in PHP. And again in Python and Ruby. What it does is loop over a list of images and resets the desktop background at a specified 
interval. And of course, there are arguments that allow specifying the interval and searching images based on search criteria. 

At the time, I used Blackbox and Windowmaker window managers. They both use the [wmsetbg](https://www.windowmaker.org/docs/manpages/wmsetbg.html) tool for setting the background. Now with this version in 
Rust, it works using Cinnamon, Fluxbox, Blackbox, Windowmaker, and KDE Plasma. 

Something to note here... Cinnamon uses a tool called Nitrogen to set backgrounds. Expensive (LOL), but powerful. It, by itself can do most of what this little application can do. BUT IT IS EXPENSIVE! 
THIS IS A WARNING! Setting low intervals (less than 10 seconds) while on Cinnamon (using Nitrogen) will affect how responsive your system is, or stops being. Over 10 seconds should be fine, but your 
system is likely different than mine so your mileage may vary. I'm running this on a refurb'ed Dell T3600 Workstation with 32gigs and 6 cores. With memory usage as low as 6gb, I'm seeing affects on 
system responsiveness when running intervals less than 10 seconds. So don't do it. You've been warned. :-)

Usage is simple. Here is the help output:

```
Options:
    -h, --help          Print this help menu
    -d, --directories Full path to image directoy. Seperate multiple directories with a comma.
    -c, --cmd Output mechanism... Nitrogen for Cinnamon. Wmsetbg for Fluxbox, Blackbox, and Windowmaker. Plasma for KDE Plasma. All lower case!
    -t, --time This is the interval between image changes
    -i, --images Search for images matching name provided
```

There is a config file. It should live in /home/you/.config/bb_bg/ and be named config. An example:

```
DEF_IMG_DIR :   ~/Downloads/images,~/Downloads/images/arcane/act_2
BG_EX       :   nitrogen
HEADS       :   2
INTERVAL    :   90
SHOW_DBG    :   0
```

Yes, you need to create the file and store it at the location mentioned above. You could copy the example above as the contents of the file. There is also a 
config.example file that can be saved to the target directory. Note the "~" at the start of those paths. Those are not required and probably better not used if you don't 
understand what they are. The last thing concerning the directory entries is that there should be no spaces around the commas. 

BG_EX is important. The options are nitrogen, wmsetbg, and plasma. Not too interested in doing any others but we can add the functionality here if you write it. 

If the config file isn't present, the application will exit. 

As you are are likely pretty smart (since you've read this far), it's prolly obvious that the app will use config file settings unless you specify them as arguments. The 
only thing not represented in the config file is the "-i" argument. With this argument you can specify that images that match a search string will be used. 

>bb_bg -t 15 -i vel

The above command will change the background every 15 seconds and only use images where the name matches (in some way) the search term "vel" (for [Vel Sartha](https://duckduckgo.com/?t=h_&q=vel+sartha&iax=images&ia=images) in this case). 
The match is fuzzy so you don't have to know the exact name of the image or match case. Image titles like VeL_s_01.png will be matched when using "VEL", "vel", or even "vE". "Ve" is likely to match more than 
you were interested in however. 


## So where are we?

There are still some issues/updates that I'm listing below. Feel free to consider this a rough analog for a todo list. 

* Main is too big. 
* In the last release, the need for the loop in main was in question. That's sorted now. It remains as it supports automatic inclusion of new images that may have been added while the prog was running. When the module exhausts it's list of images, it breaks back out to main where the new image will be found. 
* The Windowmaker, BlackBox, FluxBox, and KDE Plasma window managers are now supported / working. 
* Allow deprecation is on for now. Still using rand::Rng::shuffle for now. Plan is to write the work methods for wmsetbg and plasma first.
* If you don't want to compile this yourself, I've included a .deb file. 

This currently works with 1.85.0. 

To install via the Debian package file, the command is...

```dpkg -i bb_bg*.deb```

I've not tested this across a lot of platforms so I would appreciate feedback / error output. 


## Dependencies

This program actually requires the services of some other scripts to effectuate the desktop background changes. For the supported window managers those are...

* wmsetbg:		Choose this one if using BlackBox, FluxBox, or Window Maker. [Download here](https://www.windowmaker.org/docs/manpages/wmsetbg.html) in case it's not included when installing one of these window mangers, but that shouldn't be the case.
* nitrogen:		If you are Cinnamon, this is your choice. Take note of the "heads" option in the config file. This is the number of monitors you have. Setting this to that number will result in using a different image on each monitor. Set to 1 if you don't want this. Also, I only tested this on my system with two monitors. If you have more than 2 and see some strange behavior drop me a note and let me know about it. 
* plasma:		This is the option to choose if using KDE Plasma. But this one has a surprise. The backend tool that's actually being used is called [ksetwallpaper](https://github.com/pashazz/ksetwallpaper). The surprise is that it doesn't appear to be included (in my distro at least). Download it and install it in /usr/local/bin as "ksetwallpaper". Remove the ".py" from the end. Make sure it's executable and has it's [bam line or shebang line](https://en.wikipedia.org/wiki/Shebang_(Unix)). 

The above are the options that you would enter either at the command line via -c or what's set for the BG_EX option in the config file. 


## In closing

I hope this is fun for you. I enjoy it. I initially wrote this because too me it's cool to be spun on coffee and write code in a transparent terminal with the background changing every minute or so. 
Cool music playing, cool code being written, and cool pics of cars, bikes, planes, or peeps cycling in the background. 

Drop me a line! :-)\
Cheers

