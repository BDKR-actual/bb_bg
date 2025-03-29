
#![allow(deprecated)]


extern crate rand;              	// For shuffling the image vector
use rand::Rng;
use std::process::{Command};
use std::time::Duration;        	// For pausing the application
use std::thread;
use crate::bgset::bgset_args;



/* Where the real work is done. The actually setting of the bkg image will be done in the set_bkg method called from here */
pub fn work(img_list: &mut Vec<String>, bginfo: &mut bgset_args)
	{
	/* Setup */
    let mut ciel: 	i32				= img_list.len() as i32;						// Oft compared against below
    let mut x:		i32				= 0;
    let heads: 		u8				= bginfo.heads;
	let mut rng						= rand::thread_rng();
    let mut pushed: String  		= "".to_string();								// Current string iteration to be pushed onto the collector
    let mut reached: i8         	= 0;											// Simple flag to let us know we've reached the end ... i think.
    let mut lcl_rebuild: i8         = 0;

    loop
        {
        /* Have we hit the limit? */
        if(x >= ciel)																// Let's test this more later. Not sure it's every being reached. 
			{
			reached = 1;
			break;
			}

        loop
            {
            for i in 0..heads
                {
                pushed = img_list.get(x as usize).unwrap().to_string();             // Grab an item from the vector. We store it as we may need to push it again.
				bginfo.img_paths.push( pushed.clone() );							// Push the image string into the struct element
                x += 1;                                                              // ... and increment.

                // Are we at the end of the list? If we hit the end of the list here, we are likely to end up with an image in the vector twice.
                if(x >= ciel)
                    {
					reached = 1;
                    x = 0;                                                                  // Reset the counter
                    if( (bginfo.img_paths.len() as u8) < heads)  							// This will come be used under certain circumstances
						{ bginfo.img_paths.push( pushed.clone() ); }
                    break;
                    }
                }

			/* The background get's set here */
			set_bkg(bginfo);

			/* Sleep and cleanup */
            bginfo.img_paths.clear();														// Empty the list (vector) for the next iteration
            thread::sleep(Duration::from_secs((bginfo.interval as u32).into()));
            if (reached == 1)																// If we are at the end of the list
                {
                /* The img_list var below has 2 levels of indirection (double references) before getting here. We need to
                dereference at least level to satify the shuffle method argument requirements. */
				rng.shuffle(&mut *img_list);
				reached = 0;
				break;
				}
            }

        }

	}

/* The background is actually set here. */
pub fn set_bkg(bginfo: &mut bgset_args)
    {
    if(bginfo.heads==1)
        {
        let mut output = Command::new("nitrogen");
        output.arg("--set-zoom");
        output.arg("--head=0");
        output.arg(&bginfo.img_path);

		/* Debug output */
		if(bginfo.show_debug==1)	{ println!("{:?}", output); }

		/* Run it! */
        match output.output()
   	        {
       	    Ok(o) => { /* unsafe { println!("{}", String::from_utf8_unchecked(o.stdout)); } */ },
            Err(e)=> { println!("Somethings gone wrong in bgset::nitrogen! Error message is...\n\t \"{}\"", e); }
   	        }
        }
    else
        {
        for hd in 0..bginfo.heads
            {
            /* Generate the head portion of the argument */
            let mut head_str: String = "--head=".to_string();
            head_str.push_str(&hd.to_string());

            /*  Build and run the command */
            let mut output = Command::new("nitrogen");
            output.arg("--set-zoom");
            output.arg(head_str);
			if(&bginfo.img_paths.len()>&0)	{ output.arg(&bginfo.img_paths.pop().unwrap()); }
			else				            { output.arg(&bginfo.img_path); }

			/* Debug output */
			if(bginfo.show_debug==1)		{ println!("{:?}", output); }

			/* Run the command */
            match output.output()
   	            {
       	        Ok(o) => { /* unsafe { println!("{}", String::from_utf8_unchecked(o.stdout)); } */ },
           	    Err(e)=> { println!("Somethings gone wrong in bgset::nitrogen! Error message is...\n\t \"{}\"", e); }
               	}
            }

        }
    }
