use std::fmt;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct bgset_args
    {
    pub heads:      u8,
    pub img_path:   String,
	pub img_paths:	Vec<String>,
	pub show_debug: u8,
	pub interval:	u32,
    }

/* We'll move option data around here */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
#[derive(Clone)]
pub struct op_args
    {
    pub directory:  String,
    pub cmd:        String,
    pub image:      String,
    pub interval:   u32,
    }

/* This is data that will originate from a config file */
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[derive(Debug)]
struct config_data
    {
    directories: String,
    default_time: u8,
    engine: String
    }

pub mod nitrogen;
pub mod wmsetbg;
pub mod plasma;
pub mod img_scan;
pub mod config;
