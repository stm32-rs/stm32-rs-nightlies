#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    otg_hs_hcfg: OTG_HS_HCFG,
    otg_hs_hfir: OTG_HS_HFIR,
    otg_hs_hfnum: OTG_HS_HFNUM,
    _reserved3: [u8; 0x04],
    otg_hs_hptxsts: OTG_HS_HPTXSTS,
    otg_hs_haint: OTG_HS_HAINT,
    otg_hs_haintmsk: OTG_HS_HAINTMSK,
    _reserved6: [u8; 0x24],
    otg_hs_hprt: OTG_HS_HPRT,
    _reserved7: [u8; 0xbc],
    otg_hs_hcchar0: OTG_HS_HCCHAR0,
    otg_hs_hcsplt0: OTG_HS_HCSPLT0,
    otg_hs_hcint0: OTG_HS_HCINT0,
    otg_hs_hcintmsk0: OTG_HS_HCINTMSK0,
    otg_hs_hctsiz0: OTG_HS_HCTSIZ0,
    otg_hs_hcdma0: OTG_HS_HCDMA0,
    _reserved13: [u8; 0x08],
    otg_hs_hcchar1: OTG_HS_HCCHAR1,
    otg_hs_hcsplt1: OTG_HS_HCSPLT1,
    otg_hs_hcint1: OTG_HS_HCINT1,
    otg_hs_hcintmsk1: OTG_HS_HCINTMSK1,
    otg_hs_hctsiz1: OTG_HS_HCTSIZ1,
    otg_hs_hcdma1: OTG_HS_HCDMA1,
    _reserved19: [u8; 0x08],
    otg_hs_hcchar2: OTG_HS_HCCHAR2,
    otg_hs_hcsplt2: OTG_HS_HCSPLT2,
    otg_hs_hcint2: OTG_HS_HCINT2,
    otg_hs_hcintmsk2: OTG_HS_HCINTMSK2,
    otg_hs_hctsiz2: OTG_HS_HCTSIZ2,
    otg_hs_hcdma2: OTG_HS_HCDMA2,
    _reserved25: [u8; 0x08],
    otg_hs_hcchar3: OTG_HS_HCCHAR3,
    otg_hs_hcsplt3: OTG_HS_HCSPLT3,
    otg_hs_hcint3: OTG_HS_HCINT3,
    otg_hs_hcintmsk3: OTG_HS_HCINTMSK3,
    otg_hs_hctsiz3: OTG_HS_HCTSIZ3,
    otg_hs_hcdma3: OTG_HS_HCDMA3,
    _reserved31: [u8; 0x08],
    otg_hs_hcchar4: OTG_HS_HCCHAR4,
    otg_hs_hcsplt4: OTG_HS_HCSPLT4,
    otg_hs_hcint4: OTG_HS_HCINT4,
    otg_hs_hcintmsk4: OTG_HS_HCINTMSK4,
    otg_hs_hctsiz4: OTG_HS_HCTSIZ4,
    otg_hs_hcdma4: OTG_HS_HCDMA4,
    _reserved37: [u8; 0x08],
    otg_hs_hcchar5: OTG_HS_HCCHAR5,
    otg_hs_hcsplt5: OTG_HS_HCSPLT5,
    otg_hs_hcint5: OTG_HS_HCINT5,
    otg_hs_hcintmsk5: OTG_HS_HCINTMSK5,
    otg_hs_hctsiz5: OTG_HS_HCTSIZ5,
    otg_hs_hcdma5: OTG_HS_HCDMA5,
    _reserved43: [u8; 0x08],
    otg_hs_hcchar6: OTG_HS_HCCHAR6,
    otg_hs_hcsplt6: OTG_HS_HCSPLT6,
    otg_hs_hcint6: OTG_HS_HCINT6,
    otg_hs_hcintmsk6: OTG_HS_HCINTMSK6,
    otg_hs_hctsiz6: OTG_HS_HCTSIZ6,
    otg_hs_hcdma6: OTG_HS_HCDMA6,
    _reserved49: [u8; 0x08],
    otg_hs_hcchar7: OTG_HS_HCCHAR7,
    otg_hs_hcsplt7: OTG_HS_HCSPLT7,
    otg_hs_hcint7: OTG_HS_HCINT7,
    otg_hs_hcintmsk7: OTG_HS_HCINTMSK7,
    otg_hs_hctsiz7: OTG_HS_HCTSIZ7,
    otg_hs_hcdma7: OTG_HS_HCDMA7,
    _reserved55: [u8; 0x08],
    otg_hs_hcchar8: OTG_HS_HCCHAR8,
    otg_hs_hcsplt8: OTG_HS_HCSPLT8,
    otg_hs_hcint8: OTG_HS_HCINT8,
    otg_hs_hcintmsk8: OTG_HS_HCINTMSK8,
    otg_hs_hctsiz8: OTG_HS_HCTSIZ8,
    otg_hs_hcdma8: OTG_HS_HCDMA8,
    _reserved61: [u8; 0x08],
    otg_hs_hcchar9: OTG_HS_HCCHAR9,
    otg_hs_hcsplt9: OTG_HS_HCSPLT9,
    otg_hs_hcint9: OTG_HS_HCINT9,
    otg_hs_hcintmsk9: OTG_HS_HCINTMSK9,
    otg_hs_hctsiz9: OTG_HS_HCTSIZ9,
    otg_hs_hcdma9: OTG_HS_HCDMA9,
    _reserved67: [u8; 0x08],
    otg_hs_hcchar10: OTG_HS_HCCHAR10,
    otg_hs_hcsplt10: OTG_HS_HCSPLT10,
    otg_hs_hcint10: OTG_HS_HCINT10,
    otg_hs_hcintmsk10: OTG_HS_HCINTMSK10,
    otg_hs_hctsiz10: OTG_HS_HCTSIZ10,
    otg_hs_hcdma10: OTG_HS_HCDMA10,
    _reserved73: [u8; 0x08],
    otg_hs_hcchar11: OTG_HS_HCCHAR11,
    otg_hs_hcsplt11: OTG_HS_HCSPLT11,
    otg_hs_hcint11: OTG_HS_HCINT11,
    otg_hs_hcintmsk11: OTG_HS_HCINTMSK11,
    otg_hs_hctsiz11: OTG_HS_HCTSIZ11,
    otg_hs_hcdma11: OTG_HS_HCDMA11,
}
impl RegisterBlock {
    ///0x00 - OTG_HS host configuration register
    #[inline(always)]
    pub const fn otg_hs_hcfg(&self) -> &OTG_HS_HCFG {
        &self.otg_hs_hcfg
    }
    ///0x04 - OTG_HS Host frame interval register
    #[inline(always)]
    pub const fn otg_hs_hfir(&self) -> &OTG_HS_HFIR {
        &self.otg_hs_hfir
    }
    ///0x08 - OTG_HS host frame number/frame time remaining register
    #[inline(always)]
    pub const fn otg_hs_hfnum(&self) -> &OTG_HS_HFNUM {
        &self.otg_hs_hfnum
    }
    ///0x10 - OTG_HS_Host periodic transmit FIFO/queue status register
    #[inline(always)]
    pub const fn otg_hs_hptxsts(&self) -> &OTG_HS_HPTXSTS {
        &self.otg_hs_hptxsts
    }
    ///0x14 - OTG_HS Host all channels interrupt register
    #[inline(always)]
    pub const fn otg_hs_haint(&self) -> &OTG_HS_HAINT {
        &self.otg_hs_haint
    }
    ///0x18 - OTG_HS host all channels interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_haintmsk(&self) -> &OTG_HS_HAINTMSK {
        &self.otg_hs_haintmsk
    }
    ///0x40 - OTG_HS host port control and status register
    #[inline(always)]
    pub const fn otg_hs_hprt(&self) -> &OTG_HS_HPRT {
        &self.otg_hs_hprt
    }
    ///0x100 - OTG_HS host channel-0 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar0(&self) -> &OTG_HS_HCCHAR0 {
        &self.otg_hs_hcchar0
    }
    ///0x104 - OTG_HS host channel-0 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt0(&self) -> &OTG_HS_HCSPLT0 {
        &self.otg_hs_hcsplt0
    }
    ///0x108 - OTG_HS host channel-11 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint0(&self) -> &OTG_HS_HCINT0 {
        &self.otg_hs_hcint0
    }
    ///0x10c - OTG_HS host channel-11 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk0(&self) -> &OTG_HS_HCINTMSK0 {
        &self.otg_hs_hcintmsk0
    }
    ///0x110 - OTG_HS host channel-11 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz0(&self) -> &OTG_HS_HCTSIZ0 {
        &self.otg_hs_hctsiz0
    }
    ///0x114 - OTG_HS host channel-0 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma0(&self) -> &OTG_HS_HCDMA0 {
        &self.otg_hs_hcdma0
    }
    ///0x120 - OTG_HS host channel-1 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar1(&self) -> &OTG_HS_HCCHAR1 {
        &self.otg_hs_hcchar1
    }
    ///0x124 - OTG_HS host channel-1 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt1(&self) -> &OTG_HS_HCSPLT1 {
        &self.otg_hs_hcsplt1
    }
    ///0x128 - OTG_HS host channel-1 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint1(&self) -> &OTG_HS_HCINT1 {
        &self.otg_hs_hcint1
    }
    ///0x12c - OTG_HS host channel-1 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk1(&self) -> &OTG_HS_HCINTMSK1 {
        &self.otg_hs_hcintmsk1
    }
    ///0x130 - OTG_HS host channel-1 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz1(&self) -> &OTG_HS_HCTSIZ1 {
        &self.otg_hs_hctsiz1
    }
    ///0x134 - OTG_HS host channel-1 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma1(&self) -> &OTG_HS_HCDMA1 {
        &self.otg_hs_hcdma1
    }
    ///0x140 - OTG_HS host channel-2 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar2(&self) -> &OTG_HS_HCCHAR2 {
        &self.otg_hs_hcchar2
    }
    ///0x144 - OTG_HS host channel-2 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt2(&self) -> &OTG_HS_HCSPLT2 {
        &self.otg_hs_hcsplt2
    }
    ///0x148 - OTG_HS host channel-2 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint2(&self) -> &OTG_HS_HCINT2 {
        &self.otg_hs_hcint2
    }
    ///0x14c - OTG_HS host channel-2 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk2(&self) -> &OTG_HS_HCINTMSK2 {
        &self.otg_hs_hcintmsk2
    }
    ///0x150 - OTG_HS host channel-2 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz2(&self) -> &OTG_HS_HCTSIZ2 {
        &self.otg_hs_hctsiz2
    }
    ///0x154 - OTG_HS host channel-2 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma2(&self) -> &OTG_HS_HCDMA2 {
        &self.otg_hs_hcdma2
    }
    ///0x160 - OTG_HS host channel-3 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar3(&self) -> &OTG_HS_HCCHAR3 {
        &self.otg_hs_hcchar3
    }
    ///0x164 - OTG_HS host channel-3 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt3(&self) -> &OTG_HS_HCSPLT3 {
        &self.otg_hs_hcsplt3
    }
    ///0x168 - OTG_HS host channel-3 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint3(&self) -> &OTG_HS_HCINT3 {
        &self.otg_hs_hcint3
    }
    ///0x16c - OTG_HS host channel-3 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk3(&self) -> &OTG_HS_HCINTMSK3 {
        &self.otg_hs_hcintmsk3
    }
    ///0x170 - OTG_HS host channel-3 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz3(&self) -> &OTG_HS_HCTSIZ3 {
        &self.otg_hs_hctsiz3
    }
    ///0x174 - OTG_HS host channel-3 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma3(&self) -> &OTG_HS_HCDMA3 {
        &self.otg_hs_hcdma3
    }
    ///0x180 - OTG_HS host channel-4 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar4(&self) -> &OTG_HS_HCCHAR4 {
        &self.otg_hs_hcchar4
    }
    ///0x184 - OTG_HS host channel-4 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt4(&self) -> &OTG_HS_HCSPLT4 {
        &self.otg_hs_hcsplt4
    }
    ///0x188 - OTG_HS host channel-4 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint4(&self) -> &OTG_HS_HCINT4 {
        &self.otg_hs_hcint4
    }
    ///0x18c - OTG_HS host channel-4 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk4(&self) -> &OTG_HS_HCINTMSK4 {
        &self.otg_hs_hcintmsk4
    }
    ///0x190 - OTG_HS host channel-4 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz4(&self) -> &OTG_HS_HCTSIZ4 {
        &self.otg_hs_hctsiz4
    }
    ///0x194 - OTG_HS host channel-4 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma4(&self) -> &OTG_HS_HCDMA4 {
        &self.otg_hs_hcdma4
    }
    ///0x1a0 - OTG_HS host channel-5 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar5(&self) -> &OTG_HS_HCCHAR5 {
        &self.otg_hs_hcchar5
    }
    ///0x1a4 - OTG_HS host channel-5 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt5(&self) -> &OTG_HS_HCSPLT5 {
        &self.otg_hs_hcsplt5
    }
    ///0x1a8 - OTG_HS host channel-5 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint5(&self) -> &OTG_HS_HCINT5 {
        &self.otg_hs_hcint5
    }
    ///0x1ac - OTG_HS host channel-5 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk5(&self) -> &OTG_HS_HCINTMSK5 {
        &self.otg_hs_hcintmsk5
    }
    ///0x1b0 - OTG_HS host channel-5 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz5(&self) -> &OTG_HS_HCTSIZ5 {
        &self.otg_hs_hctsiz5
    }
    ///0x1b4 - OTG_HS host channel-5 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma5(&self) -> &OTG_HS_HCDMA5 {
        &self.otg_hs_hcdma5
    }
    ///0x1c0 - OTG_HS host channel-6 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar6(&self) -> &OTG_HS_HCCHAR6 {
        &self.otg_hs_hcchar6
    }
    ///0x1c4 - OTG_HS host channel-6 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt6(&self) -> &OTG_HS_HCSPLT6 {
        &self.otg_hs_hcsplt6
    }
    ///0x1c8 - OTG_HS host channel-6 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint6(&self) -> &OTG_HS_HCINT6 {
        &self.otg_hs_hcint6
    }
    ///0x1cc - OTG_HS host channel-6 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk6(&self) -> &OTG_HS_HCINTMSK6 {
        &self.otg_hs_hcintmsk6
    }
    ///0x1d0 - OTG_HS host channel-6 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz6(&self) -> &OTG_HS_HCTSIZ6 {
        &self.otg_hs_hctsiz6
    }
    ///0x1d4 - OTG_HS host channel-6 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma6(&self) -> &OTG_HS_HCDMA6 {
        &self.otg_hs_hcdma6
    }
    ///0x1e0 - OTG_HS host channel-7 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar7(&self) -> &OTG_HS_HCCHAR7 {
        &self.otg_hs_hcchar7
    }
    ///0x1e4 - OTG_HS host channel-7 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt7(&self) -> &OTG_HS_HCSPLT7 {
        &self.otg_hs_hcsplt7
    }
    ///0x1e8 - OTG_HS host channel-7 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint7(&self) -> &OTG_HS_HCINT7 {
        &self.otg_hs_hcint7
    }
    ///0x1ec - OTG_HS host channel-7 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk7(&self) -> &OTG_HS_HCINTMSK7 {
        &self.otg_hs_hcintmsk7
    }
    ///0x1f0 - OTG_HS host channel-7 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz7(&self) -> &OTG_HS_HCTSIZ7 {
        &self.otg_hs_hctsiz7
    }
    ///0x1f4 - OTG_HS host channel-7 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma7(&self) -> &OTG_HS_HCDMA7 {
        &self.otg_hs_hcdma7
    }
    ///0x200 - OTG_HS host channel-8 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar8(&self) -> &OTG_HS_HCCHAR8 {
        &self.otg_hs_hcchar8
    }
    ///0x204 - OTG_HS host channel-8 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt8(&self) -> &OTG_HS_HCSPLT8 {
        &self.otg_hs_hcsplt8
    }
    ///0x208 - OTG_HS host channel-8 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint8(&self) -> &OTG_HS_HCINT8 {
        &self.otg_hs_hcint8
    }
    ///0x20c - OTG_HS host channel-8 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk8(&self) -> &OTG_HS_HCINTMSK8 {
        &self.otg_hs_hcintmsk8
    }
    ///0x210 - OTG_HS host channel-8 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz8(&self) -> &OTG_HS_HCTSIZ8 {
        &self.otg_hs_hctsiz8
    }
    ///0x214 - OTG_HS host channel-8 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma8(&self) -> &OTG_HS_HCDMA8 {
        &self.otg_hs_hcdma8
    }
    ///0x220 - OTG_HS host channel-9 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar9(&self) -> &OTG_HS_HCCHAR9 {
        &self.otg_hs_hcchar9
    }
    ///0x224 - OTG_HS host channel-9 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt9(&self) -> &OTG_HS_HCSPLT9 {
        &self.otg_hs_hcsplt9
    }
    ///0x228 - OTG_HS host channel-9 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint9(&self) -> &OTG_HS_HCINT9 {
        &self.otg_hs_hcint9
    }
    ///0x22c - OTG_HS host channel-9 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk9(&self) -> &OTG_HS_HCINTMSK9 {
        &self.otg_hs_hcintmsk9
    }
    ///0x230 - OTG_HS host channel-9 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz9(&self) -> &OTG_HS_HCTSIZ9 {
        &self.otg_hs_hctsiz9
    }
    ///0x234 - OTG_HS host channel-9 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma9(&self) -> &OTG_HS_HCDMA9 {
        &self.otg_hs_hcdma9
    }
    ///0x240 - OTG_HS host channel-10 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar10(&self) -> &OTG_HS_HCCHAR10 {
        &self.otg_hs_hcchar10
    }
    ///0x244 - OTG_HS host channel-10 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt10(&self) -> &OTG_HS_HCSPLT10 {
        &self.otg_hs_hcsplt10
    }
    ///0x248 - OTG_HS host channel-10 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint10(&self) -> &OTG_HS_HCINT10 {
        &self.otg_hs_hcint10
    }
    ///0x24c - OTG_HS host channel-10 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk10(&self) -> &OTG_HS_HCINTMSK10 {
        &self.otg_hs_hcintmsk10
    }
    ///0x250 - OTG_HS host channel-10 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz10(&self) -> &OTG_HS_HCTSIZ10 {
        &self.otg_hs_hctsiz10
    }
    ///0x254 - OTG_HS host channel-10 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma10(&self) -> &OTG_HS_HCDMA10 {
        &self.otg_hs_hcdma10
    }
    ///0x260 - OTG_HS host channel-11 characteristics register
    #[inline(always)]
    pub const fn otg_hs_hcchar11(&self) -> &OTG_HS_HCCHAR11 {
        &self.otg_hs_hcchar11
    }
    ///0x264 - OTG_HS host channel-11 split control register
    #[inline(always)]
    pub const fn otg_hs_hcsplt11(&self) -> &OTG_HS_HCSPLT11 {
        &self.otg_hs_hcsplt11
    }
    ///0x268 - OTG_HS host channel-11 interrupt register
    #[inline(always)]
    pub const fn otg_hs_hcint11(&self) -> &OTG_HS_HCINT11 {
        &self.otg_hs_hcint11
    }
    ///0x26c - OTG_HS host channel-11 interrupt mask register
    #[inline(always)]
    pub const fn otg_hs_hcintmsk11(&self) -> &OTG_HS_HCINTMSK11 {
        &self.otg_hs_hcintmsk11
    }
    ///0x270 - OTG_HS host channel-11 transfer size register
    #[inline(always)]
    pub const fn otg_hs_hctsiz11(&self) -> &OTG_HS_HCTSIZ11 {
        &self.otg_hs_hctsiz11
    }
    ///0x274 - OTG_HS host channel-11 DMA address register
    #[inline(always)]
    pub const fn otg_hs_hcdma11(&self) -> &OTG_HS_HCDMA11 {
        &self.otg_hs_hcdma11
    }
}
/**OTG_HS_HCFG (rw) register accessor: OTG_HS host configuration register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCFG)

For information about available fields see [`mod@otg_hs_hcfg`] module*/
pub type OTG_HS_HCFG = crate::Reg<otg_hs_hcfg::OTG_HS_HCFGrs>;
///OTG_HS host configuration register
pub mod otg_hs_hcfg;
/**OTG_HS_HFIR (rw) register accessor: OTG_HS Host frame interval register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HFIR)

For information about available fields see [`mod@otg_hs_hfir`] module*/
pub type OTG_HS_HFIR = crate::Reg<otg_hs_hfir::OTG_HS_HFIRrs>;
///OTG_HS Host frame interval register
pub mod otg_hs_hfir;
/**OTG_HS_HFNUM (r) register accessor: OTG_HS host frame number/frame time remaining register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HFNUM)

For information about available fields see [`mod@otg_hs_hfnum`] module*/
pub type OTG_HS_HFNUM = crate::Reg<otg_hs_hfnum::OTG_HS_HFNUMrs>;
///OTG_HS host frame number/frame time remaining register
pub mod otg_hs_hfnum;
/**OTG_HS_HPTXSTS (rw) register accessor: OTG_HS_Host periodic transmit FIFO/queue status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hptxsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hptxsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HPTXSTS)

For information about available fields see [`mod@otg_hs_hptxsts`] module*/
pub type OTG_HS_HPTXSTS = crate::Reg<otg_hs_hptxsts::OTG_HS_HPTXSTSrs>;
///OTG_HS_Host periodic transmit FIFO/queue status register
pub mod otg_hs_hptxsts;
/**OTG_HS_HAINT (r) register accessor: OTG_HS Host all channels interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HAINT)

For information about available fields see [`mod@otg_hs_haint`] module*/
pub type OTG_HS_HAINT = crate::Reg<otg_hs_haint::OTG_HS_HAINTrs>;
///OTG_HS Host all channels interrupt register
pub mod otg_hs_haint;
/**OTG_HS_HAINTMSK (rw) register accessor: OTG_HS host all channels interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HAINTMSK)

For information about available fields see [`mod@otg_hs_haintmsk`] module*/
pub type OTG_HS_HAINTMSK = crate::Reg<otg_hs_haintmsk::OTG_HS_HAINTMSKrs>;
///OTG_HS host all channels interrupt mask register
pub mod otg_hs_haintmsk;
/**OTG_HS_HPRT (rw) register accessor: OTG_HS host port control and status register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HPRT)

For information about available fields see [`mod@otg_hs_hprt`] module*/
pub type OTG_HS_HPRT = crate::Reg<otg_hs_hprt::OTG_HS_HPRTrs>;
///OTG_HS host port control and status register
pub mod otg_hs_hprt;
/**OTG_HS_HCCHAR0 (rw) register accessor: OTG_HS host channel-0 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR0)

For information about available fields see [`mod@otg_hs_hcchar0`] module*/
pub type OTG_HS_HCCHAR0 = crate::Reg<otg_hs_hcchar0::OTG_HS_HCCHAR0rs>;
///OTG_HS host channel-0 characteristics register
pub mod otg_hs_hcchar0;
/**OTG_HS_HCCHAR1 (rw) register accessor: OTG_HS host channel-1 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR1)

For information about available fields see [`mod@otg_hs_hcchar1`] module*/
pub type OTG_HS_HCCHAR1 = crate::Reg<otg_hs_hcchar1::OTG_HS_HCCHAR1rs>;
///OTG_HS host channel-1 characteristics register
pub mod otg_hs_hcchar1;
/**OTG_HS_HCCHAR2 (rw) register accessor: OTG_HS host channel-2 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR2)

For information about available fields see [`mod@otg_hs_hcchar2`] module*/
pub type OTG_HS_HCCHAR2 = crate::Reg<otg_hs_hcchar2::OTG_HS_HCCHAR2rs>;
///OTG_HS host channel-2 characteristics register
pub mod otg_hs_hcchar2;
/**OTG_HS_HCCHAR3 (rw) register accessor: OTG_HS host channel-3 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR3)

For information about available fields see [`mod@otg_hs_hcchar3`] module*/
pub type OTG_HS_HCCHAR3 = crate::Reg<otg_hs_hcchar3::OTG_HS_HCCHAR3rs>;
///OTG_HS host channel-3 characteristics register
pub mod otg_hs_hcchar3;
/**OTG_HS_HCCHAR4 (rw) register accessor: OTG_HS host channel-4 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR4)

For information about available fields see [`mod@otg_hs_hcchar4`] module*/
pub type OTG_HS_HCCHAR4 = crate::Reg<otg_hs_hcchar4::OTG_HS_HCCHAR4rs>;
///OTG_HS host channel-4 characteristics register
pub mod otg_hs_hcchar4;
/**OTG_HS_HCCHAR5 (rw) register accessor: OTG_HS host channel-5 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR5)

For information about available fields see [`mod@otg_hs_hcchar5`] module*/
pub type OTG_HS_HCCHAR5 = crate::Reg<otg_hs_hcchar5::OTG_HS_HCCHAR5rs>;
///OTG_HS host channel-5 characteristics register
pub mod otg_hs_hcchar5;
/**OTG_HS_HCCHAR6 (rw) register accessor: OTG_HS host channel-6 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR6)

For information about available fields see [`mod@otg_hs_hcchar6`] module*/
pub type OTG_HS_HCCHAR6 = crate::Reg<otg_hs_hcchar6::OTG_HS_HCCHAR6rs>;
///OTG_HS host channel-6 characteristics register
pub mod otg_hs_hcchar6;
/**OTG_HS_HCCHAR7 (rw) register accessor: OTG_HS host channel-7 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR7)

For information about available fields see [`mod@otg_hs_hcchar7`] module*/
pub type OTG_HS_HCCHAR7 = crate::Reg<otg_hs_hcchar7::OTG_HS_HCCHAR7rs>;
///OTG_HS host channel-7 characteristics register
pub mod otg_hs_hcchar7;
/**OTG_HS_HCCHAR8 (rw) register accessor: OTG_HS host channel-8 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR8)

For information about available fields see [`mod@otg_hs_hcchar8`] module*/
pub type OTG_HS_HCCHAR8 = crate::Reg<otg_hs_hcchar8::OTG_HS_HCCHAR8rs>;
///OTG_HS host channel-8 characteristics register
pub mod otg_hs_hcchar8;
/**OTG_HS_HCCHAR9 (rw) register accessor: OTG_HS host channel-9 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR9)

For information about available fields see [`mod@otg_hs_hcchar9`] module*/
pub type OTG_HS_HCCHAR9 = crate::Reg<otg_hs_hcchar9::OTG_HS_HCCHAR9rs>;
///OTG_HS host channel-9 characteristics register
pub mod otg_hs_hcchar9;
/**OTG_HS_HCCHAR10 (rw) register accessor: OTG_HS host channel-10 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR10)

For information about available fields see [`mod@otg_hs_hcchar10`] module*/
pub type OTG_HS_HCCHAR10 = crate::Reg<otg_hs_hcchar10::OTG_HS_HCCHAR10rs>;
///OTG_HS host channel-10 characteristics register
pub mod otg_hs_hcchar10;
/**OTG_HS_HCCHAR11 (rw) register accessor: OTG_HS host channel-11 characteristics register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcchar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcchar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCCHAR11)

For information about available fields see [`mod@otg_hs_hcchar11`] module*/
pub type OTG_HS_HCCHAR11 = crate::Reg<otg_hs_hcchar11::OTG_HS_HCCHAR11rs>;
///OTG_HS host channel-11 characteristics register
pub mod otg_hs_hcchar11;
/**OTG_HS_HCSPLT0 (rw) register accessor: OTG_HS host channel-0 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT0)

For information about available fields see [`mod@otg_hs_hcsplt0`] module*/
pub type OTG_HS_HCSPLT0 = crate::Reg<otg_hs_hcsplt0::OTG_HS_HCSPLT0rs>;
///OTG_HS host channel-0 split control register
pub mod otg_hs_hcsplt0;
/**OTG_HS_HCSPLT1 (rw) register accessor: OTG_HS host channel-1 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT1)

For information about available fields see [`mod@otg_hs_hcsplt1`] module*/
pub type OTG_HS_HCSPLT1 = crate::Reg<otg_hs_hcsplt1::OTG_HS_HCSPLT1rs>;
///OTG_HS host channel-1 split control register
pub mod otg_hs_hcsplt1;
/**OTG_HS_HCSPLT2 (rw) register accessor: OTG_HS host channel-2 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT2)

For information about available fields see [`mod@otg_hs_hcsplt2`] module*/
pub type OTG_HS_HCSPLT2 = crate::Reg<otg_hs_hcsplt2::OTG_HS_HCSPLT2rs>;
///OTG_HS host channel-2 split control register
pub mod otg_hs_hcsplt2;
/**OTG_HS_HCSPLT3 (rw) register accessor: OTG_HS host channel-3 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT3)

For information about available fields see [`mod@otg_hs_hcsplt3`] module*/
pub type OTG_HS_HCSPLT3 = crate::Reg<otg_hs_hcsplt3::OTG_HS_HCSPLT3rs>;
///OTG_HS host channel-3 split control register
pub mod otg_hs_hcsplt3;
/**OTG_HS_HCSPLT4 (rw) register accessor: OTG_HS host channel-4 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT4)

For information about available fields see [`mod@otg_hs_hcsplt4`] module*/
pub type OTG_HS_HCSPLT4 = crate::Reg<otg_hs_hcsplt4::OTG_HS_HCSPLT4rs>;
///OTG_HS host channel-4 split control register
pub mod otg_hs_hcsplt4;
/**OTG_HS_HCSPLT5 (rw) register accessor: OTG_HS host channel-5 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT5)

For information about available fields see [`mod@otg_hs_hcsplt5`] module*/
pub type OTG_HS_HCSPLT5 = crate::Reg<otg_hs_hcsplt5::OTG_HS_HCSPLT5rs>;
///OTG_HS host channel-5 split control register
pub mod otg_hs_hcsplt5;
/**OTG_HS_HCSPLT6 (rw) register accessor: OTG_HS host channel-6 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT6)

For information about available fields see [`mod@otg_hs_hcsplt6`] module*/
pub type OTG_HS_HCSPLT6 = crate::Reg<otg_hs_hcsplt6::OTG_HS_HCSPLT6rs>;
///OTG_HS host channel-6 split control register
pub mod otg_hs_hcsplt6;
/**OTG_HS_HCSPLT7 (rw) register accessor: OTG_HS host channel-7 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT7)

For information about available fields see [`mod@otg_hs_hcsplt7`] module*/
pub type OTG_HS_HCSPLT7 = crate::Reg<otg_hs_hcsplt7::OTG_HS_HCSPLT7rs>;
///OTG_HS host channel-7 split control register
pub mod otg_hs_hcsplt7;
/**OTG_HS_HCSPLT8 (rw) register accessor: OTG_HS host channel-8 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT8)

For information about available fields see [`mod@otg_hs_hcsplt8`] module*/
pub type OTG_HS_HCSPLT8 = crate::Reg<otg_hs_hcsplt8::OTG_HS_HCSPLT8rs>;
///OTG_HS host channel-8 split control register
pub mod otg_hs_hcsplt8;
/**OTG_HS_HCSPLT9 (rw) register accessor: OTG_HS host channel-9 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT9)

For information about available fields see [`mod@otg_hs_hcsplt9`] module*/
pub type OTG_HS_HCSPLT9 = crate::Reg<otg_hs_hcsplt9::OTG_HS_HCSPLT9rs>;
///OTG_HS host channel-9 split control register
pub mod otg_hs_hcsplt9;
/**OTG_HS_HCSPLT10 (rw) register accessor: OTG_HS host channel-10 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT10)

For information about available fields see [`mod@otg_hs_hcsplt10`] module*/
pub type OTG_HS_HCSPLT10 = crate::Reg<otg_hs_hcsplt10::OTG_HS_HCSPLT10rs>;
///OTG_HS host channel-10 split control register
pub mod otg_hs_hcsplt10;
/**OTG_HS_HCSPLT11 (rw) register accessor: OTG_HS host channel-11 split control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcsplt11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcsplt11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCSPLT11)

For information about available fields see [`mod@otg_hs_hcsplt11`] module*/
pub type OTG_HS_HCSPLT11 = crate::Reg<otg_hs_hcsplt11::OTG_HS_HCSPLT11rs>;
///OTG_HS host channel-11 split control register
pub mod otg_hs_hcsplt11;
/**OTG_HS_HCINT0 (rw) register accessor: OTG_HS host channel-11 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT0)

For information about available fields see [`mod@otg_hs_hcint0`] module*/
pub type OTG_HS_HCINT0 = crate::Reg<otg_hs_hcint0::OTG_HS_HCINT0rs>;
///OTG_HS host channel-11 interrupt register
pub mod otg_hs_hcint0;
/**OTG_HS_HCINT1 (rw) register accessor: OTG_HS host channel-1 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT1)

For information about available fields see [`mod@otg_hs_hcint1`] module*/
pub type OTG_HS_HCINT1 = crate::Reg<otg_hs_hcint1::OTG_HS_HCINT1rs>;
///OTG_HS host channel-1 interrupt register
pub mod otg_hs_hcint1;
/**OTG_HS_HCINT2 (rw) register accessor: OTG_HS host channel-2 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT2)

For information about available fields see [`mod@otg_hs_hcint2`] module*/
pub type OTG_HS_HCINT2 = crate::Reg<otg_hs_hcint2::OTG_HS_HCINT2rs>;
///OTG_HS host channel-2 interrupt register
pub mod otg_hs_hcint2;
/**OTG_HS_HCINT3 (rw) register accessor: OTG_HS host channel-3 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT3)

For information about available fields see [`mod@otg_hs_hcint3`] module*/
pub type OTG_HS_HCINT3 = crate::Reg<otg_hs_hcint3::OTG_HS_HCINT3rs>;
///OTG_HS host channel-3 interrupt register
pub mod otg_hs_hcint3;
/**OTG_HS_HCINT4 (rw) register accessor: OTG_HS host channel-4 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT4)

For information about available fields see [`mod@otg_hs_hcint4`] module*/
pub type OTG_HS_HCINT4 = crate::Reg<otg_hs_hcint4::OTG_HS_HCINT4rs>;
///OTG_HS host channel-4 interrupt register
pub mod otg_hs_hcint4;
/**OTG_HS_HCINT5 (rw) register accessor: OTG_HS host channel-5 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT5)

For information about available fields see [`mod@otg_hs_hcint5`] module*/
pub type OTG_HS_HCINT5 = crate::Reg<otg_hs_hcint5::OTG_HS_HCINT5rs>;
///OTG_HS host channel-5 interrupt register
pub mod otg_hs_hcint5;
/**OTG_HS_HCINT6 (rw) register accessor: OTG_HS host channel-6 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT6)

For information about available fields see [`mod@otg_hs_hcint6`] module*/
pub type OTG_HS_HCINT6 = crate::Reg<otg_hs_hcint6::OTG_HS_HCINT6rs>;
///OTG_HS host channel-6 interrupt register
pub mod otg_hs_hcint6;
/**OTG_HS_HCINT7 (rw) register accessor: OTG_HS host channel-7 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT7)

For information about available fields see [`mod@otg_hs_hcint7`] module*/
pub type OTG_HS_HCINT7 = crate::Reg<otg_hs_hcint7::OTG_HS_HCINT7rs>;
///OTG_HS host channel-7 interrupt register
pub mod otg_hs_hcint7;
/**OTG_HS_HCINT8 (rw) register accessor: OTG_HS host channel-8 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT8)

For information about available fields see [`mod@otg_hs_hcint8`] module*/
pub type OTG_HS_HCINT8 = crate::Reg<otg_hs_hcint8::OTG_HS_HCINT8rs>;
///OTG_HS host channel-8 interrupt register
pub mod otg_hs_hcint8;
/**OTG_HS_HCINT9 (rw) register accessor: OTG_HS host channel-9 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT9)

For information about available fields see [`mod@otg_hs_hcint9`] module*/
pub type OTG_HS_HCINT9 = crate::Reg<otg_hs_hcint9::OTG_HS_HCINT9rs>;
///OTG_HS host channel-9 interrupt register
pub mod otg_hs_hcint9;
/**OTG_HS_HCINT10 (rw) register accessor: OTG_HS host channel-10 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT10)

For information about available fields see [`mod@otg_hs_hcint10`] module*/
pub type OTG_HS_HCINT10 = crate::Reg<otg_hs_hcint10::OTG_HS_HCINT10rs>;
///OTG_HS host channel-10 interrupt register
pub mod otg_hs_hcint10;
/**OTG_HS_HCINT11 (rw) register accessor: OTG_HS host channel-11 interrupt register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINT11)

For information about available fields see [`mod@otg_hs_hcint11`] module*/
pub type OTG_HS_HCINT11 = crate::Reg<otg_hs_hcint11::OTG_HS_HCINT11rs>;
///OTG_HS host channel-11 interrupt register
pub mod otg_hs_hcint11;
/**OTG_HS_HCINTMSK0 (rw) register accessor: OTG_HS host channel-11 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK0)

For information about available fields see [`mod@otg_hs_hcintmsk0`] module*/
pub type OTG_HS_HCINTMSK0 = crate::Reg<otg_hs_hcintmsk0::OTG_HS_HCINTMSK0rs>;
///OTG_HS host channel-11 interrupt mask register
pub mod otg_hs_hcintmsk0;
/**OTG_HS_HCINTMSK1 (rw) register accessor: OTG_HS host channel-1 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK1)

For information about available fields see [`mod@otg_hs_hcintmsk1`] module*/
pub type OTG_HS_HCINTMSK1 = crate::Reg<otg_hs_hcintmsk1::OTG_HS_HCINTMSK1rs>;
///OTG_HS host channel-1 interrupt mask register
pub mod otg_hs_hcintmsk1;
/**OTG_HS_HCINTMSK2 (rw) register accessor: OTG_HS host channel-2 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK2)

For information about available fields see [`mod@otg_hs_hcintmsk2`] module*/
pub type OTG_HS_HCINTMSK2 = crate::Reg<otg_hs_hcintmsk2::OTG_HS_HCINTMSK2rs>;
///OTG_HS host channel-2 interrupt mask register
pub mod otg_hs_hcintmsk2;
/**OTG_HS_HCINTMSK3 (rw) register accessor: OTG_HS host channel-3 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK3)

For information about available fields see [`mod@otg_hs_hcintmsk3`] module*/
pub type OTG_HS_HCINTMSK3 = crate::Reg<otg_hs_hcintmsk3::OTG_HS_HCINTMSK3rs>;
///OTG_HS host channel-3 interrupt mask register
pub mod otg_hs_hcintmsk3;
/**OTG_HS_HCINTMSK4 (rw) register accessor: OTG_HS host channel-4 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK4)

For information about available fields see [`mod@otg_hs_hcintmsk4`] module*/
pub type OTG_HS_HCINTMSK4 = crate::Reg<otg_hs_hcintmsk4::OTG_HS_HCINTMSK4rs>;
///OTG_HS host channel-4 interrupt mask register
pub mod otg_hs_hcintmsk4;
/**OTG_HS_HCINTMSK5 (rw) register accessor: OTG_HS host channel-5 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK5)

For information about available fields see [`mod@otg_hs_hcintmsk5`] module*/
pub type OTG_HS_HCINTMSK5 = crate::Reg<otg_hs_hcintmsk5::OTG_HS_HCINTMSK5rs>;
///OTG_HS host channel-5 interrupt mask register
pub mod otg_hs_hcintmsk5;
/**OTG_HS_HCINTMSK6 (rw) register accessor: OTG_HS host channel-6 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK6)

For information about available fields see [`mod@otg_hs_hcintmsk6`] module*/
pub type OTG_HS_HCINTMSK6 = crate::Reg<otg_hs_hcintmsk6::OTG_HS_HCINTMSK6rs>;
///OTG_HS host channel-6 interrupt mask register
pub mod otg_hs_hcintmsk6;
/**OTG_HS_HCINTMSK7 (rw) register accessor: OTG_HS host channel-7 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK7)

For information about available fields see [`mod@otg_hs_hcintmsk7`] module*/
pub type OTG_HS_HCINTMSK7 = crate::Reg<otg_hs_hcintmsk7::OTG_HS_HCINTMSK7rs>;
///OTG_HS host channel-7 interrupt mask register
pub mod otg_hs_hcintmsk7;
/**OTG_HS_HCINTMSK8 (rw) register accessor: OTG_HS host channel-8 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK8)

For information about available fields see [`mod@otg_hs_hcintmsk8`] module*/
pub type OTG_HS_HCINTMSK8 = crate::Reg<otg_hs_hcintmsk8::OTG_HS_HCINTMSK8rs>;
///OTG_HS host channel-8 interrupt mask register
pub mod otg_hs_hcintmsk8;
/**OTG_HS_HCINTMSK9 (rw) register accessor: OTG_HS host channel-9 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK9)

For information about available fields see [`mod@otg_hs_hcintmsk9`] module*/
pub type OTG_HS_HCINTMSK9 = crate::Reg<otg_hs_hcintmsk9::OTG_HS_HCINTMSK9rs>;
///OTG_HS host channel-9 interrupt mask register
pub mod otg_hs_hcintmsk9;
/**OTG_HS_HCINTMSK10 (rw) register accessor: OTG_HS host channel-10 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK10)

For information about available fields see [`mod@otg_hs_hcintmsk10`] module*/
pub type OTG_HS_HCINTMSK10 = crate::Reg<otg_hs_hcintmsk10::OTG_HS_HCINTMSK10rs>;
///OTG_HS host channel-10 interrupt mask register
pub mod otg_hs_hcintmsk10;
/**OTG_HS_HCINTMSK11 (rw) register accessor: OTG_HS host channel-11 interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcintmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcintmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCINTMSK11)

For information about available fields see [`mod@otg_hs_hcintmsk11`] module*/
pub type OTG_HS_HCINTMSK11 = crate::Reg<otg_hs_hcintmsk11::OTG_HS_HCINTMSK11rs>;
///OTG_HS host channel-11 interrupt mask register
pub mod otg_hs_hcintmsk11;
/**OTG_HS_HCTSIZ0 (rw) register accessor: OTG_HS host channel-11 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ0)

For information about available fields see [`mod@otg_hs_hctsiz0`] module*/
pub type OTG_HS_HCTSIZ0 = crate::Reg<otg_hs_hctsiz0::OTG_HS_HCTSIZ0rs>;
///OTG_HS host channel-11 transfer size register
pub mod otg_hs_hctsiz0;
/**OTG_HS_HCTSIZ1 (rw) register accessor: OTG_HS host channel-1 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ1)

For information about available fields see [`mod@otg_hs_hctsiz1`] module*/
pub type OTG_HS_HCTSIZ1 = crate::Reg<otg_hs_hctsiz1::OTG_HS_HCTSIZ1rs>;
///OTG_HS host channel-1 transfer size register
pub mod otg_hs_hctsiz1;
/**OTG_HS_HCTSIZ2 (rw) register accessor: OTG_HS host channel-2 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ2)

For information about available fields see [`mod@otg_hs_hctsiz2`] module*/
pub type OTG_HS_HCTSIZ2 = crate::Reg<otg_hs_hctsiz2::OTG_HS_HCTSIZ2rs>;
///OTG_HS host channel-2 transfer size register
pub mod otg_hs_hctsiz2;
/**OTG_HS_HCTSIZ3 (rw) register accessor: OTG_HS host channel-3 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ3)

For information about available fields see [`mod@otg_hs_hctsiz3`] module*/
pub type OTG_HS_HCTSIZ3 = crate::Reg<otg_hs_hctsiz3::OTG_HS_HCTSIZ3rs>;
///OTG_HS host channel-3 transfer size register
pub mod otg_hs_hctsiz3;
/**OTG_HS_HCTSIZ4 (rw) register accessor: OTG_HS host channel-4 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ4)

For information about available fields see [`mod@otg_hs_hctsiz4`] module*/
pub type OTG_HS_HCTSIZ4 = crate::Reg<otg_hs_hctsiz4::OTG_HS_HCTSIZ4rs>;
///OTG_HS host channel-4 transfer size register
pub mod otg_hs_hctsiz4;
/**OTG_HS_HCTSIZ5 (rw) register accessor: OTG_HS host channel-5 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ5)

For information about available fields see [`mod@otg_hs_hctsiz5`] module*/
pub type OTG_HS_HCTSIZ5 = crate::Reg<otg_hs_hctsiz5::OTG_HS_HCTSIZ5rs>;
///OTG_HS host channel-5 transfer size register
pub mod otg_hs_hctsiz5;
/**OTG_HS_HCTSIZ6 (rw) register accessor: OTG_HS host channel-6 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ6)

For information about available fields see [`mod@otg_hs_hctsiz6`] module*/
pub type OTG_HS_HCTSIZ6 = crate::Reg<otg_hs_hctsiz6::OTG_HS_HCTSIZ6rs>;
///OTG_HS host channel-6 transfer size register
pub mod otg_hs_hctsiz6;
/**OTG_HS_HCTSIZ7 (rw) register accessor: OTG_HS host channel-7 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ7)

For information about available fields see [`mod@otg_hs_hctsiz7`] module*/
pub type OTG_HS_HCTSIZ7 = crate::Reg<otg_hs_hctsiz7::OTG_HS_HCTSIZ7rs>;
///OTG_HS host channel-7 transfer size register
pub mod otg_hs_hctsiz7;
/**OTG_HS_HCTSIZ8 (rw) register accessor: OTG_HS host channel-8 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ8)

For information about available fields see [`mod@otg_hs_hctsiz8`] module*/
pub type OTG_HS_HCTSIZ8 = crate::Reg<otg_hs_hctsiz8::OTG_HS_HCTSIZ8rs>;
///OTG_HS host channel-8 transfer size register
pub mod otg_hs_hctsiz8;
/**OTG_HS_HCTSIZ9 (rw) register accessor: OTG_HS host channel-9 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ9)

For information about available fields see [`mod@otg_hs_hctsiz9`] module*/
pub type OTG_HS_HCTSIZ9 = crate::Reg<otg_hs_hctsiz9::OTG_HS_HCTSIZ9rs>;
///OTG_HS host channel-9 transfer size register
pub mod otg_hs_hctsiz9;
/**OTG_HS_HCTSIZ10 (rw) register accessor: OTG_HS host channel-10 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ10)

For information about available fields see [`mod@otg_hs_hctsiz10`] module*/
pub type OTG_HS_HCTSIZ10 = crate::Reg<otg_hs_hctsiz10::OTG_HS_HCTSIZ10rs>;
///OTG_HS host channel-10 transfer size register
pub mod otg_hs_hctsiz10;
/**OTG_HS_HCTSIZ11 (rw) register accessor: OTG_HS host channel-11 transfer size register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hctsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hctsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCTSIZ11)

For information about available fields see [`mod@otg_hs_hctsiz11`] module*/
pub type OTG_HS_HCTSIZ11 = crate::Reg<otg_hs_hctsiz11::OTG_HS_HCTSIZ11rs>;
///OTG_HS host channel-11 transfer size register
pub mod otg_hs_hctsiz11;
/**OTG_HS_HCDMA0 (rw) register accessor: OTG_HS host channel-0 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA0)

For information about available fields see [`mod@otg_hs_hcdma0`] module*/
pub type OTG_HS_HCDMA0 = crate::Reg<otg_hs_hcdma0::OTG_HS_HCDMA0rs>;
///OTG_HS host channel-0 DMA address register
pub mod otg_hs_hcdma0;
/**OTG_HS_HCDMA1 (rw) register accessor: OTG_HS host channel-1 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA1)

For information about available fields see [`mod@otg_hs_hcdma1`] module*/
pub type OTG_HS_HCDMA1 = crate::Reg<otg_hs_hcdma1::OTG_HS_HCDMA1rs>;
///OTG_HS host channel-1 DMA address register
pub mod otg_hs_hcdma1;
/**OTG_HS_HCDMA2 (rw) register accessor: OTG_HS host channel-2 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA2)

For information about available fields see [`mod@otg_hs_hcdma2`] module*/
pub type OTG_HS_HCDMA2 = crate::Reg<otg_hs_hcdma2::OTG_HS_HCDMA2rs>;
///OTG_HS host channel-2 DMA address register
pub mod otg_hs_hcdma2;
/**OTG_HS_HCDMA3 (rw) register accessor: OTG_HS host channel-3 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA3)

For information about available fields see [`mod@otg_hs_hcdma3`] module*/
pub type OTG_HS_HCDMA3 = crate::Reg<otg_hs_hcdma3::OTG_HS_HCDMA3rs>;
///OTG_HS host channel-3 DMA address register
pub mod otg_hs_hcdma3;
/**OTG_HS_HCDMA4 (rw) register accessor: OTG_HS host channel-4 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA4)

For information about available fields see [`mod@otg_hs_hcdma4`] module*/
pub type OTG_HS_HCDMA4 = crate::Reg<otg_hs_hcdma4::OTG_HS_HCDMA4rs>;
///OTG_HS host channel-4 DMA address register
pub mod otg_hs_hcdma4;
/**OTG_HS_HCDMA5 (rw) register accessor: OTG_HS host channel-5 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA5)

For information about available fields see [`mod@otg_hs_hcdma5`] module*/
pub type OTG_HS_HCDMA5 = crate::Reg<otg_hs_hcdma5::OTG_HS_HCDMA5rs>;
///OTG_HS host channel-5 DMA address register
pub mod otg_hs_hcdma5;
/**OTG_HS_HCDMA6 (rw) register accessor: OTG_HS host channel-6 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA6)

For information about available fields see [`mod@otg_hs_hcdma6`] module*/
pub type OTG_HS_HCDMA6 = crate::Reg<otg_hs_hcdma6::OTG_HS_HCDMA6rs>;
///OTG_HS host channel-6 DMA address register
pub mod otg_hs_hcdma6;
/**OTG_HS_HCDMA7 (rw) register accessor: OTG_HS host channel-7 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA7)

For information about available fields see [`mod@otg_hs_hcdma7`] module*/
pub type OTG_HS_HCDMA7 = crate::Reg<otg_hs_hcdma7::OTG_HS_HCDMA7rs>;
///OTG_HS host channel-7 DMA address register
pub mod otg_hs_hcdma7;
/**OTG_HS_HCDMA8 (rw) register accessor: OTG_HS host channel-8 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA8)

For information about available fields see [`mod@otg_hs_hcdma8`] module*/
pub type OTG_HS_HCDMA8 = crate::Reg<otg_hs_hcdma8::OTG_HS_HCDMA8rs>;
///OTG_HS host channel-8 DMA address register
pub mod otg_hs_hcdma8;
/**OTG_HS_HCDMA9 (rw) register accessor: OTG_HS host channel-9 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA9)

For information about available fields see [`mod@otg_hs_hcdma9`] module*/
pub type OTG_HS_HCDMA9 = crate::Reg<otg_hs_hcdma9::OTG_HS_HCDMA9rs>;
///OTG_HS host channel-9 DMA address register
pub mod otg_hs_hcdma9;
/**OTG_HS_HCDMA10 (rw) register accessor: OTG_HS host channel-10 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA10)

For information about available fields see [`mod@otg_hs_hcdma10`] module*/
pub type OTG_HS_HCDMA10 = crate::Reg<otg_hs_hcdma10::OTG_HS_HCDMA10rs>;
///OTG_HS host channel-10 DMA address register
pub mod otg_hs_hcdma10;
/**OTG_HS_HCDMA11 (rw) register accessor: OTG_HS host channel-11 DMA address register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#OTG_HS_HOST:OTG_HS_HCDMA11)

For information about available fields see [`mod@otg_hs_hcdma11`] module*/
pub type OTG_HS_HCDMA11 = crate::Reg<otg_hs_hcdma11::OTG_HS_HCDMA11rs>;
///OTG_HS host channel-11 DMA address register
pub mod otg_hs_hcdma11;
