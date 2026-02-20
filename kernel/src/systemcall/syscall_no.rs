use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive)]
#[repr(usize)]
pub enum SystemCallNo {
    Print = 1,
}
