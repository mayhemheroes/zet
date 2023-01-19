#![no_main]
use libfuzzer_sys::fuzz_target;
use zet::operations::*;
use zet::args::OpName;
use zet::operands::Remaining;
use assert_fs::{prelude::*, TempDir};
use std::path::PathBuf;

fn calc(operation: OpName, operands: &[&[u8]]) {
    let first = operands[0];
    let remaining = operands[1..].iter().map(|s| s.to_vec());

    let temp_dir = TempDir::new().unwrap();
    let mut paths = Vec::new();
    for operand in remaining {
        let name = format!("operand{}", paths.len());
        let op = temp_dir.child(name);
        op.write_binary(&operand[..]).unwrap();
        paths.push(PathBuf::from(op.path()));
    }

    let mut answer = Vec::new();
    calculate(operation, first, Remaining::from(paths), &mut answer).unwrap();
}

fuzz_target!(|data: &[u8]| {
    if data.len() > 3 {
        let opt = data[0];
        let d1 = &data[1..(data.len()/2)];
        let d2 = &data[(data.len()/2)+1..];
        match opt {
            0=>{
                calc(OpName::Union, &[d1,d2]);
            },
            1=>{
                calc(OpName::Diff, &[d1,d2]);
            },
            2=>{
                calc(OpName::Intersect, &[d1,d2]);
            },
            _=>()
        }
    }
});
