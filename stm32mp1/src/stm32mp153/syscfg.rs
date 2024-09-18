#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    syscfg_bootr: SYSCFG_BOOTR,
    syscfg_pmcsetr: SYSCFG_PMCSETR,
    _reserved2: [u8; 0x10],
    syscfg_ioctrlsetr: SYSCFG_IOCTRLSETR,
    syscfg_icnr: SYSCFG_ICNR,
    syscfg_cmpcr: SYSCFG_CMPCR,
    syscfg_cmpensetr: SYSCFG_CMPENSETR,
    syscfg_cmpenclrr: SYSCFG_CMPENCLRR,
    syscfg_cbr: SYSCFG_CBR,
    _reserved8: [u8; 0x14],
    syscfg_pmcclrr: SYSCFG_PMCCLRR,
    _reserved9: [u8; 0x10],
    syscfg_ioctrlclrr: SYSCFG_IOCTRLCLRR,
    _reserved10: [u8; 0x0398],
    syscfg_verr: SYSCFG_VERR,
    syscfg_ipidr: SYSCFG_IPIDR,
    syscfg_sidr: SYSCFG_SIDR,
}
impl RegisterBlock {
    ///0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
    #[inline(always)]
    pub const fn syscfg_bootr(&self) -> &SYSCFG_BOOTR {
        &self.syscfg_bootr
    }
    ///0x04 - SYSCFG peripheral mode configuration set register
    #[inline(always)]
    pub const fn syscfg_pmcsetr(&self) -> &SYSCFG_PMCSETR {
        &self.syscfg_pmcsetr
    }
    ///0x18 - SYSCFG IO control register
    #[inline(always)]
    pub const fn syscfg_ioctrlsetr(&self) -> &SYSCFG_IOCTRLSETR {
        &self.syscfg_ioctrlsetr
    }
    ///0x1c - SYSCFG interconnect control register
    #[inline(always)]
    pub const fn syscfg_icnr(&self) -> &SYSCFG_ICNR {
        &self.syscfg_icnr
    }
    ///0x20 - SYSCFG compensation cell control register
    #[inline(always)]
    pub const fn syscfg_cmpcr(&self) -> &SYSCFG_CMPCR {
        &self.syscfg_cmpcr
    }
    ///0x24 - SYSCFG compensation cell enable set register
    #[inline(always)]
    pub const fn syscfg_cmpensetr(&self) -> &SYSCFG_CMPENSETR {
        &self.syscfg_cmpensetr
    }
    ///0x28 - SYSCFG compensation cell enable set register
    #[inline(always)]
    pub const fn syscfg_cmpenclrr(&self) -> &SYSCFG_CMPENCLRR {
        &self.syscfg_cmpenclrr
    }
    ///0x2c - SYSCFG control timer break register
    #[inline(always)]
    pub const fn syscfg_cbr(&self) -> &SYSCFG_CBR {
        &self.syscfg_cbr
    }
    ///0x44 - SYSCFG peripheral mode configuration clear register
    #[inline(always)]
    pub const fn syscfg_pmcclrr(&self) -> &SYSCFG_PMCCLRR {
        &self.syscfg_pmcclrr
    }
    ///0x58 - SYSCFG IO control register
    #[inline(always)]
    pub const fn syscfg_ioctrlclrr(&self) -> &SYSCFG_IOCTRLCLRR {
        &self.syscfg_ioctrlclrr
    }
    ///0x3f4 - SYSCFG version register
    #[inline(always)]
    pub const fn syscfg_verr(&self) -> &SYSCFG_VERR {
        &self.syscfg_verr
    }
    ///0x3f8 - SYSCFG identification register
    #[inline(always)]
    pub const fn syscfg_ipidr(&self) -> &SYSCFG_IPIDR {
        &self.syscfg_ipidr
    }
    ///0x3fc - SYSCFG size identification register
    #[inline(always)]
    pub const fn syscfg_sidr(&self) -> &SYSCFG_SIDR {
        &self.syscfg_sidr
    }
}
/**SYSCFG_BOOTR (rw) register accessor: This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )

You can [`read`](crate::Reg::read) this register and get [`syscfg_bootr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_bootr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_BOOTR)

For information about available fields see [`mod@syscfg_bootr`]
module*/
pub type SYSCFG_BOOTR = crate::Reg<syscfg_bootr::SYSCFG_BOOTRrs>;
///This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
pub mod syscfg_bootr;
/**SYSCFG_PMCSETR (rw) register accessor: SYSCFG peripheral mode configuration set register

You can [`read`](crate::Reg::read) this register and get [`syscfg_pmcsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_pmcsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_PMCSETR)

For information about available fields see [`mod@syscfg_pmcsetr`]
module*/
pub type SYSCFG_PMCSETR = crate::Reg<syscfg_pmcsetr::SYSCFG_PMCSETRrs>;
///SYSCFG peripheral mode configuration set register
pub mod syscfg_pmcsetr;
/**SYSCFG_IOCTRLSETR (rw) register accessor: SYSCFG IO control register

You can [`read`](crate::Reg::read) this register and get [`syscfg_ioctrlsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_ioctrlsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_IOCTRLSETR)

For information about available fields see [`mod@syscfg_ioctrlsetr`]
module*/
pub type SYSCFG_IOCTRLSETR = crate::Reg<syscfg_ioctrlsetr::SYSCFG_IOCTRLSETRrs>;
///SYSCFG IO control register
pub mod syscfg_ioctrlsetr;
/**SYSCFG_ICNR (rw) register accessor: SYSCFG interconnect control register

You can [`read`](crate::Reg::read) this register and get [`syscfg_icnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_icnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_ICNR)

For information about available fields see [`mod@syscfg_icnr`]
module*/
pub type SYSCFG_ICNR = crate::Reg<syscfg_icnr::SYSCFG_ICNRrs>;
///SYSCFG interconnect control register
pub mod syscfg_icnr;
/**SYSCFG_CMPCR (rw) register accessor: SYSCFG compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`syscfg_cmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_CMPCR)

For information about available fields see [`mod@syscfg_cmpcr`]
module*/
pub type SYSCFG_CMPCR = crate::Reg<syscfg_cmpcr::SYSCFG_CMPCRrs>;
///SYSCFG compensation cell control register
pub mod syscfg_cmpcr;
/**SYSCFG_CMPENSETR (rw) register accessor: SYSCFG compensation cell enable set register

You can [`read`](crate::Reg::read) this register and get [`syscfg_cmpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cmpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_CMPENSETR)

For information about available fields see [`mod@syscfg_cmpensetr`]
module*/
pub type SYSCFG_CMPENSETR = crate::Reg<syscfg_cmpensetr::SYSCFG_CMPENSETRrs>;
///SYSCFG compensation cell enable set register
pub mod syscfg_cmpensetr;
/**SYSCFG_CMPENCLRR (rw) register accessor: SYSCFG compensation cell enable set register

You can [`read`](crate::Reg::read) this register and get [`syscfg_cmpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cmpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_CMPENCLRR)

For information about available fields see [`mod@syscfg_cmpenclrr`]
module*/
pub type SYSCFG_CMPENCLRR = crate::Reg<syscfg_cmpenclrr::SYSCFG_CMPENCLRRrs>;
///SYSCFG compensation cell enable set register
pub mod syscfg_cmpenclrr;
/**SYSCFG_CBR (rw) register accessor: SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`syscfg_cbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_CBR)

For information about available fields see [`mod@syscfg_cbr`]
module*/
pub type SYSCFG_CBR = crate::Reg<syscfg_cbr::SYSCFG_CBRrs>;
///SYSCFG control timer break register
pub mod syscfg_cbr;
/**SYSCFG_PMCCLRR (rw) register accessor: SYSCFG peripheral mode configuration clear register

You can [`read`](crate::Reg::read) this register and get [`syscfg_pmcclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_pmcclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_PMCCLRR)

For information about available fields see [`mod@syscfg_pmcclrr`]
module*/
pub type SYSCFG_PMCCLRR = crate::Reg<syscfg_pmcclrr::SYSCFG_PMCCLRRrs>;
///SYSCFG peripheral mode configuration clear register
pub mod syscfg_pmcclrr;
/**SYSCFG_IOCTRLCLRR (rw) register accessor: SYSCFG IO control register

You can [`read`](crate::Reg::read) this register and get [`syscfg_ioctrlclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_ioctrlclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_IOCTRLCLRR)

For information about available fields see [`mod@syscfg_ioctrlclrr`]
module*/
pub type SYSCFG_IOCTRLCLRR = crate::Reg<syscfg_ioctrlclrr::SYSCFG_IOCTRLCLRRrs>;
///SYSCFG IO control register
pub mod syscfg_ioctrlclrr;
/**SYSCFG_VERR (r) register accessor: SYSCFG version register

You can [`read`](crate::Reg::read) this register and get [`syscfg_verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_VERR)

For information about available fields see [`mod@syscfg_verr`]
module*/
pub type SYSCFG_VERR = crate::Reg<syscfg_verr::SYSCFG_VERRrs>;
///SYSCFG version register
pub mod syscfg_verr;
/**SYSCFG_IPIDR (r) register accessor: SYSCFG identification register

You can [`read`](crate::Reg::read) this register and get [`syscfg_ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_IPIDR)

For information about available fields see [`mod@syscfg_ipidr`]
module*/
pub type SYSCFG_IPIDR = crate::Reg<syscfg_ipidr::SYSCFG_IPIDRrs>;
///SYSCFG identification register
pub mod syscfg_ipidr;
/**SYSCFG_SIDR (r) register accessor: SYSCFG size identification register

You can [`read`](crate::Reg::read) this register and get [`syscfg_sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SYSCFG:SYSCFG_SIDR)

For information about available fields see [`mod@syscfg_sidr`]
module*/
pub type SYSCFG_SIDR = crate::Reg<syscfg_sidr::SYSCFG_SIDRrs>;
///SYSCFG size identification register
pub mod syscfg_sidr;
