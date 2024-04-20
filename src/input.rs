
#![allow(dead_code)]

pub static INPUT_I: &str = "
Graphic +
z = 2x1 + 2x2
2x1 + 1.2x2 <= 100
0x1 + 2x2 <= 80
1x1 + 0x2 <= 45
0x1 + 1x2 <= 100
1x1 + 0x2 >= 0
0x1 + 1x2 >= 0
";

pub static INPUT_II: &str = "
Simplex +
z = 2x1 + 2x2
2.2x1 + 1x2 <= 100
1x1 + 0.2x2 <= 80
1x1 + 0x2 <= 45
0x1 + 1x2 <= 100
";

pub static INPUT_III: &str = "
Simplex +
z = 5x1 + 8x2
1x1 + 1x2 <= 10
0x1 + 1x2 <= 8
";

pub static INPUT_IV: &str = "
Graphic +
z = 3x + 5y 
1x + 0y <= 4
0x + 2y <= 12
3x + 2y <= 18
0x + 1y >= 0
1x + 0y >= 0
";

pub static INPUT_V: &str = "
Simplex -
z = 0.12x1 + 0.15x2
60x1 + 60x2 >= 300
12x1 + 6x2 >= 36
10x1 + 30x2 >= 90
";