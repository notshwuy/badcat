use std::io::{Read, Write};

pub trait IntoBadcat {
    fn into_badcat(&self, writer: &mut impl Write) -> Result<usize, ()>;
}

pub trait FromBadcat {
    fn from_badcat(writer: &mut impl Read) -> Result<Self, ()>
    where
        Self: Sized;
}
