use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(isize)]
pub enum SystemCallNo {
    Write = 1 as isize,
}
