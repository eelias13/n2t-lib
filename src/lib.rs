pub mod cpu;
pub mod hdl;
pub mod vm;

pub mod test_script;

static SP: usize = 0;
static LCL: usize = 1;
static ARG: usize = 2;
static THIS: usize = 3;
static THAT: usize = 4;
static SCREEN: usize = 16384;
static KBD: usize = 24576;
static PTR: usize = 3;
static TEMP: usize = 5;
static STATIC: usize = 16;