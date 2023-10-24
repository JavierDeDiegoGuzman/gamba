pub fn hello(args: Vec<String>) {
    println!("Hello from my_module!");

    for(index, args) in args.iter().enumerate().skip(1){
        println!("{}-{}", index, args);
    }
}
