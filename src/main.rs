use cudarc::driver;
use anyhow::{Result, Error};

fn main() -> Result<(), Error> {
    let dev_num: usize = driver::CudaDevice::count()?.try_into().unwrap();
    println!("Found {} CUDA devices", dev_num); // 3 on my machine
    // let dev_list = for i in 0..dev_num {
        // let dev: std::sync::Arc<driver::CudaDevice> = driver::CudaDevice::new(i)?;
    //     dbg!("Device {}: {}", i, dev);
    // };
    let dev_num_list = (0..dev_num).collect::<Vec<usize>>();
    let dev_list = 
        dev_num_list.iter().map(|i| driver::CudaDevice::new(*i).unwrap()).collect::<Vec<std::sync::Arc<driver::CudaDevice>>>();
    let dev_name_list = dev_list.iter().map(|dev| dev.ordinal()).collect::<Vec<usize>>();
    println!("Device list: {:?}", dev_name_list);
    Ok(())
}