#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gisr0: GISR0,
    _reserved1: [u8; 0x3c],
    ch: [CH; 16],
}
impl RegisterBlock {
    ///0x00 - MDMA Global Interrupt/Status Register
    #[inline(always)]
    pub const fn gisr0(&self) -> &GISR0 {
        &self.gisr0
    }
    ///0x40..0x440 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x40..0x440 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
/**GISR0 (r) register accessor: MDMA Global Interrupt/Status Register

You can [`read`](crate::Reg::read) this register and get [`gisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#MDMA:GISR0)

For information about available fields see [`mod@gisr0`] module*/
pub type GISR0 = crate::Reg<gisr0::GISR0rs>;
///MDMA Global Interrupt/Status Register
pub mod gisr0;
///Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
pub use self::ch::CH;
///Cluster
///Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
pub mod ch;
