#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    dcr1: DCR1,
    dcr2: DCR2,
    dcr3: DCR3,
    dcr4: DCR4,
    _reserved5: [u8; 0x08],
    sr: SR,
    fcr: FCR,
    _reserved7: [u8; 0x18],
    dlr: DLR,
    _reserved8: [u8; 0x04],
    ar: AR,
    _reserved9: [u8; 0x04],
    dr: DR,
    _reserved10: [u8; 0x2c],
    psmkr: PSMKR,
    _reserved11: [u8; 0x04],
    psmar: PSMAR,
    _reserved12: [u8; 0x04],
    pir: PIR,
    _reserved13: [u8; 0x6c],
    ccr: CCR,
    _reserved14: [u8; 0x04],
    tcr: TCR,
    _reserved15: [u8; 0x04],
    ir: IR,
    _reserved16: [u8; 0x0c],
    abr: ABR,
    _reserved17: [u8; 0x0c],
    lptr: LPTR,
    _reserved18: [u8; 0x0c],
    wpccr: WPCCR,
    _reserved19: [u8; 0x04],
    wptcr: WPTCR,
    _reserved20: [u8; 0x04],
    wpir: WPIR,
    _reserved21: [u8; 0x0c],
    wpabr: WPABR,
    _reserved22: [u8; 0x1c],
    wccr: WCCR,
    _reserved23: [u8; 0x04],
    wtcr: WTCR,
    _reserved24: [u8; 0x04],
    wir: WIR,
    _reserved25: [u8; 0x0c],
    wabr: WABR,
    _reserved26: [u8; 0x5c],
    hlcr: HLCR,
}
impl RegisterBlock {
    #[doc = "0x00 - OCTOSPI control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - OCTOSPI device configuration register 1"]
    #[inline(always)]
    pub const fn dcr1(&self) -> &DCR1 {
        &self.dcr1
    }
    #[doc = "0x0c - OCTOSPI device configuration register 2"]
    #[inline(always)]
    pub const fn dcr2(&self) -> &DCR2 {
        &self.dcr2
    }
    #[doc = "0x10 - OCTOSPI device configuration register 3"]
    #[inline(always)]
    pub const fn dcr3(&self) -> &DCR3 {
        &self.dcr3
    }
    #[doc = "0x14 - OCTOSPI device configuration register 4"]
    #[inline(always)]
    pub const fn dcr4(&self) -> &DCR4 {
        &self.dcr4
    }
    #[doc = "0x20 - OCTOSPI status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x24 - OCTOSPI flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x40 - OCTOSPI data length register"]
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    #[doc = "0x48 - OCTOSPI address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x50 - OCTOSPI data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x80 - OCTOSPI polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    #[doc = "0x88 - OCTOSPI polling status match register"]
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    #[doc = "0x90 - OCTOSPI polling interval register"]
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    #[doc = "0x100 - OCTOSPI communication configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x108 - OCTOSPI timing configuration register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    #[doc = "0x110 - OCTOSPI instruction register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x120 - OCTOSPI alternate bytes register"]
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    #[doc = "0x130 - OCTOSPI low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
    #[doc = "0x140 - OCTOSPI wrap communication configuration register"]
    #[inline(always)]
    pub const fn wpccr(&self) -> &WPCCR {
        &self.wpccr
    }
    #[doc = "0x148 - OCTOSPI wrap timing configuration register"]
    #[inline(always)]
    pub const fn wptcr(&self) -> &WPTCR {
        &self.wptcr
    }
    #[doc = "0x150 - OCTOSPI wrap instruction register"]
    #[inline(always)]
    pub const fn wpir(&self) -> &WPIR {
        &self.wpir
    }
    #[doc = "0x160 - OCTOSPI wrap alternate bytes register"]
    #[inline(always)]
    pub const fn wpabr(&self) -> &WPABR {
        &self.wpabr
    }
    #[doc = "0x180 - OCTOSPI write communication configuration register"]
    #[inline(always)]
    pub const fn wccr(&self) -> &WCCR {
        &self.wccr
    }
    #[doc = "0x188 - OCTOSPI write timing configuration register"]
    #[inline(always)]
    pub const fn wtcr(&self) -> &WTCR {
        &self.wtcr
    }
    #[doc = "0x190 - OCTOSPI write instruction register"]
    #[inline(always)]
    pub const fn wir(&self) -> &WIR {
        &self.wir
    }
    #[doc = "0x1a0 - OCTOSPI write alternate bytes register"]
    #[inline(always)]
    pub const fn wabr(&self) -> &WABR {
        &self.wabr
    }
    #[doc = "0x200 - OCTOSPI HyperBus latency configuration register"]
    #[inline(always)]
    pub const fn hlcr(&self) -> &HLCR {
        &self.hlcr
    }
}
#[doc = "CR (rw) register accessor: OCTOSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "OCTOSPI control register"]
pub mod cr;
#[doc = "DCR1 (rw) register accessor: OCTOSPI device configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr1`]
module"]
pub type DCR1 = crate::Reg<dcr1::DCR1rs>;
#[doc = "OCTOSPI device configuration register 1"]
pub mod dcr1;
#[doc = "DCR2 (rw) register accessor: OCTOSPI device configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr2`]
module"]
pub type DCR2 = crate::Reg<dcr2::DCR2rs>;
#[doc = "OCTOSPI device configuration register 2"]
pub mod dcr2;
#[doc = "DCR3 (rw) register accessor: OCTOSPI device configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr3`]
module"]
pub type DCR3 = crate::Reg<dcr3::DCR3rs>;
#[doc = "OCTOSPI device configuration register 3"]
pub mod dcr3;
#[doc = "DCR4 (rw) register accessor: OCTOSPI device configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr4`]
module"]
pub type DCR4 = crate::Reg<dcr4::DCR4rs>;
#[doc = "OCTOSPI device configuration register 4"]
pub mod dcr4;
#[doc = "SR (r) register accessor: OCTOSPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "OCTOSPI status register"]
pub mod sr;
#[doc = "FCR (w) register accessor: OCTOSPI flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "OCTOSPI flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: OCTOSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`]
module"]
pub type DLR = crate::Reg<dlr::DLRrs>;
#[doc = "OCTOSPI data length register"]
pub mod dlr;
#[doc = "AR (rw) register accessor: OCTOSPI address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::ARrs>;
#[doc = "OCTOSPI address register"]
pub mod ar;
#[doc = "DR (rw) register accessor: OCTOSPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "OCTOSPI data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: OCTOSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmkr`]
module"]
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
#[doc = "OCTOSPI polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: OCTOSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmar`]
module"]
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
#[doc = "OCTOSPI polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: OCTOSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pir`]
module"]
pub type PIR = crate::Reg<pir::PIRrs>;
#[doc = "OCTOSPI polling interval register"]
pub mod pir;
#[doc = "CCR (rw) register accessor: OCTOSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "OCTOSPI communication configuration register"]
pub mod ccr;
#[doc = "TCR (rw) register accessor: OCTOSPI timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCRrs>;
#[doc = "OCTOSPI timing configuration register"]
pub mod tcr;
#[doc = "IR (rw) register accessor: OCTOSPI instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "OCTOSPI instruction register"]
pub mod ir;
#[doc = "ABR (rw) register accessor: OCTOSPI alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abr`]
module"]
pub type ABR = crate::Reg<abr::ABRrs>;
#[doc = "OCTOSPI alternate bytes register"]
pub mod abr;
#[doc = "LPTR (rw) register accessor: OCTOSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptr`]
module"]
pub type LPTR = crate::Reg<lptr::LPTRrs>;
#[doc = "OCTOSPI low-power timeout register"]
pub mod lptr;
#[doc = "WPCCR (rw) register accessor: OCTOSPI wrap communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpccr`]
module"]
pub type WPCCR = crate::Reg<wpccr::WPCCRrs>;
#[doc = "OCTOSPI wrap communication configuration register"]
pub mod wpccr;
#[doc = "WPTCR (rw) register accessor: OCTOSPI wrap timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wptcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wptcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wptcr`]
module"]
pub type WPTCR = crate::Reg<wptcr::WPTCRrs>;
#[doc = "OCTOSPI wrap timing configuration register"]
pub mod wptcr;
#[doc = "WPIR (rw) register accessor: OCTOSPI wrap instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpir`]
module"]
pub type WPIR = crate::Reg<wpir::WPIRrs>;
#[doc = "OCTOSPI wrap instruction register"]
pub mod wpir;
#[doc = "WPABR (rw) register accessor: OCTOSPI wrap alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpabr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpabr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpabr`]
module"]
pub type WPABR = crate::Reg<wpabr::WPABRrs>;
#[doc = "OCTOSPI wrap alternate bytes register"]
pub mod wpabr;
#[doc = "WCCR (rw) register accessor: OCTOSPI write communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wccr`]
module"]
pub type WCCR = crate::Reg<wccr::WCCRrs>;
#[doc = "OCTOSPI write communication configuration register"]
pub mod wccr;
#[doc = "WTCR (rw) register accessor: OCTOSPI write timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtcr`]
module"]
pub type WTCR = crate::Reg<wtcr::WTCRrs>;
#[doc = "OCTOSPI write timing configuration register"]
pub mod wtcr;
#[doc = "WIR (rw) register accessor: OCTOSPI write instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wir`]
module"]
pub type WIR = crate::Reg<wir::WIRrs>;
#[doc = "OCTOSPI write instruction register"]
pub mod wir;
#[doc = "WABR (rw) register accessor: OCTOSPI write alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wabr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wabr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wabr`]
module"]
pub type WABR = crate::Reg<wabr::WABRrs>;
#[doc = "OCTOSPI write alternate bytes register"]
pub mod wabr;
#[doc = "HLCR (rw) register accessor: OCTOSPI HyperBus latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hlcr`]
module"]
pub type HLCR = crate::Reg<hlcr::HLCRrs>;
#[doc = "OCTOSPI HyperBus latency configuration register"]
pub mod hlcr;
