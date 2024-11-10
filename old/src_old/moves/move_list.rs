use super::piece_move::Move;

const MAX_MOVES: usize = 128;

#[derive(Clone, Copy, Debug)]
pub struct MoveList {
	list: [Move; MAX_MOVES],
	pub length: usize,
}

impl MoveList {
	pub fn new() -> Self {
		Self {
			list: unsafe {
				let block = std::mem::MaybeUninit::uninit();
				block.assume_init()
			},
			length: 0
		}
	}

	pub fn push(&mut self, value: Move) {
		self.list[self.length] = value;
		self.length += 1;
	}

	pub fn get(&self, index: usize) -> Move {
		self.list[index]
	}

	pub fn get_mut(&mut self, index: usize) -> &mut Move {
		&mut self.list[index]
	}

	pub fn swap(&mut self, a: usize, b: usize) {
		unsafe {
			let ptr_a: *mut Move = &mut self.list[a];
			let ptr_b: *mut Move = &mut self.list[b];

			std::ptr::swap(ptr_a, ptr_b)
		}
	}
}