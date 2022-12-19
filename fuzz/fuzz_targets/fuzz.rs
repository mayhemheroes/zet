#![no_main]

extern crate core;

use anyhow::Result;
use arbitrary::{Arbitrary, Unstructured};
use bstr::ByteSlice;
use libfuzzer_sys::fuzz_target;

use zet::args::OpName;
use zet::operations::{calculate, LaterOperand};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    #[arbitrary(with = arbitrary_op)]
    op: OpName,
    first: String,
    rest: Vec<String>,
}

fn arbitrary_op(u: &mut Unstructured<'_>) -> arbitrary::Result<OpName> {
    u.int_in_range(0..=4).map(|x| match x {
        0 => OpName::Intersect,
        1 => OpName::Union,
        2 => OpName::Diff,
        3 => OpName::Single,
        _ => OpName::Multiple,
    })
}

/** Needed because neither LaterOperand nor &[u8] were defined in this crate */
struct InputFile<'a> {
    text: &'a [u8],
}

impl<'a> LaterOperand for InputFile<'a> {
    fn for_byte_line(self, for_each_line: impl FnMut(&[u8])) -> Result<()> {
        self.text.lines().for_each(for_each_line);
        Ok(())
    }
}

fuzz_target!(|data: FuzzInput| {
    let rest = data.rest
        .iter()
        .map(|text| Ok(InputFile { text: text.as_bytes() }));

    let mut answer = Vec::new();
    calculate(data.op, data.first.as_bytes(), rest, &mut answer).unwrap();
    String::from_utf8(answer).unwrap();
});
