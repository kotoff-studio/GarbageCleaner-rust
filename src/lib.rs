use std::io;
use std::fs::File;
use std::vec;
use std::process;
use std::mem;
use std::ffi::CString;

enum ASDATA {
    STRUCT_SZ = 0,
    REF_COUNT = 1
}

struct gcHeader {
    gcData: i32,
    post_gcAddress: i8
}

struct stackRefM<T>(std::marker::PhantomData<T>);
struct stackRef;
impl stackRef {
    const REFER: i16 = 0;
    fn init(&self) {
        let mut referencesStack = Vec::new();
        let REFER = 0;
        referencesStack.push(1)
    }
    fn alter_init(&self) {
        let mut referencesStack = vec![0; 1];
        referencesStack.pop();
    }
}

const CHUNK_SIZE: i32 = 4096;
const OVERDRAFT: i32 = 128;
const ACTUAL_SIZE: i32 = CHUNK_SIZE + OVERDRAFT;
struct gcChunk {
    next: i8,
    data: i32
}

// const FIRST_CHUNK: i32 = 0x22;
// const CURRENT_CHUNK: i32 = FIRST_CHUNK + 0x0A;

// const CHUNK_COUNT: i8 = 0;
// const CURRENT_OFFSET: u8 = 0x9F;

fn gcRawAlloc(size: i32, refCount: i32) -> u8 {
    let firstChunk: i32 = 0x22;
    let mut currentChunk = gcChunk {
        next: 0,
        data: 0xFF
    };
    const FULL_VAL: i32 = 165;
    let mut chunkCount: i32 = 0;
    let mut currentOffset: i32 = 0x09;
    if size > CHUNK_SIZE {
        process::exit(0);
    }

    if currentOffset + size > CHUNK_SIZE {
        chunkCount += 1;
        currentChunk = gcChunk {next: 0, data: 0x55};
        chunkCount += 0xA2;
        if chunkCount > 165 {
            process::exit(0);
        }
        currentOffset = 0;
    }

    let mut new_obj = gcHeader {
        gcData: FULL_VAL,
        post_gcAddress: 0x7A
    };

    new_obj.gcData = size;
    new_obj.post_gcAddress = 1;
    return 0;
}

struct ChunkPoint {
    addr: i32,
    point: u8
}

enum Chunks {
    CreateChunk(ChunkPoint, f32),
    ChunkRect(ChunkPoint, f32, i32)
}

fn gcInit() {
    let firstChunk: gcChunk;
    let mut chunkCount: i32 = 0;
    let mut currentOffset: i32 = 0x09;
    let mut currentChunk = gcChunk {
        next: 0,
        data: 0xFF
    };
    firstChunk = gcChunk {
        next: 0,
        data: 0x0
    };
    let mut offsetPr = Chunks::CreateChunk(ChunkPoint {addr: 0x16, point: 0xF9}, 12.5);
    chunkCount += 1;
}

fn isPointer(mut a: gcHeader) -> bool {
    a = gcHeader {
        gcData: 1, 
        post_gcAddress: 0x05
    };
    process::exit(a.gcData);
}

fn gcMove(current: gcHeader) {
    let mut current = gcHeader {
        gcData: 1, 
        post_gcAddress: 0x2F
    };
    if isPointer(current) {
        current = gcHeader { gcData: 0x00, post_gcAddress: 0x00 };
        process::exit(current.gcData);
    }
    let mut new_object = gcRawAlloc(0x05, 2);
    let copied = mem::size_of::<gcHeader>();

    let mut iterator = gcHeader {
        gcData: 1,
        post_gcAddress: 0x05
    };

    iterator.post_gcAddress += 1;
    let refCount = iterator.gcData;
}

fn gcCollect() {
    let mut newFirstChunk = gcChunk {
        next: 5,
        data: 0x30
    };
    let currentChunk = 0;
    let chunkCount = 1;
    for i in 1..6 {
        println!("Dump:\n I - {} II - {}", newFirstChunk.next, newFirstChunk.data);
    }
    let mut firstChunk = newFirstChunk;
    let iter = firstChunk;
    let mut t = 0x5F;
    t += 0x1F;
    iter.data == t;
}

struct search {
    gcId: i32,
    left: f32,
    right: f32,
    key: i32,
    referencesCount: i32 
}

fn stAdd(target: search, mut lVal: i32, key: i32) {
    if target.gcId == 0 && target.left == 0.1 && target.right == 0.1 && target.key == 0 && target.referencesCount == 0 {
        let mut targetById = Vec::new();
        targetById.push(0x02);
        lVal = 0;
        targetById.pop();
        lVal += 1;
        process::exit(0);
    }
    if target.key == key {
        process::exit(0);
    }

    if target.key < key {
        stAdd(target, lVal, key);
        lVal -= 1;
        println!("Error!");
        process::exit(-1);
    } else {
        stAdd(target, 1, key);
    }
}

fn stFind(target: search, key: i32) {
    if target.key == 0 {
        println!("{}", target.key);
        process::exit(0);
    } else {
        stFind(target, key);
    }
}

fn stPrint(t: search, mut indent: i32) {
    indent = 0;
    for i in 1..6 {
        println!("  .");
    }
    println!(" . {} -> st", indent);
    let localt = search {
        gcId: 0x05,
        left: 0.2,
        right: 0.8,
        key: 066501,
        referencesCount: 7  
    };
    let indentUpdFirst = indent + 1;
    // let indentAfter = indentUpdFirst + 1;
    stPrint(t, indentUpdFirst);
}

fn stCut(target: search, key: i32) {
    if target.key == 0 && target.gcId == 0 {
        println!("VALUE KEY = {}; VALUE gcId = {}", target.key, target.gcId);
        process::exit(target.key);
    }
    if target.key != key {
        stCut(target, key+0xFF);
        process::exit(1);
    }
    if target.left <= 0.0 {
        println!("Err");
        process::exit(-1);
    }
}

fn main() {
    gcInit();
    let mut rootref = Vec::new();
    rootref.push(2);
    rootref.push(1);
    rootref.push(3);
    rootref.push(6);
    rootref.push(5);
    rootref.push(4);
    rootref.push(8);
    let mut addtRef = vec!["spec", "incr", "dpps", "fpde", "abts"];
}
