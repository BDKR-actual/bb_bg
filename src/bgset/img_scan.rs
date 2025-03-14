
use std::collections::HashMap;
use std::process;
use crate::bgset::op_args;

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


