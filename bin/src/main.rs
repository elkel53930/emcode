use emcode::*;

fn main() {
    let test_script = "let $x = 123.456;
    let $s = 0.0;
    let $last = 0.0;
    
    if ($x > 1.0){
        $s = $x
    } else {
        $s = 1.0
    };
    
    let $flag = 1;
    
    while $flag {
        $last = $s;
        $s = ((($x / $s) + $s) / 2.0);
        if ($s >= $last) {
            $flag = 0
        } else {
            pass
        }
    }";
    let mut env = interpreter::Environment::new();
    let parser = parser::p_statement();
    let mut s = parser::ParserState::new(test_script);
    let result = parser(&mut s);
    match result {
        Ok(statement) => {
            let _ = interpreter::evaluate_statement(&mut env, &statement);
        }
        Err(e) => {
            println!("Error at {} : {}", s.pos_string(), e);
        }
    }
    println!("{:?}", env.variables);
}
