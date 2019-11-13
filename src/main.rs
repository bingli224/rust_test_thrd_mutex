
/**
 * By BingLi224
 * 22:28 THA 25/08/2019
 *
 * Test Mutex+Arc
 *
 * Each threads are the players that can change the racing position with
 * predefined role to be either -1 or 1. The player sleeps after each action.
 * The game stops when the game position reaches the positive-or-negative
 * target, and all other players will stop.
 */

extern crate rand;

use std:: {
	thread,
	thread::sleep,
	time::Duration,

	io::Write,
	io::stdout,

	sync::Mutex,
	sync::Arc
};
use rand::Rng;

struct Race {
	target : i8,
	over : bool,
	position : i8,
}

fn play ( arc : &mut Arc<Mutex<Race>>, power : i8 ) {
	let mut rnd = rand::thread_rng ( );
	let x_offset = 2 + {
		arc.lock ( ).unwrap ( ).target
	};

	loop {
		sleep ( Duration::from_millis ( rnd.gen_range ( 10, 1000 ) ) );

		let mut race = arc.lock ( ).unwrap ( );

		// check if the game is already over
		if race.over {
			break;
		}

		// update
		race.position += power;

		// print debug data
		print ! ( "{};\t", race.position );

		// print status
		if race.position < 0 {
			println ! ( "{none:<space1$}[{none:<space2$}{none:<<w$}|{none:>space3$}]",
				none="",
				space1=(x_offset-race.target) as usize,
				space2=(race.target+race.position) as usize,
				w=(-race.position) as usize,
				space3=(race.target) as usize
			);
                } else {
			print ! ( "{none:<space1$}[{none:<space2$}|",
				none="",
				space1=( x_offset - race.target ) as usize,
				space2=(race.target) as usize
			);
			if race.position > 0 {
				println ! ( "{none:>>w$}{none:>space$}]", none="", w=(race.position) as usize, space=(race.target-race.position) as usize );
			} else {
				println ! ( "{none:>space$}]", none="", space=race.target as usize );
			}
		}
		stdout ( ).flush ( ).unwrap ( );

		// check if end of the game
		if power < 0 {
			if race.position <= -race.target {
				race.over = true;
				break;
			}
		} else if race.position >= race.target {
			race.over = true;
			break;
		}

	}
}

fn main ( ) {
	let game = Race { 
		target	: 10,
		over	: false,
		position	: 0
	};

	let mut players = vec! [ ];

	let arc = Arc::new ( Mutex::new ( game ) );

	// create players
	for _ in 1..5 {
		{
		    let mut new_arc = arc.clone ( );
		    players.push ( 
			    thread::spawn ( move || {
				    play ( &mut new_arc, -1 );
			    } )
		    );
		}

		{
		    let mut new_arc = arc.clone ( );
		    players.push ( 
			    thread::spawn ( move || {
				    play ( &mut new_arc, 1 );
			    } )
		    );
		}
	}

	for p in players {
		p.join ( ).unwrap ( );
	}
}

mod test;
