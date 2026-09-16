#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    cfgr: CFGR,
    _reserved2: [u8; 0x0c],
    cier: CIER,
    cifr: CIFR,
    cscmdr: CSCMDR,
    _reserved5: [u8; 0x0c],
    ahbrstr: AHBRSTR,
    apb0rstr: APB0RSTR,
    apb1rstr: APB1RSTR,
    _reserved8: [u8; 0x04],
    apb2rstr: APB2RSTR,
    _reserved9: [u8; 0x0c],
    ahbenr: AHBENR,
    apb0enr: APB0ENR,
    apb1enr: APB1ENR,
    _reserved12: [u8; 0x04],
    apb2enr: APB2ENR,
    _reserved13: [u8; 0x30],
    csr: CSR,
    rfswhsecr: RFSWHSECR,
    rfhsecr: RFHSECR,
}
impl RegisterBlock {
    ///0x00 - CR register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x08 - CFGR register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x18 - CIER register
    #[inline(always)]
    pub const fn cier(&self) -> &CIER {
        &self.cier
    }
    ///0x1c - CIFR register
    #[inline(always)]
    pub const fn cifr(&self) -> &CIFR {
        &self.cifr
    }
    ///0x20 - CSCMDR register
    #[inline(always)]
    pub const fn cscmdr(&self) -> &CSCMDR {
        &self.cscmdr
    }
    ///0x30 - AHBRSTR register
    #[inline(always)]
    pub const fn ahbrstr(&self) -> &AHBRSTR {
        &self.ahbrstr
    }
    ///0x34 - APB0RSTR register
    #[inline(always)]
    pub const fn apb0rstr(&self) -> &APB0RSTR {
        &self.apb0rstr
    }
    ///0x38 - APB1RSTR register
    #[inline(always)]
    pub const fn apb1rstr(&self) -> &APB1RSTR {
        &self.apb1rstr
    }
    ///0x40 - APB2RSTR register
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &APB2RSTR {
        &self.apb2rstr
    }
    ///0x50 - AHBENR register
    #[inline(always)]
    pub const fn ahbenr(&self) -> &AHBENR {
        &self.ahbenr
    }
    ///0x54 - APB0ENR register
    #[inline(always)]
    pub const fn apb0enr(&self) -> &APB0ENR {
        &self.apb0enr
    }
    ///0x58 - APB1ENR register
    #[inline(always)]
    pub const fn apb1enr(&self) -> &APB1ENR {
        &self.apb1enr
    }
    ///0x60 - APB2ENR register
    #[inline(always)]
    pub const fn apb2enr(&self) -> &APB2ENR {
        &self.apb2enr
    }
    ///0x94 - CSR register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x98 - RFSWHSECR register
    #[inline(always)]
    pub const fn rfswhsecr(&self) -> &RFSWHSECR {
        &self.rfswhsecr
    }
    ///0x9c - RFHSECR register
    #[inline(always)]
    pub const fn rfhsecr(&self) -> &RFHSECR {
        &self.rfhsecr
    }
}
/**CR (rw) register accessor: CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///CR register
pub mod cr;
/**CFGR (rw) register accessor: CFGR register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///CFGR register
pub mod cfgr;
/**CIER (rw) register accessor: CIER register

You can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CIER)

For information about available fields see [`mod@cier`] module*/
pub type CIER = crate::Reg<cier::CIERrs>;
///CIER register
pub mod cier;
/**CIFR (rw) register accessor: CIFR register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CIFR)

For information about available fields see [`mod@cifr`] module*/
pub type CIFR = crate::Reg<cifr::CIFRrs>;
///CIFR register
pub mod cifr;
/**CSCMDR (rw) register accessor: CSCMDR register

You can [`read`](crate::Reg::read) this register and get [`cscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CSCMDR)

For information about available fields see [`mod@cscmdr`] module*/
pub type CSCMDR = crate::Reg<cscmdr::CSCMDRrs>;
///CSCMDR register
pub mod cscmdr;
/**AHBRSTR (rw) register accessor: AHBRSTR register

You can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:AHBRSTR)

For information about available fields see [`mod@ahbrstr`] module*/
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTRrs>;
///AHBRSTR register
pub mod ahbrstr;
/**APB0RSTR (rw) register accessor: APB0RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb0rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB0RSTR)

For information about available fields see [`mod@apb0rstr`] module*/
pub type APB0RSTR = crate::Reg<apb0rstr::APB0RSTRrs>;
///APB0RSTR register
pub mod apb0rstr;
/**APB1RSTR (rw) register accessor: APB1RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB1RSTR)

For information about available fields see [`mod@apb1rstr`] module*/
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTRrs>;
///APB1RSTR register
pub mod apb1rstr;
/**APB2RSTR (rw) register accessor: APB2RSTR register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB2RSTR)

For information about available fields see [`mod@apb2rstr`] module*/
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTRrs>;
///APB2RSTR register
pub mod apb2rstr;
/**AHBENR (rw) register accessor: AHBENR register

You can [`read`](crate::Reg::read) this register and get [`ahbenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:AHBENR)

For information about available fields see [`mod@ahbenr`] module*/
pub type AHBENR = crate::Reg<ahbenr::AHBENRrs>;
///AHBENR register
pub mod ahbenr;
/**APB0ENR (rw) register accessor: APB0ENR register

You can [`read`](crate::Reg::read) this register and get [`apb0enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB0ENR)

For information about available fields see [`mod@apb0enr`] module*/
pub type APB0ENR = crate::Reg<apb0enr::APB0ENRrs>;
///APB0ENR register
pub mod apb0enr;
/**APB1ENR (rw) register accessor: APB1ENR register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB1ENR)

For information about available fields see [`mod@apb1enr`] module*/
pub type APB1ENR = crate::Reg<apb1enr::APB1ENRrs>;
///APB1ENR register
pub mod apb1enr;
/**APB2ENR (rw) register accessor: APB2ENR register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:APB2ENR)

For information about available fields see [`mod@apb2enr`] module*/
pub type APB2ENR = crate::Reg<apb2enr::APB2ENRrs>;
///APB2ENR register
pub mod apb2enr;
/**CSR (rw) register accessor: CSR register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///CSR register
pub mod csr;
/**RFSWHSECR (rw) register accessor: RFSWHSECR register

You can [`read`](crate::Reg::read) this register and get [`rfswhsecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfswhsecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:RFSWHSECR)

For information about available fields see [`mod@rfswhsecr`] module*/
pub type RFSWHSECR = crate::Reg<rfswhsecr::RFSWHSECRrs>;
///RFSWHSECR register
pub mod rfswhsecr;
/**RFHSECR (r) register accessor: RFHSECR register

You can [`read`](crate::Reg::read) this register and get [`rfhsecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RCC:RFHSECR)

For information about available fields see [`mod@rfhsecr`] module*/
pub type RFHSECR = crate::Reg<rfhsecr::RFHSECRrs>;
///RFHSECR register
pub mod rfhsecr;
