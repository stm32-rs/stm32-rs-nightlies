#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    wwdg_cr: WWDG_CR,
    wwdg_cfr: WWDG_CFR,
    wwdg_sr: WWDG_SR,
}
impl RegisterBlock {
    ///0x00 - WWDG control register
    #[inline(always)]
    pub const fn wwdg_cr(&self) -> &WWDG_CR {
        &self.wwdg_cr
    }
    ///0x04 - WWDG configuration register
    #[inline(always)]
    pub const fn wwdg_cfr(&self) -> &WWDG_CFR {
        &self.wwdg_cfr
    }
    ///0x08 - WWDG status register
    #[inline(always)]
    pub const fn wwdg_sr(&self) -> &WWDG_SR {
        &self.wwdg_sr
    }
}
/**WWDG_CR (rw) register accessor: WWDG control register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#WWDG:WWDG_CR)

For information about available fields see [`mod@wwdg_cr`] module*/
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CRrs>;
///WWDG control register
pub mod wwdg_cr;
/**WWDG_CFR (rw) register accessor: WWDG configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#WWDG:WWDG_CFR)

For information about available fields see [`mod@wwdg_cfr`] module*/
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFRrs>;
///WWDG configuration register
pub mod wwdg_cfr;
/**WWDG_SR (rw) register accessor: WWDG status register

You can [`read`](crate::Reg::read) this register and get [`wwdg_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#WWDG:WWDG_SR)

For information about available fields see [`mod@wwdg_sr`] module*/
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SRrs>;
///WWDG status register
pub mod wwdg_sr;
