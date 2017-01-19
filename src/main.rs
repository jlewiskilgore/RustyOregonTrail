extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	print_introduction();

	//Give player instructions if they choose
	let mut need_instructions = String::new();

	io::stdin()
		.read_line(&mut need_instructions)
		.expect("failed to read line");

	if need_instructions.trim().to_uppercase() == "YES" {
		print_instructions();
	}	

	//Player makes initial purchases
	initial_shopping();
}

fn print_introduction() {
	println!("");
	println!("*==================================================*");
	println!("|                                				     |");
	println!("|                                                  |");
	println!("|                 THE OREGON TRAIL                 |");
	println!("|                                                  |");
	println!("|                                                  |");
	println!("*==================================================*");
	println!("");
	println!("DO YOU NEED INSTRUCTIONS (YES/NO)");
}

fn print_instructions() {
	println!("");
	println!("THIS PROGRAM SIMULATES A TRIP OVER THE OREGON TRAIL FROM");
	println!("INDEPENDENCE, MISSOURI TO OREGON CITY, OREGON IN 1847.");
	println!("YOUR FAMILY OF FIVE WILL COVER THE 2000 MILE OREGON TRAIL");
	println!("IN 5-6 MONTHS --- IF YOU MAKE IT ALIVE.");
	println!("");
	println!("YOU HAD SAVED $900 TO SPEND FOR THE TRIP, AND YOU'VE JUST");
	println!("   PAID $200 FOR A WAGON.");
	println!("YOU WILL NEED TO SPEND THE REST OF YOUR MONEY ON THE");
	println!("   FOLLOWING ITEMS:");
	println!("");
	println!("     OXEN - YOU CAN SPEND $200-$300 ON YOUR TEAM");
	println!("            THE MORE YOU SPEND, THE FASTER YOU'LL GO");
	println!("               BECAUSE YOU'LL HAVE BETTER ANIMALS");
	println!("");
	println!("     FOOD - THE MORE YOU HAVE, THE LESS CHANCE THERE");
	println!("               IS OF GETTING SICK");
	println!("");
	println!("     AMMUNITION - $1 BUYS A BELT OF 50 BULLETS");
	println!("            YOU WILL NEED BULLETS FOR ATTACKS BY ANIMALS");
	println!("               AND BANDITS, AND FOR HUNTING FOOD");
	println!("");
	println!("     CLOTHING - THIS IS ESPECIALLY IMPORTANT FOR THE COLD");
	println!("               WEATHER YOU WILL ENCOUNTER WHEN CROSSING");
	println!("               THE MOUNTAINS");
	println!("");
	println!("     MISCELLANEOUS SUPPLIES - THIS INCLUDES MEDICINE AND");
	println!("               OTHER THINGS YOU WILL NEED FOR SICKNESS");
	println!("               AND EMERGENCY REPAIRS");
	println!("");
	println!("");
	println!("YOU CAN SPEND ALL YOUR MONEY BEFORE YOU START YOUR TRIP -");
	println!("OR YOU CAN SAVE SOME OF YOUR CASH TO SPEND AT FORTS ALONG");
	println!("THE WAY WHEN YOU RUN LOW.  HOWEVER, ITEMS COST MORE AT");
	println!("THE FORTS.  YOU CAN ALSO GO HUNTING ALONG THE WAY TO GET");
	println!("MORE FOOD.");
	println!("WHENEVER YOU HAVE TO USE YOUR TRUSTY RIFLE ALONG THE WAY,");
	println!("YOU WILL SEE THE WORDS: TYPE BANG.  THE FASTER YOU TYPE");
	println!("IN THE WORD 'BANG' AND HIT THE 'RETURN' KEY, THE BETTER");
	println!("LUCK YOU'LL HAVE WITH YOUR GUN.");
	println!("");
	println!("WHEN ASKED TO ENTER MONEY AMOUNTS, DON'T USE A '$'.");
	println!("");
	println!("GOOD LUCK!!!");
}

fn initial_shopping() {
	let mut oxen = 0;
	let mut food = 0;
	let mut ammo = 0;
	let mut clothing = 0;
	let mut misc_supplies = 0;
	let mut total_spent = 0;

	let oxen_low = 200;
	let oxen_high = 300;
	let min_amount = 0;
	let budget = 700;

	println!("");
	//Player can spend between 200 and 300 on Oxen
	println!("HOW MUCH DO YOU WANT TO SPEND ON YOUR OXEN TEAM");
	
	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("failed to read line");
	let amount: u32 = amount.trim().parse().expect("Please type a number!");

	if &amount < &oxen_low {
		println!("NOT ENOUGH");
	}
	if &amount > &oxen_high {
		println!("TOO MUCH");
	}

	oxen = amount;

	//Player can spend anything greater than 0 on Food
	println!("HOW MUCH DO YOU WANT TO SPEND ON FOOD");

	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("failed to read line");
	let amount: u32 = amount.trim().parse().expect("Please type a number!");
	
	if &amount < &min_amount {
		println!("IMPOSSIBLE");
	}

	food = amount;

	//Player can spend anything greater than 0 on Ammunition
	println!("HOW MUCH DO YOU WANT TO SPEND ON AMMUNITION");

	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("failed to read line");
	let amount: u32 = amount.trim().parse().expect("Please type a number!");

	if &amount < &min_amount {
		println!("IMPOSSIBLE");
	}

	ammo = amount;
	
	//Player can spend anything greater than 0 on Clothing
	println!("HOW MUCH DO YOU WANT TO SPEND ON CLOTHING");

	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("failed to read line");
	let amount: u32 = amount.trim().parse().expect("Please type a number!");

	if &amount < &min_amount {
		println!("IMPOSSIBLE");
	}

	clothing = amount;

	//Player can spend anything greater than 0 on Misc Supplies
	println!("HOW MUCH DO YOU WANT TO SPEND ON MISCELANEOUS SUPPLIES");

	let mut amount = String::new();
	io::stdin().read_line(&mut amount).expect("failed to read line");
	let amount: u32 = amount.trim().parse().expect("Please type a number!");

	if &amount < &min_amount {
		println!("IMPOSSIBLE");
	}

	misc_supplies = amount;

	//Sum of all spent must be less than $700
	total_spent = oxen + food + ammo + clothing + misc_supplies;
	println!("{}", total_spent);

	if &total_spent > &budget {
		println!("YOU OVERSPENT--YOU ONLY HAD $700 TO SPEND.  BUY AGAIN");
	}
	println!("");
	println!("AFTER ALL YOUR PURCHASES, YOU NOW HAVE {} DOLLARS LEFT", &budget - &total_spent);
	println!("");
}