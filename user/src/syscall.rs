// use core::arch::asm;

// const SYSCALL_WRITE: usize = 64;
// const SYSCALL_EXIT: usize = 93;

// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret: isize;
//     unsafe {
//         asm!(
//             "ecall",
//             inlateout("x10") args[0] => ret,
//             in("x11") args[1],
//             in("x12") args[2],
//             in("x17") id
//         );
//     }
//     ret
// }

// pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
//     syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
// }

// pub fn sys_exit(exit_code: i32) -> isize {
//     syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
// }

// const FD_STDOUT: usize = 1;
// const APP_BASE_ADDRESS: usize = 0x80400000;
// const APP_SIZE_LIMIT: usize = 0x20000;
// use crate::batch::{USER_STACK};
// const USER_STACK_SIZE: usize = 4096;


// pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
//     match fd {
//         FD_STDOUT => {
//             // unsafe {println!("#{:#x} {:#x} #", buf as usize , USER_STACK.get_sp() - USER_STACK_SIZE);}
//             if (((buf as usize)  >= USER_STACK.get_sp() - USER_STACK_SIZE) && ((buf as usize) + len <= USER_STACK.get_sp())) 
//             || (((buf as usize) + len <= APP_SIZE_LIMIT + APP_BASE_ADDRESS) && ((buf as usize) >= APP_BASE_ADDRESS)){
//                 let slice = unsafe { core::slice::from_raw_parts(buf, len) };
//                 let str = core::str::from_utf8(slice).unwrap();
//                 print!("{}", str);
//                 len as isize
//             }else{
//                 -1 as isize
//             }
//         },
//         _ => {
//             -1 as isize
//             //panic!("Unsupported fd in sys_write!");
//         }
//     }
// }
use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}