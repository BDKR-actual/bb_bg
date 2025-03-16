/* ********************************************************
This is my first attempt to write someting OTHER than
examples. What this does is look at a directory full of images
and loops over the contents as such setting each as a background
for some period of time.

In the version written in PHP, things like the directory, using
images that match a search criteria, and the interval set are all
configurable and/or can be passed in as arguments. Here, those
things may come later as I learn more/get better with Rust.

To that end, there are some external dependencies that are
included but not yet used. Uncommenting "![allow(unsused)]" will
let you see what's extra.
******************************************************** */
#![allow(unused)]
// #![allow(deprecated)]
#![warn(non_camel_case_types)]

extern crate envmnt;
extern crate getopts;
extern crate rand;              // For shuffling the image vector

use crate::rand::Rng;
use std::slice::Iter;
use getopts::Options;
use memory_stats::memory_stats;
use std::char;
use std::collections::HashMap;
use std::env;
use std::fs;					// For reading directories
use std::fmt;
use std::fmt::Display;			// For pretty printing debug output
use std::process::{Command};	// For executing commands
use std::time::Duration;		// For pausing the application / sleep
use std::io::{Write, stderr};
use std::iter;
use std::iter::Enumerate;
use std::thread;
use std::thread::sleep;
use std::process;
use std::ptr::{null, null_mut};
// use std::rc::Rc;
use std::str::Split;            // Used in config::check_command
use system::system_output;      // Used in config::check_command
use libc::{c_char, c_void};

/* Modules */
use bb_bg::bgset::bgset_args;
use bb_bg::bgset::op_args;
use bb_bg::bgset::nitrogen;		// For Cinnamon
use bb_bg::bgset::wmsetbg;		// This supports Windowmake and Black Box
use bb_bg::bgset::plasma;		// For use with KDE Plash
use bb_bg::bgset::img_scan;     // Scan the images. Make sure they are. Filter based on search term if requested.
use bb_bg::bgset::config;       // Config related shizzle. The big one being that this is where the config file is read.

const DEV_DEBUG: i8 = 1;



fn main()
	{
	/* Setup */
    let home_dir		 						= envmnt::get_or_panic("HOME".to_string());
	let mut fnl_img_dir 						= "".to_string();	    // Setup the image directory string
	let mut fnl_cmd								= "".to_string();	    // What command will be used later
	let mut opt_data	 						= match_args();		    // Let's look at the command line arguments
	let mut imgs_otr: Vec<String>				= vec![];				// vec![] is a macro! Learn about it! Images are stored here
	let mut rng_otr 							= rand::thread_rng();
	let mut otr_cntr: u32						= 0;
	let mut conf_data: HashMap<String, String> 	= HashMap::new();// I guess 'into()' converts HashMap::new into the targeted Rc type
	let tmp_i: u8								= 0;
	let mut conf_interval: u32					= 0;
	// let mut rng									= rand::rng();

	/* Some expect error string to have setup already */
	let img_dir_err: String						="Expecting a string representing one or more image directories.".to_string();
	let bg_ex_err: String						="Error getting the name of the process that sets the background image. Check the config file!".to_string();
	let intvl_err: String						="Interval data is either not alpha-numeric or missing. Check the config file!".to_string();
	let hd_err:	String							="Number of heads (monitors) missing or malformed. Check the config file!".to_string();
	let dbg_err: String							="The debug entry is either missing or malformed. Check the config file!".to_string();

	// let mut bg_args = &mut bb_bg::bgset::bgset_args
	let mut bg_args = bb_bg::bgset::bgset_args
		{
		heads:		0,
		img_path:	"".to_string(),
		img_paths:	vec![],
        show_debug: 0,
		interval:	0
		};

	/* Now let's read the config file and mess around with the interval element */
    bb_bg::bgset::config::read_config(opt_data.clone(), &mut conf_data);
	conf_interval					= conf_data.get("INTERVAL").clone().expect(&intvl_err).to_string().parse().unwrap();

	/* Check if an image directory was passed at the command line */
	if(opt_data.directory.len()>0)	{ fnl_img_dir = opt_data.directory.clone(); }
	else							{ fnl_img_dir = conf_data.get("DEF_IMG_DIR").expect(&img_dir_err).to_string(); }

	/* Let's do the same as above for commands */
	if(opt_data.cmd.len()>0)		{ fnl_cmd = opt_data.cmd.clone(); }
	else							{ fnl_cmd = conf_data.get("BG_EX").expect(&bg_ex_err).to_string(); }

	/* Check the interval */
	if(opt_data.interval==0 && conf_interval > 0)
		{
		opt_data.interval			= conf_interval;																		// Should go away once all modules are changed.
		bg_args.interval			= conf_interval;																		// This is the newer way. 
		}
	else
		{ bg_args.interval			= opt_data.interval; }

	/* Yet more */
	bg_args.heads					= conf_data.get("HEADS").clone().expect(&hd_err).to_string().parse().unwrap();			// Number of heads (monitors).
	bg_args.show_debug				= conf_data.get("SHOW_DBG").clone().expect(&dbg_err).to_string().parse().unwrap();		// Debug output?

	/* Before going any further, let's check if we have the command line tools we need */
    /* The program will exit with a message in this function if there is a problem */
	bb_bg::bgset::config::check_command(&fnl_cmd);

	/* If there is a need */
	if(bg_args.show_debug==1)
		{
		println!("---------------------------------------------------------------------------------");
		dbg!(&bg_args.heads);
		println!("\n");
		dbg!(&conf_data);
		println!("---------------------------------------------------------------------------------\n\n");
		}

    /* Now get to work for real */
	loop
		{
		/* Let's bring something things inside of the scope for this loop */
		let loop_cntr	= &mut otr_cntr;
		let imgs_innr 	= &mut imgs_otr;			// Borrowed form line 57 above by referrence
		let rng 		= &mut rng_otr;				// Just like the line above but from line 58

		/* Check if we got more than one directory */
		if fnl_img_dir.contains(",")
			{
			for ind_dir in (fnl_img_dir.split(","))
				{
				let mut final_dir_str = "".to_string();	// This is the directory strng used by read_dir().

				/* Look for the tilde */
                if( ind_dir.contains("~") )	{ final_dir_str = ind_dir.replace("~", &home_dir); }
				else						{ final_dir_str = ind_dir.to_string(); }

				/* Push images into the vector */
			    for multi_path in (fs::read_dir(final_dir_str).expect("\nERROR:Unable to read (one of) the direct(y/ies). Are you sure the directory exists?\n\n"))
					{
	        		let cur_str 		= &String::from(multi_path.expect("\nError:There may be an issue with the dir string!\n\n").path().display().to_string());
					if(bg_args.show_debug==1)	{ println!("The dir string is {}", cur_str); }
					imgs_innr.push(cur_str.to_string());
					}
				}
			}
		else
			{
			/* Shove the images into a vector */
		    for path in (fs::read_dir(fnl_img_dir.clone()).expect("\nERROR: Problem encountered while reading from directory.\n\n"))	// <-- Iter for dir to the loop
				{
	        	let cur_str 		= String::from(path.unwrap().path().display().to_string());
			    imgs_innr.push(cur_str);
				}
			}

		/* Now filter and shuffle the vector and start looping, re-setting the background at each iteration */
		let mut imgs = img_scan::filter_images(imgs_innr.to_vec(), &opt_data);
		rng.shuffle(&mut imgs);

        /* Show memory output? */
        if (bg_args.show_debug==1)
            {
            if let Some(usage) = memory_stats()
                {
                println!("Current physical memory usage: {}", usage.physical_mem);
                println!("Current virtual memory usage: {}", usage.virtual_mem);
                }
            else
                { println!("Couldn't get the current memory usage :("); }
            }

		/* Hand off the data set and arguments to the module */		
		match &fnl_cmd
			{
			nitrogen	=> { bb_bg::bgset::nitrogen::work(&mut imgs, &mut bg_args) },
			wmsetbg		=> { bb_bg::bgset::wmsetbg::set_bkg(&bg_args) },				// Needs finishing and testing
			plasma		=> { bb_bg::bgset::plasma::set_bkg(&bg_args) }					// Needs finishing and testing
			}

		/* Our interval : This is now taken care of in the modules */
		// thread::sleep(Duration::from_secs(opt_data.interval as u64));
		// imgs_innr.clear();				// Empty the vector for rebuilding.

		*loop_cntr += 1;				// This is de-referenced. 
		}
	}


