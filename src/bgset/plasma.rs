
/* For executing commands */
use std::process::{Command};
use crate::bgset::bgset_args;


pub fn work() {}

/* The background is actually set here. */
pub fn set_bkg(bginfo: &bgset_args)
    {
    let mut output = Command::new("ksetwallpaper");
    output.arg("--file");
    output.arg(&bginfo.img_path);

	/* Conditional debug output */
	if(bginfo.show_debug==1) { println!("{:?}", output); }
    match output.output()
		{
       	Ok(o) => { unsafe { println!("{}", String::from_utf8_unchecked(o.stdout)); } },
        Err(e)=> { println!("Somethings gone wrong in bgset::plasma! Error message is...\n\t \"{}\"", e); }
   	    }
	}

