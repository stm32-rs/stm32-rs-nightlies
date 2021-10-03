#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS host configuration register"]
    pub otg_hs_hcfg: crate::Reg<otg_hs_hcfg::OTG_HS_HCFG_SPEC>,
    #[doc = "0x04 - OTG_HS Host frame interval register"]
    pub otg_hs_hfir: crate::Reg<otg_hs_hfir::OTG_HS_HFIR_SPEC>,
    #[doc = "0x08 - OTG_HS host frame number/frame time remaining register"]
    pub otg_hs_hfnum: crate::Reg<otg_hs_hfnum::OTG_HS_HFNUM_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_HS_Host periodic transmit FIFO/queue status register"]
    pub otg_hs_hptxsts: crate::Reg<otg_hs_hptxsts::OTG_HS_HPTXSTS_SPEC>,
    #[doc = "0x14 - OTG_HS Host all channels interrupt register"]
    pub otg_hs_haint: crate::Reg<otg_hs_haint::OTG_HS_HAINT_SPEC>,
    #[doc = "0x18 - OTG_HS host all channels interrupt mask register"]
    pub otg_hs_haintmsk: crate::Reg<otg_hs_haintmsk::OTG_HS_HAINTMSK_SPEC>,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_HS host port control and status register"]
    pub otg_hs_hprt: crate::Reg<otg_hs_hprt::OTG_HS_HPRT_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_HS host channel-0 characteristics register"]
    pub otg_hs_hcchar0: crate::Reg<otg_hs_hcchar0::OTG_HS_HCCHAR0_SPEC>,
    #[doc = "0x104 - OTG_HS host channel-0 split control register"]
    pub otg_hs_hcsplt0: crate::Reg<otg_hs_hcsplt0::OTG_HS_HCSPLT0_SPEC>,
    #[doc = "0x108 - OTG_HS host channel-11 interrupt register"]
    pub otg_hs_hcint0: crate::Reg<otg_hs_hcint0::OTG_HS_HCINT0_SPEC>,
    #[doc = "0x10c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk0: crate::Reg<otg_hs_hcintmsk0::OTG_HS_HCINTMSK0_SPEC>,
    #[doc = "0x110 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz0: crate::Reg<otg_hs_hctsiz0::OTG_HS_HCTSIZ0_SPEC>,
    #[doc = "0x114 - OTG_HS host channel-0 DMA address register"]
    pub otg_hs_hcdma0: crate::Reg<otg_hs_hcdma0::OTG_HS_HCDMA0_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x120 - OTG_HS host channel-1 characteristics register"]
    pub otg_hs_hcchar1: crate::Reg<otg_hs_hcchar1::OTG_HS_HCCHAR1_SPEC>,
    #[doc = "0x124 - OTG_HS host channel-1 split control register"]
    pub otg_hs_hcsplt1: crate::Reg<otg_hs_hcsplt1::OTG_HS_HCSPLT1_SPEC>,
    #[doc = "0x128 - OTG_HS host channel-1 interrupt register"]
    pub otg_hs_hcint1: crate::Reg<otg_hs_hcint1::OTG_HS_HCINT1_SPEC>,
    #[doc = "0x12c - OTG_HS host channel-1 interrupt mask register"]
    pub otg_hs_hcintmsk1: crate::Reg<otg_hs_hcintmsk1::OTG_HS_HCINTMSK1_SPEC>,
    #[doc = "0x130 - OTG_HS host channel-1 transfer size register"]
    pub otg_hs_hctsiz1: crate::Reg<otg_hs_hctsiz1::OTG_HS_HCTSIZ1_SPEC>,
    #[doc = "0x134 - OTG_HS host channel-1 DMA address register"]
    pub otg_hs_hcdma1: crate::Reg<otg_hs_hcdma1::OTG_HS_HCDMA1_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x140 - OTG_HS host channel-2 characteristics register"]
    pub otg_hs_hcchar2: crate::Reg<otg_hs_hcchar2::OTG_HS_HCCHAR2_SPEC>,
    #[doc = "0x144 - OTG_HS host channel-2 split control register"]
    pub otg_hs_hcsplt2: crate::Reg<otg_hs_hcsplt2::OTG_HS_HCSPLT2_SPEC>,
    #[doc = "0x148 - OTG_HS host channel-2 interrupt register"]
    pub otg_hs_hcint2: crate::Reg<otg_hs_hcint2::OTG_HS_HCINT2_SPEC>,
    #[doc = "0x14c - OTG_HS host channel-2 interrupt mask register"]
    pub otg_hs_hcintmsk2: crate::Reg<otg_hs_hcintmsk2::OTG_HS_HCINTMSK2_SPEC>,
    #[doc = "0x150 - OTG_HS host channel-2 transfer size register"]
    pub otg_hs_hctsiz2: crate::Reg<otg_hs_hctsiz2::OTG_HS_HCTSIZ2_SPEC>,
    #[doc = "0x154 - OTG_HS host channel-2 DMA address register"]
    pub otg_hs_hcdma2: crate::Reg<otg_hs_hcdma2::OTG_HS_HCDMA2_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x160 - OTG_HS host channel-3 characteristics register"]
    pub otg_hs_hcchar3: crate::Reg<otg_hs_hcchar3::OTG_HS_HCCHAR3_SPEC>,
    #[doc = "0x164 - OTG_HS host channel-3 split control register"]
    pub otg_hs_hcsplt3: crate::Reg<otg_hs_hcsplt3::OTG_HS_HCSPLT3_SPEC>,
    #[doc = "0x168 - OTG_HS host channel-3 interrupt register"]
    pub otg_hs_hcint3: crate::Reg<otg_hs_hcint3::OTG_HS_HCINT3_SPEC>,
    #[doc = "0x16c - OTG_HS host channel-3 interrupt mask register"]
    pub otg_hs_hcintmsk3: crate::Reg<otg_hs_hcintmsk3::OTG_HS_HCINTMSK3_SPEC>,
    #[doc = "0x170 - OTG_HS host channel-3 transfer size register"]
    pub otg_hs_hctsiz3: crate::Reg<otg_hs_hctsiz3::OTG_HS_HCTSIZ3_SPEC>,
    #[doc = "0x174 - OTG_HS host channel-3 DMA address register"]
    pub otg_hs_hcdma3: crate::Reg<otg_hs_hcdma3::OTG_HS_HCDMA3_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0x180 - OTG_HS host channel-4 characteristics register"]
    pub otg_hs_hcchar4: crate::Reg<otg_hs_hcchar4::OTG_HS_HCCHAR4_SPEC>,
    #[doc = "0x184 - OTG_HS host channel-4 split control register"]
    pub otg_hs_hcsplt4: crate::Reg<otg_hs_hcsplt4::OTG_HS_HCSPLT4_SPEC>,
    #[doc = "0x188 - OTG_HS host channel-4 interrupt register"]
    pub otg_hs_hcint4: crate::Reg<otg_hs_hcint4::OTG_HS_HCINT4_SPEC>,
    #[doc = "0x18c - OTG_HS host channel-4 interrupt mask register"]
    pub otg_hs_hcintmsk4: crate::Reg<otg_hs_hcintmsk4::OTG_HS_HCINTMSK4_SPEC>,
    #[doc = "0x190 - OTG_HS host channel-4 transfer size register"]
    pub otg_hs_hctsiz4: crate::Reg<otg_hs_hctsiz4::OTG_HS_HCTSIZ4_SPEC>,
    #[doc = "0x194 - OTG_HS host channel-4 DMA address register"]
    pub otg_hs_hcdma4: crate::Reg<otg_hs_hcdma4::OTG_HS_HCDMA4_SPEC>,
    _reserved37: [u8; 0x08],
    #[doc = "0x1a0 - OTG_HS host channel-5 characteristics register"]
    pub otg_hs_hcchar5: crate::Reg<otg_hs_hcchar5::OTG_HS_HCCHAR5_SPEC>,
    #[doc = "0x1a4 - OTG_HS host channel-5 split control register"]
    pub otg_hs_hcsplt5: crate::Reg<otg_hs_hcsplt5::OTG_HS_HCSPLT5_SPEC>,
    #[doc = "0x1a8 - OTG_HS host channel-5 interrupt register"]
    pub otg_hs_hcint5: crate::Reg<otg_hs_hcint5::OTG_HS_HCINT5_SPEC>,
    #[doc = "0x1ac - OTG_HS host channel-5 interrupt mask register"]
    pub otg_hs_hcintmsk5: crate::Reg<otg_hs_hcintmsk5::OTG_HS_HCINTMSK5_SPEC>,
    #[doc = "0x1b0 - OTG_HS host channel-5 transfer size register"]
    pub otg_hs_hctsiz5: crate::Reg<otg_hs_hctsiz5::OTG_HS_HCTSIZ5_SPEC>,
    #[doc = "0x1b4 - OTG_HS host channel-5 DMA address register"]
    pub otg_hs_hcdma5: crate::Reg<otg_hs_hcdma5::OTG_HS_HCDMA5_SPEC>,
    _reserved43: [u8; 0x08],
    #[doc = "0x1c0 - OTG_HS host channel-6 characteristics register"]
    pub otg_hs_hcchar6: crate::Reg<otg_hs_hcchar6::OTG_HS_HCCHAR6_SPEC>,
    #[doc = "0x1c4 - OTG_HS host channel-6 split control register"]
    pub otg_hs_hcsplt6: crate::Reg<otg_hs_hcsplt6::OTG_HS_HCSPLT6_SPEC>,
    #[doc = "0x1c8 - OTG_HS host channel-6 interrupt register"]
    pub otg_hs_hcint6: crate::Reg<otg_hs_hcint6::OTG_HS_HCINT6_SPEC>,
    #[doc = "0x1cc - OTG_HS host channel-6 interrupt mask register"]
    pub otg_hs_hcintmsk6: crate::Reg<otg_hs_hcintmsk6::OTG_HS_HCINTMSK6_SPEC>,
    #[doc = "0x1d0 - OTG_HS host channel-6 transfer size register"]
    pub otg_hs_hctsiz6: crate::Reg<otg_hs_hctsiz6::OTG_HS_HCTSIZ6_SPEC>,
    #[doc = "0x1d4 - OTG_HS host channel-6 DMA address register"]
    pub otg_hs_hcdma6: crate::Reg<otg_hs_hcdma6::OTG_HS_HCDMA6_SPEC>,
    _reserved49: [u8; 0x08],
    #[doc = "0x1e0 - OTG_HS host channel-7 characteristics register"]
    pub otg_hs_hcchar7: crate::Reg<otg_hs_hcchar7::OTG_HS_HCCHAR7_SPEC>,
    #[doc = "0x1e4 - OTG_HS host channel-7 split control register"]
    pub otg_hs_hcsplt7: crate::Reg<otg_hs_hcsplt7::OTG_HS_HCSPLT7_SPEC>,
    #[doc = "0x1e8 - OTG_HS host channel-7 interrupt register"]
    pub otg_hs_hcint7: crate::Reg<otg_hs_hcint7::OTG_HS_HCINT7_SPEC>,
    #[doc = "0x1ec - OTG_HS host channel-7 interrupt mask register"]
    pub otg_hs_hcintmsk7: crate::Reg<otg_hs_hcintmsk7::OTG_HS_HCINTMSK7_SPEC>,
    #[doc = "0x1f0 - OTG_HS host channel-7 transfer size register"]
    pub otg_hs_hctsiz7: crate::Reg<otg_hs_hctsiz7::OTG_HS_HCTSIZ7_SPEC>,
    #[doc = "0x1f4 - OTG_HS host channel-7 DMA address register"]
    pub otg_hs_hcdma7: crate::Reg<otg_hs_hcdma7::OTG_HS_HCDMA7_SPEC>,
    _reserved55: [u8; 0x08],
    #[doc = "0x200 - OTG_HS host channel-8 characteristics register"]
    pub otg_hs_hcchar8: crate::Reg<otg_hs_hcchar8::OTG_HS_HCCHAR8_SPEC>,
    #[doc = "0x204 - OTG_HS host channel-8 split control register"]
    pub otg_hs_hcsplt8: crate::Reg<otg_hs_hcsplt8::OTG_HS_HCSPLT8_SPEC>,
    #[doc = "0x208 - OTG_HS host channel-8 interrupt register"]
    pub otg_hs_hcint8: crate::Reg<otg_hs_hcint8::OTG_HS_HCINT8_SPEC>,
    #[doc = "0x20c - OTG_HS host channel-8 interrupt mask register"]
    pub otg_hs_hcintmsk8: crate::Reg<otg_hs_hcintmsk8::OTG_HS_HCINTMSK8_SPEC>,
    #[doc = "0x210 - OTG_HS host channel-8 transfer size register"]
    pub otg_hs_hctsiz8: crate::Reg<otg_hs_hctsiz8::OTG_HS_HCTSIZ8_SPEC>,
    #[doc = "0x214 - OTG_HS host channel-8 DMA address register"]
    pub otg_hs_hcdma8: crate::Reg<otg_hs_hcdma8::OTG_HS_HCDMA8_SPEC>,
    _reserved61: [u8; 0x08],
    #[doc = "0x220 - OTG_HS host channel-9 characteristics register"]
    pub otg_hs_hcchar9: crate::Reg<otg_hs_hcchar9::OTG_HS_HCCHAR9_SPEC>,
    #[doc = "0x224 - OTG_HS host channel-9 split control register"]
    pub otg_hs_hcsplt9: crate::Reg<otg_hs_hcsplt9::OTG_HS_HCSPLT9_SPEC>,
    #[doc = "0x228 - OTG_HS host channel-9 interrupt register"]
    pub otg_hs_hcint9: crate::Reg<otg_hs_hcint9::OTG_HS_HCINT9_SPEC>,
    #[doc = "0x22c - OTG_HS host channel-9 interrupt mask register"]
    pub otg_hs_hcintmsk9: crate::Reg<otg_hs_hcintmsk9::OTG_HS_HCINTMSK9_SPEC>,
    #[doc = "0x230 - OTG_HS host channel-9 transfer size register"]
    pub otg_hs_hctsiz9: crate::Reg<otg_hs_hctsiz9::OTG_HS_HCTSIZ9_SPEC>,
    #[doc = "0x234 - OTG_HS host channel-9 DMA address register"]
    pub otg_hs_hcdma9: crate::Reg<otg_hs_hcdma9::OTG_HS_HCDMA9_SPEC>,
    _reserved67: [u8; 0x08],
    #[doc = "0x240 - OTG_HS host channel-10 characteristics register"]
    pub otg_hs_hcchar10: crate::Reg<otg_hs_hcchar10::OTG_HS_HCCHAR10_SPEC>,
    #[doc = "0x244 - OTG_HS host channel-10 split control register"]
    pub otg_hs_hcsplt10: crate::Reg<otg_hs_hcsplt10::OTG_HS_HCSPLT10_SPEC>,
    #[doc = "0x248 - OTG_HS host channel-10 interrupt register"]
    pub otg_hs_hcint10: crate::Reg<otg_hs_hcint10::OTG_HS_HCINT10_SPEC>,
    #[doc = "0x24c - OTG_HS host channel-10 interrupt mask register"]
    pub otg_hs_hcintmsk10: crate::Reg<otg_hs_hcintmsk10::OTG_HS_HCINTMSK10_SPEC>,
    #[doc = "0x250 - OTG_HS host channel-10 transfer size register"]
    pub otg_hs_hctsiz10: crate::Reg<otg_hs_hctsiz10::OTG_HS_HCTSIZ10_SPEC>,
    #[doc = "0x254 - OTG_HS host channel-10 DMA address register"]
    pub otg_hs_hcdma10: crate::Reg<otg_hs_hcdma10::OTG_HS_HCDMA10_SPEC>,
    _reserved73: [u8; 0x08],
    #[doc = "0x260 - OTG_HS host channel-11 characteristics register"]
    pub otg_hs_hcchar11: crate::Reg<otg_hs_hcchar11::OTG_HS_HCCHAR11_SPEC>,
    #[doc = "0x264 - OTG_HS host channel-11 split control register"]
    pub otg_hs_hcsplt11: crate::Reg<otg_hs_hcsplt11::OTG_HS_HCSPLT11_SPEC>,
    #[doc = "0x268 - OTG_HS host channel-11 interrupt register"]
    pub otg_hs_hcint11: crate::Reg<otg_hs_hcint11::OTG_HS_HCINT11_SPEC>,
    #[doc = "0x26c - OTG_HS host channel-11 interrupt mask register"]
    pub otg_hs_hcintmsk11: crate::Reg<otg_hs_hcintmsk11::OTG_HS_HCINTMSK11_SPEC>,
    #[doc = "0x270 - OTG_HS host channel-11 transfer size register"]
    pub otg_hs_hctsiz11: crate::Reg<otg_hs_hctsiz11::OTG_HS_HCTSIZ11_SPEC>,
    #[doc = "0x274 - OTG_HS host channel-11 DMA address register"]
    pub otg_hs_hcdma11: crate::Reg<otg_hs_hcdma11::OTG_HS_HCDMA11_SPEC>,
}
#[doc = "OTG_HS_HCFG register accessor: an alias for `Reg<OTG_HS_HCFG_SPEC>`"]
pub type OTG_HS_HCFG = crate::Reg<otg_hs_hcfg::OTG_HS_HCFG_SPEC>;
#[doc = "OTG_HS host configuration register"]
pub mod otg_hs_hcfg;
#[doc = "OTG_HS_HFIR register accessor: an alias for `Reg<OTG_HS_HFIR_SPEC>`"]
pub type OTG_HS_HFIR = crate::Reg<otg_hs_hfir::OTG_HS_HFIR_SPEC>;
#[doc = "OTG_HS Host frame interval register"]
pub mod otg_hs_hfir;
#[doc = "OTG_HS_HFNUM register accessor: an alias for `Reg<OTG_HS_HFNUM_SPEC>`"]
pub type OTG_HS_HFNUM = crate::Reg<otg_hs_hfnum::OTG_HS_HFNUM_SPEC>;
#[doc = "OTG_HS host frame number/frame time remaining register"]
pub mod otg_hs_hfnum;
#[doc = "OTG_HS_HPTXSTS register accessor: an alias for `Reg<OTG_HS_HPTXSTS_SPEC>`"]
pub type OTG_HS_HPTXSTS = crate::Reg<otg_hs_hptxsts::OTG_HS_HPTXSTS_SPEC>;
#[doc = "OTG_HS_Host periodic transmit FIFO/queue status register"]
pub mod otg_hs_hptxsts;
#[doc = "OTG_HS_HAINT register accessor: an alias for `Reg<OTG_HS_HAINT_SPEC>`"]
pub type OTG_HS_HAINT = crate::Reg<otg_hs_haint::OTG_HS_HAINT_SPEC>;
#[doc = "OTG_HS Host all channels interrupt register"]
pub mod otg_hs_haint;
#[doc = "OTG_HS_HAINTMSK register accessor: an alias for `Reg<OTG_HS_HAINTMSK_SPEC>`"]
pub type OTG_HS_HAINTMSK = crate::Reg<otg_hs_haintmsk::OTG_HS_HAINTMSK_SPEC>;
#[doc = "OTG_HS host all channels interrupt mask register"]
pub mod otg_hs_haintmsk;
#[doc = "OTG_HS_HPRT register accessor: an alias for `Reg<OTG_HS_HPRT_SPEC>`"]
pub type OTG_HS_HPRT = crate::Reg<otg_hs_hprt::OTG_HS_HPRT_SPEC>;
#[doc = "OTG_HS host port control and status register"]
pub mod otg_hs_hprt;
#[doc = "OTG_HS_HCCHAR0 register accessor: an alias for `Reg<OTG_HS_HCCHAR0_SPEC>`"]
pub type OTG_HS_HCCHAR0 = crate::Reg<otg_hs_hcchar0::OTG_HS_HCCHAR0_SPEC>;
#[doc = "OTG_HS host channel-0 characteristics register"]
pub mod otg_hs_hcchar0;
#[doc = "OTG_HS_HCCHAR1 register accessor: an alias for `Reg<OTG_HS_HCCHAR1_SPEC>`"]
pub type OTG_HS_HCCHAR1 = crate::Reg<otg_hs_hcchar1::OTG_HS_HCCHAR1_SPEC>;
#[doc = "OTG_HS host channel-1 characteristics register"]
pub mod otg_hs_hcchar1;
#[doc = "OTG_HS_HCCHAR2 register accessor: an alias for `Reg<OTG_HS_HCCHAR2_SPEC>`"]
pub type OTG_HS_HCCHAR2 = crate::Reg<otg_hs_hcchar2::OTG_HS_HCCHAR2_SPEC>;
#[doc = "OTG_HS host channel-2 characteristics register"]
pub mod otg_hs_hcchar2;
#[doc = "OTG_HS_HCCHAR3 register accessor: an alias for `Reg<OTG_HS_HCCHAR3_SPEC>`"]
pub type OTG_HS_HCCHAR3 = crate::Reg<otg_hs_hcchar3::OTG_HS_HCCHAR3_SPEC>;
#[doc = "OTG_HS host channel-3 characteristics register"]
pub mod otg_hs_hcchar3;
#[doc = "OTG_HS_HCCHAR4 register accessor: an alias for `Reg<OTG_HS_HCCHAR4_SPEC>`"]
pub type OTG_HS_HCCHAR4 = crate::Reg<otg_hs_hcchar4::OTG_HS_HCCHAR4_SPEC>;
#[doc = "OTG_HS host channel-4 characteristics register"]
pub mod otg_hs_hcchar4;
#[doc = "OTG_HS_HCCHAR5 register accessor: an alias for `Reg<OTG_HS_HCCHAR5_SPEC>`"]
pub type OTG_HS_HCCHAR5 = crate::Reg<otg_hs_hcchar5::OTG_HS_HCCHAR5_SPEC>;
#[doc = "OTG_HS host channel-5 characteristics register"]
pub mod otg_hs_hcchar5;
#[doc = "OTG_HS_HCCHAR6 register accessor: an alias for `Reg<OTG_HS_HCCHAR6_SPEC>`"]
pub type OTG_HS_HCCHAR6 = crate::Reg<otg_hs_hcchar6::OTG_HS_HCCHAR6_SPEC>;
#[doc = "OTG_HS host channel-6 characteristics register"]
pub mod otg_hs_hcchar6;
#[doc = "OTG_HS_HCCHAR7 register accessor: an alias for `Reg<OTG_HS_HCCHAR7_SPEC>`"]
pub type OTG_HS_HCCHAR7 = crate::Reg<otg_hs_hcchar7::OTG_HS_HCCHAR7_SPEC>;
#[doc = "OTG_HS host channel-7 characteristics register"]
pub mod otg_hs_hcchar7;
#[doc = "OTG_HS_HCCHAR8 register accessor: an alias for `Reg<OTG_HS_HCCHAR8_SPEC>`"]
pub type OTG_HS_HCCHAR8 = crate::Reg<otg_hs_hcchar8::OTG_HS_HCCHAR8_SPEC>;
#[doc = "OTG_HS host channel-8 characteristics register"]
pub mod otg_hs_hcchar8;
#[doc = "OTG_HS_HCCHAR9 register accessor: an alias for `Reg<OTG_HS_HCCHAR9_SPEC>`"]
pub type OTG_HS_HCCHAR9 = crate::Reg<otg_hs_hcchar9::OTG_HS_HCCHAR9_SPEC>;
#[doc = "OTG_HS host channel-9 characteristics register"]
pub mod otg_hs_hcchar9;
#[doc = "OTG_HS_HCCHAR10 register accessor: an alias for `Reg<OTG_HS_HCCHAR10_SPEC>`"]
pub type OTG_HS_HCCHAR10 = crate::Reg<otg_hs_hcchar10::OTG_HS_HCCHAR10_SPEC>;
#[doc = "OTG_HS host channel-10 characteristics register"]
pub mod otg_hs_hcchar10;
#[doc = "OTG_HS_HCCHAR11 register accessor: an alias for `Reg<OTG_HS_HCCHAR11_SPEC>`"]
pub type OTG_HS_HCCHAR11 = crate::Reg<otg_hs_hcchar11::OTG_HS_HCCHAR11_SPEC>;
#[doc = "OTG_HS host channel-11 characteristics register"]
pub mod otg_hs_hcchar11;
#[doc = "OTG_HS_HCSPLT0 register accessor: an alias for `Reg<OTG_HS_HCSPLT0_SPEC>`"]
pub type OTG_HS_HCSPLT0 = crate::Reg<otg_hs_hcsplt0::OTG_HS_HCSPLT0_SPEC>;
#[doc = "OTG_HS host channel-0 split control register"]
pub mod otg_hs_hcsplt0;
#[doc = "OTG_HS_HCSPLT1 register accessor: an alias for `Reg<OTG_HS_HCSPLT1_SPEC>`"]
pub type OTG_HS_HCSPLT1 = crate::Reg<otg_hs_hcsplt1::OTG_HS_HCSPLT1_SPEC>;
#[doc = "OTG_HS host channel-1 split control register"]
pub mod otg_hs_hcsplt1;
#[doc = "OTG_HS_HCSPLT2 register accessor: an alias for `Reg<OTG_HS_HCSPLT2_SPEC>`"]
pub type OTG_HS_HCSPLT2 = crate::Reg<otg_hs_hcsplt2::OTG_HS_HCSPLT2_SPEC>;
#[doc = "OTG_HS host channel-2 split control register"]
pub mod otg_hs_hcsplt2;
#[doc = "OTG_HS_HCSPLT3 register accessor: an alias for `Reg<OTG_HS_HCSPLT3_SPEC>`"]
pub type OTG_HS_HCSPLT3 = crate::Reg<otg_hs_hcsplt3::OTG_HS_HCSPLT3_SPEC>;
#[doc = "OTG_HS host channel-3 split control register"]
pub mod otg_hs_hcsplt3;
#[doc = "OTG_HS_HCSPLT4 register accessor: an alias for `Reg<OTG_HS_HCSPLT4_SPEC>`"]
pub type OTG_HS_HCSPLT4 = crate::Reg<otg_hs_hcsplt4::OTG_HS_HCSPLT4_SPEC>;
#[doc = "OTG_HS host channel-4 split control register"]
pub mod otg_hs_hcsplt4;
#[doc = "OTG_HS_HCSPLT5 register accessor: an alias for `Reg<OTG_HS_HCSPLT5_SPEC>`"]
pub type OTG_HS_HCSPLT5 = crate::Reg<otg_hs_hcsplt5::OTG_HS_HCSPLT5_SPEC>;
#[doc = "OTG_HS host channel-5 split control register"]
pub mod otg_hs_hcsplt5;
#[doc = "OTG_HS_HCSPLT6 register accessor: an alias for `Reg<OTG_HS_HCSPLT6_SPEC>`"]
pub type OTG_HS_HCSPLT6 = crate::Reg<otg_hs_hcsplt6::OTG_HS_HCSPLT6_SPEC>;
#[doc = "OTG_HS host channel-6 split control register"]
pub mod otg_hs_hcsplt6;
#[doc = "OTG_HS_HCSPLT7 register accessor: an alias for `Reg<OTG_HS_HCSPLT7_SPEC>`"]
pub type OTG_HS_HCSPLT7 = crate::Reg<otg_hs_hcsplt7::OTG_HS_HCSPLT7_SPEC>;
#[doc = "OTG_HS host channel-7 split control register"]
pub mod otg_hs_hcsplt7;
#[doc = "OTG_HS_HCSPLT8 register accessor: an alias for `Reg<OTG_HS_HCSPLT8_SPEC>`"]
pub type OTG_HS_HCSPLT8 = crate::Reg<otg_hs_hcsplt8::OTG_HS_HCSPLT8_SPEC>;
#[doc = "OTG_HS host channel-8 split control register"]
pub mod otg_hs_hcsplt8;
#[doc = "OTG_HS_HCSPLT9 register accessor: an alias for `Reg<OTG_HS_HCSPLT9_SPEC>`"]
pub type OTG_HS_HCSPLT9 = crate::Reg<otg_hs_hcsplt9::OTG_HS_HCSPLT9_SPEC>;
#[doc = "OTG_HS host channel-9 split control register"]
pub mod otg_hs_hcsplt9;
#[doc = "OTG_HS_HCSPLT10 register accessor: an alias for `Reg<OTG_HS_HCSPLT10_SPEC>`"]
pub type OTG_HS_HCSPLT10 = crate::Reg<otg_hs_hcsplt10::OTG_HS_HCSPLT10_SPEC>;
#[doc = "OTG_HS host channel-10 split control register"]
pub mod otg_hs_hcsplt10;
#[doc = "OTG_HS_HCSPLT11 register accessor: an alias for `Reg<OTG_HS_HCSPLT11_SPEC>`"]
pub type OTG_HS_HCSPLT11 = crate::Reg<otg_hs_hcsplt11::OTG_HS_HCSPLT11_SPEC>;
#[doc = "OTG_HS host channel-11 split control register"]
pub mod otg_hs_hcsplt11;
#[doc = "OTG_HS_HCINT0 register accessor: an alias for `Reg<OTG_HS_HCINT0_SPEC>`"]
pub type OTG_HS_HCINT0 = crate::Reg<otg_hs_hcint0::OTG_HS_HCINT0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint0;
#[doc = "OTG_HS_HCINT1 register accessor: an alias for `Reg<OTG_HS_HCINT1_SPEC>`"]
pub type OTG_HS_HCINT1 = crate::Reg<otg_hs_hcint1::OTG_HS_HCINT1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt register"]
pub mod otg_hs_hcint1;
#[doc = "OTG_HS_HCINT2 register accessor: an alias for `Reg<OTG_HS_HCINT2_SPEC>`"]
pub type OTG_HS_HCINT2 = crate::Reg<otg_hs_hcint2::OTG_HS_HCINT2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt register"]
pub mod otg_hs_hcint2;
#[doc = "OTG_HS_HCINT3 register accessor: an alias for `Reg<OTG_HS_HCINT3_SPEC>`"]
pub type OTG_HS_HCINT3 = crate::Reg<otg_hs_hcint3::OTG_HS_HCINT3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt register"]
pub mod otg_hs_hcint3;
#[doc = "OTG_HS_HCINT4 register accessor: an alias for `Reg<OTG_HS_HCINT4_SPEC>`"]
pub type OTG_HS_HCINT4 = crate::Reg<otg_hs_hcint4::OTG_HS_HCINT4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt register"]
pub mod otg_hs_hcint4;
#[doc = "OTG_HS_HCINT5 register accessor: an alias for `Reg<OTG_HS_HCINT5_SPEC>`"]
pub type OTG_HS_HCINT5 = crate::Reg<otg_hs_hcint5::OTG_HS_HCINT5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt register"]
pub mod otg_hs_hcint5;
#[doc = "OTG_HS_HCINT6 register accessor: an alias for `Reg<OTG_HS_HCINT6_SPEC>`"]
pub type OTG_HS_HCINT6 = crate::Reg<otg_hs_hcint6::OTG_HS_HCINT6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt register"]
pub mod otg_hs_hcint6;
#[doc = "OTG_HS_HCINT7 register accessor: an alias for `Reg<OTG_HS_HCINT7_SPEC>`"]
pub type OTG_HS_HCINT7 = crate::Reg<otg_hs_hcint7::OTG_HS_HCINT7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt register"]
pub mod otg_hs_hcint7;
#[doc = "OTG_HS_HCINT8 register accessor: an alias for `Reg<OTG_HS_HCINT8_SPEC>`"]
pub type OTG_HS_HCINT8 = crate::Reg<otg_hs_hcint8::OTG_HS_HCINT8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt register"]
pub mod otg_hs_hcint8;
#[doc = "OTG_HS_HCINT9 register accessor: an alias for `Reg<OTG_HS_HCINT9_SPEC>`"]
pub type OTG_HS_HCINT9 = crate::Reg<otg_hs_hcint9::OTG_HS_HCINT9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt register"]
pub mod otg_hs_hcint9;
#[doc = "OTG_HS_HCINT10 register accessor: an alias for `Reg<OTG_HS_HCINT10_SPEC>`"]
pub type OTG_HS_HCINT10 = crate::Reg<otg_hs_hcint10::OTG_HS_HCINT10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt register"]
pub mod otg_hs_hcint10;
#[doc = "OTG_HS_HCINT11 register accessor: an alias for `Reg<OTG_HS_HCINT11_SPEC>`"]
pub type OTG_HS_HCINT11 = crate::Reg<otg_hs_hcint11::OTG_HS_HCINT11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt register"]
pub mod otg_hs_hcint11;
#[doc = "OTG_HS_HCINTMSK0 register accessor: an alias for `Reg<OTG_HS_HCINTMSK0_SPEC>`"]
pub type OTG_HS_HCINTMSK0 = crate::Reg<otg_hs_hcintmsk0::OTG_HS_HCINTMSK0_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk0;
#[doc = "OTG_HS_HCINTMSK1 register accessor: an alias for `Reg<OTG_HS_HCINTMSK1_SPEC>`"]
pub type OTG_HS_HCINTMSK1 = crate::Reg<otg_hs_hcintmsk1::OTG_HS_HCINTMSK1_SPEC>;
#[doc = "OTG_HS host channel-1 interrupt mask register"]
pub mod otg_hs_hcintmsk1;
#[doc = "OTG_HS_HCINTMSK2 register accessor: an alias for `Reg<OTG_HS_HCINTMSK2_SPEC>`"]
pub type OTG_HS_HCINTMSK2 = crate::Reg<otg_hs_hcintmsk2::OTG_HS_HCINTMSK2_SPEC>;
#[doc = "OTG_HS host channel-2 interrupt mask register"]
pub mod otg_hs_hcintmsk2;
#[doc = "OTG_HS_HCINTMSK3 register accessor: an alias for `Reg<OTG_HS_HCINTMSK3_SPEC>`"]
pub type OTG_HS_HCINTMSK3 = crate::Reg<otg_hs_hcintmsk3::OTG_HS_HCINTMSK3_SPEC>;
#[doc = "OTG_HS host channel-3 interrupt mask register"]
pub mod otg_hs_hcintmsk3;
#[doc = "OTG_HS_HCINTMSK4 register accessor: an alias for `Reg<OTG_HS_HCINTMSK4_SPEC>`"]
pub type OTG_HS_HCINTMSK4 = crate::Reg<otg_hs_hcintmsk4::OTG_HS_HCINTMSK4_SPEC>;
#[doc = "OTG_HS host channel-4 interrupt mask register"]
pub mod otg_hs_hcintmsk4;
#[doc = "OTG_HS_HCINTMSK5 register accessor: an alias for `Reg<OTG_HS_HCINTMSK5_SPEC>`"]
pub type OTG_HS_HCINTMSK5 = crate::Reg<otg_hs_hcintmsk5::OTG_HS_HCINTMSK5_SPEC>;
#[doc = "OTG_HS host channel-5 interrupt mask register"]
pub mod otg_hs_hcintmsk5;
#[doc = "OTG_HS_HCINTMSK6 register accessor: an alias for `Reg<OTG_HS_HCINTMSK6_SPEC>`"]
pub type OTG_HS_HCINTMSK6 = crate::Reg<otg_hs_hcintmsk6::OTG_HS_HCINTMSK6_SPEC>;
#[doc = "OTG_HS host channel-6 interrupt mask register"]
pub mod otg_hs_hcintmsk6;
#[doc = "OTG_HS_HCINTMSK7 register accessor: an alias for `Reg<OTG_HS_HCINTMSK7_SPEC>`"]
pub type OTG_HS_HCINTMSK7 = crate::Reg<otg_hs_hcintmsk7::OTG_HS_HCINTMSK7_SPEC>;
#[doc = "OTG_HS host channel-7 interrupt mask register"]
pub mod otg_hs_hcintmsk7;
#[doc = "OTG_HS_HCINTMSK8 register accessor: an alias for `Reg<OTG_HS_HCINTMSK8_SPEC>`"]
pub type OTG_HS_HCINTMSK8 = crate::Reg<otg_hs_hcintmsk8::OTG_HS_HCINTMSK8_SPEC>;
#[doc = "OTG_HS host channel-8 interrupt mask register"]
pub mod otg_hs_hcintmsk8;
#[doc = "OTG_HS_HCINTMSK9 register accessor: an alias for `Reg<OTG_HS_HCINTMSK9_SPEC>`"]
pub type OTG_HS_HCINTMSK9 = crate::Reg<otg_hs_hcintmsk9::OTG_HS_HCINTMSK9_SPEC>;
#[doc = "OTG_HS host channel-9 interrupt mask register"]
pub mod otg_hs_hcintmsk9;
#[doc = "OTG_HS_HCINTMSK10 register accessor: an alias for `Reg<OTG_HS_HCINTMSK10_SPEC>`"]
pub type OTG_HS_HCINTMSK10 = crate::Reg<otg_hs_hcintmsk10::OTG_HS_HCINTMSK10_SPEC>;
#[doc = "OTG_HS host channel-10 interrupt mask register"]
pub mod otg_hs_hcintmsk10;
#[doc = "OTG_HS_HCINTMSK11 register accessor: an alias for `Reg<OTG_HS_HCINTMSK11_SPEC>`"]
pub type OTG_HS_HCINTMSK11 = crate::Reg<otg_hs_hcintmsk11::OTG_HS_HCINTMSK11_SPEC>;
#[doc = "OTG_HS host channel-11 interrupt mask register"]
pub mod otg_hs_hcintmsk11;
#[doc = "OTG_HS_HCTSIZ0 register accessor: an alias for `Reg<OTG_HS_HCTSIZ0_SPEC>`"]
pub type OTG_HS_HCTSIZ0 = crate::Reg<otg_hs_hctsiz0::OTG_HS_HCTSIZ0_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz0;
#[doc = "OTG_HS_HCTSIZ1 register accessor: an alias for `Reg<OTG_HS_HCTSIZ1_SPEC>`"]
pub type OTG_HS_HCTSIZ1 = crate::Reg<otg_hs_hctsiz1::OTG_HS_HCTSIZ1_SPEC>;
#[doc = "OTG_HS host channel-1 transfer size register"]
pub mod otg_hs_hctsiz1;
#[doc = "OTG_HS_HCTSIZ2 register accessor: an alias for `Reg<OTG_HS_HCTSIZ2_SPEC>`"]
pub type OTG_HS_HCTSIZ2 = crate::Reg<otg_hs_hctsiz2::OTG_HS_HCTSIZ2_SPEC>;
#[doc = "OTG_HS host channel-2 transfer size register"]
pub mod otg_hs_hctsiz2;
#[doc = "OTG_HS_HCTSIZ3 register accessor: an alias for `Reg<OTG_HS_HCTSIZ3_SPEC>`"]
pub type OTG_HS_HCTSIZ3 = crate::Reg<otg_hs_hctsiz3::OTG_HS_HCTSIZ3_SPEC>;
#[doc = "OTG_HS host channel-3 transfer size register"]
pub mod otg_hs_hctsiz3;
#[doc = "OTG_HS_HCTSIZ4 register accessor: an alias for `Reg<OTG_HS_HCTSIZ4_SPEC>`"]
pub type OTG_HS_HCTSIZ4 = crate::Reg<otg_hs_hctsiz4::OTG_HS_HCTSIZ4_SPEC>;
#[doc = "OTG_HS host channel-4 transfer size register"]
pub mod otg_hs_hctsiz4;
#[doc = "OTG_HS_HCTSIZ5 register accessor: an alias for `Reg<OTG_HS_HCTSIZ5_SPEC>`"]
pub type OTG_HS_HCTSIZ5 = crate::Reg<otg_hs_hctsiz5::OTG_HS_HCTSIZ5_SPEC>;
#[doc = "OTG_HS host channel-5 transfer size register"]
pub mod otg_hs_hctsiz5;
#[doc = "OTG_HS_HCTSIZ6 register accessor: an alias for `Reg<OTG_HS_HCTSIZ6_SPEC>`"]
pub type OTG_HS_HCTSIZ6 = crate::Reg<otg_hs_hctsiz6::OTG_HS_HCTSIZ6_SPEC>;
#[doc = "OTG_HS host channel-6 transfer size register"]
pub mod otg_hs_hctsiz6;
#[doc = "OTG_HS_HCTSIZ7 register accessor: an alias for `Reg<OTG_HS_HCTSIZ7_SPEC>`"]
pub type OTG_HS_HCTSIZ7 = crate::Reg<otg_hs_hctsiz7::OTG_HS_HCTSIZ7_SPEC>;
#[doc = "OTG_HS host channel-7 transfer size register"]
pub mod otg_hs_hctsiz7;
#[doc = "OTG_HS_HCTSIZ8 register accessor: an alias for `Reg<OTG_HS_HCTSIZ8_SPEC>`"]
pub type OTG_HS_HCTSIZ8 = crate::Reg<otg_hs_hctsiz8::OTG_HS_HCTSIZ8_SPEC>;
#[doc = "OTG_HS host channel-8 transfer size register"]
pub mod otg_hs_hctsiz8;
#[doc = "OTG_HS_HCTSIZ9 register accessor: an alias for `Reg<OTG_HS_HCTSIZ9_SPEC>`"]
pub type OTG_HS_HCTSIZ9 = crate::Reg<otg_hs_hctsiz9::OTG_HS_HCTSIZ9_SPEC>;
#[doc = "OTG_HS host channel-9 transfer size register"]
pub mod otg_hs_hctsiz9;
#[doc = "OTG_HS_HCTSIZ10 register accessor: an alias for `Reg<OTG_HS_HCTSIZ10_SPEC>`"]
pub type OTG_HS_HCTSIZ10 = crate::Reg<otg_hs_hctsiz10::OTG_HS_HCTSIZ10_SPEC>;
#[doc = "OTG_HS host channel-10 transfer size register"]
pub mod otg_hs_hctsiz10;
#[doc = "OTG_HS_HCTSIZ11 register accessor: an alias for `Reg<OTG_HS_HCTSIZ11_SPEC>`"]
pub type OTG_HS_HCTSIZ11 = crate::Reg<otg_hs_hctsiz11::OTG_HS_HCTSIZ11_SPEC>;
#[doc = "OTG_HS host channel-11 transfer size register"]
pub mod otg_hs_hctsiz11;
#[doc = "OTG_HS_HCDMA0 register accessor: an alias for `Reg<OTG_HS_HCDMA0_SPEC>`"]
pub type OTG_HS_HCDMA0 = crate::Reg<otg_hs_hcdma0::OTG_HS_HCDMA0_SPEC>;
#[doc = "OTG_HS host channel-0 DMA address register"]
pub mod otg_hs_hcdma0;
#[doc = "OTG_HS_HCDMA1 register accessor: an alias for `Reg<OTG_HS_HCDMA1_SPEC>`"]
pub type OTG_HS_HCDMA1 = crate::Reg<otg_hs_hcdma1::OTG_HS_HCDMA1_SPEC>;
#[doc = "OTG_HS host channel-1 DMA address register"]
pub mod otg_hs_hcdma1;
#[doc = "OTG_HS_HCDMA2 register accessor: an alias for `Reg<OTG_HS_HCDMA2_SPEC>`"]
pub type OTG_HS_HCDMA2 = crate::Reg<otg_hs_hcdma2::OTG_HS_HCDMA2_SPEC>;
#[doc = "OTG_HS host channel-2 DMA address register"]
pub mod otg_hs_hcdma2;
#[doc = "OTG_HS_HCDMA3 register accessor: an alias for `Reg<OTG_HS_HCDMA3_SPEC>`"]
pub type OTG_HS_HCDMA3 = crate::Reg<otg_hs_hcdma3::OTG_HS_HCDMA3_SPEC>;
#[doc = "OTG_HS host channel-3 DMA address register"]
pub mod otg_hs_hcdma3;
#[doc = "OTG_HS_HCDMA4 register accessor: an alias for `Reg<OTG_HS_HCDMA4_SPEC>`"]
pub type OTG_HS_HCDMA4 = crate::Reg<otg_hs_hcdma4::OTG_HS_HCDMA4_SPEC>;
#[doc = "OTG_HS host channel-4 DMA address register"]
pub mod otg_hs_hcdma4;
#[doc = "OTG_HS_HCDMA5 register accessor: an alias for `Reg<OTG_HS_HCDMA5_SPEC>`"]
pub type OTG_HS_HCDMA5 = crate::Reg<otg_hs_hcdma5::OTG_HS_HCDMA5_SPEC>;
#[doc = "OTG_HS host channel-5 DMA address register"]
pub mod otg_hs_hcdma5;
#[doc = "OTG_HS_HCDMA6 register accessor: an alias for `Reg<OTG_HS_HCDMA6_SPEC>`"]
pub type OTG_HS_HCDMA6 = crate::Reg<otg_hs_hcdma6::OTG_HS_HCDMA6_SPEC>;
#[doc = "OTG_HS host channel-6 DMA address register"]
pub mod otg_hs_hcdma6;
#[doc = "OTG_HS_HCDMA7 register accessor: an alias for `Reg<OTG_HS_HCDMA7_SPEC>`"]
pub type OTG_HS_HCDMA7 = crate::Reg<otg_hs_hcdma7::OTG_HS_HCDMA7_SPEC>;
#[doc = "OTG_HS host channel-7 DMA address register"]
pub mod otg_hs_hcdma7;
#[doc = "OTG_HS_HCDMA8 register accessor: an alias for `Reg<OTG_HS_HCDMA8_SPEC>`"]
pub type OTG_HS_HCDMA8 = crate::Reg<otg_hs_hcdma8::OTG_HS_HCDMA8_SPEC>;
#[doc = "OTG_HS host channel-8 DMA address register"]
pub mod otg_hs_hcdma8;
#[doc = "OTG_HS_HCDMA9 register accessor: an alias for `Reg<OTG_HS_HCDMA9_SPEC>`"]
pub type OTG_HS_HCDMA9 = crate::Reg<otg_hs_hcdma9::OTG_HS_HCDMA9_SPEC>;
#[doc = "OTG_HS host channel-9 DMA address register"]
pub mod otg_hs_hcdma9;
#[doc = "OTG_HS_HCDMA10 register accessor: an alias for `Reg<OTG_HS_HCDMA10_SPEC>`"]
pub type OTG_HS_HCDMA10 = crate::Reg<otg_hs_hcdma10::OTG_HS_HCDMA10_SPEC>;
#[doc = "OTG_HS host channel-10 DMA address register"]
pub mod otg_hs_hcdma10;
#[doc = "OTG_HS_HCDMA11 register accessor: an alias for `Reg<OTG_HS_HCDMA11_SPEC>`"]
pub type OTG_HS_HCDMA11 = crate::Reg<otg_hs_hcdma11::OTG_HS_HCDMA11_SPEC>;
#[doc = "OTG_HS host channel-11 DMA address register"]
pub mod otg_hs_hcdma11;
