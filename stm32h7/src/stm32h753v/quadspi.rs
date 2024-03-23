#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    dcr: DCR,
    sr: SR,
    fcr: FCR,
    dlr: DLR,
    ccr: CCR,
    ar: AR,
    abr: ABR,
    dr: DR,
    psmkr: PSMKR,
    psmar: PSMAR,
    pir: PIR,
    lptr: LPTR,
}
impl RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - QUADSPI device configuration register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &DCR {
        &self.dcr
    }
    #[doc = "0x08 - QUADSPI status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x0c - QUADSPI flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x10 - QUADSPI data length register"]
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    #[doc = "0x14 - QUADSPI communication configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x18 - QUADSPI address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    #[doc = "0x20 - QUADSPI data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x24 - QUADSPI polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    #[doc = "0x28 - QUADSPI polling status match register"]
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    #[doc = "0x2c - QUADSPI polling interval register"]
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
}
#[doc = "CR (rw) register accessor: QUADSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "QUADSPI control register"]
pub mod cr;
#[doc = "DCR (rw) register accessor: QUADSPI device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
pub type DCR = crate::Reg<dcr::DCRrs>;
#[doc = "QUADSPI device configuration register"]
pub mod dcr;
#[doc = "SR (r) register accessor: QUADSPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "QUADSPI status register"]
pub mod sr;
#[doc = "FCR (rw) register accessor: QUADSPI flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "QUADSPI flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: QUADSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`]
module"]
pub type DLR = crate::Reg<dlr::DLRrs>;
#[doc = "QUADSPI data length register"]
pub mod dlr;
#[doc = "CCR (rw) register accessor: QUADSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "QUADSPI communication configuration register"]
pub mod ccr;
#[doc = "AR (rw) register accessor: QUADSPI address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::ARrs>;
#[doc = "QUADSPI address register"]
pub mod ar;
#[doc = "ABR (rw) register accessor: QUADSPI alternate bytes registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abr`]
module"]
pub type ABR = crate::Reg<abr::ABRrs>;
#[doc = "QUADSPI alternate bytes registers"]
pub mod abr;
#[doc = "DR (rw) register accessor: QUADSPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "QUADSPI data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: QUADSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmkr`]
module"]
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
#[doc = "QUADSPI polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: QUADSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmar`]
module"]
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
#[doc = "QUADSPI polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: QUADSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pir`]
module"]
pub type PIR = crate::Reg<pir::PIRrs>;
#[doc = "QUADSPI polling interval register"]
pub mod pir;
#[doc = "LPTR (rw) register accessor: QUADSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptr`]
module"]
pub type LPTR = crate::Reg<lptr::LPTRrs>;
#[doc = "QUADSPI low-power timeout register"]
pub mod lptr;
