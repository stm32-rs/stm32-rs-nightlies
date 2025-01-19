#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crs_cr: CRS_CR,
    crs_cfgr: CRS_CFGR,
    crs_isr: CRS_ISR,
    crs_icr: CRS_ICR,
}
impl RegisterBlock {
    ///0x00 - CRS control register
    #[inline(always)]
    pub const fn crs_cr(&self) -> &CRS_CR {
        &self.crs_cr
    }
    ///0x04 - CRS configuration register
    #[inline(always)]
    pub const fn crs_cfgr(&self) -> &CRS_CFGR {
        &self.crs_cfgr
    }
    ///0x08 - CRS interrupt and status register
    #[inline(always)]
    pub const fn crs_isr(&self) -> &CRS_ISR {
        &self.crs_isr
    }
    ///0x0c - CRS interrupt flag clear register
    #[inline(always)]
    pub const fn crs_icr(&self) -> &CRS_ICR {
        &self.crs_icr
    }
}
/**CRS_CR (rw) register accessor: CRS control register

You can [`read`](crate::Reg::read) this register and get [`crs_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:CRS_CR)

For information about available fields see [`mod@crs_cr`]
module*/
pub type CRS_CR = crate::Reg<crs_cr::CRS_CRrs>;
///CRS control register
pub mod crs_cr;
/**CRS_CFGR (rw) register accessor: CRS configuration register

You can [`read`](crate::Reg::read) this register and get [`crs_cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs_cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:CRS_CFGR)

For information about available fields see [`mod@crs_cfgr`]
module*/
pub type CRS_CFGR = crate::Reg<crs_cfgr::CRS_CFGRrs>;
///CRS configuration register
pub mod crs_cfgr;
/**CRS_ISR (r) register accessor: CRS interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`crs_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:CRS_ISR)

For information about available fields see [`mod@crs_isr`]
module*/
pub type CRS_ISR = crate::Reg<crs_isr::CRS_ISRrs>;
///CRS interrupt and status register
pub mod crs_isr;
/**CRS_ICR (rw) register accessor: CRS interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`crs_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crs_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#CRS:CRS_ICR)

For information about available fields see [`mod@crs_icr`]
module*/
pub type CRS_ICR = crate::Reg<crs_icr::CRS_ICRrs>;
///CRS interrupt flag clear register
pub mod crs_icr;
