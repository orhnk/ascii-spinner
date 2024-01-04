use std::fmt;

struct Canvas
{
    buffer: Vec<String>,
    bwidth: usize,
    bheight: usize,
    brush: char,
}

impl Canvas
{
    fn new(width: usize, height: usize) -> Self
    {
        Self {
            buffer: vec![String::from(" ".repeat(width)); height],
            bwidth: width,
            bheight: height,
            brush: '#',
        }
    }

    fn brush(&mut self, brush: char)
    {
        self.brush = brush;
    }

    fn new_square(len: usize) -> Self
    {
        Self::new(len, len)
    }

    fn clear_stroke(&mut self, posx: usize, posy: usize)
    {
        self.clear();
        self.stroke(posx, posy);
    }

    fn clear(&mut self)
    {
        self.buffer = vec![String::from(" ".repeat(self.bwidth)); self.bheight];
    }

    fn stroke(&mut self, posx: usize, posy: usize)
    {
        let spinchar = self.brush.to_string();
        self.put_char((posx, posy), &spinchar)
    }

    fn put_char(&mut self, pos: (usize, usize), spinchar: &str)
    {
        self.buffer[pos.0].replace_range((pos.1)..(pos.1 + 1), spinchar);
    }
}

impl fmt::Display for Canvas
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        self.buffer
            .iter()
            .for_each(|line| write!(f, "{}\n", line).unwrap());

        Ok(())
    }
}

struct CirclePosCalc
{
    offset: f64,
    angle: f64,
    size: f64,
}

impl CirclePosCalc
{
    fn new(offset: f64, size: f64) -> Self
    {
        Self {
            offset,
            size,
            angle: 0_f64,
        }
    }

    fn offset_angle(&mut self)
    {
        self.angle += self.offset;
        self.regulate_angle();
    }

    fn regulate_angle(&mut self)
    {
        if self.angle >= 360_f64 {
            self.angle -= 360_f64;
        }
    }

    pub fn yield_angle(&mut self) -> f64
    {
        self.offset_angle();
        self.angle
    }

    pub fn yield_xy(&mut self) -> (f64, f64)
    {
        let angle = self.yield_angle();
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();

        let y = cos_angle * self.size;
        let x = sin_angle * self.size;

        dbg!(x, y);

        (x, y)
    }
}

fn main()
{
    let mut canvas = {
        let mut canvas = Canvas::new(100, 50);
        canvas.brush('.');
        canvas
    };

    let mut pos_calc = CirclePosCalc::new(0.01, 25_f64);
    let sleep_duration = std::time::Duration::from_millis(10);
    let origin = (25.0, 25.0);

    loop {
        let (x, y) = pos_calc.yield_xy();
        let (x, y) = ((x + origin.0) as usize, (y + origin.1) as usize * 2);
        canvas.stroke(x, y);
        println!("{}", canvas);
        // clear screen
        print!("\x1B[2J\x1B[1;1H");
        std::thread::sleep(sleep_duration);
    }
}
