use std::ptr::{
        read_volatile, // Выполняет volatile-чтение значения из src без его перемещения.
        // Выполняет volatile запись местоположения памяти с данным значением без чтения или уничтожения старого значения.
        write_volatile,
    };

fn main() {
    let mut v: u32 = 10 ;

    let ptr = &mut v as *mut u32 ;

    unsafe {
        write_volatile(ptr, 40);
        let res = read_volatile(ptr) ;
        println!("res: {res}") ;    // Out: res: 40
    }
}
