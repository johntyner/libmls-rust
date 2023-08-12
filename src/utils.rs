#[allow(dead_code)]
fn encode_length(mut len: u32) -> Vec<u8> {
    let mut vec = Vec::with_capacity(4);

    if len < (1 << 30) {
        let enc: u8;
        let shr: u8;

        if len < (1 << 6) {
            enc = 0;
        } else if len < (1 << 14) {
            enc = 1;
        } else {
            enc = 2;
        }

        // shr will be 0, 1, or 3
        shr = (1 << enc) - 1;

        // add the encoding bits just above the actual length
        len |= (enc as u32) << ((8 * shr) + 6);

        // convert the length (with the encoding bits "mixed" in)
        // to bytes in the vector in network order
        for i in 0..=shr {
            vec.push((len >> (8 * (shr - i))) as u8)
        }
    }

    return vec;
}

#[allow(dead_code)]
fn decode_length(vec: &[u8]) -> i32 {
    let mut len: i32 = -1;

    if vec.len() > 0 {
        let enc = vec[0] >> 6;

        if enc < 3 && vec.len() == 1 << enc {
            len = (vec[0] & 0x3f) as i32;
            for i in 1..vec.len() {
                len <<= 8;
                len |= vec[i] as i32;
            }

            if (enc == 1 && len < (1 << 6)) || (enc == 2 && len < (1 << 14)) {
                len = -1;
            }
        }
    }

    return len;
}

#[cfg(test)]
mod tests {
    struct EncodingTV<'a> {
        val: u32,
        enc: &'a [u8],
    }

    const ENCODING_TV: &'static [EncodingTV] = &[
        EncodingTV {
            val: 37,
            enc: &[0x25],
        },
        EncodingTV {
            val: 15293,
            enc: &[0x7b, 0xbd],
        },
        EncodingTV {
            val: 494878333,
            enc: &[0x9d, 0x7f, 0x3e, 0x7d],
        },
    ];

    #[test]
    fn encode_length() {
        for i in 0..ENCODING_TV.len() {
            let e = super::encode_length(ENCODING_TV[i].val);
            assert_eq!(&e[..], ENCODING_TV[i].enc);
        }
    }

    #[test]
    fn decode_length() {
        for i in 0..ENCODING_TV.len() {
            let v = super::decode_length(ENCODING_TV[i].enc);
            assert_eq!(v, ENCODING_TV[i].val as i32);
        }
    }
}
