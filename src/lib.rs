pub mod rpn{

    pub fn rpn(string: &Vec<String>)->f64{
        let mut stack: Vec<f64> = Vec::new();
        for argument in string{
            match argument.as_str(){
                "+"=> {
                    let r =stack.pop().unwrap();
                    let l =stack.pop().unwrap();
                    stack.push(l+r);
                },
                "-"=> {
                    let r =stack.pop().unwrap();
                    let l =stack.pop().unwrap();
                    stack.push(l-r);
                },
                
                "/"=> {
                    let r =stack.pop().unwrap();
                    let l =stack.pop().unwrap();
                    stack.push(l/r);
                },
                "*"|"x"=> {
                    let r =stack.pop().unwrap();
                    let l =stack.pop().unwrap();
                    stack.push(l*r);
                },
                _ => stack.push(
                    argument.parse::<f64>().unwrap()
                ),
                
            }
        }
        return stack.pop().unwrap()
    }
}
#[test]
fn first(){
    let mut arg: Vec<String> = Vec::new();
    arg.push("2".to_string());
    arg.push("2".to_string());
    arg.push("+".to_string());
    arg.push("4".to_string());
    arg.push("*".to_string());

    assert_eq!( rpn::rpn(&arg) ,16.0 );
}