// fn print_usage(program: &str, opts: Options)
fn print_usage(opts: Options)
    {
    let brief = format!("Usage: ");
    print!("{}", opts.usage(&brief));
    println!("\n");
	process::exit(1);
    }


/* Using getopts, check the arguments and shove those into a struct */
fn match_args() -> op_args
    {
	// let mut ltr_arr = "h,d,c,t,i".split(',');
	let _ltr_arr = "h,d,c,t,i".split(',');
    let mut opt_data = op_args
        {
        directory:  String::from(""),
        cmd:        String::from(""),
        image:      String::from(""),
        interval:   0,
        };

    /* Let's first read the command arguments */
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();

	/* Current methodology */
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("d", "directories", "", "Full path to image directoy. Seperate multiple directories with a comma.");
    opts.optopt("c", "cmd", "", "Output mechanism... wmsetbg or nitrogen. Nitrogen for Ubuntu. Wmsetbg for Fluxbox, Blackbox, and Windowmaker");
    opts.optopt("t", "time", "", "This is the interval between image changes");
    opts.optopt("i", "images", "", "Search for images matching name provided");

    let matches = match opts.parse(&args[1..])
        {
        Ok(m) => { m },
        Err(f) => { panic!("{}", f.to_string()) }
        };

    /* Check if the user is just looking for work */
    if matches.opt_present("h")
        {
        print_usage(opts);
        return opt_data;
        }

    /* What command are we using? */
    if matches.opt_present("c")
        {
        opt_data.cmd = match matches.opt_str("c") {
            Some(s) => s,
            None => String::from("default"),
            };
        }

    /* Look for directories */
    if matches.opt_present("d")
        {
        opt_data.directory = match matches.opt_str("d") {
            Some(s) => s,
            None => ("".to_string()),
            };
        }

    /* Any images we are interested in? */
    if matches.opt_present("i")
        {
        opt_data.image = match matches.opt_str("i") {
            Some(s) => s,
            None => String::from("na"),
            };
        }

    /* The interval */
    if matches.opt_present("t")
        {
        let mut local_interval = match matches.opt_str("t") {
            Some(s) => s,
            None => 0.to_string(),
            } as String;

		if(DEV_DEBUG==1) 
			{ 
			dbg!(&local_interval);
			}

		if (local_interval != 0.to_string())
			{
	        if(local_interval.chars().all(char::is_alphanumeric))
    	        { opt_data.interval = local_interval.parse::<u32>().unwrap(); }
        	else
            	{
	            println!("Non-numeric value entered for the time argument. Using the default interval of 90 seconds.");
    	        opt_data.interval = 90;
        	    }
			}
		else
			{
			dbg!(&local_interval);
			process::exit(0);
			}

        }

    /* */
    let input = if !matches.free.is_empty()
        { matches.free[0].clone() }
    else
        { return opt_data; };

    /* Return the struct to be used else where */
    opt_data
    }



fn print_type_of<T>(_: &T)
	{
    println!("{}", std::any::type_name::<T>());
	}
