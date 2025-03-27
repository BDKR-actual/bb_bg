

use crate::bgset;
use std::collections::HashMap;
use std::fs;
use std::process;
use crate::bgset::op_args;


/* Take the passed image directory, home directory, target vector, and bgset args then read the contained images into the vector. */ 
pub fn load_images( fnl_img_dir: &String, imgs_innr: &mut Vec<String>, home_dir: &String, bg_args: &bgset::bgset_args )
	{
	/* Check if we got more than one directory */
	if fnl_img_dir.contains(",")
		{
		for ind_dir in (fnl_img_dir.split(","))
        	{
			let mut final_dir_str = "".to_string(); // This is the directory strng used by read_dir().

			/* Look for the tilde */
			if( ind_dir.contains("~") ) { final_dir_str = ind_dir.replace("~", &home_dir); }
			else                        { final_dir_str = ind_dir.to_string(); }

			/* Push images into the vector */
			for multi_path in (fs::read_dir(final_dir_str).expect("\nERROR:Unable to read (one of) the direct(y/ies). Are you sure the directory exists?\n\n"))
				{
				let cur_str         = &String::from(multi_path.expect("\nError:There may be an issue with the dir string!\n\n").path().display().to_string());
				if(bg_args.show_debug==1)   { println!("The dir string is {}", cur_str); }
				imgs_innr.push(cur_str.to_string());
				}
			}
		}
	else
		{
		/* Shove the images into a vector */
		for path in (fs::read_dir(fnl_img_dir.clone()).expect("\nERROR: Problem encountered while reading from directory.\n\n"))    // <-- Iter for dir to the loop
			{
			let cur_str         = String::from(path.unwrap().path().display().to_string());
			imgs_innr.push(cur_str);
			}
		}
	}


/* 
Super simplistic check that files from provided directories are images. 
If the -i command line argument was used, this will also filter the list searching 
each image against the provided string.
*/
pub fn filter_images(raw_list: Vec<String>, opt_data: &op_args) -> Vec<String>
	{
	/* Create a map of strings that we'll use to compare to each file that's read from a directory */
	let mut img_types: HashMap<&str, String>	= HashMap::new();
	let to_search								=[".jpg", ".jpeg", ".png", ".gif"];	
	let ts_len 	 								=to_search.len();
	let mut selected_files: Vec<String> 		= vec![];

	/* Loop over list of files and make sure all are images */
	for ti in &raw_list
		{			
		for x in 0..ts_len
			{
			if(ti.contains(to_search[x]))
				{ 
				let lc_ti = ti.to_lowercase();

				/* If an image search string was included, look for it here. */
				if(opt_data.image.len()>0)
					{
					/* This condition checks for the case where mulitple search terms are entered */
					if(opt_data.image.contains(','))
						{
						for sp in opt_data.image.split(',')
							{
							if(lc_ti.contains(&sp.to_lowercase()))
								{ selected_files.push(ti.to_string()); }
							}
						}
					/* In this case, there is only one search term */
					else
						{
						/* Convert the file name and argument to lowercase then compare/search */
						if(lc_ti.contains(&opt_data.image.to_lowercase()))
							{ selected_files.push(ti.to_string()); }
						}
					}
				else
					{ selected_files.push(ti.to_string()); }

				continue;
				}
			}
		}

	/* Cheers! */
	selected_files	
	}


