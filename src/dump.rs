use crate::bits;

fn diff(data: &[u32]) -> Vec<u32> {
    let len = data.len();
    let mut diff = Vec::with_capacity(len);
    let mut before = 0;
    for i in 0..len {
        let n = unsafe { data.get_unchecked(i) };
        let d = n - before;
        unsafe {
            *diff.get_unchecked_mut(i) = 32 - d.leading_zeros();
        }
        before = *n;
    }
    unsafe { diff.set_len(len) };
    diff
}

fn is_contain(diff: &[u32], bit: &'static bits::Bit) -> bool {
    if diff.len() < bit.len {
        return false;
    }
    for i in 0..bit.len {
        let size = unsafe { bit.data.get_unchecked(i) };
        if unsafe { diff.get_unchecked(i) } > size {
            return false;
        }
    }
    true
}

fn search_bit(diff: &[u32]) -> &'static bits::Bit {
    for id in 0..16 {
        let bit = unsafe { bits::S16_BITS.get_unchecked(id) };
        if is_contain(diff, bit) {
            return bit;
        }
    }
    panic!("not found bit");
}

pub unsafe fn compose(data: &[u32], bit: &'static bits::Bit, before: &mut u32) -> u32 {
    let mut buf = bit.id;
    for i in 0..bit.len {
        let size = bit.data.get_unchecked(i);
        buf <<= size;
        let n = data.get_unchecked(i);
        buf |= *n - *before;
        *before = *n;
    }
    buf
}

unsafe fn dump_with_not_size_check(data: &[u32], buf: &mut [u8]) -> usize {
    let len = data.len();
    let diff = diff(data);
    let mut index = 0;
    let mut pos = 0;
    let res = unsafe {
        let ptr = buf.as_mut_ptr() as *mut u32;
        std::slice::from_raw_parts_mut(ptr, 0)
    };
    let mut before = 0;
    while pos < len {
        unsafe {
            let bit = search_bit(diff.get_unchecked(pos..len));
            let data = data.get_unchecked(pos..len);
            *res.get_unchecked_mut(index) = compose(data, bit, &mut before);
            pos += bit.len;
        }
        index += 1;
    }
    index * 4
}

pub fn dump(data: &[u32], buf: &mut Vec<u8>) {
    buf.reserve(data.len() * 4 - buf.capacity());
    unsafe {
        let size = dump_with_not_size_check(data, buf.as_mut_slice());
        buf.set_len(size);
    }
}

pub fn dump_with_size(data: &[u32], buf: &mut Vec<u8>) {
    buf.reserve(data.len() * 4 - buf.capacity() + 4);
    unsafe {
        let ptr = buf.as_mut_ptr() as *mut u32;
        *ptr = data.len() as u32;
        let size = dump_with_not_size_check(data, buf.get_unchecked_mut(4..));
        buf.set_len(size + 4);
    }
}
