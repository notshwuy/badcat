use crate::codec::{FromBadcat, IntoBadcat};
use std::io::{Read, Write};

pub const ATOM_TYPE: u8 = 0x0A;

#[derive(Debug, PartialEq)]
pub struct Atom(u16);

impl From<u16> for Atom {
    #[inline]
    fn from(id: u16) -> Self {
        Atom(id)
    }
}

impl IntoBadcat for Atom {
    fn into_badcat(&self, writer: &mut impl Write) -> Result<usize, ()> {
        writer.write(&[ATOM_TYPE]).unwrap();
        writer.write(&u16::to_be_bytes(self.0)).unwrap();

        Ok(3)
    }
}

impl FromBadcat for Atom {
    fn from_badcat(reader: &mut impl Read) -> Result<Self, ()> {
        let mut atom = [0; 3];
        reader.read(&mut atom).unwrap();

        match atom[0] {
            ATOM_TYPE => {
                let atom_id = u16::from_be_bytes([atom[1], atom[2]]);

                Ok(Atom::from(atom_id))
            }
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        builtin::atom::{Atom, ATOM_TYPE},
        codec::{FromBadcat, IntoBadcat},
    };

    #[test]
    fn to_badcat_serializes_to_u16_plus_1() {
        let atom = Atom::from(0x1BAD);

        let mut atom_writer = Cursor::new(Vec::new());
        atom.into_badcat(&mut atom_writer).unwrap();

        let atom_bytes = atom_writer.into_inner();

        assert_eq!(atom_bytes, [ATOM_TYPE, 27, 173]);
    }

    #[test]
    fn from_badcat_deserializes_u8_as_atom() {
        let atom_id: u16 = 0x1BAD;
        let mut atom_cursor = Cursor::new([ATOM_TYPE, 27, 173]);

        assert_eq!(
            Atom::from_badcat(&mut atom_cursor).unwrap(),
            Atom::from(atom_id)
        );
    }

    #[test]
    fn from_badcat_only_accept_tagged_bytes() {
        let mut atom_cursor = Cursor::new([0, 170, 255]);
        assert_eq!(Atom::from_badcat(&mut atom_cursor), Err(()));

        let mut atom_cursor = Cursor::new([123, 255, 255]);
        assert_eq!(Atom::from_badcat(&mut atom_cursor), Err(()));

        let mut atom_cursor = Cursor::new([ATOM_TYPE, 255, 255]);
        assert_eq!(Atom::from_badcat(&mut atom_cursor), Ok(Atom::from(0xFFFF)));
    }
}
