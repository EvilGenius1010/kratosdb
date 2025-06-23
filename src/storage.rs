use crate::errors::StorageErrors;

#[derive(Clone)]
enum DeltaEncodingInput {
    IntArray(Vec<u32>),
    Str(String),
}

//encoding format is
//[ flag ]               // 1 bit: 0 = add, 1 = sub
//[ varint bits_len ]    // variable length integer: number of bits for delta
//[ delta_bits ]         // exact number of bits as bits_len

struct DeltaEncodingOutput {
    flag: bool,
    num_bits: u8,
}

/// Delta encode inputs.
// will be either numbers like 12903,3911,31 or strings like "magnus is the goat"
fn delta_encode(input_data: Vec<DeltaEncodingInput>)
// -> Result<Vec<DeltaEncodingOutput>, StorageErrors>
{
    let mut result: Vec<DeltaEncodingInput> = Vec::new();

    for item in input_data.iter() {
        match item {
            DeltaEncodingInput::IntArray(val) => {
                //single element case
                if (*val).len() == 1 {
                    continue;
                }

                //
                let base = (*val)[0];
            }
            DeltaEncodingInput::Str(val) => {
                let base = DeltaEncodingInput::Str(val.clone());
            } // None => {
              //     eprintln!("Invalid input.Please try again.");
              //     let err = StorageErrors::InvalidDeltaEncodingInput;
              // }
        }
    }
}

struct BitWriter {
    buffer: Vec<u8>,
    curr_byte: u8,
    bits_used: u8,
}

impl BitWriter {
    /// creates a new bitwriter which can write bytes.
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            curr_byte: 0,
            bits_used: 0,
        }
    }

    fn write_bit(&mut self, bit: bool) {
        if bit {
            self.curr_byte |= 1 << (7 - self.bits_used);
        }
    }
}
