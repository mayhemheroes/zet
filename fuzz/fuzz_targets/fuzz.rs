#![no_main]

extern crate core;

use std::path::PathBuf;

use arbitrary::{Arbitrary, Unstructured};
use assert_fs::{prelude::*, TempDir};
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
        _ => OpName::Multiple
    })
}

// todo reuse the calc function inside operations::test instead of copying it
fn calc(operation: OpName, operands: Vec<String>) -> String {
    let first = operands[0].as_bytes();
    let remaining = operands[1..].iter();

    let temp_dir = TempDir::new().unwrap();
    let mut paths = Vec::new();
    for operand in remaining {
        let name = format!("operand{}", paths.len());
        let op = temp_dir.child(name);
        op.write_str(operand).unwrap();
        paths.push(PathBuf::from(op.path()));
    }

    let mut answer = Vec::new();
    calculate(operation, first, Remaining::from(paths), &mut answer).unwrap();
    String::from_utf8(answer).unwrap()
}

fuzz_target!(|data: FuzzInput| {
    if !data.files.is_empty() {
        calc(data.op, data.files);
    }
});
