use ironrdp_core::codecs::rfx::Quant;

const FIRST_LEVEL_SIZE: usize = 1024;
const SECOND_LEVEL_SIZE: usize = 256;
const THIRD_LEVEL_SIZE: usize = 64;

const FISRT_LEVEL_SUBBANDS_COUNT: usize = 3;
const SECOND_LEVEL_SUBBANDS_COUNT: usize = 3;

pub fn decode(buffer: &mut [i16], quant: &Quant) {
    let (first_level, buffer) = buffer.split_at_mut(FISRT_LEVEL_SUBBANDS_COUNT * FIRST_LEVEL_SIZE);
    let (second_level, third_level) = buffer.split_at_mut(SECOND_LEVEL_SUBBANDS_COUNT * SECOND_LEVEL_SIZE);

    let decode_chunk = |a: (&mut [i16], u8)| decode_block(a.0, a.1 as i16 - 1);

    first_level
        .chunks_mut(FIRST_LEVEL_SIZE)
        .zip([quant.hl1, quant.lh1, quant.hh1].iter().copied())
        .for_each(decode_chunk);

    second_level
        .chunks_mut(SECOND_LEVEL_SIZE)
        .zip([quant.hl2, quant.lh2, quant.hh2].iter().copied())
        .for_each(decode_chunk);

    third_level
        .chunks_mut(THIRD_LEVEL_SIZE)
        .zip([quant.hl3, quant.lh3, quant.hh3, quant.ll3].iter().copied())
        .for_each(decode_chunk);
}

