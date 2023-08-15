mod bits;
use std::{fs, path::Path};

fn compose(data: &[u32]) -> Vec<u32> {
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
    unsafe {
        diff.set_len(len);
    }
    let mut index = 0;
    let mut pos = 0;
    let mut res = Vec::with_capacity(len);
    let mut before = 0;
    'a: while pos < len {
        'b: for id in 0..16 {
            let bit = unsafe { bits::S16_BITS.get_unchecked(id) };
            if pos + bit.len > len {
                continue;
            }
            for j in 0..bit.len {
                let size = unsafe { bit.data.get_unchecked(j) };
                if unsafe { diff.get_unchecked(pos + j) } > size {
                    continue 'b;
                }
            }
            let mut tmp = id as u32;
            for j in 0..bit.len {
                let size = unsafe { bit.data.get_unchecked(j) };
                tmp <<= size;
                let n = unsafe { data.get_unchecked(pos + j) };
                tmp |= *n - before;
                before = *n;
            }
            unsafe {
                *res.get_unchecked_mut(index) = tmp;
            }
            index += 1;
            pos += bit.len;
            continue 'a;
        }
    }
    unsafe {
        res.set_len(index);
    }
    res
}

fn compose2(data: &[u32], buf: &mut Vec<u8>) {
    //forgot need fix
    buf.reserve(data.len() * 4 - buf.len());
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
    unsafe {
        diff.set_len(len);
    }
    let mut index = 0;
    let mut pos = 0;
    let mut res = unsafe {
        let ptr = buf.as_mut_ptr() as *mut u32;
        Vec::from_raw_parts(ptr, 0, 0)
    };
    let mut before = 0;
    'a: while pos < len {
        'b: for id in 0..16 {
            let bit = unsafe { bits::S16_BITS.get_unchecked(id) };
            if pos + bit.len > len {
                continue;
            }
            for j in 0..bit.len {
                let size = unsafe { bit.data.get_unchecked(j) };
                if unsafe { diff.get_unchecked(pos + j) } > size {
                    continue 'b;
                }
            }
            let mut tmp = id as u32;
            for j in 0..bit.len {
                let size = unsafe { bit.data.get_unchecked(j) };
                tmp <<= size;
                let n = unsafe { data.get_unchecked(pos + j) };
                tmp |= *n - before;
                before = *n;
            }
            unsafe {
                *res.get_unchecked_mut(index) = tmp;
            }
            index += 1;
            pos += bit.len;
            continue 'a;
        }
    }
    unsafe {
        buf.set_len(index * 4);
    }
    std::mem::forget(res);
}

fn decompose(v: &[u32]) -> Vec<u32> {
    let mut res = Vec::with_capacity(v.len() * 28);
    let mut before = 0;
    let mut index = 0;
    for i in 0..v.len() {
        let n = unsafe { v.get_unchecked(i) };
        let id = (*n >> 28) & 0xF;
        let bit = unsafe { bits::S16_BITS.get_unchecked(id as usize) };
        let mut shift = 28;
        for i in 0..bit.len {
            let size = unsafe { bit.data.get_unchecked(i) };
            shift -= size;
            let diff = (*n >> shift) & unsafe { *bit.mask.get_unchecked(i) };
            before += diff;
            unsafe {
                *res.get_unchecked_mut(index) = before;
            }
            index += 1;
        }
    }
    unsafe {
        res.set_len(index);
    }
    res
}

fn decompose2(data: &[u8], buf: &mut Vec<u32>) -> Result<(), &'static str> {
    //forgot need fix
    if data.len() % 4 != 0 {
        return Err("data.len must be multiple of 4");
    }
    buf.reserve(data.len() * 7 - buf.len());
    let v: Vec<u32> = unsafe {
        let ptr = data.as_ptr() as *mut u32;
        Vec::from_raw_parts(ptr, 0, 0)
    };
    let mut before = 0;
    let mut index = 0;
    for i in 0..(data.len() / 4) {
        let n = unsafe { v.get_unchecked(i) };
        let id = (*n >> 28) & 0xF;
        let bit = unsafe { bits::S16_BITS.get_unchecked(id as usize) };
        let mut shift = 28;
        for i in 0..bit.len {
            let size = unsafe { bit.data.get_unchecked(i) };
            shift -= size;
            let diff = (*n >> shift) & unsafe { *bit.mask.get_unchecked(i) };
            before += diff;
            unsafe {
                *buf.get_unchecked_mut(index) = before;
            }
            index += 1;
        }
    }
    std::mem::forget(v);
    unsafe {
        buf.set_len(index);
    }
    Ok(())
}

fn main() {
    let path = Path::new(r"\data\all.bin");
    let mut data = Vec::new();
    let start = std::time::Instant::now();
    fs::read(path).unwrap().chunks_exact(4).for_each(|chunk| {
        data.push(u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
    });
    println!("read {:?}", start.elapsed());

    data.sort();
    println!("{:?}", data.len());
    println!("{:?}", data[0..10].to_vec());
    println!("-------------------");

    let start = std::time::Instant::now();
    let comp = compose(&data);
    println!("comp {:?}", start.elapsed());
    println!("{:?}", comp.len());
    println!("{:?}", comp[0..10].to_vec());
    println!("-------------------");

    let start = std::time::Instant::now();
    let res = decompose(&comp);
    println!("deco {:?}", start.elapsed());
    println!("{:?}", res.len());
    println!("{:?}", res[0..10].to_vec());
    println!("-------------------");
    assert_eq!(data, res);

    let mut comp: Vec<u8> = Vec::new();
    let start = std::time::Instant::now();
    compose2(&data, &mut comp);
    println!("comp2 {:?}", start.elapsed());
    println!("{:?}", comp.len());
    println!("{:?}", comp[0..10].to_vec());
    println!("-------------------");

    let mut res: Vec<u32> = Vec::new();
    let start = std::time::Instant::now();
    decompose2(&comp, &mut res).unwrap();
    println!("deco2 {:?}", start.elapsed());
    println!("{:?}", res.len());
    println!("{:?}", res[0..10].to_vec());
    println!("-------------------");
    assert_eq!(data, res);
}
