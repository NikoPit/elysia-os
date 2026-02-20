use x86_64::instructions::hlt;

pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

#[macro_export]
macro_rules! read_addr {
    ($addr: expr, $type: ty) => {
        *($addr as *mut $type)
    };
}

#[macro_export]
macro_rules! write_addr {
    ($addr: expr, $type: ty, $value: expr) => {
        read_addr!($addr, $type) = $value
    };
}

#[macro_export]
macro_rules! read_port {
    ($port: expr) => {
        Port::new($port).read()
    };
}

#[macro_export]
macro_rules! write_port {
    ($port: expr,$value: expr) => {
        Port::new($port).write($value)
    };
}
