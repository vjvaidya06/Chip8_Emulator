use super::{CPU, CpuError};
use super::operations;
use crossterm::style::Color;

pub const FONTSET: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F];
        ];

pub(super) const LOOKUP: [fn(&mut CPU) -> Result<(), CpuError>;14] = [
    operations::op_screen,      //0???
    operations::jmp_addr,       //1NNN
    operations::call,           //2NNN
    operations::jmp_if_eq,      //3XNN
    operations::jmp_if_neq,     //4XNN
    operations::jmp_if_reg_eq,  //5XY0
    operations::set_reg,        //6XNN
    operations::add_reg,        //7XNN
    operations::reg_op,         //8XY?
    operations::jmp_if_reg_eq,  //9XY0
    operations::set_i_to_addr,  //ANNN
    operations::jmp_plus,       //BNNN
    operations::rand_and,       //CXNN
    operations::draw_sprite     //DXYN
];

//pub const THEMES: [char;2] = ['⬜', '⬛'];
//pub const THEMES: [char;2] = ['0', 'X'];

pub const COLORS: [Color; 17] = [
        Color::Reset,
        // Dark Base Colors
        Color::Black,
        Color::DarkGrey,
        Color::DarkRed,
        Color::DarkGreen,
        Color::DarkYellow,
        Color::DarkBlue,
        Color::DarkMagenta,
        Color::DarkCyan,
        // Light Base Colors
        Color::Grey,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        // Custom / Dynamic Values
    ];
