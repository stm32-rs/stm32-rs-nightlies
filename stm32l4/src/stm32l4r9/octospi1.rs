#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x04],
    dcr1: DCR1,
    dcr2: DCR2,
    dcr3: DCR3,
    _reserved4: [u8; 0x0c],
    sr: SR,
    fcr: FCR,
    _reserved6: [u8; 0x18],
    dlr: DLR,
    _reserved7: [u8; 0x04],
    ar: AR,
    _reserved8: [u8; 0x04],
    dr: DR,
    _reserved9: [u8; 0x2c],
    psmkr: PSMKR,
    _reserved10: [u8; 0x04],
    psmar: PSMAR,
    _reserved11: [u8; 0x04],
    pir: PIR,
    _reserved12: [u8; 0x6c],
    ccr: CCR,
    _reserved13: [u8; 0x04],
    tcr: TCR,
    _reserved14: [u8; 0x04],
    ir: IR,
    _reserved15: [u8; 0x0c],
    abr: ABR,
    _reserved16: [u8; 0x0c],
    lptr: LPTR,
    _reserved17: [u8; 0x4c],
    wccr: WCCR,
    _reserved18: [u8; 0x04],
    wtcr: WTCR,
    _reserved19: [u8; 0x04],
    wir: WIR,
    _reserved20: [u8; 0x0c],
    wabr: WABR,
    _reserved21: [u8; 0x5c],
    hlcr: HLCR,
    _reserved22: [u8; 0x01ec],
    hwcfgr: HWCFGR,
    ver: VER,
    id: ID,
    mid: MID,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x08 - device configuration register"]
    #[inline(always)]
    pub const fn dcr1(&self) -> &DCR1 {
        &self.dcr1
    }
    #[doc = "0x0c - device configuration register 2"]
    #[inline(always)]
    pub const fn dcr2(&self) -> &DCR2 {
        &self.dcr2
    }
    #[doc = "0x10 - device configuration register 3"]
    #[inline(always)]
    pub const fn dcr3(&self) -> &DCR3 {
        &self.dcr3
    }
    #[doc = "0x20 - status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x24 - flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        &self.fcr
    }
    #[doc = "0x40 - data length register"]
    #[inline(always)]
    pub const fn dlr(&self) -> &DLR {
        &self.dlr
    }
    #[doc = "0x48 - address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &AR {
        &self.ar
    }
    #[doc = "0x50 - data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x80 - polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(&self) -> &PSMKR {
        &self.psmkr
    }
    #[doc = "0x88 - polling status match register"]
    #[inline(always)]
    pub const fn psmar(&self) -> &PSMAR {
        &self.psmar
    }
    #[doc = "0x90 - polling interval register"]
    #[inline(always)]
    pub const fn pir(&self) -> &PIR {
        &self.pir
    }
    #[doc = "0x100 - communication configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x108 - timing configuration register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    #[doc = "0x110 - instruction register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x120 - alternate bytes register"]
    #[inline(always)]
    pub const fn abr(&self) -> &ABR {
        &self.abr
    }
    #[doc = "0x130 - low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(&self) -> &LPTR {
        &self.lptr
    }
    #[doc = "0x180 - write communication configuration register"]
    #[inline(always)]
    pub const fn wccr(&self) -> &WCCR {
        &self.wccr
    }
    #[doc = "0x188 - write timing configuration register"]
    #[inline(always)]
    pub const fn wtcr(&self) -> &WTCR {
        &self.wtcr
    }
    #[doc = "0x190 - write instruction register"]
    #[inline(always)]
    pub const fn wir(&self) -> &WIR {
        &self.wir
    }
    #[doc = "0x1a0 - write alternate bytes register"]
    #[inline(always)]
    pub const fn wabr(&self) -> &WABR {
        &self.wabr
    }
    #[doc = "0x200 - HyperBusTM latency configuration register"]
    #[inline(always)]
    pub const fn hlcr(&self) -> &HLCR {
        &self.hlcr
    }
    #[doc = "0x3f0 - HW configuration register"]
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    #[doc = "0x3f4 - version register"]
    #[inline(always)]
    pub const fn ver(&self) -> &VER {
        &self.ver
    }
    #[doc = "0x3f8 - identification"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x3fc - magic ID"]
    #[inline(always)]
    pub const fn mid(&self) -> &MID {
        &self.mid
    }
}
#[doc = "CR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "control register"]
pub mod cr;
#[doc = "DCR1 (rw) register accessor: device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr1`]
module"]
pub type DCR1 = crate::Reg<dcr1::DCR1rs>;
#[doc = "device configuration register"]
pub mod dcr1;
#[doc = "DCR2 (rw) register accessor: device configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr2`]
module"]
pub type DCR2 = crate::Reg<dcr2::DCR2rs>;
#[doc = "device configuration register 2"]
pub mod dcr2;
#[doc = "DCR3 (rw) register accessor: device configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr3`]
module"]
pub type DCR3 = crate::Reg<dcr3::DCR3rs>;
#[doc = "device configuration register 3"]
pub mod dcr3;
#[doc = "SR (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "status register"]
pub mod sr;
#[doc = "FCR (w) register accessor: flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCRrs>;
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`]
module"]
pub type DLR = crate::Reg<dlr::DLRrs>;
#[doc = "data length register"]
pub mod dlr;
#[doc = "AR (rw) register accessor: address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
pub type AR = crate::Reg<ar::ARrs>;
#[doc = "address register"]
pub mod ar;
#[doc = "DR (rw) register accessor: data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmkr`]
module"]
pub type PSMKR = crate::Reg<psmkr::PSMKRrs>;
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmar`]
module"]
pub type PSMAR = crate::Reg<psmar::PSMARrs>;
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pir`]
module"]
pub type PIR = crate::Reg<pir::PIRrs>;
#[doc = "polling interval register"]
pub mod pir;
#[doc = "CCR (rw) register accessor: communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "TCR (rw) register accessor: timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
pub type TCR = crate::Reg<tcr::TCRrs>;
#[doc = "timing configuration register"]
pub mod tcr;
#[doc = "IR (rw) register accessor: instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IRrs>;
#[doc = "instruction register"]
pub mod ir;
#[doc = "ABR (rw) register accessor: alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abr`]
module"]
pub type ABR = crate::Reg<abr::ABRrs>;
#[doc = "alternate bytes register"]
pub mod abr;
#[doc = "LPTR (rw) register accessor: low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptr`]
module"]
pub type LPTR = crate::Reg<lptr::LPTRrs>;
#[doc = "low-power timeout register"]
pub mod lptr;
#[doc = "WCCR (rw) register accessor: write communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wccr`]
module"]
pub type WCCR = crate::Reg<wccr::WCCRrs>;
#[doc = "write communication configuration register"]
pub mod wccr;
#[doc = "WTCR (rw) register accessor: write timing configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wtcr`]
module"]
pub type WTCR = crate::Reg<wtcr::WTCRrs>;
#[doc = "write timing configuration register"]
pub mod wtcr;
#[doc = "WIR (rw) register accessor: write instruction register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wir`]
module"]
pub type WIR = crate::Reg<wir::WIRrs>;
#[doc = "write instruction register"]
pub mod wir;
#[doc = "WABR (rw) register accessor: write alternate bytes register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wabr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wabr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wabr`]
module"]
pub type WABR = crate::Reg<wabr::WABRrs>;
#[doc = "write alternate bytes register"]
pub mod wabr;
#[doc = "HLCR (rw) register accessor: HyperBusTM latency configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hlcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hlcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hlcr`]
module"]
pub type HLCR = crate::Reg<hlcr::HLCRrs>;
#[doc = "HyperBusTM latency configuration register"]
pub mod hlcr;
#[doc = "HWCFGR (r) register accessor: HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr`]
module"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
#[doc = "HW configuration register"]
pub mod hwcfgr;
#[doc = "VER (r) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ver`]
module"]
pub type VER = crate::Reg<ver::VERrs>;
#[doc = "version register"]
pub mod ver;
#[doc = "ID (r) register accessor: identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::IDrs>;
#[doc = "identification"]
pub mod id;
#[doc = "MID (r) register accessor: magic ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mid`]
module"]
pub type MID = crate::Reg<mid::MIDrs>;
#[doc = "magic ID"]
pub mod mid;