fn decode_block(buffer: &mut [i16], factor: i16) {
    if factor > 0 {
        for value in buffer {
            *value <<= factor;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_does_not_change_buffer_with_null_quant_values() {
        let mut buffer = QUANTIZED_BUFFER;
        let expected = QUANTIZED_BUFFER;
        let quant = Quant {
            ll3: 0,
            lh3: 0,
            hl3: 0,
            hh3: 0,
            lh2: 0,
            hl2: 0,
            hh2: 0,
            lh1: 0,
            hl1: 0,
            hh1: 0,
        };

        decode(&mut buffer, &quant);
        assert_eq!(expected.as_ref(), buffer.as_ref());
    }

    #[test]
    fn decode_works_with_not_empty_quant_values() {
        let mut buffer = QUANTIZED_BUFFER;
        let expected = DEQUANTIZED_BUFFER.as_ref();
        let quant = Quant {
            ll3: 6,
            lh3: 6,
            hl3: 6,
            hh3: 6,
            lh2: 7,
            hl2: 7,
            hh2: 8,
            lh1: 8,
            hl1: 8,
            hh1: 9,
        };

        decode(&mut buffer, &quant);
        assert_eq!(expected, buffer.as_ref());
    }

    const QUANTIZED_BUFFER: [i16; 4096] = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, -2, 4,
        -5, -1, 0, 0, 0, 0, 0, 0, -1, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 3, 0, 0, 0, 0, 0, 0, -1, 4, 4, -5, -1,
        0, 0, 0, 0, 0, 0, -2, 5, 0, 0, 0, 0, 0, 0, 2, 1, 0, 2, -5, -4, 1, 0, 0, 0, 0, 0, 0, 1, 3, -1, 5, 0, -1, 0, 0,
        0, 0, 0, -2, -2, -3, 0, 0, 0, 0, 0, 3, 0, 0, 7, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 2, -1, 0, 0, 0, 0, 0,
        -3, 1, 5, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, -2,
        0, 0, 0, 0, 0, 1, 0, -9, -3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0,
        0, 0, 0, 6, -2, -8, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0,
        5, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, -1, 2, 0, 0, 1, 0, 0, -1, 0, -1, 7, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -3, -2, 0, 0, 0, 0, 0, 3, -1, 0, -1, 0, 1, 0, 0, 0, 1, 2, -1, 0, -5, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, -3, -4, 0, 0, 0, 0, 0, -1, -6, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 6, 0, -1, 0, 1, 0, 0, 1, -1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 10, 0, 0, 0, -1, 1, 0, 0, 0, 0, -1, 1,
        0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -5, -5, 3, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 0,
        0, -1, -2, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 3, -1, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -6, 0, 0, 0, 0, 0, 0, -8,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 4, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 6, -1, 0, 0, 0, 0, -1, 7, -1, 0, 0, 0,
        -1, -1, 0, 0, 0, 0, 0, -3, -1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 0, 4, 1, 2, -2, -2, 0, -1, 0,
        0, 0, 0, 1, -1, -7, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, -1, -10, 3, 1, 2, 0, -2, 1, 1, 0, 0, 0,
        0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 3, -2, 0, 0, 0, 0, 0, 6, -8, -2, 0, -2, 1, 0, 0, -1, -1, -1, 1, 1, 1,
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0, 0, 0, 0, 3, 2, 0, 1, 0, 0, 1, 1, 0, 0, 1, -2, 5, -3, 2, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, -3, 0, 1, 0, 0, 1, 1, 0, 0,
        0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, -1, -1, 0, 0, 1, 0, 0, 0, 3, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 1, 1, -2, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, -1, 0, 0, -1, -1, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, -2, -8, 2, 0,
        0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, -2, 4, 4, -1, 0, 0, 0, 0, 0,
        0, -1, 3, -1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 3, 3, -7, -1, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 6, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -2, -2, 2, -1,
        0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, -5, -3, -4, -6, -6, 7, 1, 1, -1, 0, 0, 0, 0, 0,
        0, 0, -1, -2, -3, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, -1, 0, -2, -3, 1, 1, -8, 6, 0, -1, 1, 0, 0, 0, 0, 1,
        1, 0, 0, 6, -3, -1, 0, 0, 0, 0, 0, -1, 5, 2, -6, -2, 0, 0, 0, 2, 7, -6, -1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        -1, -1, 1, 6, 0, 0, 0, 0, 0, 0, -2, 1, 0, 0, 0, 0, 0, 0, 0, -1, 6, -1, 1, -6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1,
        -1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -2, 0, -8, 6, 6, 4, 3, 3, 4, -1, -2, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, -1, -1, 7, -8, 0, 1, -2, -4, -3, -1, 1, -3, -4, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, -3, -5, 0, 1, 0, 0, 8, -1, -7, -9, -7, -2, 6, 3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, -1, -1, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, -1, 1, 0, 0, 1, -1, 0, 0, 0, 0, -1, 1, -1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, -1, 0, 0, 0, 0, 0, 0, 1, 1, 3, 3, 0, 1, 1, -1, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2, 1, -4, -5, -8, -6, -4, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 3, -9, -5, 10, 2, 2, 0, -1, 2, 6, -4, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 2, -1, -1, 1, 0, 0, 0, 0, 0, -1, 4, -2, -1, 1, -1, 1, -1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, -1, 1, 1, 0, 1, 0, -1, 3, -1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, 1, 1, 1, 0, -2, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -1, 1, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
        0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 2, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1,
        0, 0, 1, 0, 0, 0, 0, 0, 0, -1, 0, 0, -1, -1, 0, -5, 3, -2, 1, 0, -1, 0, 0, 0, 0, 1, -4, -1, 0, 0, 1, -5, -2, 1,
        1, 1, 6, -2, -1, 0, 1, -4, 21, 6, 0, 0, -5, -1, 1, 2, 4, 4, 3, 5, 0, 0, -1, 4, -8, -11, 0, 0, -2, 8, -1, 0, 3,
        1, -18, 16, 1, -1, 2, 0, -4, -4, 0, 0, -6, 1, -3, -1, 1, 14, -23, 1, 0, 1, 0, 1, -3, -1, 0, 0, 0, -5, -2, 3,
        -1, 11, -8, -3, 0, 1, 0, -1, -3, 1, 0, 14, 1, 0, -1, 0, 0, -15, -2, 7, 1, -1, 1, -3, -1, -1, -2, -14, 0, 1, 0,
        0, 5, -18, 14, -7, -1, -1, -2, 13, 1, -1, 3, -14, 1, -2, 2, -1, 11, -16, -2, 2, -1, 1, 0, -9, 0, -1, 19, 1, 1,
        1, 0, 2, 2, 8, 0, 0, 0, 0, 1, -13, 0, 0, -6, -3, -1, 0, -2, -2, 6, 13, 0, 0, 0, 1, 9, 1, 0, 0, -1, 2, 0, 0, 1,
        0, -5, 0, 0, -1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, -2, -4, 1, 4, 4, -2, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, -4, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, -1, 1, 1, -1, -3, 0, -2, -3, 0,
        0, 0, 0, 0, 0, 0, -2, -2, 0, 1, -1, 2, 0, 0, 0, 1, -1, 1, 0, 1, -8, -1, 0, 0, 1, -3, 0, 0, -1, 3, 0, 0, 1, -3,
        -3, -3, 3, -4, 0, 0, 0, 1, 0, 0, 3, -3, 3, -1, 1, 1, 0, 1, -5, 20, 0, 0, 0, -1, 2, 7, -1, 0, 3, -2, 1, 0, 4,
        -1, 0, 1, 0, 1, -3, 2, -9, -12, -19, 10, -1, 2, 0, -1, 1, 9, -13, 0, 0, 1, -4, -13, 0, 0, 16, -7, 0, 16, -3, 1,
        -2, -1, 2, -1, 0, 0, 0, 1, 1, 0, 0, 4, -3, -17, -14, 2, -4, -14, -1, 0, -1, 1, 0, -1, 0, 1, 1, -2, 1, 3, 17,
        13, 15, 9, -2, 0, 2, -3, 2, 2, -2, 2, 1, -1, 1, 0, 0, -2, 5, 7, -1, 0, 1, 4, -7, -14, -7, 2, -2, 0, 0, 0, 0, 0,
        0, -1, 1, 0, -3, 17, 8, -1, 4, 8, -9, 6, 0, 1, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, -2, -2, 2, 0, 0, 5, -1,
        0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -5, -4, -2, -6, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -1, 0, -4, -1, 0, 0, 0, 0, 0, 0, -1, -1, 1, -1, 0, 0, 0, 0, 2, 2, 0, 0, 0, 3, 0, 0, 1, 1, -1,
        1, 0, 0, 0, 0, -4, -8, 0, 0, 0, -1, 0, 0, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, -3, 3, 0, 0, 0,
        -1, 1, 0, 0, 0, 0, 1, -2, 0, 0, -1, 3, 1, 2, -1, 0, 0, 0, -2, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, -3,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, 2, 0, -1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, -1, 0, 0, 0, -1, 0, -2, 0, 0, 2,
        2, 1, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 2, 0, 0, 1, -2, 1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, -1, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, -2, -2, 1, 0, -1, -2, 0, -1, 8, 6, 3, 0, -1, -6, 0, 4, 5, -13, -53, -3, -2, 0,
        -2, -15, -7, 25, 8, 3, 7, -25, 26, -12, 5, 20, 6, 7, -5, -13, 2, -2, 5, -54, 10, -2, 30, 3, -46, 11, -9, -1,
        -2, 1, -18, -2, 8, -3, 3, -5, -4, -1, -5, -2, -3, 0, 1, -10, -3, 1, -1, 1, -5, -1, -3, 3, 7, -14, -13, 10, 0,
        -1, 11, 2, 5, 2, 12, 0, -8, 24, -21, -49, -4, 10, 9, 31, -1, 5, 10, 1, 13, -57, -52, 9, 12, -6, -18, 5, 1, -3,
        -4, -1, 13, 21, 8, 11, 3, 7, 7, -2, -3, -1, 0, 2, -14, -19, -7, 1, -1, -1, -3, -2, 0, 0, -1, -1, -1, -7, 3, 4,
        7, -1, -6, -14, -1, -7, 3, -12, -16, -2, 2, -1, -9, 1, 7, -21, 25, -4, -2, -23, 2, 2, 3, -4, 22, -4, 5, -4, -2,
        -2, 4, -7, 0, 0, 3, 2, 17, -6, 7, 5, 3, 1, -5, 0, 1, 0, 0, 6, 1, -4, 2, 0, -1, 0, -1, -15, -12, -12, -19, -23,
        27, 14, 3, 37, 11, -12, -15, -16, 29, 25, 46, 103, 86, 43, 26, 11, 38, 31, 56, 86, 63, 128, 115, 27, 72, -31,
        54, 125, 9, 97, 67, 10, 67, -34, 60, 85, 102, 107, 113, 6, 21, -10, 10, -16, 126, 108, 85, -10, 59, 52, 39, 30,
        35, 53, 15, -3,
    ];

    const DEQUANTIZED_BUFFER: [i16; 4096] = [
        0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 256, 0, 0, 0, 0, -128, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0,
        -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 128, 0, 0, 0, 0, 0, 0, 0, 0, -384, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 256,
        0, 0, 0, 0, 0, 0, 0, 128, 0, -256, 512, -640, -128, 0, 0, 0, 0, 0, 0, -128, -384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -384, 0, 384, 0, 0, 0, 0, 0, 0, -128, 512, 512, -640, -128, 0, 0, 0, 0, 0, 0, -256, 640, 0, 0, 0, 0, 0, 0, 256,
        128, 0, 256, -640, -512, 128, 0, 0, 0, 0, 0, 0, 128, 384, -128, 640, 0, -128, 0, 0, 0, 0, 0, -256, -256, -384,
        0, 0, 0, 0, 0, 384, 0, 0, 896, 0, -512, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, -128, 256, -128, 0, 0, 0, 0, 0, -384,
        128, 640, 0, 0, 0, 0, 0, 0, -128, 0, 0, 128, 896, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 256, 0,
        0, 0, -256, 0, 0, 0, 0, 0, 128, 0, -1152, -384, 640, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0,
        0, -128, -128, -128, 0, 0, 0, 0, 0, 0, 768, -256, -1024, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0,
        0, 0, -384, 0, 0, 0, 0, 0, 0, -128, 0, -128, 0, 640, 256, 384, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 256, 0, 0, 0, 0,
        0, 0, -128, 128, 0, 0, -128, 256, 0, 0, 128, 0, 0, -128, 0, -128, 896, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -384,
        -256, 0, 0, 0, 0, 0, 384, -128, 0, -128, 0, 128, 0, 0, 0, 128, 256, -128, 0, -640, 128, 0, 0, 0, 0, 0, 0, 0, 0,
        0, -384, -512, 0, 0, 0, 0, 0, -128, -768, 128, 0, -128, 0, 0, 0, 0, 0, 0, 0, 768, 0, -128, 0, 128, 0, 0, 128,
        -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 384, -640, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, -128, 128, 1280, 0, 0, 0,
        -128, 128, 0, 0, 0, 0, -128, 128, 0, 0, 0, 0, 0, 0, 896, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, -640, -640, 384, 0,
        0, 0, 0, 0, 0, 0, 0, -128, 0, -128, 0, 0, 0, 0, 0, -128, -256, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 384, -128,
        -896, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, -768, 0, 0, 0, 0, 0, 0, -1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 384, 256,
        512, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 768, -128, 0, 0, 0, 0, -128, 896, -128, 0, 0, 0, -128, -128, 0, 0, 0, 0,
        0, -384, -128, 768, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 384, 128, 0, 0, 0, 0, 0, 512, 128, 256, -256, -256, 0, -128,
        0, 0, 0, 0, 128, -128, -896, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -768, 0, 0, 0, 0, 0, -128, -1280, 384, 128,
        256, 0, -256, 128, 128, 0, 0, 0, 0, 0, -128, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 384, -256, 0, 0, 0, 0, 0, 768,
        -1024, -256, 0, -256, 128, 0, 0, -128, -128, -128, 128, 128, 128, 768, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 512, 128,
        0, 0, 0, 0, 0, 384, 256, 0, 128, 0, 0, 128, 128, 0, 0, 128, -256, 640, -384, 256, 0, 0, 0, 0, 128, 0, 0, 0, 0,
        0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, -384, 0, 128, 0, 0, 128, 128, 0, 0, 0, 0,
        0, -512, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 128, -128, -128, 0, 0, 128, 0, 0, 0,
        384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 128, 128, 128, -256, 0, 128,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, -256, 128, 128, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 384, -128, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0,
        -128, -128, 0, 0, -128, -128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, -128, -128, 0, 0, 0, 0,
        -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, -128, -128, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, -128, 128, 0, 0, 0, 0, 0, 0, -256, -1024, 256, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 128, 0, -256, 512, 512, -128, 0, 0, 0, 0, 0, 0, -128, 384, -128,
        0, 0, 0, 0, 0, 0, 128, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 384, 384, -896, -128, 0, 0, 0, 0, 0, 0, 0, 0,
        128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 768, 384, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 128, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1024, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 128, -256, -256, 256, -128, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0,
        128, 128, 384, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, -128,
        -640, -384, -512, -768, -768, 896, 128, 128, -128, 0, 0, 0, 0, 0, 0, 0, -128, -256, -384, 0, 128, 0, 0, 0, 0,
        0, 0, 0, 0, 128, 0, 256, -128, 0, -256, -384, 128, 128, -1024, 768, 0, -128, 128, 0, 0, 0, 0, 128, 128, 0, 0,
        768, -384, -128, 0, 0, 0, 0, 0, -128, 640, 256, -768, -256, 0, 0, 0, 256, 896, -768, -128, 0, 0, 128, 0, 0, 0,
        0, 0, 0, 0, 0, -128, -128, 128, 768, 0, 0, 0, 0, 0, 0, -256, 128, 0, 0, 0, 0, 0, 0, 0, -128, 768, -128, 128,
        -768, 896, 0, 0, 0, 0, 0, 0, 0, 0, 128, -128, -128, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -128, -256, 0, -1024, 768, 768, 512, 384, 384, 512, -128, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 256, -128, -128, 896, -1024, 0, 128, -256, -512, -384, -128, 128, -384, -512, 128, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 128, 0, 128, -384, -640, 0, 128, 0, 0, 1024, -128, -896, -1152, -896, -256, 768, 384, 0, 0,
        0, 0, 0, 0, 0, 128, 0, 0, 0, -128, -128, 0, 0, 0, 0, 0, 128, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, -128, 128, 0, 0, 0, 0, 0, -128, 128, 0, 0, 128, -128, 0,
        0, 0, 0, -128, 128, -128, 128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 128, -128,
        0, 0, 0, 0, 0, 0, 128, 128, 384, 384, 0, 128, 128, -128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        128, 0, 0, 0, 128, 0, 0, 0, 256, 128, -512, -640, -1024, -768, -512, 128, 0, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 384, -1152, -640, 1280, 256, 256, 0, -128, 256, 768, -512, -128, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 256, -128, -128, 128, 0, 0, 0, 0, 0, -128, 512, -256,
        -128, 128, -128, 128, -128, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 128, -128, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        -128, -128, 0, -128, 128, 128, 0, 128, 0, -128, 384, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, -128, -128, -128, 128, 128, 128, 0, -256, -384, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 512, 256, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -512, -256, 256, 0, 0, 0, 0, 0, 0, 0, -256, -256, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, -256, 256, -512, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 256, -256, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        256, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 256,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0,
        0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -512, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, -256, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 256, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, -256, 512, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, -256, 0, 0, 0, 0, 0, 0, 0, 256, -256, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 256, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, -256, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 64, 0, 0, 64, 0, 0, 64, 0, 0, 0, 0, 0, 0, -64, 0, 0, -64, -64, 0, -320, 192, -128, 64, 0, -64, 0, 0, 0, 0,
        64, -256, -64, 0, 0, 64, -320, -128, 64, 64, 64, 384, -128, -64, 0, 64, -256, 1344, 384, 0, 0, -320, -64, 64,
        128, 256, 256, 192, 320, 0, 0, -64, 256, -512, -704, 0, 0, -128, 512, -64, 0, 192, 64, -1152, 1024, 64, -64,
        128, 0, -256, -256, 0, 0, -384, 64, -192, -64, 64, 896, -1472, 64, 0, 64, 0, 64, -192, -64, 0, 0, 0, -320,
        -128, 192, -64, 704, -512, -192, 0, 64, 0, -64, -192, 64, 0, 896, 64, 0, -64, 0, 0, -960, -128, 448, 64, -64,
        64, -192, -64, -64, -128, -896, 0, 64, 0, 0, 320, -1152, 896, -448, -64, -64, -128, 832, 64, -64, 192, -896,
        64, -128, 128, -64, 704, -1024, -128, 128, -64, 64, 0, -576, 0, -64, 1216, 64, 64, 64, 0, 128, 128, 512, 0, 0,
        0, 0, 64, -832, 0, 0, -384, -192, -64, 0, -128, -128, 384, 832, 0, 0, 0, 64, 576, 64, 0, 0, -64, 128, 0, 0, 64,
        0, -320, 0, 0, -64, 0, 0, 64, 64, 0, 0, 0, 0, 0, 0, 0, 0, 64, -128, -256, 64, 256, 256, -128, 0, 0, -64, 0, 0,
        0, 0, 0, 0, 0, 0, 64, 0, -256, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, -64, 64,
        64, -64, -192, 0, -128, -192, 0, 0, 0, 0, 0, 0, 0, -128, -128, 0, 64, -64, 128, 0, 0, 0, 64, -64, 64, 0, 64,
        -512, -64, 0, 0, 64, -192, 0, 0, -64, 192, 0, 0, 64, -192, -192, -192, 192, -256, 0, 0, 0, 64, 0, 0, 192, -192,
        192, -64, 64, 64, 0, 64, -320, 1280, 0, 0, 0, -64, 128, 448, -64, 0, 192, -128, 64, 0, 256, -64, 0, 64, 0, 64,
        -192, 128, -576, -768, -1216, 640, -64, 128, 0, -64, 64, 576, -832, 0, 0, 64, -256, -832, 0, 0, 1024, -448, 0,
        1024, -192, 64, -128, -64, 128, -64, 0, 0, 0, 64, 64, 0, 0, 256, -192, -1088, -896, 128, -256, -896, -64, 0,
        -64, 64, 0, -64, 0, 64, 64, -128, 64, 192, 1088, 832, 960, 576, -128, 0, 128, -192, 128, 128, -128, 128, 64,
        -64, 64, 0, 0, -128, 320, 448, -64, 0, 64, 256, -448, -896, -448, 128, -128, 0, 0, 0, 0, 0, 0, -64, 64, 0,
        -192, 1088, 512, -64, 256, 512, -576, 384, 0, 64, 0, 0, 0, 0, 0, 0, -64, 64, 0, 0, 0, 0, 0, -128, -128, 128, 0,
        0, 320, -64, 0, 0, -64, 0, 0, 0, 0, 0, 0, 0, -320, -256, -128, -384, -64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, -448, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, -512, -128, 0, 0, 0, 0, 0, 0, -128, -128, 128, -128,
        0, 0, 0, 0, 256, 256, 0, 0, 0, 384, 0, 0, 128, 128, -128, 128, 0, 0, 0, 0, -512, -1024, 0, 0, 0, -128, 0, 0,
        -128, -128, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, -128, -384, 384, 0, 0, 0, -128, 128, 0, 0, 0, 0, 128,
        -256, 0, 0, -128, 384, 128, 256, -128, 0, 0, 0, -256, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 128, 128, 0,
        -384, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, -128, 256, 0, -128, 128, 0, 0, 0, 128, 128, 0, 0, 0, 0, 0, -128, 0, 0, 0,
        -128, 0, -256, 0, 0, 256, 256, 128, -128, 0, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -256, 256, 0, 0, 128, -256,
        128, -128, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, -128, -256, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 128, 128, -128, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, -64, -64, 32,
        0, -32, -64, 0, -32, 256, 192, 96, 0, -32, -192, 0, 128, 160, -416, -1696, -96, -64, 0, -64, -480, -224, 800,
        256, 96, 224, -800, 832, -384, 160, 640, 192, 224, -160, -416, 64, -64, 160, -1728, 320, -64, 960, 96, -1472,
        352, -288, -32, -64, 32, -576, -64, 256, -96, 96, -160, -128, -32, -160, -64, -96, 0, 32, -320, -96, 32, -32,
        32, -160, -32, -96, 96, 224, -448, -416, 320, 0, -32, 352, 64, 160, 64, 384, 0, -256, 768, -672, -1568, -128,
        320, 288, 992, -32, 160, 320, 32, 416, -1824, -1664, 288, 384, -192, -576, 160, 32, -96, -128, -32, 416, 672,
        256, 352, 96, 224, 224, -64, -96, -32, 0, 64, -448, -608, -224, 32, -32, -32, -96, -64, 0, 0, -32, -32, -32,
        -224, 96, 128, 224, -32, -192, -448, -32, -224, 96, -384, -512, -64, 64, -32, -288, 32, 224, -672, 800, -128,
        -64, -736, 64, 64, 96, -128, 704, -128, 160, -128, -64, -64, 128, -224, 0, 0, 96, 64, 544, -192, 224, 160, 96,
        32, -160, 0, 32, 0, 0, 192, 32, -128, 64, 0, -32, 0, -32, -480, -384, -384, -608, -736, 864, 448, 96, 1184,
        352, -384, -480, -512, 928, 800, 1472, 3296, 2752, 1376, 832, 352, 1216, 992, 1792, 2752, 2016, 4096, 3680,
        864, 2304, -992, 1728, 4000, 288, 3104, 2144, 320, 2144, -1088, 1920, 2720, 3264, 3424, 3616, 192, 672, -320,
        320, -512, 4032, 3456, 2720, -320, 1888, 1664, 1248, 960, 1120, 1696, 480, -96,
    ];
}