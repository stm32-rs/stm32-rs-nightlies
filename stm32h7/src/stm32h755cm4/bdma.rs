#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ifcr: IFCR,
    ch: [CH; 8],
}
impl RegisterBlock {
    ///0x00 - DMA interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - DMA interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    ///Iterator for array of:
    ///0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
}
/**ISR (r) register accessor: DMA interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#BDMA:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DMA interrupt status register
pub mod isr;
/**IFCR (w) register accessor: DMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#BDMA:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///DMA interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR? registers
pub mod ch;
