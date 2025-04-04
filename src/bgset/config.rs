

extern crate dirs;

use crate::bgset::op_args;
use std::collections::HashMap;
use std::fs::File;
use std::fs::exists;
use std::env;
use std::
	{
    io::{prelude::*, BufReader},
    path::Path,
	};
use std::process;
use system::system_output;


const CONFIG_PATH: &str = "/.config/bb_bg/config";

/* Check if the person is running as root AND that a config file exists. */
pub fn can_run(home_dir: &String) -> bool
	{
	/* Quick setup */
	let mut ret_val = true;

	/* Check if this is running as root */
	if(home_dir == "/root")
		{
		println!("It appears you are running as root!\nExiting!");
		process::exit(1);
		}

	/* Now check if the config file exists */
	let fe = exists(home_dir.to_owned()+CONFIG_PATH);
	match(fe)
		{
		Ok(true)	=> ret_val = true,
		Ok(false)	=> ret_val = false,
		Err(..)		=> ret_val = false
		};

	if(ret_val==false)
		{
		println!("\nThere is no config file. Refer to the readme for how to create one.\nExiting!\n");
		process::exit(1);
		return false;
		}

	ret_val
	}


/* This is really a sub-function of read_config. Does as named and returns the file by lines in a vector */
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>
    {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
		.collect()
    }


pub fn read_config(ops: op_args, conf_store: &mut HashMap<String, String>)
	{
    /* Let's generate a string rep of our configuration file */
    let conf_tail                           = "/bb_bg/config";
    let dirs_act1                           = dirs::config_dir().expect("Error: Failed to open the home directory!!\n");	// Assumes ~/.config
    let dirs_act2: String                   = dirs_act1.to_str().unwrap().to_string();										// Converts findings above to String
    let conf_path: String                   = dirs_act2+conf_tail;

    /* Let's now open the file, iterate, and assign pertinent values */
    let lines_ref	= lines_from_file(conf_path);   // Get each line from the config file as an entry in a vector
	let lines 		= lines_ref.clone();
    for l in &lines
        {
        if(l.contains("//") || l.chars().count()==0) { /* Do nothing */ }	/* Yes, I could do this another way, but this feels explicit. */
        else
            {
            if(l.contains(":"))
                {
				let mut b2: Vec<&str>		= l.split(":").collect::<Vec<&str>>();
				/* I know the correct config format should only result in two elements. If more, I'll ignore the third + element(s). */
				/* Parse the string and return it to it's own value */
				let mut b2_00: String = b2[0].replace("\t", "").trim().to_string();
				let mut b2_01: String = b2[1].replace("\t", "").trim().to_string();

				// println!("{} -> {}", b2_00, b2_01);
		
				/* Now push those good strings into the HashMap */
				conf_store.insert(String::from(b2_00), String::from(b2_01));
                }
            }
        }

	}


pub fn check_command(cmd_name: &String) -> bool
	{
	/* Straight into it */
	let cmd 	= "whereis ".to_string()+cmd_name;
	let out 	= system_output(&cmd).expect("Failed to run whereis command!");
	let so_res	= String::from_utf8_lossy(&out.stdout);
	let boom	= so_res.split(" ").collect::<Vec<&str>>();

	if(boom.len() >= 2)	{ return true; }
	else
		{
		println!("It appears there may be a problem. Are you sure {} exists on your system?", cmd_name);
		process::exit(0);
		}
	}


// pub fn filter_dir_str(dir_str: &String) -> String {}

