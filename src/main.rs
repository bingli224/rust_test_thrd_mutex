
/**
 * By BingLi224
 * 01:43 THA 25/08/2019
 *
 * Test Mutex+Arc
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
	over : bool,
	position : i8
}

//fn play ( race : &mut Race, power : i8, target : i8 ) {
fn play ( arc : &mut Arc<Mutex<Race>>, power : i8, target : i8 ) {
	let mut rnd = rand::thread_rng ( );
        let MAX_WIDTH = 12;

	loop {
		sleep ( Duration::from_millis ( rnd.gen_range ( 100, 1000 ) ) );

                let mut race = arc.lock ( ).unwrap ( );

		if race.over {
			break;
		}

		if power < 0 {
                    if race.position <= target {
                        race.over = true;
                        break;
                    }
		} else if race.position >= target {
                    race.over = true;
                    break;
		}

		race.position += power;

                println ! ( "{mark:>w$}", w=(MAX_WIDTH + race.position) as usize, mark="*" );
                /*
                if race.position < 0 {
                    println ! ( "{mark:*<w$}", mark="", w=(-race.position - 1) as usize );
                    //println ! ( "{mark:>w$}", space_sz=(MAX_WIDTH + race.position) as usize, mark=race.position );
                    //print ! ( "\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08{pos:>w1$}", w1=MAX_WIDTH, pos=race.position );
                } else {
                    println ! ( "{mark:*<w$}", mark="", w=(race.position - 1) as usize );
                    //print ! ( "\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08{space:>w1$}{pos:>w2$}", space=" ", w1=MAX_WIDTH, w2=(race.position - 1) as usize, pos=race.position );
                }
                */
                //print ! ( "\r                 \r{mark:>w$}", w=(MAX_WIDTH + race.position) as usize, mark=race.position );
                //println ! ( "{mark:>w$}", w=(MAX_WIDTH + race.position) as usize, mark=race.position );
		stdout ( ).flush ( ).unwrap ( );

	}
}

fn main ( ) {
	let mut game = Race { 
		over : false,
		position : 0
	};

	let mut players = vec! [ ];

        let arc = Arc::new ( Mutex::new ( game ) );

        {
            let mut new_arc = arc.clone ( );
            players.push ( 
                    thread::spawn ( move || {
                            play ( &mut new_arc, -1, -8 )
                    } )
            );
        }

        {
            let mut new_arc = arc.clone ( );
            players.push ( 
                    thread::spawn ( move || {
                            play ( &mut new_arc, 1, 8 );
                    } )
            );
        }

	for p in players {
		p.join ( ).unwrap ( );
	}
}
