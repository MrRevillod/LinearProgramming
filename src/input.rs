
#![allow(dead_code)]

pub static INPUT_I: &str = "
Simplex +
z = 2x1 + 2x2
2x1 + 2x2 <= 100
0x1 + 2x2 <= 80
1x1 + 0x2 <= 45
0x1 + 1x2 <= 100
";

pub static INPUT_II: &str = "
Simplex -
z = 2x1 + 2x2
2x1 + 2x2 <= 100
0x1 + 2x2 <= 80
1x1 + 0x2 <= 45
0x1 + 1x2 <= 100
";

pub static INPUT_III: &str = "
Simplex -
z = 2x1 + 1x2 + 3x3
5x1 + 2x2 + 7x3 <= 420
3x1 + 2x2 + 5x3 >= 280
1x1 + 0x2 + 1x3 <= 100
";

pub static INPUT_IV: &str = "
Simplex -
z = 0.12x1 + 0.15x2
60x1 + 60x2 >= 300
12x1 + 6x2 >= 36
10x1 + 30x2 <= 90
";

pub static INPUT_V: &str = "
Simplex -
z = 0.4x1 + 0.5x2 
0.3x1 + 0.1x2 <= 2.7
0.5x1 + 0.5x2 <= 6
0.6x1 + 0.4x2 >= 6
";


