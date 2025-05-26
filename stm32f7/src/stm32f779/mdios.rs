#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    wrfr: WRFR,
    cwrfr: CWRFR,
    rdfr: RDFR,
    crdfr: CRDFR,
    sr: SR,
    clrfr: CLRFR,
    dinr: [DINR; 32],
    doutr: [DOUTR; 32],
}
impl RegisterBlock {
    ///0x00 - MDIOS configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - MDIOS write flag register
    #[inline(always)]
    pub const fn wrfr(&self) -> &WRFR {
        &self.wrfr
    }
    ///0x08 - MDIOS clear write flag register
    #[inline(always)]
    pub const fn cwrfr(&self) -> &CWRFR {
        &self.cwrfr
    }
    ///0x0c - MDIOS read flag register
    #[inline(always)]
    pub const fn rdfr(&self) -> &RDFR {
        &self.rdfr
    }
    ///0x10 - MDIOS clear read flag register
    #[inline(always)]
    pub const fn crdfr(&self) -> &CRDFR {
        &self.crdfr
    }
    ///0x14 - MDIOS status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x18 - MDIOS clear flag register
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    ///0x1c..0x9c - MDIOS input data register %s
    #[inline(always)]
    pub const fn dinr(&self, n: usize) -> &DINR {
        &self.dinr[n]
    }
    ///Iterator for array of:
    ///0x1c..0x9c - MDIOS input data register %s
    #[inline(always)]
    pub fn dinr_iter(&self) -> impl Iterator<Item = &DINR> {
        self.dinr.iter()
    }
    ///0x9c..0x11c - MDIOS output data register %s
    #[inline(always)]
    pub const fn doutr(&self, n: usize) -> &DOUTR {
        &self.doutr[n]
    }
    ///Iterator for array of:
    ///0x9c..0x11c - MDIOS output data register %s
    #[inline(always)]
    pub fn doutr_iter(&self) -> impl Iterator<Item = &DOUTR> {
        self.doutr.iter()
    }
}
/**CR (rw) register accessor: MDIOS configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///MDIOS configuration register
pub mod cr;
/**WRFR (r) register accessor: MDIOS write flag register

You can [`read`](crate::Reg::read) this register and get [`wrfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:WRFR)

For information about available fields see [`mod@wrfr`] module*/
pub type WRFR = crate::Reg<wrfr::WRFRrs>;
///MDIOS write flag register
pub mod wrfr;
/**CWRFR (rw) register accessor: MDIOS clear write flag register

You can [`read`](crate::Reg::read) this register and get [`cwrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:CWRFR)

For information about available fields see [`mod@cwrfr`] module*/
pub type CWRFR = crate::Reg<cwrfr::CWRFRrs>;
///MDIOS clear write flag register
pub mod cwrfr;
/**RDFR (r) register accessor: MDIOS read flag register

You can [`read`](crate::Reg::read) this register and get [`rdfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:RDFR)

For information about available fields see [`mod@rdfr`] module*/
pub type RDFR = crate::Reg<rdfr::RDFRrs>;
///MDIOS read flag register
pub mod rdfr;
/**CRDFR (rw) register accessor: MDIOS clear read flag register

You can [`read`](crate::Reg::read) this register and get [`crdfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crdfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:CRDFR)

For information about available fields see [`mod@crdfr`] module*/
pub type CRDFR = crate::Reg<crdfr::CRDFRrs>;
///MDIOS clear read flag register
pub mod crdfr;
/**SR (r) register accessor: MDIOS status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///MDIOS status register
pub mod sr;
/**CLRFR (rw) register accessor: MDIOS clear flag register

You can [`read`](crate::Reg::read) this register and get [`clrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:CLRFR)

For information about available fields see [`mod@clrfr`] module*/
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
///MDIOS clear flag register
pub mod clrfr;
/**DINR (r) register accessor: MDIOS input data register %s

You can [`read`](crate::Reg::read) this register and get [`dinr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:DINR[0])

For information about available fields see [`mod@dinr`] module*/
pub type DINR = crate::Reg<dinr::DINRrs>;
///MDIOS input data register %s
pub mod dinr;
/**DOUTR (rw) register accessor: MDIOS output data register %s

You can [`read`](crate::Reg::read) this register and get [`doutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#MDIOS:DOUTR[0])

For information about available fields see [`mod@doutr`] module*/
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
///MDIOS output data register %s
pub mod doutr;
