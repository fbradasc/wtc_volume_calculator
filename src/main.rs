use std::env;

use std::f64;

#[derive(Debug)]
struct HullSection
{
    r1: f64,
    r2: f64,
    d : f64,
}

#[derive(Debug)]
struct Hull
{
    sections: Vec<HullSection>,
}

impl std::fmt::Display for HullSection
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f,
               "R1: {}, R2: {}, D: {}, volume: {}",
               self.r1,
               self.r2,
               self.d,
               self.volume())
    }
}

impl HullSection
{
    fn volume(&self) -> f64
    {
        ((self.r1*self.r1 + self.r1*self.r2 + self.r2*self.r2)*self.d*f64::consts::PI) / 3.0
    }
}

impl std::fmt::Display for Hull
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f,
               "Sections: {}, total volume: {}",
               self.sections.len(),
               self.volume())
    }
}

impl Hull
{
    fn new() -> Hull
    {
        Hull { sections: Vec::new() }
    }

    fn add(&mut self, hs : HullSection) -> ()
    {
        self.sections.push(hs);
    }

    fn volume(&self) -> f64
    {
        let mut volume : f64 = 0.0;

        for section in &self.sections
        {
            volume += section.volume()
        }

        volume
    }
}

fn main()
{
    let args_count = env::args().count() - 1;

    let exe_name   = std::env::current_exe().expect("Can't get the exec path")
                                            .file_name()
                                            .expect("Can't get the exec name")
                                            .to_string_lossy()
                                            .into_owned();

    if (args_count % 3) != 0
    {
        eprintln!("\nWrong arguments count\n");
        eprintln!("Usage:");
        eprintln!("\t{} <diameter 1> <diameter 2> <height> ...\n", exe_name);
        std::process::exit(-1);
    }

    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut hull = Hull::new();

    let mut index = 0;

    for section_data in args.chunks(3)
    {
        let hs = HullSection
        {
            r1: section_data[0].parse::<f64>().unwrap() / 2.0,
            r2: section_data[1].parse::<f64>().unwrap() / 2.0,
            d : section_data[2].parse::<f64>().unwrap(),
        };

        println!("Section #{}: {}", index, hs);

        hull.add(hs);

        index += 1;
    }

    println!("{}", hull);
}
