#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bootr: BOOTR,
    pmcsetr: PMCSETR,
    _reserved2: [u8; 0x10],
    ioctrlsetr: IOCTRLSETR,
    icnr: ICNR,
    cmpcr: CMPCR,
    cmpensetr: CMPENSETR,
    cmpenclrr: CMPENCLRR,
    cbr: CBR,
    _reserved8: [u8; 0x14],
    pmcclrr: PMCCLRR,
    _reserved9: [u8; 0x10],
    ioctrlclrr: IOCTRLCLRR,
    _reserved10: [u8; 0x0398],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
    #[inline(always)]
    pub const fn bootr(&self) -> &BOOTR {
        &self.bootr
    }
    ///0x04 - SYSCFG peripheral mode configuration set register
    #[inline(always)]
    pub const fn pmcsetr(&self) -> &PMCSETR {
        &self.pmcsetr
    }
    ///0x18 - SYSCFG IO control register
    #[inline(always)]
    pub const fn ioctrlsetr(&self) -> &IOCTRLSETR {
        &self.ioctrlsetr
    }
    ///0x1c - SYSCFG interconnect control register
    #[inline(always)]
    pub const fn icnr(&self) -> &ICNR {
        &self.icnr
    }
    ///0x20 - SYSCFG compensation cell control register
    #[inline(always)]
    pub const fn cmpcr(&self) -> &CMPCR {
        &self.cmpcr
    }
    ///0x24 - SYSCFG compensation cell enable set register
    #[inline(always)]
    pub const fn cmpensetr(&self) -> &CMPENSETR {
        &self.cmpensetr
    }
    ///0x28 - SYSCFG compensation cell enable set register
    #[inline(always)]
    pub const fn cmpenclrr(&self) -> &CMPENCLRR {
        &self.cmpenclrr
    }
    ///0x2c - SYSCFG control timer break register
    #[inline(always)]
    pub const fn cbr(&self) -> &CBR {
        &self.cbr
    }
    ///0x44 - SYSCFG peripheral mode configuration clear register
    #[inline(always)]
    pub const fn pmcclrr(&self) -> &PMCCLRR {
        &self.pmcclrr
    }
    ///0x58 - SYSCFG IO control register
    #[inline(always)]
    pub const fn ioctrlclrr(&self) -> &IOCTRLCLRR {
        &self.ioctrlclrr
    }
    ///0x3f4 - SYSCFG version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - SYSCFG identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - SYSCFG size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**BOOTR (rw) register accessor: This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )

You can [`read`](crate::Reg::read) this register and get [`bootr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:BOOTR)

For information about available fields see [`mod@bootr`] module*/
pub type BOOTR = crate::Reg<bootr::BOOTRrs>;
///This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )
pub mod bootr;
/**PMCSETR (rw) register accessor: SYSCFG peripheral mode configuration set register

You can [`read`](crate::Reg::read) this register and get [`pmcsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:PMCSETR)

For information about available fields see [`mod@pmcsetr`] module*/
pub type PMCSETR = crate::Reg<pmcsetr::PMCSETRrs>;
///SYSCFG peripheral mode configuration set register
pub mod pmcsetr;
/**IOCTRLSETR (rw) register accessor: SYSCFG IO control register

You can [`read`](crate::Reg::read) this register and get [`ioctrlsetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioctrlsetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:IOCTRLSETR)

For information about available fields see [`mod@ioctrlsetr`] module*/
pub type IOCTRLSETR = crate::Reg<ioctrlsetr::IOCTRLSETRrs>;
///SYSCFG IO control register
pub mod ioctrlsetr;
/**ICNR (rw) register accessor: SYSCFG interconnect control register

You can [`read`](crate::Reg::read) this register and get [`icnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:ICNR)

For information about available fields see [`mod@icnr`] module*/
pub type ICNR = crate::Reg<icnr::ICNRrs>;
///SYSCFG interconnect control register
pub mod icnr;
/**CMPCR (rw) register accessor: SYSCFG compensation cell control register

You can [`read`](crate::Reg::read) this register and get [`cmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:CMPCR)

For information about available fields see [`mod@cmpcr`] module*/
pub type CMPCR = crate::Reg<cmpcr::CMPCRrs>;
///SYSCFG compensation cell control register
pub mod cmpcr;
/**CMPENSETR (rw) register accessor: SYSCFG compensation cell enable set register

You can [`read`](crate::Reg::read) this register and get [`cmpensetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpensetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:CMPENSETR)

For information about available fields see [`mod@cmpensetr`] module*/
pub type CMPENSETR = crate::Reg<cmpensetr::CMPENSETRrs>;
///SYSCFG compensation cell enable set register
pub mod cmpensetr;
/**CMPENCLRR (rw) register accessor: SYSCFG compensation cell enable set register

You can [`read`](crate::Reg::read) this register and get [`cmpenclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpenclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:CMPENCLRR)

For information about available fields see [`mod@cmpenclrr`] module*/
pub type CMPENCLRR = crate::Reg<cmpenclrr::CMPENCLRRrs>;
///SYSCFG compensation cell enable set register
pub mod cmpenclrr;
/**CBR (rw) register accessor: SYSCFG control timer break register

You can [`read`](crate::Reg::read) this register and get [`cbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:CBR)

For information about available fields see [`mod@cbr`] module*/
pub type CBR = crate::Reg<cbr::CBRrs>;
///SYSCFG control timer break register
pub mod cbr;
/**PMCCLRR (rw) register accessor: SYSCFG peripheral mode configuration clear register

You can [`read`](crate::Reg::read) this register and get [`pmcclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:PMCCLRR)

For information about available fields see [`mod@pmcclrr`] module*/
pub type PMCCLRR = crate::Reg<pmcclrr::PMCCLRRrs>;
///SYSCFG peripheral mode configuration clear register
pub mod pmcclrr;
/**IOCTRLCLRR (rw) register accessor: SYSCFG IO control register

You can [`read`](crate::Reg::read) this register and get [`ioctrlclrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioctrlclrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:IOCTRLCLRR)

For information about available fields see [`mod@ioctrlclrr`] module*/
pub type IOCTRLCLRR = crate::Reg<ioctrlclrr::IOCTRLCLRRrs>;
///SYSCFG IO control register
pub mod ioctrlclrr;
/**VERR (r) register accessor: SYSCFG version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///SYSCFG version register
pub mod verr;
/**IPIDR (r) register accessor: SYSCFG identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///SYSCFG identification register
pub mod ipidr;
/**SIDR (r) register accessor: SYSCFG size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SYSCFG:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///SYSCFG size identification register
pub mod sidr;
