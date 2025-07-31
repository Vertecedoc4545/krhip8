#[inline(always)]
pub fn x(upcode: u16) -> usize {
    return ((upcode & 0x0F00) >> 8) as usize;
}
#[inline(always)]
pub fn y(upcode: u16) -> usize {
    return ((upcode & 0x00F0) >> 4) as usize;
}
#[inline(always)]
pub fn n(upcode: u16) -> u16 {
    return (upcode & 0x000F);
}
#[inline(always)]
pub fn nnn(upcode: u16) -> u16 {
    return (upcode & 0x0FFF);
}
#[inline(always)]
pub fn kk(upcode: u16) -> u8 {
    return (upcode & 0x00FF) as u8;
}
#[inline(always)]
pub fn bg_id(upcode: u16) -> u16 {
    return (upcode & 0xF000) >> 12;
}
#[inline(always)]
pub fn end_id(upcode: u16) -> u16 {
    return (upcode & 0x000F);
}
#[inline(always)]
pub fn two_end_id(upcode: u16) -> u16 {
    return (upcode & 0x00FF);
}
#[inline(always)]
pub fn index(xx: usize, yy: usize) -> usize {
    return (yy * 64) + xx;
}
