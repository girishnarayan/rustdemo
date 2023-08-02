#![no_std]
#![no_main]

//3F20_0008 fsel2 1<<3 turn 1 into an output
//3f20_001c gpio_set 1<<21 turns 21on
//3f20_0028 gpio_set 1<<21 turns 21off

use core::panic::PanicInfo;
use core::arch::asm;
mod start {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    let gpio_fsel2 =1<<3;
    unsafe{
        //turn pin21 on
        core::ptr::write_volatile (0x3F200008 as *mut u32,gpio_fsel2);
        }
     loop{
            unsafe{
        //turn pin on
            core::ptr::write_volatile (0x3F20001C as *mut u32,1<<21);  
        
         for _ in 1..50000{

            asm!("nop");
            }   
        
        
            core::ptr::write_volatile (0x3F200028 as *mut u32,1<<21);
        
            for _ in 1..50000{

                asm!("nop");
            }   
       }
    }


}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop{}
}  