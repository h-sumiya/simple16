use crate::bits;

pub unsafe fn decompose(mut data: u32, buf: &mut [u32]) -> usize {
    let id = (data >> 28) & 0xF;
    let bit = bits::S16_BITS.get_unchecked(id as usize);
    for i in (0..bit.len).rev() {
        let size = *bit.data.get_unchecked(i);
        *buf.get_unchecked_mut(i) = data & *bit.mask.get_unchecked(i);
        data >>= size;
    }
    bit.len
}

fn diff_to_data(data: &mut [u32], len: usize) {
    let mut before = 0;
    for i in 0..len {
        let diff = unsafe { data.get_unchecked_mut(i) };
        *diff += before;
        before = *diff;
    }
}

unsafe fn load_with_not_size_check(data: &[u8], buf: &mut [u32]) -> Result<usize, &'static str> {
    if data.len() % 4 != 0 {
        return Err("data.len must be multiple of 4");
    }
    let v: &[u32] = unsafe {
        let ptr = data.as_ptr() as *mut u32;
        std::slice::from_raw_parts(ptr, 0)
    };
    let mut index = 0;
    for i in 0..(data.len() / 4) {
        let n = *unsafe { v.get_unchecked(i) };
        index += unsafe {
            let diff = std::slice::from_raw_parts_mut(buf.as_mut_ptr().add(index), 0);
            decompose(n, diff)
        };
    }
    diff_to_data(buf, index);
    Ok(index)
}

pub fn load(data: &[u8], buf: &mut Vec<u32>) -> Result<(), &'static str> {
    buf.reserve(data.len() * 7 - buf.capacity());
    let size = unsafe { load_with_not_size_check(data, buf.as_mut_slice()) }?;
    unsafe { buf.set_len(size) };
    Ok(())
}

pub unsafe fn load_with_size(data: &[u8], buf: &mut Vec<u32>) -> Result<(), &'static str> {
    let size = data.as_ptr() as *const u32;
    let size = *size as usize;
    buf.reserve(size - buf.capacity());
    let res = load_with_not_size_check(
        std::slice::from_raw_parts(data.as_ptr().add(4), data.len() - 4),
        buf.as_mut_slice(),
    )?;
    if res != size {
        return Err("size is not match");
    }
    buf.set_len(res);
    Ok(())
}
