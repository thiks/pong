use paddle::Paddle;

pub struct Ball {
    pub rectangle: [f64; 4],
    pub position: (f64, f64),
    pub angle: f64,
    pub reference: (f64, f64),
    pub direction: f64
}

impl Ball {
    pub fn update(&mut self) {
        self.position.0 += self.direction;
        self.position.1 = self.angle *
            (self.position.0 - self.reference.0) + self.reference.1;
    }

    pub fn hit(&mut self, paddle: &Paddle) -> Option<f64> {
        let min = paddle.position.1;
        let mid = paddle.position.1 + 20.0;
        let max = paddle.position.1 + 40.0;

        if self.position.1 > min && self.position.1 < max {
            let angle = (self.position.1 - mid).abs() as f64 / 20.0;
            if self.position.1 > mid {
                Some(angle)
            } else {
                Some(angle * -1.0)
            }
        } else {
            None
        }
    }

    pub fn reverse(&mut self, reverse_angle: bool) {
        self.direction *= -1.0;
        if reverse_angle { self.angle *= -1.0; }
        self.reference = self.position;
    }

    pub fn bounce(&mut self) {
        self.angle *= -1.0;
        self.reference = self.position;
    }

    pub fn reset(&mut self) {
        self.position = (240.0, 180.0);
        self.reference = (240.0, 180.0);
        self.angle = 0.0;
    }
}
