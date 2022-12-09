#![no_main]

extern crate core;

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;

use zet::args::OpName;
use zet::operands::Remaining;
use zet::operations::calculate;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    #[arbitrary(with = arbitrary_op)]
    op: OpName,
    files: Vec<String>,
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

fn calc(operation: OpName, operands: Vec<String>) -> String {
    let first = operands[0].as_bytes();
    let remaining: Vec<(String, &[u8])> = operands[1..]
        .iter()
        .enumerate()
        .map(|(i, text)| (format!("operand{i}"), text.as_bytes()))
        .collect();

    let mut answer = Vec::new();
    calculate(operation, first, Remaining::from(remaining), &mut answer).unwrap();
    String::from_utf8(answer).unwrap()
}

fuzz_target!(|data: FuzzInput| {
    if !data.files.is_empty() {
        calc(data.op, data.files);
    }
});
