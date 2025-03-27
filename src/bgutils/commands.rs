
// use anyhow::{Context, Result};
use crate::bgset::op_args;
use std::collections::HashMap;
use std::process;

/* Some constants to setup */
const DEV_DEBUG:    i8          = 1;
const IMG_DIR_ERR:  &str        ="Expecting a string representing one or more image directories.";
const BG_EX_ERR:    &str        ="Error getting the name of the process that sets the background image. Check the config file!";
const INTVL_ERR:    &str        ="Interval data is either not alpha-numeric or missing. Check the config file!";
const HD_ERR:       &str        ="Number of heads (monitors) missing or malformed. Check the config file!";
const DBG_ERR:      &str        ="The debug entry is either missing or malformed. Check the config file!";


// pub fn build_comm_strings(opt_data: &op_args, conf_data: &HashMap<String, String>, comm_list: &mut Vec<&str>)
pub fn build_comm_strings(opt_data: &op_args, conf_data: &HashMap<String, String>) -> Vec<String>
	{
	/* Setup */
	let mut comm_list: Vec<String> = vec![];	
	let local_dir					= opt_data.directory.clone();
	let local_comm					= opt_data.cmd.clone();
	let mut conf_interval: u32		= 0;
	let mut ci_string: String		= "".to_string(); 

    /* Check if an image directory was passed at the command line */
    if(opt_data.directory.len()>0)  { comm_list.push( local_dir.clone() ); }
    else                            { comm_list.push( conf_data.get("DEF_IMG_DIR").expect(&IMG_DIR_ERR).to_string() ); }

    /* Let's do the same as above for commands */
    if(opt_data.cmd.len()>0)        { comm_list.push( local_comm.clone() ); }
    else                            { comm_list.push( conf_data.get("BG_EX").expect(&BG_EX_ERR).to_string() ); }

	/* Get the interval data */
    conf_interval                   = conf_data.get("INTERVAL").clone().expect(&INTVL_ERR).to_string().parse().unwrap();
    if(opt_data.interval==0 && conf_interval > 0)   { comm_list.push( conf_interval.to_string() ); }
    else                                            { comm_list.push( opt_data.interval.to_string() ); }

	/* Return the goods */
	comm_list
	}
















