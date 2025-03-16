

# BlackBox Background (bb_bg)

First off, please be kind. This is my first bit of code in Rust. I'm totally game for comments but there are somethings I'm not going to pay attention to. Those include...

* Comments on my choice of using snake_case for MY code. I'll use other stuff for an employer, but not here. 
* On style. Yes! I prefer the [Whitesmiths](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) or [Allman](https://en.wikipedia.org/wiki/Indentation_style#Whitesmiths) style over K&R. If you 
don't like that, here is some [help](https://www.therapyden.com/).



## So what is BlackBox Background? 

This is something I originally wrote a long time ago ('02 I think) in PHP. And again in Python and Ruby. What it does is loop over a list of images and resets the desktop background at a specified 
interval. And of course, there are arguments that allow specifying the interval and searching images based on search criteria. 

At the time, I used Blackbox and Windowmaker window managers. They both use the [wmsetbg](https://www.windowmaker.org/docs/manpages/wmsetbg.html) tool for setting the background. Now with this version in 
Rust, it works using Cinnamon, WAS (altered the logic in main) working for Fluxbox, Blackbox, and Windowmaker, and nearly finished for KDE Plasma. 

Something to note here... Cinnamon uses a tool called Nitrogen to set backgrounds. Expensive (LOL), but powerful. It, by itself can do most of what this little application can do. BUT IT IS EXPENSIVE! 
THIS IS A WARNING! Setting low intervals (less than 10 seconds) while on Cinnamon (using Nitrogen) will affect how responsive your system is, or stops being. Over 10 seconds should be fine, but your 
system is likely different than mine so your mileage may vary. I'm running this on a refurb'ed Dell T3600 Workstation with 32gigs and 6 cores. With memory usage as low as 6gb, I'm seeing affects on 
system responsiveness when running intervals less than 10 seconds. So don't do it. You've been warned. :-)

Usage is simple. Here is the help output:

```
Options:
    -h, --help          Print this help menu
    -d, --directories Full path to image directoy. Seperate multiple directories with a comma.
    -c, --cmd Output mechanism... wmsetbg or nitrogen. Nitrogen for Ubuntu. Wmsetbg for Fluxbox, Blackbox, and Windowmaker 
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

As you are are likely pretty smart (since you've read this far), it's prolly obvious that the app will use config file settings unless you specify them as arguments. The only thing not represented in 
the config file is the "-i" argument. With this argument you can specify that images that match a search string will be used. 

>bb_bg -t 15 -i vel

The above command will change the background every 15 seconds and only use images where the name matches (in some way) the search term "vel" (for [Vel Sartha](https://duckduckgo.com/?t=h_&q=vel+sartha&iax=images&ia=images) in this case). 
The match is fuzzy so you don't have to know the exact name of the image or match case. Image titles like VeL_s_01.png will be matched when using "VEL", "vel", or even "vE". "Ve" is likely to match more than 
you were interested in however. 



## So where are we?

Well I'm putting this on github before I wanted to. My intent was to finish the Plasma module,  but it turns out some recruiters don't think you are serious if you don't have a github and code shizzle in it. 

That said, here it is, but just a little ahead of schedule. There are still some issue that I'm listing below. Feel free to consider this a rough analog for a todo list. 

* Main is too big. 
* There is a loop in main that I don't think is needed anymore. The loop itself is moved into the modules. See the nitrogen.rs file (nitrogen::work).
* The wmsetbg and plasma work() methods need writing. 
* Allow deprecation is on for now. Still using rand::Rng::shuffle for now. Plan is to write the work methods for wmsetbg and plasma first.
* The help output needs to be updated to account for the inclusion of KDE Plasma. 
* An installer? 

This currently works with 1.85.0. 



## In closing

I hope this is fun for you. I enjoy it. I initially wrote this because too me it's cool to be spun on coffee and write code in a transparent terminal with the background changing every minute or so. 
Cool music playing, cool code being written, and cool pics of cars, bikes, planes, or peeps cycling in the background. 

Drop me a line! :-)\
Cheers

