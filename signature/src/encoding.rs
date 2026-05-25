//! Encoding support.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
use fallible_vec::SliceExt;
use fallible_vec::TryClone;

/// Support for decoding/encoding signatures as bytes.
pub trait SignatureEncoding:
    TryClone + Sized + for<'a> TryFrom<&'a [u8]> + TryInto<Self::Repr>
{
    /// Byte representation of a signature.
    type Repr: 'static + AsRef<[u8]> + TryClone + Send + Sync;

    /// Encode signature as its byte representation.
    fn to_bytes(&self) -> Self::Repr {
        self.try_clone()
            .expect("TODO")
            .try_into()
            .ok()
            .expect("signature encoding error")
    }

    /// Encode signature as a byte vector.
    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<u8> {
        self.to_bytes().as_ref().try_to_vec().expect("TODO")
    }

    /// Get the length of this signature when encoded.
    fn encoded_len(&self) -> usize {
        self.to_bytes().as_ref().len()
    }
}
