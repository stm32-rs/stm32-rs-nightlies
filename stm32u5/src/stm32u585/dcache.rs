#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dcache_cr: DCACHE_CR,
    dcache_sr: DCACHE_SR,
    dcache_ier: DCACHE_IER,
    dcache_fcr: DCACHE_FCR,
    dcache_rhmonr: DCACHE_RHMONR,
    dcache_rmmonr: DCACHE_RMMONR,
    _reserved6: [u8; 0x08],
    dcache_whmonr: DCACHE_WHMONR,
    dcache_wmmonr: DCACHE_WMMONR,
    dcache_cmdrsaddrr: DCACHE_CMDRSADDRR,
    dcache_cmdreaddrr: DCACHE_CMDREADDRR,
}
impl RegisterBlock {
    ///0x00 - DCACHE control register
    #[inline(always)]
    pub const fn dcache_cr(&self) -> &DCACHE_CR {
        &self.dcache_cr
    }
    ///0x04 - DCACHE status register
    #[inline(always)]
    pub const fn dcache_sr(&self) -> &DCACHE_SR {
        &self.dcache_sr
    }
    ///0x08 - DCACHE interrupt enable register
    #[inline(always)]
    pub const fn dcache_ier(&self) -> &DCACHE_IER {
        &self.dcache_ier
    }
    ///0x0c - DCACHE flag clear register
    #[inline(always)]
    pub const fn dcache_fcr(&self) -> &DCACHE_FCR {
        &self.dcache_fcr
    }
    ///0x10 - DCACHE read-hit monitor register
    #[inline(always)]
    pub const fn dcache_rhmonr(&self) -> &DCACHE_RHMONR {
        &self.dcache_rhmonr
    }
    ///0x14 - DCACHE read-miss monitor register
    #[inline(always)]
    pub const fn dcache_rmmonr(&self) -> &DCACHE_RMMONR {
        &self.dcache_rmmonr
    }
    ///0x20 - write-hit monitor register
    #[inline(always)]
    pub const fn dcache_whmonr(&self) -> &DCACHE_WHMONR {
        &self.dcache_whmonr
    }
    ///0x24 - write-miss monitor register
    #[inline(always)]
    pub const fn dcache_wmmonr(&self) -> &DCACHE_WMMONR {
        &self.dcache_wmmonr
    }
    ///0x28 - command range start address register
    #[inline(always)]
    pub const fn dcache_cmdrsaddrr(&self) -> &DCACHE_CMDRSADDRR {
        &self.dcache_cmdrsaddrr
    }
    ///0x2c - command range start address register
    #[inline(always)]
    pub const fn dcache_cmdreaddrr(&self) -> &DCACHE_CMDREADDRR {
        &self.dcache_cmdreaddrr
    }
}
/**DCACHE_CR (rw) register accessor: DCACHE control register

You can [`read`](crate::Reg::read) this register and get [`dcache_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_CR)

For information about available fields see [`mod@dcache_cr`]
module*/
pub type DCACHE_CR = crate::Reg<dcache_cr::DCACHE_CRrs>;
///DCACHE control register
pub mod dcache_cr;
/**DCACHE_SR (r) register accessor: DCACHE status register

You can [`read`](crate::Reg::read) this register and get [`dcache_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_SR)

For information about available fields see [`mod@dcache_sr`]
module*/
pub type DCACHE_SR = crate::Reg<dcache_sr::DCACHE_SRrs>;
///DCACHE status register
pub mod dcache_sr;
/**DCACHE_IER (rw) register accessor: DCACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dcache_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_IER)

For information about available fields see [`mod@dcache_ier`]
module*/
pub type DCACHE_IER = crate::Reg<dcache_ier::DCACHE_IERrs>;
///DCACHE interrupt enable register
pub mod dcache_ier;
/**DCACHE_FCR (w) register accessor: DCACHE flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_FCR)

For information about available fields see [`mod@dcache_fcr`]
module*/
pub type DCACHE_FCR = crate::Reg<dcache_fcr::DCACHE_FCRrs>;
///DCACHE flag clear register
pub mod dcache_fcr;
/**DCACHE_RHMONR (r) register accessor: DCACHE read-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_rhmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_RHMONR)

For information about available fields see [`mod@dcache_rhmonr`]
module*/
pub type DCACHE_RHMONR = crate::Reg<dcache_rhmonr::DCACHE_RHMONRrs>;
///DCACHE read-hit monitor register
pub mod dcache_rhmonr;
/**DCACHE_RMMONR (r) register accessor: DCACHE read-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_rmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_RMMONR)

For information about available fields see [`mod@dcache_rmmonr`]
module*/
pub type DCACHE_RMMONR = crate::Reg<dcache_rmmonr::DCACHE_RMMONRrs>;
///DCACHE read-miss monitor register
pub mod dcache_rmmonr;
/**DCACHE_WHMONR (r) register accessor: write-hit monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_whmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_WHMONR)

For information about available fields see [`mod@dcache_whmonr`]
module*/
pub type DCACHE_WHMONR = crate::Reg<dcache_whmonr::DCACHE_WHMONRrs>;
///write-hit monitor register
pub mod dcache_whmonr;
/**DCACHE_WMMONR (r) register accessor: write-miss monitor register

You can [`read`](crate::Reg::read) this register and get [`dcache_wmmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_WMMONR)

For information about available fields see [`mod@dcache_wmmonr`]
module*/
pub type DCACHE_WMMONR = crate::Reg<dcache_wmmonr::DCACHE_WMMONRrs>;
///write-miss monitor register
pub mod dcache_wmmonr;
/**DCACHE_CMDRSADDRR (rw) register accessor: command range start address register

You can [`read`](crate::Reg::read) this register and get [`dcache_cmdrsaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_cmdrsaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_CMDRSADDRR)

For information about available fields see [`mod@dcache_cmdrsaddrr`]
module*/
pub type DCACHE_CMDRSADDRR = crate::Reg<dcache_cmdrsaddrr::DCACHE_CMDRSADDRRrs>;
///command range start address register
pub mod dcache_cmdrsaddrr;
/**DCACHE_CMDREADDRR (rw) register accessor: command range start address register

You can [`read`](crate::Reg::read) this register and get [`dcache_cmdreaddrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_cmdreaddrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#DCACHE:DCACHE_CMDREADDRR)

For information about available fields see [`mod@dcache_cmdreaddrr`]
module*/
pub type DCACHE_CMDREADDRR = crate::Reg<dcache_cmdreaddrr::DCACHE_CMDREADDRRrs>;
///command range start address register
pub mod dcache_cmdreaddrr;
