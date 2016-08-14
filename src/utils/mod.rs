extern crate time;


pub fn loop_with_report<F : FnMut(f64)>(mut body : F )  
{
    loop 
	{
        let mut fps_accum :f64 = 0.0;
        let mut samples :u32 = 0;
        let mut delta : f64 = 0.0;

        let start = time::PreciseTime::now();
        while start.to(time::PreciseTime::now()) < time::Duration::seconds(5) 
		{
            let start_t = time::precise_time_s();

            body(delta);

            let end_t = time::precise_time_s();
            delta = end_t-start_t;
            fps_accum += delta;
            samples += 1;
        }

        print!("fps: {} \n", (samples as f64)/fps_accum);
    }
}
