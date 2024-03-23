#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    din: DIN,
    str: STR,
    hr: [HR; 5],
    imr: IMR,
    sr: SR,
    _reserved6: [u8; 0xd0],
    csr: [CSR; 54],
    _reserved7: [u8; 0x0140],
    hash_hr: [HASH_HR; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - data input register"]
    #[inline(always)]
    pub const fn din(&self) -> &DIN {
        &self.din
    }
    #[doc = "0x08 - start register"]
    #[inline(always)]
    pub const fn str(&self) -> &STR {
        &self.str
    }
    #[doc = "0x0c..0x20 - digest registers"]
    #[inline(always)]
    pub const fn hr(&self, n: usize) -> &HR {
        &self.hr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x20 - digest registers"]
    #[inline(always)]
    pub fn hr_iter(&self) -> impl Iterator<Item = &HR> {
        self.hr.iter()
    }
    #[doc = "0x20 - interrupt enable register"]
    #[inline(always)]
    pub const fn imr(&self) -> &IMR {
        &self.imr
    }
    #[doc = "0x24 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0xf8..0x1d0 - context swap registers"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &CSR {
        &self.csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf8..0x1d0 - context swap registers"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &CSR> {
        self.csr.iter()
    }
    #[doc = "0x310..0x330 - HASH digest register %s"]
    #[inline(always)]
    pub const fn hash_hr(&self, n: usize) -> &HASH_HR {
        &self.hash_hr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x310..0x330 - HASH digest register %s"]
    #[inline(always)]
    pub fn hash_hr_iter(&self) -> impl Iterator<Item = &HASH_HR> {
        self.hash_hr.iter()
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "DIN (rw) register accessor: data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din`]
module"]
pub type DIN = crate::Reg<din::DINrs>;
#[doc = "data input register"]
pub mod din;
#[doc = "STR (rw) register accessor: start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`str::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`]
module"]
pub type STR = crate::Reg<str::STRrs>;
#[doc = "start register"]
pub mod str;
#[doc = "HR (r) register accessor: digest registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hr`]
module"]
pub type HR = crate::Reg<hr::HRrs>;
#[doc = "digest registers"]
pub mod hr;
#[doc = "IMR (rw) register accessor: interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
pub type IMR = crate::Reg<imr::IMRrs>;
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "CSR (rw) register accessor: context swap registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "context swap registers"]
pub mod csr;
#[doc = "HASH_HR (r) register accessor: HASH digest register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_hr`]
module"]
pub type HASH_HR = crate::Reg<hash_hr::HASH_HRrs>;
#[doc = "HASH digest register %s"]
pub mod hash_hr;
