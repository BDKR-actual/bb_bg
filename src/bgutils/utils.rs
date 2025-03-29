


use memory_stats::memory_stats;





pub fn print_type_of<T>(_: &T)
    { println!("{}", std::any::type_name::<T>()); }


pub fn is_integer(s: &str) -> bool	
	{ s.chars().all(|c| c.is_ascii_digit()) }


pub fn print_memory_usage()
    {
    if let Some(usage) = memory_stats()
        {
        println!("Current physical memory usage: {}", usage.physical_mem);
        println!("Current virtual memory usage: {}", usage.virtual_mem);
        }
    else
        { println!("Couldn't get the current memory usage :("); }
    }


