
#[cfg(test)]
mod tests {

    use crate::play;
    use crate::Race;

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
        

	#[test]
	fn test_play ( ) {
		let game = Race { 
			target	: 30,
			over	: false,
			position	: 0
		};

		let mut players = vec! [ ];

		let arc = Arc::new ( Mutex::new ( game ) );

		// create players
		for _ in 1..25 {
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
}

