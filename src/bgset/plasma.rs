

#![allow(deprecated)]

/* For executing commands */
extern crate rand;                  // For shuffling the image vector
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::process;
use std::process::{Command};
use crate::bgset::bgset_args;


pub fn work(img_list: &mut Vec<String>, bginfo: &mut bgset_args) 
	{
    /* Setup */
    let mut ciel:   i32             = img_list.len() as i32;                        // Oft compared against below
    let mut x:      i32             = 0;
    let mut rng                     = rand::thread_rng();
    let mut pushed: String          = "".to_string();                               // Current string iteration to be pushed onto the collector
    let mut reached: i8             = 0;                                            // Simple flag to let us know we've reached the end ... i think.
	let mut lcl_rebuild: i8			= 0;

    loop
        {
        /* Have we hit the limit? */
		if(lcl_rebuild == 1)
			{ return; }
        else if(x >= ciel)                                                          // We are either going to shuffle here or return and rebuild
            {
            if(bginfo.rebuild==1)   { lcl_rebuild = 1; }
            else                    { rng.shuffle(&mut *img_list); }
            x=0;
            }
        else
            {
            pushed = img_list.get(x as usize).unwrap().to_string();                 // Grab an item from the vector. We store it as we may need to push it again.
            bginfo.img_path = pushed.clone();                                       // Push the image string into the struct element
            x += 1;                                                                 // ... and increment.

            /* The background get's set here */
            set_bkg(bginfo);

            /* Sleep and cleanup */
            bginfo.img_paths.clear();                                               // Empty the list (vector) for the next iteration
            thread::sleep(Duration::from_secs((bginfo.interval as u32).into()));
            }
        }
	}



/* The background is actually set here. */
pub fn set_bkg(bginfo: &bgset_args)
    {
// qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript 'var allDesktops = desktops(); for (i=0;i<allDesktops.length;i++) { d = allDesktops[i]; d.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"]; d.writeConfig("Image", "file:///path/to/wallpaper"); }'

    let mut output = Command::new("ksetwallpaper");
    output.arg("--file");
    output.arg(&bginfo.img_path);

	/* Conditional debug output */
	// println!("{:?}", bginfo);
	println!("{:?}\n", output);
	// process::exit(1);

	if(bginfo.show_debug==1) { println!("{:?}", output); }
    match output.output()
		{
       	Ok(o) => { unsafe { println!("{}", String::from_utf8_unchecked(o.stdout)); } },
        Err(e)=> { println!("Somethings gone wrong in bgset::plasma! Error message is...\n\t \"{}\"", e); }
   	    }
	}

