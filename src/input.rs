
#![allow(dead_code)]

//FUNCIONA => 112 x1: 44 y x2: 12
pub static INPUT_I: &str = "
Simplex +
z = 2x1 + 2x2
2x1 + 1x2 <= 100
1x1 + 3x2 <= 80
1x1 + 0x2 <= 45
0x1 + 1x2 <= 100
";

pub static ALOOOOOO : &str = "
Simplex +
z = 1500x1 + 1400x2 + 1600x3 + 1450x4
1x1 + 0x2 + 1x3 + 0x4 >= 40
0x1 + 1x2 + 0x3 + 1x4 >= 70
2x1 - 1x2 + 2x3 - 1x4 <= 0
1x1 + 1x2 + 0x3 + 0x4 <= 180
0x1 + 0x2 + 1x3 + 1x4 <= 45
";

// FUNCIONA => 36K x1: 2 y x2: 6
pub static INPUT_II: &str = "
Simplex -
z = 30000x1 + 50000x2
1x1 + 0x2 <= 4
0x1 + 2x2 <= 12
3x1 + 2x2 <= 18
";

pub static INPUT_III: &str = "
Simplex -
z = 2x1 + 1x2 + 3x3
5x1 + 2x2 + 7x3 <= 420
3x1 + 2x2 + 5x3 >= 280
1x1 + 0x2 + 1x3 <= 100
";

// 0.66
pub static INPUT_IV: &str = "
Simplex -
z = 0.12x1 + 0.15x2
60x1 + 60x2 >= 300
12x1 + 6x2 >= 36
10x1 + 30x2 >= 90
";

// 1.08 => DEBERIA DAR 0.6 según el fakin phpsimplex
pub static INPUT_V: &str = "
Simplex -
z = 0.12x1 + 0.15x2
60x1 + 60x2 >= 300
12x1 + 6x2 >= 36
10x1 + 30x2 <= 90
";

// Resultado 327K
pub static INPUT_VI: &str = "
Simplex -
z = 1500x1 + 1400x2 + 1600x3 + 1450x4
1x1 + 0x2 + 1x3 + 0x4 >= 40
0x1 + 1x2 + 0x3 + 1x1 >= 70
2x1 - 1x2 + 2x3 - 1x4 <= 0
1x1 + 1x2 + 0x3 + 0x4 <= 180
0x1 + 0x2 + 1x3 + 1x4 <= 45
";
