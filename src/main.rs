extern crate rand;

use rand::Rng;
use std::fmt;
use std::io;


static SUITS:[&'static str; 4] = ["Club", "Diamond", "Heart", "Spade"];

static FACES:[&'static str; 13] = ["Ace", "2", "3", "4", "5",
									"6", "7", "8", "9", "10",
									"King", "Queen", "Jack"];

static NUM_SUITS: i32 = 4;
static NUM_FACES: i32 = 13;


fn shuffle_vec_in_place<T>(vs: &mut Vec<T>) {
	let vlen = vs.len();

	for shuffled in 0..(vlen-1) {
		let random_idx = rand::thread_rng().gen_range(shuffled, vlen);
		let item = vs.swap_remove(random_idx);
		vs.insert(shuffled, item);
	}
}


struct Card {
	suit: i32,
	face: i32,
}

impl fmt::Display for Card {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "C: {} {}",
				SUITS[self.suit as usize],
				FACES[self.face as usize])
	}
}


struct Deck {
	cards: Vec<Card>,
}

impl Deck {

	fn new() -> Deck {
		let mut cards = Vec::new();

		for s in 0..NUM_SUITS {
			for f in 0..NUM_FACES {
				cards.push(Card{ suit: s,
									face: f});
			}
		}

		shuffle_vec_in_place(&mut cards);

		Deck {
			cards: cards,
		}
	}

	#[allow(dead_code)]
	fn shuffle_deck(&mut self) {
		shuffle_vec_in_place(&mut self.cards);
	}

	fn pop_card(&mut self) -> Option<Card> {
		return self.cards.pop();
	}

	#[allow(dead_code)]
	fn print_deck(&self) {
		println!("Deck ->");
		for c in self.cards.iter() {
			println!("\t{}  {}", SUITS[c.suit as usize], FACES[c.face as usize]);
		}
	}  
}


enum PlayerState {
	CanAsk,
	Stay,
	BlackJack,
	Busted,
}

struct Player {
	id: i32,
	hand: Vec<Card>,
	is_dealer: bool,
	score: i32,
	state: PlayerState,
}

impl Player {

	fn new(id: i32, is_dealer: bool) -> Player {
		Player {
			id: id,
			hand: Vec::new(),
			is_dealer: is_dealer,
			score: 0,
			state: PlayerState::CanAsk,
		}
	}

	fn deal_card(&mut self, c: Card) {
		// Update player score
		match c.face {
			v @ 0 ... 9 => self.score += v+1,
			10 ... 12 => self.score += 10,
			_ => (),
		}

		self.hand.push(c);

		// Update player state
		match self.score {
			0 ... 16 => self.state = PlayerState::CanAsk,
			17 ... 20 => self.state = PlayerState::Stay,
			21 => self.state = PlayerState::BlackJack,
			_ => self.state = PlayerState::Busted,
		}
	}
}

impl fmt::Display for Player {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let hand_str = self.hand.iter()
			.map(|ref c| format!("\n\t-> {}", c))
			.collect::<Vec<String>>()
			.connect("");

		write!(fmt, "\nPlayer: {} score: {}\n\t{}",
				self.id, self.score, hand_str)
	}
}


struct Game {
	d: Deck,
	ps: Vec<Player>,
	end_game: bool,
}

impl Game {
	fn new(num_players: i32) -> Game {
		let mut players: Vec<Player> = Vec::new();

		// dealer
		players.push(Player::new(-1, true));

		// players
		for i in 0..num_players {
			players.push(Player::new(i, false));
		}

		Game {
			d: Deck::new(),
			ps: players,
			end_game: false,
		}
	}

	fn start_game(&mut self) {
		
		// bootstrap game with dealing 2 cards to each player
		for player in &mut self.ps {
			for _ in 0..2 {
				let c = self.d.pop_card();
				match c {
					Some(tc) => player.deal_card(tc),
					None => panic!("No card left in deck"), 
				}
			}
		}

		// game loop
		while self.end_game == false {
			let mut has_turn = false;

			for player in &mut self.ps {
				// let player = &mut self.ps[player_idx];

				match player.state {
					PlayerState::CanAsk => {
							has_turn = true;

							let c = self.d.pop_card();

							match c {
								Some(tc) => player.deal_card(tc),
								None => panic!("No card left in deck"),
							};
						},
					PlayerState::Stay => (),
					PlayerState::BlackJack => if player.is_dealer { self.end_game = true } else {},
					PlayerState::Busted => if player.is_dealer { self.end_game = true } else {},
				}
			}

			// Terminal case, where no player could ask for a card
			if has_turn == false { self.end_game = true }
		}

		// Execution reaching this point means that we have reached any of the
		// terminal case for the game loop.
		let dealer = self.ps.remove(0);
		let players_in_game = self.ps.iter().filter(|&p| match p.state {
			PlayerState::BlackJack => true,
			PlayerState::Stay => true,
			_ => false,
		});
		
		println!("Dealer {}\n", dealer);

		match dealer.state {
			PlayerState::Busted => {
				println!("Winners:");
				for p in players_in_game.collect::<Vec<&Player>>() {
					println!("{}", p);
				}
			},
			PlayerState::BlackJack => {
				println!("Winners:");
				for p in players_in_game.filter(|&p| match p.state {PlayerState::BlackJack => true, _ => false})
					.collect::<Vec<&Player>>() {
						println!("{}", p);
					}
			},
			PlayerState::Stay => {
				println!("Winners:");
				for p in players_in_game.filter(|&p| p.score > dealer.score)
					.collect::<Vec<&Player>>() {
						println!("{}", p);
					}
			},
			_ => println!("Dealer can't be CanAsk case after reaching the terminal case"),
		}

		println!("\nThe End");

	}

	#[allow(dead_code)]
	fn display_status(&mut self) {
		for p in self.ps.iter() {
			println!("{}", p);
		}
	}
}


fn main() {
	println!("Welcome to BlackJack");
	println!("Enter the no. of players in the game");

	let mut line = String::new();

	io::stdin().read_line(&mut line)
		.ok()
		.expect("failed to read line");

	let mut num_players:i32;

	match line.trim().parse::<i32>() {
		Ok(val) => num_players = val,
		Err(e) => panic!("{}", e),
	}

	let mut g = Game::new(num_players);
	g.start_game();
}
