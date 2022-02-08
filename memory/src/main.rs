#[derive(Debug)]

struct StructEx1 {
    a: i32,
    b: f64
}

fn some_struct_procedure(ex1: &StructEx1){
    println!("{:?}", ex1);
}
fn main() {

    let vars = StructEx1 {a:9,b:10.};
    some_struct_procedure(&vars);
    println!("{:?}", vars);
    let var_a = String::from("Hello World!");
    let var_b = var_a;

    println!(" try to print value of var_b {}",var_b);
    //println!(" try to print value of var_a {}",var_a); variable move it's not possible var_a loose ownership of string Hello World!
    
    let stack_i8 : i8 = 10;
    let _stack_f32 : f32 = 20.;
    let _stack_bool: bool = true;
    let _stack_char: char = 'a';

    // create a scope
    if stack_i8 == 3 {
        let inside_scope : i8 = 10;
        println!("the value of inside_scope is: {}",inside_scope);
    }
    // println!("the value of inside_scope is: {}",inside_scope); can't find this variable because is outside of it's scope

    let _heap_vector : Vec<i8> = Vec::new();
    let _heap_string : String = String::from("Hello World!");
    let heap_i8 : Box<i8> = Box::new(20);

    let stack_i8_2 = stack_i8; // in stack the copy is very cheap, stack_i8_2 and stack_i8 own different memory
    println!("stack_i8_2 is : {}",stack_i8_2);
    println!("stack_i8 is : {}",stack_i8);

    let heap_i8_2 : Box<i8> = heap_i8; // allocated memory remains intact, ownership is moved to heap_i8_2
    println!("heap_i8_2 is : {}",heap_i8_2);
    // println!("heap_i8 is : {}",heap_i8); borrow of moved value: `heap_i8` value borrowed here after move

    let stack_var_c : f64 = 100.;
    let heap_var_c : Box<f64> = Box::new(200.);

    stack_procedure(stack_var_c); // stack_var_c is copied to param stack_param

    println!("stack_param is still exist after is moved to stack_procedure: {}",stack_var_c);


    //heap_procedure(heap_var_c); // the owner of memory associated with heap_var_c gets transfered to heap_param

    //println!("heap_param is still exist after is moved to stack_procedure: {}",heap_var_c);

    heap_procedure(&heap_var_c);

    println!("heap_param is after called by procedure: {}",heap_var_c);


    // string slicing
    let some_str : &str = "Hello"; // &str is a pointer ... either heap or stack
    let string : String = String::from("World!");

    str_procedure(some_str,&string);

    println!("{} {}", some_str,string);


    // references

    let mut var_s :String = String::from("blabla");
    let var_s1 = &var_s;
    let var_s2 = &var_s;

    // var_s.push('b'); not possible, as var_s garantee to var_s1 and _s2 the data will not change 

    println!("{} {} {}", var_s,var_s1,var_s2);
    var_s.push('b');


    let  var_z :String = String::from("bla");
    let  var_h :String = String::from("... bla bla");

    let data: Vec<&String> = vec![&var_z,&var_h];
    println!("{}",comp_calcul(&data));
    println!("{} {} {:?}", var_z,var_h,data);
}

fn comp_calcul(_data : &Vec<&String>) -> i64{
    
    100000000000000
}
fn str_procedure (some_str : &str, string: &String) {
    println!("some string is : {} ",string);
    println!("some str is : {} ",some_str);
        
}
fn stack_procedure(mut stack_param: f64) {
    stack_param += 10.;
    println!("stack_param is : {}",stack_param);
}

fn heap_procedure(heap_param: &Box<f64>) {
    
    println!("heap_param is : {}",heap_param);
}

// fn heap_procedure(heap_param: Box<f64>) {
    
//     println!("heap_param is : {}",heap_param);
// }


