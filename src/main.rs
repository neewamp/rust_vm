// Starting to implement stack based virtual machine
// Learning rust at the same time

use std::fmt::Debug;
use std::ops::Add;
use std::ops::Sub;

const STACK_CAPACITY : usize = 128;
const INSTR_CAPACITY : usize = 1024;

#[derive(Debug, Clone, Copy)]
enum Instr<T> {
    VmPlus,
    VmMinus,
    VmPush(T),
    VmPop
}

type Prog<T> = Vec<Instr<T>>;

#[derive(Debug)]
struct Vm<'a, T> {
    stack_size : &'a mut usize,
    stack : &'a mut [T; STACK_CAPACITY],
    prog : Vec<Instr<T>>,
    ip : i64
	
}

#[macro_export]
macro_rules! VmPush {
    ($x:expr) => { 
	Instr::VmPush($x)
    };
}

#[macro_export]
macro_rules! VmPlus {
    () => { 
	Instr::VmPlus
    };
}

#[macro_export]
macro_rules! VmMinus {
    () => { 
	Instr::VmMinus
    };
}


#[macro_export]
macro_rules! VmPop {
    () => { 
       Instr::VmPop
    };
}

#[derive(Debug)]
enum Trap {
    StackOverflow,
    Ok,
    StackUnderflow
}

#[macro_export]
macro_rules! TSO {
    () => { 
       Trap::StackOverflow
    };
}

#[macro_export]
macro_rules! TOK {
    () => { 
       Trap::Ok
    };
}

#[macro_export]
macro_rules! TSU {
    () => { 
       Trap::StackUnderflow
    };
}

#[inline]
fn binary_op<T : Copy>(vm : &mut Vm<T>, op : &Fn (T, T) -> T) -> Trap {
    if *vm.stack_size > 1 {
	let op1 : T = vm.stack[*vm.stack_size-1];
	let op2 : T = vm.stack[*vm.stack_size-2];
	*vm.stack_size-=1;
	vm.stack[*vm.stack_size-1] = op(op1, op2);
	TOK!()
    }
    else {
	TSU!()
    }
}
fn vm_execute<T : Copy + Add<Output = T> + Sub<Output = T>>(vm : &mut Vm<T>, instr : &Instr<T>) -> Trap {
    match *instr {
	VmPlus!() => binary_op(vm, &|x, y| x + y),
	VmMinus!() => binary_op(vm, &|x, y| x - y),
	Instr::VmPush(elt) => {
	    if *vm.stack_size >= STACK_CAPACITY {
		TSO!()
	    }
	    else {
		vm.stack[*vm.stack_size] = elt;
		*vm.stack_size+=1;
		TOK!()
	    }
	}
	VmPop!() => {
	    if *vm.stack_size <= 0 {
		TSU!()
	    }
	    else {
		*vm.stack_size-=1;
		TOK!()
	    }

	}
    }
}

fn trace_prog<T : Sized + Debug + Copy + Add<Output = T> + Sub<Output = T>>(vm : &mut Vm<T>, p : &Prog<T>){
    println!("Initial State size({}):\n{:?}", vm.stack_size, &vm.stack[0..*vm.stack_size]);
    for (i, instr) in p.iter().enumerate() {
	let code = vm_execute(vm, instr);
	println!("{}^th State size({}) code({:?}):\n{:?}",
		 i, vm.stack_size, code, &vm.stack[0..*vm.stack_size].to_vec(),
	);
    }
}

fn main() {
    let prog : &Prog<i32> =
	&vec!(VmPush!(1), VmPush!(2), VmPlus!(),
	      VmPush!(1), VmPlus!(),
	      VmPush!(22), VmMinus!(), VmPop!(), VmPop!(), VmPop!()
	);
    let mut vm : Vm<i32> = Vm {
	stack_size : &mut 0,
	stack : &mut [0; STACK_CAPACITY],
	prog : prog.to_vec(),
	ip : 0
    };

    trace_prog(&mut vm, prog);
}
