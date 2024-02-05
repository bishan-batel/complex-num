use std::ops::{Add, Mul, Sub};


#[derive(Debug, Default, PartialOrd, PartialEq, Copy, Clone)]
pub struct Complex {
	a: f32,
	b: f32,
}

#[macro_export]
macro_rules! complex {
	($a: expr, $b: expr) => {
		complex::Complex::new($a as f32, $b as f32)
	};

	($a: expr) => {
		complex::Complex::real($a as f32)
	}
}

#[macro_export]
macro_rules! imaginary {
	($i: expr) => {
		complex::Complex::imaginary($i as f32)
	};
}


#[allow(unused)]
impl Complex {
	/// Creates a new complex number
	///
	/// # Arguments
	///
	/// * `a`: Real Part
	/// * `b`: Imaginary Part
	///
	/// returns: Complex Number
	///
	/// # Examples
	///
	/// ```
	/// // 3 + 2i
	/// let num = Complex::new(3., 2.);
	///
	/// ```
	pub const fn new(a: f32, b: f32) -> Self { Self { a, b } }

	pub const fn real(a: f32) -> Self { Self::new(a, 0.) }

	pub const fn imaginary(b: f32) -> Self { Self::new(0., b) }


	pub fn abs(self) -> f32 {
		self.a.hypot(self.b)
	}

	pub fn exp(self) -> Self {
		Self::new(self.b.cos(), self.b.sin()) * self.a.exp()
	}

	pub fn ln(self) -> Self {
		Self::new(0.5 * self.a.mul_add(self.a, self.b * self.b).ln(), self.arg())
	}

	pub fn pow(self, m: Self) -> Self {
		(self.ln() * m).exp()
	}

	pub fn squared(self) -> Self {
		self * self
	}

	pub fn arg(self) -> f32 {
		(self.b / self.a).atan()
	}
}

impl From<f32> for Complex {
	fn from(real: f32) -> Self {
		Self::real(real)
	}
}

impl Add<Self> for Complex {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::new(self.a + rhs.a, self.b + rhs.b)
	}
}

impl Sub<Self> for Complex {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::new(self.a - rhs.a, self.b - rhs.b)
	}
}

impl Mul<f32> for Complex {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self::new(self.a * rhs, self.b * rhs)
	}
}

impl Mul<Self> for Complex {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let a = rhs.a;

		Self::new(
			a + self.a.mul_add(rhs.a, -(self.b * rhs.b)),
			self.a.mul_add(rhs.b, self.b * rhs.a),
		)
	}
}