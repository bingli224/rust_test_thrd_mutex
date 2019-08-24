
extern crate rand;

use std:: {
	thread,
	thread::sleep,
	time::Duration,

        io::Write,
        io::stdout,

	sync::Mutex
};
use rand::Rng;

struct Race {
	over : bool,
	position : i8
}

fn play ( race : &mut Race, power : i8, target : i8 ) {
	let mut rnd = rand::thread_rng ( );

	loop {
		sleep ( Duration::from_millis ( rnd.gen_range ( 100, 500 ) ) );

		if race.over {
			break;
		}

		if target < 0 {
			if race.position < target {
				break;
			}
		} else if race.position > target {
			break;
		}

		race.position += power;

		print ! ( "\r{pos:>w$}", w=(race.position + 12) as usize, pos="*" );
		stdout ( ).flush ( ).unwrap ( );

	}
}

fn main ( ) {
	let mut game = Race { 
		over : false,
		position : 0
	};

	let mut players = vec! [ ];

	players.push ( 
		thread::spawn ( move || {
			play ( &mut game, -1, -10 )
		} )
	);

	players.push ( 
		thread::spawn ( move || {
			play ( &mut game, 1, 10 );
		} )
	);

	for p in players {
		p.join ( ).unwrap ( );
	}
}
