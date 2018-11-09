use std;
use std::slice;

pub fn to_u8_slice<'a, T>(src: &'a T) -> &'a [u8]
where
    T: Sized,
{
    let sz = std::mem::size_of::<T>();
    unsafe { slice::from_raw_parts(src as *const T as *const u8, sz) }
}

#[cfg(test)]
mod tests {
    use super::to_u8_slice;

    #[test]
    pub fn it_works() {
        let a = 60000;
        println!("{:x?}", to_u8_slice(&a));
        assert_eq!([0x60, 0xea, 0, 0], to_u8_slice(&a));
    }
}
