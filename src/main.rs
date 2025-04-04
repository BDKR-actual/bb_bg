/* ********************************************************
This is my first attempt to write someting OTHER than
examples. What this does is look at a directory full of images
and loops over the contents as such setting each as a background
for some period of time.

There are some external dependencies that are
included but not yet used. Uncommenting "![allow(unsused)]" will
let you see what's extra.
******************************************************** */
#![allow(unused)]
#![allow(deprecated)]
#![warn(non_camel_case_types)]

/* Std lib and Crates */
extern crate rand;              	// For shuffling the image vector
use anyhow::{Context, Result};		// Let's leave this since I plan to use it later
use getopts::Options;
use crate::rand::Rng;
use std::collections::HashMap;
use std::env;
use std::process;

/* Modules */
use bb_bg::bgset::bgset_args;
use bb_bg::bgset::op_args;
use bb_bg::bgset::nitrogen;			// For Cinnamon
use bb_bg::bgset::wmsetbg;			// This supports Windowmake and Black Box
use bb_bg::bgset::plasma;			// For use with KDE Plash
use bb_bg::bgset::img_scan;     	// Scan the images. Make sure they are. Filter based on search term if requested.
use bb_bg::bgset::config;       	// Config related shizzle. The big one being that this is where the config file is read.
use bb_bg::bgutils::commands;		// 
use bb_bg::bgutils::utils;			// 

/* Some constants to setup */
const DEV_DEBUG: 	i8 			= 0;
const HD_ERR:		&str		="Number of heads (monitors) missing or malformed. Check the config file!";
const DBG_ERR: 		&str		="The debug entry is either missing or malformed. Check the config file!";


fn main() -> Result<(), anyhow::Error>
	{
	/* Setup */
    let home_dir		 						= envmnt::get_or_panic("HOME".to_string());
	let mut fnl_img_dir 						= "".to_string();	    // Setup the image directory string
	let mut fnl_cmd								= "".to_string();	    // What command will be used later
	let mut opt_data	 						= match_args();		    // Let's look at the command line arguments
	let mut imgs_otr: Vec<String>				= vec![];				// Images are stored here
	let mut rng_otr 							= rand::thread_rng();
	let mut otr_cntr: u32						= 0;
	let mut conf_data: HashMap<String, String> 	= HashMap::new();		// Configuration data
	let mut bg_args = bb_bg::bgset::bgset_args							// Setup the bg_args dataset 
		{
		heads:		0,					rebuild:	1,
		img_path:	"".to_string(),		img_paths:	vec![],
        show_debug: 0,					interval:	0
		};

	/* Get some real work done */
	bb_bg::bgset::config::can_run(&home_dir);																	// Is the config file there? Someone running this as root?
    bb_bg::bgset::config::read_config(opt_data.clone(), &mut conf_data);										// Read the config file 
	let comm_list		= bb_bg::bgutils::commands::build_comm_strings(&opt_data, &conf_data);					// Generate our final dir and command strings and the interval value 
	fnl_img_dir 		= comm_list[0].clone();																	// This and the next two lines finalize some commands and parameters
	fnl_cmd				= comm_list[1].clone();
	bg_args.interval	= comm_list[2].clone().parse().unwrap();
	bg_args.heads		= conf_data.get("HEADS").clone().expect(&HD_ERR).to_string().parse().unwrap();			// Number of heads (monitors).
	bg_args.show_debug	= conf_data.get("SHOW_DBG").clone().expect(&DBG_ERR).to_string().parse().unwrap();		// Debug output?
	bb_bg::bgset::config::check_command(&fnl_cmd);																// Check if we have the command line tools we need. Exit if we don't.

    /* Load and filter images then set them to screen(s) */
	loop
		{
		/* Let's bring something things inside of the scope for this loop */
		let loop_cntr	= &mut otr_cntr;
		let imgs_innr 	= &mut imgs_otr;
		let rng 		= &mut rng_otr;

		/* Image load and filter stuff */
		img_scan::load_images(&fnl_img_dir, imgs_innr, &home_dir, &bg_args);			/* Read the directories and shove the images into the imgs_innr vector */
		let mut imgs = img_scan::filter_images(imgs_innr.to_vec(), &opt_data);			/* Now filter and shuffle the vector */ 
		rng.shuffle(&mut imgs);

        /* Show memory output? */
        if (DEV_DEBUG==1) 	{ bb_bg::bgutils::utils::print_memory_usage(); }

		/* Hand off the data set and arguments to the module responsible for putting images on the desktop */		
		match fnl_cmd.as_str()
			{
			"nitrogen"		=> { bb_bg::bgset::nitrogen::work(&mut imgs,&mut bg_args) },
			"plasma"		=> { bb_bg::bgset::plasma::work(&mut imgs,  &mut bg_args) },		
			"wmsetbg"		=> { bb_bg::bgset::wmsetbg::work(&mut imgs, &mut bg_args) },
			 _ => println!("Shouldn't be here!"),
			}

		/* If we find ourselves back out here, we are looking to rebuild the image vectors. Empty them first! */
		imgs_innr.clear();
		imgs.clear();
		*loop_cntr += 1;
		}
	}


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
    opts.optopt("c", "cmd", "", "Output mechanism... Nitrogen for Cinnamon. Wmsetbg for Fluxbox, Blackbox, and Windowmaker. Plasma for KDE Plasma. All lower case!");
    opts.optopt("t", "time", "", "This is the interval between image changes. 9999 is the max.");
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
            None => 45.to_string(),
            } as String;
		let li_len			= local_interval.len();
	
		/* Did we really get an interval? Are we pulling it from the config file? Or setting a default? */
		if (local_interval != 0.to_string())
			{
			if( li_len < 5 && bb_bg::bgutils::utils::is_integer(&local_interval) )
    	        { opt_data.interval = local_interval.parse::<u32>().unwrap(); }
        	else
            	{
	            println!("Problematic value entered for the time argument. Using the default interval of 90 seconds.");
    	        opt_data.interval = 90;
        	    }
			}
		else
			{
			dbg!(&local_interval);
			process::exit(0);
			}

		if(DEV_DEBUG==1) 	{ dbg!(&local_interval); }
        }


    /* */
    let input = if !matches.free.is_empty()
        { matches.free[0].clone() }
    else
        { return opt_data; };

    /* Return the struct to be used else where */
    opt_data
    }

