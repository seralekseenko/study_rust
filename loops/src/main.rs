fn main() {
    loop_example();
    breake_with_mark();
    try_continue_with_mark();
}



fn loop_example() {
    println!("this is a LOOP");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


fn breake_with_mark() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn try_continue_with_mark() {
    println!("TRY CONTINUE");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        count += 1;

        loop {
            println!("remaining = {remaining}");
            remaining -= 1;

	    if remaining > 6 {
                println!("continue internal");
		continue;
            }
            if count < 3 {
	        println!("continue external");
                continue 'counting_up;
            }
            if remaining == 6 && count != 3 {
	        println!("remaining: {remaining} → break internal loop!");
		break;
	    }
	    if count == 3 {
	        println!("count: {count} → break external loop!");
		break 'counting_up;
	    }
        }

    }
    println!("FINISH TRY CONTINUE");
}
