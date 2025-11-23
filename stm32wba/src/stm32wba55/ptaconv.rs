#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ptaconv_actcr: PTACONV_ACTCR,
    ptaconv_pricr: PTACONV_PRICR,
    ptaconv_cr: PTACONV_CR,
}
impl RegisterBlock {
    ///0x00 - PTACONV active control register
    #[inline(always)]
    pub const fn ptaconv_actcr(&self) -> &PTACONV_ACTCR {
        &self.ptaconv_actcr
    }
    ///0x04 - PTACONV priority control register
    #[inline(always)]
    pub const fn ptaconv_pricr(&self) -> &PTACONV_PRICR {
        &self.ptaconv_pricr
    }
    ///0x08 - PTACONV control register
    #[inline(always)]
    pub const fn ptaconv_cr(&self) -> &PTACONV_CR {
        &self.ptaconv_cr
    }
}
/**PTACONV_ACTCR (rw) register accessor: PTACONV active control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_actcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_actcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_ACTCR)

For information about available fields see [`mod@ptaconv_actcr`] module*/
pub type PTACONV_ACTCR = crate::Reg<ptaconv_actcr::PTACONV_ACTCRrs>;
///PTACONV active control register
pub mod ptaconv_actcr;
/**PTACONV_PRICR (rw) register accessor: PTACONV priority control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_pricr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_pricr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_PRICR)

For information about available fields see [`mod@ptaconv_pricr`] module*/
pub type PTACONV_PRICR = crate::Reg<ptaconv_pricr::PTACONV_PRICRrs>;
///PTACONV priority control register
pub mod ptaconv_pricr;
/**PTACONV_CR (rw) register accessor: PTACONV control register

You can [`read`](crate::Reg::read) this register and get [`ptaconv_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptaconv_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#PTACONV:PTACONV_CR)

For information about available fields see [`mod@ptaconv_cr`] module*/
pub type PTACONV_CR = crate::Reg<ptaconv_cr::PTACONV_CRrs>;
///PTACONV control register
pub mod ptaconv_cr;
