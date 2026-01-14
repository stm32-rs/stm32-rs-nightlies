#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    pll1: PLL1,
    _reserved1: [u8; 0x08],
    tune: TUNE,
    _reserved2: [u8; 0x08],
    ldo: LDO,
}
impl RegisterBlock {
    ///0x00 - USBPHYC PLL1 control register
    #[inline(always)]
    pub const fn pll1(&self) -> &PLL1 {
        &self.pll1
    }
    ///0x0c - USBPHYC tuning control register
    #[inline(always)]
    pub const fn tune(&self) -> &TUNE {
        &self.tune
    }
    ///0x18 - USBPHYC LDO control and status register
    #[inline(always)]
    pub const fn ldo(&self) -> &LDO {
        &self.ldo
    }
}
/**PLL1 (rw) register accessor: USBPHYC PLL1 control register

You can [`read`](crate::Reg::read) this register and get [`pll1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:PLL1)

For information about available fields see [`mod@pll1`] module*/
pub type PLL1 = crate::Reg<pll1::PLL1rs>;
///USBPHYC PLL1 control register
pub mod pll1;
/**TUNE (rw) register accessor: USBPHYC tuning control register

You can [`read`](crate::Reg::read) this register and get [`tune::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:TUNE)

For information about available fields see [`mod@tune`] module*/
pub type TUNE = crate::Reg<tune::TUNErs>;
///USBPHYC tuning control register
pub mod tune;
/**LDO (rw) register accessor: USBPHYC LDO control and status register

You can [`read`](crate::Reg::read) this register and get [`ldo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F730.html#USBPHYC:LDO)

For information about available fields see [`mod@ldo`] module*/
pub type LDO = crate::Reg<ldo::LDOrs>;
///USBPHYC LDO control and status register
pub mod ldo;
