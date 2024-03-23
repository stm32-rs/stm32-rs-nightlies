#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otg_gotgctl: OTG_GOTGCTL,
    otg_gotgint: OTG_GOTGINT,
    otg_gahbcfg: OTG_GAHBCFG,
    otg_gusbcfg: OTG_GUSBCFG,
    otg_grstctl: OTG_GRSTCTL,
    otg_gintsts: OTG_GINTSTS,
    otg_gintmsk: OTG_GINTMSK,
    otg_grxstsr: OTG_GRXSTSR,
    otg_grxstsp: OTG_GRXSTSP,
    otg_grxfsiz: OTG_GRXFSIZ,
    otg_hnptxfsiz: OTG_HNPTXFSIZ,
    otg_hnptxsts: OTG_HNPTXSTS,
    _reserved12: [u8; 0x08],
    otg_gccfg: OTG_GCCFG,
    otg_cid: OTG_CID,
    _reserved14: [u8; 0x14],
    otg_glpmcfg: OTG_GLPMCFG,
    _reserved15: [u8; 0xa8],
    otg_hptxfsiz: OTG_HPTXFSIZ,
    otg_dieptxf1: OTG_DIEPTXF1,
    otg_dieptxf2: OTG_DIEPTXF2,
    otg_dieptxf3: OTG_DIEPTXF3,
    otg_dieptxf4: OTG_DIEPTXF4,
    otg_dieptxf5: OTG_DIEPTXF5,
    otg_dieptxf6: OTG_DIEPTXF6,
    otg_dieptxf7: OTG_DIEPTXF7,
    otg_dieptxf8: OTG_DIEPTXF8,
    _reserved24: [u8; 0x02dc],
    otg_hcfg: OTG_HCFG,
    otg_hfir: OTG_HFIR,
    otg_hfnum: OTG_HFNUM,
    _reserved27: [u8; 0x04],
    otg_hptxsts: OTG_HPTXSTS,
    otg_haint: OTG_HAINT,
    otg_haintmsk: OTG_HAINTMSK,
    otg_hflbaddr: OTG_HFLBADDR,
    _reserved31: [u8; 0x20],
    otg_hprt: OTG_HPRT,
    _reserved32: [u8; 0xbc],
    otg_hcchar0: OTG_HCCHAR0,
    otg_hcsplt0: OTG_HCSPLT0,
    otg_hcint0: OTG_HCINT0,
    otg_hcintmsk0: OTG_HCINTMSK0,
    otg_hctsiz0: OTG_HCTSIZ0,
    otg_hcdma0: OTG_HCDMA0,
    _reserved38: [u8; 0x04],
    otg_hcdmab0: OTG_HCDMAB0,
    otg_hcchar1: OTG_HCCHAR1,
    otg_hcsplt1: OTG_HCSPLT1,
    otg_hcint1: OTG_HCINT1,
    otg_hcintmsk1: OTG_HCINTMSK1,
    otg_hctsiz1: OTG_HCTSIZ1,
    otg_hcdma1: OTG_HCDMA1,
    _reserved45: [u8; 0x04],
    otg_hcdmab1: OTG_HCDMAB1,
    otg_hcchar2: OTG_HCCHAR2,
    otg_hcsplt2: OTG_HCSPLT2,
    otg_hcint2: OTG_HCINT2,
    otg_hcintmsk2: OTG_HCINTMSK2,
    otg_hctsiz2: OTG_HCTSIZ2,
    otg_hcdma2: OTG_HCDMA2,
    _reserved52: [u8; 0x04],
    otg_hcdmab2: OTG_HCDMAB2,
    otg_hcchar3: OTG_HCCHAR3,
    otg_hcsplt3: OTG_HCSPLT3,
    otg_hcint3: OTG_HCINT3,
    otg_hcintmsk3: OTG_HCINTMSK3,
    otg_hctsiz3: OTG_HCTSIZ3,
    otg_hcdma3: OTG_HCDMA3,
    _reserved59: [u8; 0x04],
    otg_hcdmab3: OTG_HCDMAB3,
    otg_hcchar4: OTG_HCCHAR4,
    otg_hcsplt4: OTG_HCSPLT4,
    otg_hcint4: OTG_HCINT4,
    otg_hcintmsk4: OTG_HCINTMSK4,
    otg_hctsiz4: OTG_HCTSIZ4,
    otg_hcdma4: OTG_HCDMA4,
    _reserved66: [u8; 0x04],
    otg_hcdmab4: OTG_HCDMAB4,
    otg_hcchar5: OTG_HCCHAR5,
    otg_hcsplt5: OTG_HCSPLT5,
    otg_hcint5: OTG_HCINT5,
    otg_hcintmsk5: OTG_HCINTMSK5,
    otg_hctsiz5: OTG_HCTSIZ5,
    otg_hcdma5: OTG_HCDMA5,
    _reserved73: [u8; 0x04],
    otg_hcdmab5: OTG_HCDMAB5,
    otg_hcchar6: OTG_HCCHAR6,
    otg_hcsplt6: OTG_HCSPLT6,
    otg_hcint6: OTG_HCINT6,
    otg_hcintmsk6: OTG_HCINTMSK6,
    otg_hctsiz6: OTG_HCTSIZ6,
    otg_hcdma6: OTG_HCDMA6,
    _reserved80: [u8; 0x04],
    otg_hcdmab6: OTG_HCDMAB6,
    otg_hcchar7: OTG_HCCHAR7,
    otg_hcsplt7: OTG_HCSPLT7,
    otg_hcint7: OTG_HCINT7,
    otg_hcintmsk7: OTG_HCINTMSK7,
    otg_hctsiz7: OTG_HCTSIZ7,
    otg_hcdma7: OTG_HCDMA7,
    _reserved87: [u8; 0x04],
    otg_hcdmab7: OTG_HCDMAB7,
    otg_hcchar8: OTG_HCCHAR8,
    otg_hcsplt8: OTG_HCSPLT8,
    otg_hcint8: OTG_HCINT8,
    otg_hcintmsk8: OTG_HCINTMSK8,
    otg_hctsiz8: OTG_HCTSIZ8,
    otg_hcdma8: OTG_HCDMA8,
    _reserved94: [u8; 0x04],
    otg_hcdmab8: OTG_HCDMAB8,
    otg_hcchar9: OTG_HCCHAR9,
    otg_hcsplt9: OTG_HCSPLT9,
    otg_hcint9: OTG_HCINT9,
    otg_hcintmsk9: OTG_HCINTMSK9,
    otg_hctsiz9: OTG_HCTSIZ9,
    otg_hcdma9: OTG_HCDMA9,
    _reserved101: [u8; 0x04],
    otg_hcdmab9: OTG_HCDMAB9,
    otg_hcchar10: OTG_HCCHAR10,
    otg_hcsplt10: OTG_HCSPLT10,
    otg_hcint10: OTG_HCINT10,
    otg_hcintmsk10: OTG_HCINTMSK10,
    otg_hctsiz10: OTG_HCTSIZ10,
    otg_hcdma10: OTG_HCDMA10,
    _reserved108: [u8; 0x04],
    otg_hcdmab10: OTG_HCDMAB10,
    otg_hcchar11: OTG_HCCHAR11,
    otg_hcsplt11: OTG_HCSPLT11,
    otg_hcint11: OTG_HCINT11,
    otg_hcintmsk11: OTG_HCINTMSK11,
    otg_hctsiz11: OTG_HCTSIZ11,
    otg_hcdma11: OTG_HCDMA11,
    _reserved115: [u8; 0x04],
    otg_hcdmab11: OTG_HCDMAB11,
    otg_hcchar12: OTG_HCCHAR12,
    otg_hcsplt12: OTG_HCSPLT12,
    otg_hcint12: OTG_HCINT12,
    otg_hcintmsk12: OTG_HCINTMSK12,
    otg_hctsiz12: OTG_HCTSIZ12,
    otg_hcdma12: OTG_HCDMA12,
    _reserved122: [u8; 0x04],
    otg_hcdmab12: OTG_HCDMAB12,
    otg_hcchar13: OTG_HCCHAR13,
    otg_hcsplt13: OTG_HCSPLT13,
    otg_hcint13: OTG_HCINT13,
    otg_hcintmsk13: OTG_HCINTMSK13,
    otg_hctsiz13: OTG_HCTSIZ13,
    otg_hcdma13: OTG_HCDMA13,
    _reserved129: [u8; 0x04],
    otg_hcdmab13: OTG_HCDMAB13,
    otg_hcchar14: OTG_HCCHAR14,
    otg_hcsplt14: OTG_HCSPLT14,
    otg_hcint14: OTG_HCINT14,
    otg_hcintmsk14: OTG_HCINTMSK14,
    otg_hctsiz14: OTG_HCTSIZ14,
    otg_hcdma14: OTG_HCDMA14,
    _reserved136: [u8; 0x04],
    otg_hcdmab14: OTG_HCDMAB14,
    otg_hcchar15: OTG_HCCHAR15,
    otg_hcsplt15: OTG_HCSPLT15,
    otg_hcint15: OTG_HCINT15,
    otg_hcintmsk15: OTG_HCINTMSK15,
    otg_hctsiz15: OTG_HCTSIZ15,
    otg_hcdma15: OTG_HCDMA15,
    _reserved143: [u8; 0x04],
    otg_hcdmab15: OTG_HCDMAB15,
    _reserved144: [u8; 0x0100],
    otg_dcfg: OTG_DCFG,
    otg_dctl: OTG_DCTL,
    otg_dsts: OTG_DSTS,
    _reserved147: [u8; 0x04],
    otg_diepmsk: OTG_DIEPMSK,
    otg_doepmsk: OTG_DOEPMSK,
    otg_daint: OTG_DAINT,
    otg_daintmsk: OTG_DAINTMSK,
    _reserved151: [u8; 0x08],
    otg_dvbusdis: OTG_DVBUSDIS,
    otg_dvbuspulse: OTG_DVBUSPULSE,
    otg_dthrctl: OTG_DTHRCTL,
    otg_diepempmsk: OTG_DIEPEMPMSK,
    otg_deachint: OTG_DEACHINT,
    otg_deachintmsk: OTG_DEACHINTMSK,
    _reserved157: [u8; 0x04],
    otg_hs_diepeachmsk1: OTG_HS_DIEPEACHMSK1,
    _reserved158: [u8; 0x3c],
    otg_hs_doepeachmsk1: OTG_HS_DOEPEACHMSK1,
    _reserved159: [u8; 0x78],
    otg_diepctl0: OTG_DIEPCTL0,
    _reserved160: [u8; 0x04],
    otg_diepint0: OTG_DIEPINT0,
    _reserved161: [u8; 0x04],
    otg_dieptsiz0: OTG_DIEPTSIZ0,
    otg_diepdma0: OTG_DIEPDMA0,
    otg_dtxfsts0: OTG_DTXFSTS0,
    _reserved164: [u8; 0x04],
    otg_diepctl1: OTG_DIEPCTL1,
    _reserved165: [u8; 0x04],
    otg_diepint1: OTG_DIEPINT1,
    _reserved166: [u8; 0x04],
    otg_dieptsiz1: OTG_DIEPTSIZ1,
    otg_diepdma1: OTG_DIEPDMA1,
    otg_dtxfsts1: OTG_DTXFSTS1,
    _reserved169: [u8; 0x04],
    otg_diepctl2: OTG_DIEPCTL2,
    _reserved170: [u8; 0x04],
    otg_diepint2: OTG_DIEPINT2,
    _reserved171: [u8; 0x04],
    otg_dieptsiz2: OTG_DIEPTSIZ2,
    otg_diepdma2: OTG_DIEPDMA2,
    otg_dtxfsts2: OTG_DTXFSTS2,
    _reserved174: [u8; 0x04],
    otg_diepctl3: OTG_DIEPCTL3,
    _reserved175: [u8; 0x04],
    otg_diepint3: OTG_DIEPINT3,
    _reserved176: [u8; 0x04],
    otg_dieptsiz3: OTG_DIEPTSIZ3,
    otg_diepdma3: OTG_DIEPDMA3,
    otg_dtxfsts3: OTG_DTXFSTS3,
    _reserved179: [u8; 0x04],
    otg_diepctl4: OTG_DIEPCTL4,
    _reserved180: [u8; 0x04],
    otg_diepint4: OTG_DIEPINT4,
    _reserved181: [u8; 0x04],
    otg_dieptsiz4: OTG_DIEPTSIZ4,
    otg_diepdma4: OTG_DIEPDMA4,
    otg_dtxfsts4: OTG_DTXFSTS4,
    _reserved184: [u8; 0x04],
    otg_diepctl5: OTG_DIEPCTL5,
    _reserved185: [u8; 0x04],
    otg_diepint5: OTG_DIEPINT5,
    _reserved186: [u8; 0x04],
    otg_dieptsiz5: OTG_DIEPTSIZ5,
    otg_diepdma5: OTG_DIEPDMA5,
    otg_dtxfsts5: OTG_DTXFSTS5,
    _reserved189: [u8; 0x04],
    otg_diepctl6: OTG_DIEPCTL6,
    _reserved190: [u8; 0x04],
    otg_diepint6: OTG_DIEPINT6,
    _reserved191: [u8; 0x04],
    otg_dieptsiz6: OTG_DIEPTSIZ6,
    otg_diepdma6: OTG_DIEPDMA6,
    otg_dtxfsts6: OTG_DTXFSTS6,
    _reserved194: [u8; 0x04],
    otg_diepctl7: OTG_DIEPCTL7,
    _reserved195: [u8; 0x04],
    otg_diepint7: OTG_DIEPINT7,
    _reserved196: [u8; 0x04],
    otg_dieptsiz7: OTG_DIEPTSIZ7,
    otg_diepdma7: OTG_DIEPDMA7,
    otg_dtxfsts7: OTG_DTXFSTS7,
    _reserved199: [u8; 0x04],
    otg_diepctl8: OTG_DIEPCTL8,
    _reserved200: [u8; 0x04],
    otg_diepint8: OTG_DIEPINT8,
    _reserved201: [u8; 0x04],
    otg_dieptsiz8: OTG_DIEPTSIZ8,
    otg_diepdma8: OTG_DIEPDMA8,
    otg_dtxfsts8: OTG_DTXFSTS8,
    _reserved204: [u8; 0xe4],
    otg_doepctl0: OTG_DOEPCTL0,
    _reserved205: [u8; 0x04],
    otg_doepint0: OTG_DOEPINT0,
    _reserved206: [u8; 0x04],
    otg_doeptsiz0: OTG_DOEPTSIZ0,
    otg_doepdma0: OTG_DOEPDMA0,
    _reserved208: [u8; 0x08],
    otg_doepctl1: OTG_DOEPCTL1,
    _reserved209: [u8; 0x04],
    otg_doepint1: OTG_DOEPINT1,
    _reserved210: [u8; 0x04],
    otg_doeptsiz1: OTG_DOEPTSIZ1,
    otg_doepdma1: OTG_DOEPDMA1,
    _reserved212: [u8; 0x08],
    otg_doepctl2: OTG_DOEPCTL2,
    _reserved213: [u8; 0x04],
    otg_doepint2: OTG_DOEPINT2,
    _reserved214: [u8; 0x04],
    otg_doeptsiz2: OTG_DOEPTSIZ2,
    otg_doepdma2: OTG_DOEPDMA2,
    _reserved216: [u8; 0x08],
    otg_doepctl3: OTG_DOEPCTL3,
    _reserved217: [u8; 0x04],
    otg_doepint3: OTG_DOEPINT3,
    _reserved218: [u8; 0x04],
    otg_doeptsiz3: OTG_DOEPTSIZ3,
    otg_doepdma3: OTG_DOEPDMA3,
    _reserved220: [u8; 0x08],
    otg_doepctl4: OTG_DOEPCTL4,
    _reserved221: [u8; 0x04],
    otg_doepint4: OTG_DOEPINT4,
    _reserved222: [u8; 0x04],
    otg_doeptsiz4: OTG_DOEPTSIZ4,
    otg_doepdma4: OTG_DOEPDMA4,
    _reserved224: [u8; 0x08],
    otg_doepctl5: OTG_DOEPCTL5,
    _reserved225: [u8; 0x04],
    otg_doepint5: OTG_DOEPINT5,
    _reserved226: [u8; 0x04],
    otg_doeptsiz5: OTG_DOEPTSIZ5,
    otg_doepdma5: OTG_DOEPDMA5,
    _reserved228: [u8; 0x08],
    otg_doepctl6: OTG_DOEPCTL6,
    _reserved229: [u8; 0x04],
    otg_doepint6: OTG_DOEPINT6,
    _reserved230: [u8; 0x04],
    otg_doeptsiz6: OTG_DOEPTSIZ6,
    otg_doepdma6: OTG_DOEPDMA6,
    _reserved232: [u8; 0x08],
    otg_doepctl7: OTG_DOEPCTL7,
    _reserved233: [u8; 0x04],
    otg_doepint7: OTG_DOEPINT7,
    _reserved234: [u8; 0x04],
    otg_doeptsiz7: OTG_DOEPTSIZ7,
    otg_doepdma7: OTG_DOEPDMA7,
    _reserved236: [u8; 0x08],
    otg_doepctl8: OTG_DOEPCTL8,
    _reserved237: [u8; 0x04],
    otg_doepint8: OTG_DOEPINT8,
    _reserved238: [u8; 0x04],
    otg_doeptsiz8: OTG_DOEPTSIZ8,
    otg_doepdma8: OTG_DOEPDMA8,
    _reserved240: [u8; 0x01e8],
    otg_pcgcctl: OTG_PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
    #[inline(always)]
    pub const fn otg_gotgctl(&self) -> &OTG_GOTGCTL {
        &self.otg_gotgctl
    }
    #[doc = "0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
    #[inline(always)]
    pub const fn otg_gotgint(&self) -> &OTG_GOTGINT {
        &self.otg_gotgint
    }
    #[doc = "0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
    #[inline(always)]
    pub const fn otg_gahbcfg(&self) -> &OTG_GAHBCFG {
        &self.otg_gahbcfg
    }
    #[doc = "0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
    #[inline(always)]
    pub const fn otg_gusbcfg(&self) -> &OTG_GUSBCFG {
        &self.otg_gusbcfg
    }
    #[doc = "0x10 - The application uses this register to reset various hardware features inside the core."]
    #[inline(always)]
    pub const fn otg_grstctl(&self) -> &OTG_GRSTCTL {
        &self.otg_grstctl
    }
    #[doc = "0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
    #[inline(always)]
    pub const fn otg_gintsts(&self) -> &OTG_GINTSTS {
        &self.otg_gintsts
    }
    #[doc = "0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
    #[inline(always)]
    pub const fn otg_gintmsk(&self) -> &OTG_GINTMSK {
        &self.otg_gintmsk
    }
    #[doc = "0x1c - This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
    #[inline(always)]
    pub const fn otg_grxstsr(&self) -> &OTG_GRXSTSR {
        &self.otg_grxstsr
    }
    #[doc = "0x20 - This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
    #[inline(always)]
    pub const fn otg_grxstsp(&self) -> &OTG_GRXSTSP {
        &self.otg_grxstsp
    }
    #[doc = "0x24 - The application can program the RAM size that must be allocated to the Rx FIFO."]
    #[inline(always)]
    pub const fn otg_grxfsiz(&self) -> &OTG_GRXFSIZ {
        &self.otg_grxfsiz
    }
    #[doc = "0x28 - Host mode"]
    #[inline(always)]
    pub const fn otg_hnptxfsiz(&self) -> &OTG_HNPTXFSIZ {
        &self.otg_hnptxfsiz
    }
    #[doc = "0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
    #[inline(always)]
    pub const fn otg_hnptxsts(&self) -> &OTG_HNPTXSTS {
        &self.otg_hnptxsts
    }
    #[doc = "0x38 - OTG general core configuration register"]
    #[inline(always)]
    pub const fn otg_gccfg(&self) -> &OTG_GCCFG {
        &self.otg_gccfg
    }
    #[doc = "0x3c - This is a register containing the Product ID as reset value."]
    #[inline(always)]
    pub const fn otg_cid(&self) -> &OTG_CID {
        &self.otg_cid
    }
    #[doc = "0x54 - OTG core LPM configuration register"]
    #[inline(always)]
    pub const fn otg_glpmcfg(&self) -> &OTG_GLPMCFG {
        &self.otg_glpmcfg
    }
    #[doc = "0x100 - OTG host periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn otg_hptxfsiz(&self) -> &OTG_HPTXFSIZ {
        &self.otg_hptxfsiz
    }
    #[doc = "0x104 - OTG device IN endpoint transmit FIFO 1 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf1(&self) -> &OTG_DIEPTXF1 {
        &self.otg_dieptxf1
    }
    #[doc = "0x108 - OTG device IN endpoint transmit FIFO 2 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf2(&self) -> &OTG_DIEPTXF2 {
        &self.otg_dieptxf2
    }
    #[doc = "0x10c - OTG device IN endpoint transmit FIFO 3 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf3(&self) -> &OTG_DIEPTXF3 {
        &self.otg_dieptxf3
    }
    #[doc = "0x110 - OTG device IN endpoint transmit FIFO 4 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf4(&self) -> &OTG_DIEPTXF4 {
        &self.otg_dieptxf4
    }
    #[doc = "0x114 - OTG device IN endpoint transmit FIFO 5 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf5(&self) -> &OTG_DIEPTXF5 {
        &self.otg_dieptxf5
    }
    #[doc = "0x118 - OTG device IN endpoint transmit FIFO 6 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf6(&self) -> &OTG_DIEPTXF6 {
        &self.otg_dieptxf6
    }
    #[doc = "0x11c - OTG device IN endpoint transmit FIFO 7 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf7(&self) -> &OTG_DIEPTXF7 {
        &self.otg_dieptxf7
    }
    #[doc = "0x120 - OTG device IN endpoint transmit FIFO 8 size register"]
    #[inline(always)]
    pub const fn otg_dieptxf8(&self) -> &OTG_DIEPTXF8 {
        &self.otg_dieptxf8
    }
    #[doc = "0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host."]
    #[inline(always)]
    pub const fn otg_hcfg(&self) -> &OTG_HCFG {
        &self.otg_hcfg
    }
    #[doc = "0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
    #[inline(always)]
    pub const fn otg_hfir(&self) -> &OTG_HFIR {
        &self.otg_hfir
    }
    #[doc = "0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
    #[inline(always)]
    pub const fn otg_hfnum(&self) -> &OTG_HFNUM {
        &self.otg_hfnum
    }
    #[doc = "0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
    #[inline(always)]
    pub const fn otg_hptxsts(&self) -> &OTG_HPTXSTS {
        &self.otg_hptxsts
    }
    #[doc = "0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
    #[inline(always)]
    pub const fn otg_haint(&self) -> &OTG_HAINT {
        &self.otg_haint
    }
    #[doc = "0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
    #[inline(always)]
    pub const fn otg_haintmsk(&self) -> &OTG_HAINTMSK {
        &self.otg_haintmsk
    }
    #[doc = "0x41c - This register holds the starting address of the frame list information (scatter/gather mode)."]
    #[inline(always)]
    pub const fn otg_hflbaddr(&self) -> &OTG_HFLBADDR {
        &self.otg_hflbaddr
    }
    #[doc = "0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
    #[inline(always)]
    pub const fn otg_hprt(&self) -> &OTG_HPRT {
        &self.otg_hprt
    }
    #[doc = "0x500 - OTG host channel 0 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar0(&self) -> &OTG_HCCHAR0 {
        &self.otg_hcchar0
    }
    #[doc = "0x504 - OTG host channel 0 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt0(&self) -> &OTG_HCSPLT0 {
        &self.otg_hcsplt0
    }
    #[doc = "0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint0(&self) -> &OTG_HCINT0 {
        &self.otg_hcint0
    }
    #[doc = "0x50c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk0(&self) -> &OTG_HCINTMSK0 {
        &self.otg_hcintmsk0
    }
    #[doc = "0x510 - OTG host channel 0 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz0(&self) -> &OTG_HCTSIZ0 {
        &self.otg_hctsiz0
    }
    #[doc = "0x514 - OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma0(&self) -> &OTG_HCDMA0 {
        &self.otg_hcdma0
    }
    #[doc = "0x51c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab0(&self) -> &OTG_HCDMAB0 {
        &self.otg_hcdmab0
    }
    #[doc = "0x520 - OTG host channel 1 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar1(&self) -> &OTG_HCCHAR1 {
        &self.otg_hcchar1
    }
    #[doc = "0x524 - OTG host channel 1 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt1(&self) -> &OTG_HCSPLT1 {
        &self.otg_hcsplt1
    }
    #[doc = "0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint1(&self) -> &OTG_HCINT1 {
        &self.otg_hcint1
    }
    #[doc = "0x52c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk1(&self) -> &OTG_HCINTMSK1 {
        &self.otg_hcintmsk1
    }
    #[doc = "0x530 - OTG host channel 1 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz1(&self) -> &OTG_HCTSIZ1 {
        &self.otg_hctsiz1
    }
    #[doc = "0x534 - OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma1(&self) -> &OTG_HCDMA1 {
        &self.otg_hcdma1
    }
    #[doc = "0x53c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab1(&self) -> &OTG_HCDMAB1 {
        &self.otg_hcdmab1
    }
    #[doc = "0x540 - OTG host channel 2 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar2(&self) -> &OTG_HCCHAR2 {
        &self.otg_hcchar2
    }
    #[doc = "0x544 - OTG host channel 2 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt2(&self) -> &OTG_HCSPLT2 {
        &self.otg_hcsplt2
    }
    #[doc = "0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint2(&self) -> &OTG_HCINT2 {
        &self.otg_hcint2
    }
    #[doc = "0x54c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk2(&self) -> &OTG_HCINTMSK2 {
        &self.otg_hcintmsk2
    }
    #[doc = "0x550 - OTG host channel 2 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz2(&self) -> &OTG_HCTSIZ2 {
        &self.otg_hctsiz2
    }
    #[doc = "0x554 - OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma2(&self) -> &OTG_HCDMA2 {
        &self.otg_hcdma2
    }
    #[doc = "0x55c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab2(&self) -> &OTG_HCDMAB2 {
        &self.otg_hcdmab2
    }
    #[doc = "0x560 - OTG host channel 3 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar3(&self) -> &OTG_HCCHAR3 {
        &self.otg_hcchar3
    }
    #[doc = "0x564 - OTG host channel 3 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt3(&self) -> &OTG_HCSPLT3 {
        &self.otg_hcsplt3
    }
    #[doc = "0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint3(&self) -> &OTG_HCINT3 {
        &self.otg_hcint3
    }
    #[doc = "0x56c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk3(&self) -> &OTG_HCINTMSK3 {
        &self.otg_hcintmsk3
    }
    #[doc = "0x570 - OTG host channel 3 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz3(&self) -> &OTG_HCTSIZ3 {
        &self.otg_hctsiz3
    }
    #[doc = "0x574 - OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma3(&self) -> &OTG_HCDMA3 {
        &self.otg_hcdma3
    }
    #[doc = "0x57c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab3(&self) -> &OTG_HCDMAB3 {
        &self.otg_hcdmab3
    }
    #[doc = "0x580 - OTG host channel 4 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar4(&self) -> &OTG_HCCHAR4 {
        &self.otg_hcchar4
    }
    #[doc = "0x584 - OTG host channel 4 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt4(&self) -> &OTG_HCSPLT4 {
        &self.otg_hcsplt4
    }
    #[doc = "0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint4(&self) -> &OTG_HCINT4 {
        &self.otg_hcint4
    }
    #[doc = "0x58c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk4(&self) -> &OTG_HCINTMSK4 {
        &self.otg_hcintmsk4
    }
    #[doc = "0x590 - OTG host channel 4 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz4(&self) -> &OTG_HCTSIZ4 {
        &self.otg_hctsiz4
    }
    #[doc = "0x594 - OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma4(&self) -> &OTG_HCDMA4 {
        &self.otg_hcdma4
    }
    #[doc = "0x59c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab4(&self) -> &OTG_HCDMAB4 {
        &self.otg_hcdmab4
    }
    #[doc = "0x5a0 - OTG host channel 5 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar5(&self) -> &OTG_HCCHAR5 {
        &self.otg_hcchar5
    }
    #[doc = "0x5a4 - OTG host channel 5 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt5(&self) -> &OTG_HCSPLT5 {
        &self.otg_hcsplt5
    }
    #[doc = "0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint5(&self) -> &OTG_HCINT5 {
        &self.otg_hcint5
    }
    #[doc = "0x5ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk5(&self) -> &OTG_HCINTMSK5 {
        &self.otg_hcintmsk5
    }
    #[doc = "0x5b0 - OTG host channel 5 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz5(&self) -> &OTG_HCTSIZ5 {
        &self.otg_hctsiz5
    }
    #[doc = "0x5b4 - OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma5(&self) -> &OTG_HCDMA5 {
        &self.otg_hcdma5
    }
    #[doc = "0x5bc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab5(&self) -> &OTG_HCDMAB5 {
        &self.otg_hcdmab5
    }
    #[doc = "0x5c0 - OTG host channel 6 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar6(&self) -> &OTG_HCCHAR6 {
        &self.otg_hcchar6
    }
    #[doc = "0x5c4 - OTG host channel 6 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt6(&self) -> &OTG_HCSPLT6 {
        &self.otg_hcsplt6
    }
    #[doc = "0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint6(&self) -> &OTG_HCINT6 {
        &self.otg_hcint6
    }
    #[doc = "0x5cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk6(&self) -> &OTG_HCINTMSK6 {
        &self.otg_hcintmsk6
    }
    #[doc = "0x5d0 - OTG host channel 6 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz6(&self) -> &OTG_HCTSIZ6 {
        &self.otg_hctsiz6
    }
    #[doc = "0x5d4 - OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma6(&self) -> &OTG_HCDMA6 {
        &self.otg_hcdma6
    }
    #[doc = "0x5dc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab6(&self) -> &OTG_HCDMAB6 {
        &self.otg_hcdmab6
    }
    #[doc = "0x5e0 - OTG host channel 7 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar7(&self) -> &OTG_HCCHAR7 {
        &self.otg_hcchar7
    }
    #[doc = "0x5e4 - OTG host channel 7 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt7(&self) -> &OTG_HCSPLT7 {
        &self.otg_hcsplt7
    }
    #[doc = "0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint7(&self) -> &OTG_HCINT7 {
        &self.otg_hcint7
    }
    #[doc = "0x5ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk7(&self) -> &OTG_HCINTMSK7 {
        &self.otg_hcintmsk7
    }
    #[doc = "0x5f0 - OTG host channel 7 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz7(&self) -> &OTG_HCTSIZ7 {
        &self.otg_hctsiz7
    }
    #[doc = "0x5f4 - OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma7(&self) -> &OTG_HCDMA7 {
        &self.otg_hcdma7
    }
    #[doc = "0x5fc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab7(&self) -> &OTG_HCDMAB7 {
        &self.otg_hcdmab7
    }
    #[doc = "0x600 - OTG host channel 8 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar8(&self) -> &OTG_HCCHAR8 {
        &self.otg_hcchar8
    }
    #[doc = "0x604 - OTG host channel 8 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt8(&self) -> &OTG_HCSPLT8 {
        &self.otg_hcsplt8
    }
    #[doc = "0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint8(&self) -> &OTG_HCINT8 {
        &self.otg_hcint8
    }
    #[doc = "0x60c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk8(&self) -> &OTG_HCINTMSK8 {
        &self.otg_hcintmsk8
    }
    #[doc = "0x610 - OTG host channel 8 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz8(&self) -> &OTG_HCTSIZ8 {
        &self.otg_hctsiz8
    }
    #[doc = "0x614 - OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma8(&self) -> &OTG_HCDMA8 {
        &self.otg_hcdma8
    }
    #[doc = "0x61c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab8(&self) -> &OTG_HCDMAB8 {
        &self.otg_hcdmab8
    }
    #[doc = "0x620 - OTG host channel 9 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar9(&self) -> &OTG_HCCHAR9 {
        &self.otg_hcchar9
    }
    #[doc = "0x624 - OTG host channel 9 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt9(&self) -> &OTG_HCSPLT9 {
        &self.otg_hcsplt9
    }
    #[doc = "0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint9(&self) -> &OTG_HCINT9 {
        &self.otg_hcint9
    }
    #[doc = "0x62c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk9(&self) -> &OTG_HCINTMSK9 {
        &self.otg_hcintmsk9
    }
    #[doc = "0x630 - OTG host channel 9 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz9(&self) -> &OTG_HCTSIZ9 {
        &self.otg_hctsiz9
    }
    #[doc = "0x634 - OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma9(&self) -> &OTG_HCDMA9 {
        &self.otg_hcdma9
    }
    #[doc = "0x63c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab9(&self) -> &OTG_HCDMAB9 {
        &self.otg_hcdmab9
    }
    #[doc = "0x640 - OTG host channel 10 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar10(&self) -> &OTG_HCCHAR10 {
        &self.otg_hcchar10
    }
    #[doc = "0x644 - OTG host channel 10 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt10(&self) -> &OTG_HCSPLT10 {
        &self.otg_hcsplt10
    }
    #[doc = "0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint10(&self) -> &OTG_HCINT10 {
        &self.otg_hcint10
    }
    #[doc = "0x64c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk10(&self) -> &OTG_HCINTMSK10 {
        &self.otg_hcintmsk10
    }
    #[doc = "0x650 - OTG host channel 10 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz10(&self) -> &OTG_HCTSIZ10 {
        &self.otg_hctsiz10
    }
    #[doc = "0x654 - OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma10(&self) -> &OTG_HCDMA10 {
        &self.otg_hcdma10
    }
    #[doc = "0x65c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab10(&self) -> &OTG_HCDMAB10 {
        &self.otg_hcdmab10
    }
    #[doc = "0x660 - OTG host channel 11 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar11(&self) -> &OTG_HCCHAR11 {
        &self.otg_hcchar11
    }
    #[doc = "0x664 - OTG host channel 11 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt11(&self) -> &OTG_HCSPLT11 {
        &self.otg_hcsplt11
    }
    #[doc = "0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint11(&self) -> &OTG_HCINT11 {
        &self.otg_hcint11
    }
    #[doc = "0x66c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk11(&self) -> &OTG_HCINTMSK11 {
        &self.otg_hcintmsk11
    }
    #[doc = "0x670 - OTG host channel 11 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz11(&self) -> &OTG_HCTSIZ11 {
        &self.otg_hctsiz11
    }
    #[doc = "0x674 - OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma11(&self) -> &OTG_HCDMA11 {
        &self.otg_hcdma11
    }
    #[doc = "0x67c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab11(&self) -> &OTG_HCDMAB11 {
        &self.otg_hcdmab11
    }
    #[doc = "0x680 - OTG host channel 12 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar12(&self) -> &OTG_HCCHAR12 {
        &self.otg_hcchar12
    }
    #[doc = "0x684 - OTG host channel 12 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt12(&self) -> &OTG_HCSPLT12 {
        &self.otg_hcsplt12
    }
    #[doc = "0x688 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint12(&self) -> &OTG_HCINT12 {
        &self.otg_hcint12
    }
    #[doc = "0x68c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk12(&self) -> &OTG_HCINTMSK12 {
        &self.otg_hcintmsk12
    }
    #[doc = "0x690 - OTG host channel 12 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz12(&self) -> &OTG_HCTSIZ12 {
        &self.otg_hctsiz12
    }
    #[doc = "0x694 - OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma12(&self) -> &OTG_HCDMA12 {
        &self.otg_hcdma12
    }
    #[doc = "0x69c - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab12(&self) -> &OTG_HCDMAB12 {
        &self.otg_hcdmab12
    }
    #[doc = "0x6a0 - OTG host channel 13 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar13(&self) -> &OTG_HCCHAR13 {
        &self.otg_hcchar13
    }
    #[doc = "0x6a4 - OTG host channel 13 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt13(&self) -> &OTG_HCSPLT13 {
        &self.otg_hcsplt13
    }
    #[doc = "0x6a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint13(&self) -> &OTG_HCINT13 {
        &self.otg_hcint13
    }
    #[doc = "0x6ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk13(&self) -> &OTG_HCINTMSK13 {
        &self.otg_hcintmsk13
    }
    #[doc = "0x6b0 - OTG host channel 13 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz13(&self) -> &OTG_HCTSIZ13 {
        &self.otg_hctsiz13
    }
    #[doc = "0x6b4 - OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma13(&self) -> &OTG_HCDMA13 {
        &self.otg_hcdma13
    }
    #[doc = "0x6bc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab13(&self) -> &OTG_HCDMAB13 {
        &self.otg_hcdmab13
    }
    #[doc = "0x6c0 - OTG host channel 14 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar14(&self) -> &OTG_HCCHAR14 {
        &self.otg_hcchar14
    }
    #[doc = "0x6c4 - OTG host channel 14 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt14(&self) -> &OTG_HCSPLT14 {
        &self.otg_hcsplt14
    }
    #[doc = "0x6c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint14(&self) -> &OTG_HCINT14 {
        &self.otg_hcint14
    }
    #[doc = "0x6cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk14(&self) -> &OTG_HCINTMSK14 {
        &self.otg_hcintmsk14
    }
    #[doc = "0x6d0 - OTG host channel 14 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz14(&self) -> &OTG_HCTSIZ14 {
        &self.otg_hctsiz14
    }
    #[doc = "0x6d4 - OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma14(&self) -> &OTG_HCDMA14 {
        &self.otg_hcdma14
    }
    #[doc = "0x6dc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab14(&self) -> &OTG_HCDMAB14 {
        &self.otg_hcdmab14
    }
    #[doc = "0x6e0 - OTG host channel 15 characteristics register"]
    #[inline(always)]
    pub const fn otg_hcchar15(&self) -> &OTG_HCCHAR15 {
        &self.otg_hcchar15
    }
    #[doc = "0x6e4 - OTG host channel 15 split control register"]
    #[inline(always)]
    pub const fn otg_hcsplt15(&self) -> &OTG_HCSPLT15 {
        &self.otg_hcsplt15
    }
    #[doc = "0x6e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_hcint15(&self) -> &OTG_HCINT15 {
        &self.otg_hcint15
    }
    #[doc = "0x6ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn otg_hcintmsk15(&self) -> &OTG_HCINTMSK15 {
        &self.otg_hcintmsk15
    }
    #[doc = "0x6f0 - OTG host channel 15 transfer size register"]
    #[inline(always)]
    pub const fn otg_hctsiz15(&self) -> &OTG_HCTSIZ15 {
        &self.otg_hctsiz15
    }
    #[doc = "0x6f4 - OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
    #[inline(always)]
    pub const fn otg_hcdma15(&self) -> &OTG_HCDMA15 {
        &self.otg_hcdma15
    }
    #[doc = "0x6fc - OTG host channel-n DMA address buffer register"]
    #[inline(always)]
    pub const fn otg_hcdmab15(&self) -> &OTG_HCDMAB15 {
        &self.otg_hcdmab15
    }
    #[doc = "0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
    #[inline(always)]
    pub const fn otg_dcfg(&self) -> &OTG_DCFG {
        &self.otg_dcfg
    }
    #[doc = "0x804 - OTG device control register"]
    #[inline(always)]
    pub const fn otg_dctl(&self) -> &OTG_DCTL {
        &self.otg_dctl
    }
    #[doc = "0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
    #[inline(always)]
    pub const fn otg_dsts(&self) -> &OTG_DSTS {
        &self.otg_dsts
    }
    #[doc = "0x810 - This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn otg_diepmsk(&self) -> &OTG_DIEPMSK {
        &self.otg_diepmsk
    }
    #[doc = "0x814 - This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn otg_doepmsk(&self) -> &OTG_DOEPMSK {
        &self.otg_doepmsk
    }
    #[doc = "0x818 - When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
    #[inline(always)]
    pub const fn otg_daint(&self) -> &OTG_DAINT {
        &self.otg_daint
    }
    #[doc = "0x81c - The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
    #[inline(always)]
    pub const fn otg_daintmsk(&self) -> &OTG_DAINTMSK {
        &self.otg_daintmsk
    }
    #[doc = "0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
    #[inline(always)]
    pub const fn otg_dvbusdis(&self) -> &OTG_DVBUSDIS {
        &self.otg_dvbusdis
    }
    #[doc = "0x82c - This register specifies the VBUS pulsing time during SRP."]
    #[inline(always)]
    pub const fn otg_dvbuspulse(&self) -> &OTG_DVBUSPULSE {
        &self.otg_dvbuspulse
    }
    #[doc = "0x830 - OTG device threshold control register"]
    #[inline(always)]
    pub const fn otg_dthrctl(&self) -> &OTG_DTHRCTL {
        &self.otg_dthrctl
    }
    #[doc = "0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
    #[inline(always)]
    pub const fn otg_diepempmsk(&self) -> &OTG_DIEPEMPMSK {
        &self.otg_diepempmsk
    }
    #[doc = "0x838 - OTG device each endpoint interrupt register"]
    #[inline(always)]
    pub const fn otg_deachint(&self) -> &OTG_DEACHINT {
        &self.otg_deachint
    }
    #[doc = "0x83c - There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
    #[inline(always)]
    pub const fn otg_deachintmsk(&self) -> &OTG_DEACHINTMSK {
        &self.otg_deachintmsk
    }
    #[doc = "0x844 - This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn otg_hs_diepeachmsk1(&self) -> &OTG_HS_DIEPEACHMSK1 {
        &self.otg_hs_diepeachmsk1
    }
    #[doc = "0x884 - This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn otg_hs_doepeachmsk1(&self) -> &OTG_HS_DOEPEACHMSK1 {
        &self.otg_hs_doepeachmsk1
    }
    #[doc = "0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl0(&self) -> &OTG_DIEPCTL0 {
        &self.otg_diepctl0
    }
    #[doc = "0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint0(&self) -> &OTG_DIEPINT0 {
        &self.otg_diepint0
    }
    #[doc = "0x910 - The application must modify this register before enabling endpoint 0."]
    #[inline(always)]
    pub const fn otg_dieptsiz0(&self) -> &OTG_DIEPTSIZ0 {
        &self.otg_dieptsiz0
    }
    #[doc = "0x914 - OTG device IN endpoint 0 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma0(&self) -> &OTG_DIEPDMA0 {
        &self.otg_diepdma0
    }
    #[doc = "0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts0(&self) -> &OTG_DTXFSTS0 {
        &self.otg_dtxfsts0
    }
    #[doc = "0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl1(&self) -> &OTG_DIEPCTL1 {
        &self.otg_diepctl1
    }
    #[doc = "0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint1(&self) -> &OTG_DIEPINT1 {
        &self.otg_diepint1
    }
    #[doc = "0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz1(&self) -> &OTG_DIEPTSIZ1 {
        &self.otg_dieptsiz1
    }
    #[doc = "0x934 - OTG device IN endpoint 1 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma1(&self) -> &OTG_DIEPDMA1 {
        &self.otg_diepdma1
    }
    #[doc = "0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts1(&self) -> &OTG_DTXFSTS1 {
        &self.otg_dtxfsts1
    }
    #[doc = "0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl2(&self) -> &OTG_DIEPCTL2 {
        &self.otg_diepctl2
    }
    #[doc = "0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint2(&self) -> &OTG_DIEPINT2 {
        &self.otg_diepint2
    }
    #[doc = "0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz2(&self) -> &OTG_DIEPTSIZ2 {
        &self.otg_dieptsiz2
    }
    #[doc = "0x954 - OTG device IN endpoint 2 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma2(&self) -> &OTG_DIEPDMA2 {
        &self.otg_diepdma2
    }
    #[doc = "0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts2(&self) -> &OTG_DTXFSTS2 {
        &self.otg_dtxfsts2
    }
    #[doc = "0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl3(&self) -> &OTG_DIEPCTL3 {
        &self.otg_diepctl3
    }
    #[doc = "0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint3(&self) -> &OTG_DIEPINT3 {
        &self.otg_diepint3
    }
    #[doc = "0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz3(&self) -> &OTG_DIEPTSIZ3 {
        &self.otg_dieptsiz3
    }
    #[doc = "0x974 - OTG device IN endpoint 3 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma3(&self) -> &OTG_DIEPDMA3 {
        &self.otg_diepdma3
    }
    #[doc = "0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts3(&self) -> &OTG_DTXFSTS3 {
        &self.otg_dtxfsts3
    }
    #[doc = "0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl4(&self) -> &OTG_DIEPCTL4 {
        &self.otg_diepctl4
    }
    #[doc = "0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint4(&self) -> &OTG_DIEPINT4 {
        &self.otg_diepint4
    }
    #[doc = "0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz4(&self) -> &OTG_DIEPTSIZ4 {
        &self.otg_dieptsiz4
    }
    #[doc = "0x994 - OTG device IN endpoint 4 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma4(&self) -> &OTG_DIEPDMA4 {
        &self.otg_diepdma4
    }
    #[doc = "0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts4(&self) -> &OTG_DTXFSTS4 {
        &self.otg_dtxfsts4
    }
    #[doc = "0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl5(&self) -> &OTG_DIEPCTL5 {
        &self.otg_diepctl5
    }
    #[doc = "0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint5(&self) -> &OTG_DIEPINT5 {
        &self.otg_diepint5
    }
    #[doc = "0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz5(&self) -> &OTG_DIEPTSIZ5 {
        &self.otg_dieptsiz5
    }
    #[doc = "0x9b4 - OTG device IN endpoint 5 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma5(&self) -> &OTG_DIEPDMA5 {
        &self.otg_diepdma5
    }
    #[doc = "0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts5(&self) -> &OTG_DTXFSTS5 {
        &self.otg_dtxfsts5
    }
    #[doc = "0x9c0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl6(&self) -> &OTG_DIEPCTL6 {
        &self.otg_diepctl6
    }
    #[doc = "0x9c8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint6(&self) -> &OTG_DIEPINT6 {
        &self.otg_diepint6
    }
    #[doc = "0x9d0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz6(&self) -> &OTG_DIEPTSIZ6 {
        &self.otg_dieptsiz6
    }
    #[doc = "0x9d4 - OTG device IN endpoint 6 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma6(&self) -> &OTG_DIEPDMA6 {
        &self.otg_diepdma6
    }
    #[doc = "0x9d8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts6(&self) -> &OTG_DTXFSTS6 {
        &self.otg_dtxfsts6
    }
    #[doc = "0x9e0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl7(&self) -> &OTG_DIEPCTL7 {
        &self.otg_diepctl7
    }
    #[doc = "0x9e8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint7(&self) -> &OTG_DIEPINT7 {
        &self.otg_diepint7
    }
    #[doc = "0x9f0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz7(&self) -> &OTG_DIEPTSIZ7 {
        &self.otg_dieptsiz7
    }
    #[doc = "0x9f4 - OTG device IN endpoint 7 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma7(&self) -> &OTG_DIEPDMA7 {
        &self.otg_diepdma7
    }
    #[doc = "0x9f8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts7(&self) -> &OTG_DTXFSTS7 {
        &self.otg_dtxfsts7
    }
    #[doc = "0xa00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_diepctl8(&self) -> &OTG_DIEPCTL8 {
        &self.otg_diepctl8
    }
    #[doc = "0xa08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_diepint8(&self) -> &OTG_DIEPINT8 {
        &self.otg_diepint8
    }
    #[doc = "0xa10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_dieptsiz8(&self) -> &OTG_DIEPTSIZ8 {
        &self.otg_dieptsiz8
    }
    #[doc = "0xa14 - OTG device IN endpoint 8 DMA address register"]
    #[inline(always)]
    pub const fn otg_diepdma8(&self) -> &OTG_DIEPDMA8 {
        &self.otg_diepdma8
    }
    #[doc = "0xa18 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn otg_dtxfsts8(&self) -> &OTG_DTXFSTS8 {
        &self.otg_dtxfsts8
    }
    #[doc = "0xb00 - This section describes the OTG_DOEPCTL0 register."]
    #[inline(always)]
    pub const fn otg_doepctl0(&self) -> &OTG_DOEPCTL0 {
        &self.otg_doepctl0
    }
    #[doc = "0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint0(&self) -> &OTG_DOEPINT0 {
        &self.otg_doepint0
    }
    #[doc = "0xb10 - The application must modify this register before enabling endpoint 0."]
    #[inline(always)]
    pub const fn otg_doeptsiz0(&self) -> &OTG_DOEPTSIZ0 {
        &self.otg_doeptsiz0
    }
    #[doc = "0xb14 - OTG device OUT endpoint 0 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma0(&self) -> &OTG_DOEPDMA0 {
        &self.otg_doepdma0
    }
    #[doc = "0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl1(&self) -> &OTG_DOEPCTL1 {
        &self.otg_doepctl1
    }
    #[doc = "0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint1(&self) -> &OTG_DOEPINT1 {
        &self.otg_doepint1
    }
    #[doc = "0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz1(&self) -> &OTG_DOEPTSIZ1 {
        &self.otg_doeptsiz1
    }
    #[doc = "0xb34 - OTG device OUT endpoint 1 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma1(&self) -> &OTG_DOEPDMA1 {
        &self.otg_doepdma1
    }
    #[doc = "0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl2(&self) -> &OTG_DOEPCTL2 {
        &self.otg_doepctl2
    }
    #[doc = "0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint2(&self) -> &OTG_DOEPINT2 {
        &self.otg_doepint2
    }
    #[doc = "0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz2(&self) -> &OTG_DOEPTSIZ2 {
        &self.otg_doeptsiz2
    }
    #[doc = "0xb54 - OTG device OUT endpoint 2 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma2(&self) -> &OTG_DOEPDMA2 {
        &self.otg_doepdma2
    }
    #[doc = "0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl3(&self) -> &OTG_DOEPCTL3 {
        &self.otg_doepctl3
    }
    #[doc = "0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint3(&self) -> &OTG_DOEPINT3 {
        &self.otg_doepint3
    }
    #[doc = "0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz3(&self) -> &OTG_DOEPTSIZ3 {
        &self.otg_doeptsiz3
    }
    #[doc = "0xb74 - OTG device OUT endpoint 3 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma3(&self) -> &OTG_DOEPDMA3 {
        &self.otg_doepdma3
    }
    #[doc = "0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl4(&self) -> &OTG_DOEPCTL4 {
        &self.otg_doepctl4
    }
    #[doc = "0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint4(&self) -> &OTG_DOEPINT4 {
        &self.otg_doepint4
    }
    #[doc = "0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz4(&self) -> &OTG_DOEPTSIZ4 {
        &self.otg_doeptsiz4
    }
    #[doc = "0xb94 - OTG device OUT endpoint 4 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma4(&self) -> &OTG_DOEPDMA4 {
        &self.otg_doepdma4
    }
    #[doc = "0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl5(&self) -> &OTG_DOEPCTL5 {
        &self.otg_doepctl5
    }
    #[doc = "0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint5(&self) -> &OTG_DOEPINT5 {
        &self.otg_doepint5
    }
    #[doc = "0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz5(&self) -> &OTG_DOEPTSIZ5 {
        &self.otg_doeptsiz5
    }
    #[doc = "0xbb4 - OTG device OUT endpoint 5 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma5(&self) -> &OTG_DOEPDMA5 {
        &self.otg_doepdma5
    }
    #[doc = "0xbc0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl6(&self) -> &OTG_DOEPCTL6 {
        &self.otg_doepctl6
    }
    #[doc = "0xbc8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint6(&self) -> &OTG_DOEPINT6 {
        &self.otg_doepint6
    }
    #[doc = "0xbd0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz6(&self) -> &OTG_DOEPTSIZ6 {
        &self.otg_doeptsiz6
    }
    #[doc = "0xbd4 - OTG device OUT endpoint 6 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma6(&self) -> &OTG_DOEPDMA6 {
        &self.otg_doepdma6
    }
    #[doc = "0xbe0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl7(&self) -> &OTG_DOEPCTL7 {
        &self.otg_doepctl7
    }
    #[doc = "0xbe8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint7(&self) -> &OTG_DOEPINT7 {
        &self.otg_doepint7
    }
    #[doc = "0xbf0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz7(&self) -> &OTG_DOEPTSIZ7 {
        &self.otg_doeptsiz7
    }
    #[doc = "0xbf4 - OTG device OUT endpoint 7 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma7(&self) -> &OTG_DOEPDMA7 {
        &self.otg_doepdma7
    }
    #[doc = "0xc00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn otg_doepctl8(&self) -> &OTG_DOEPCTL8 {
        &self.otg_doepctl8
    }
    #[doc = "0xc08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    #[inline(always)]
    pub const fn otg_doepint8(&self) -> &OTG_DOEPINT8 {
        &self.otg_doepint8
    }
    #[doc = "0xc10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn otg_doeptsiz8(&self) -> &OTG_DOEPTSIZ8 {
        &self.otg_doeptsiz8
    }
    #[doc = "0xc14 - OTG device OUT endpoint 8 DMA address register"]
    #[inline(always)]
    pub const fn otg_doepdma8(&self) -> &OTG_DOEPDMA8 {
        &self.otg_doepdma8
    }
    #[doc = "0xe00 - This register is available in host and device modes."]
    #[inline(always)]
    pub const fn otg_pcgcctl(&self) -> &OTG_PCGCCTL {
        &self.otg_pcgcctl
    }
}
#[doc = "OTG_GOTGCTL (rw) register accessor: The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gotgctl`]
module"]
pub type OTG_GOTGCTL = crate::Reg<otg_gotgctl::OTG_GOTGCTLrs>;
#[doc = "The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
pub mod otg_gotgctl;
#[doc = "OTG_GOTGINT (rw) register accessor: The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gotgint`]
module"]
pub type OTG_GOTGINT = crate::Reg<otg_gotgint::OTG_GOTGINTrs>;
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
pub mod otg_gotgint;
#[doc = "OTG_GAHBCFG (rw) register accessor: This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gahbcfg`]
module"]
pub type OTG_GAHBCFG = crate::Reg<otg_gahbcfg::OTG_GAHBCFGrs>;
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
pub mod otg_gahbcfg;
#[doc = "OTG_GUSBCFG (rw) register accessor: This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gusbcfg`]
module"]
pub type OTG_GUSBCFG = crate::Reg<otg_gusbcfg::OTG_GUSBCFGrs>;
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
pub mod otg_gusbcfg;
#[doc = "OTG_GRSTCTL (rw) register accessor: The application uses this register to reset various hardware features inside the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_grstctl`]
module"]
pub type OTG_GRSTCTL = crate::Reg<otg_grstctl::OTG_GRSTCTLrs>;
#[doc = "The application uses this register to reset various hardware features inside the core."]
pub mod otg_grstctl;
#[doc = "OTG_GINTSTS (rw) register accessor: This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gintsts`]
module"]
pub type OTG_GINTSTS = crate::Reg<otg_gintsts::OTG_GINTSTSrs>;
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
pub mod otg_gintsts;
#[doc = "OTG_GINTMSK (rw) register accessor: This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gintmsk`]
module"]
pub type OTG_GINTMSK = crate::Reg<otg_gintmsk::OTG_GINTMSKrs>;
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
pub mod otg_gintmsk;
#[doc = "OTG_GRXSTSR (r) register accessor: This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grxstsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_grxstsr`]
module"]
pub type OTG_GRXSTSR = crate::Reg<otg_grxstsr::OTG_GRXSTSRrs>;
#[doc = "This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
pub mod otg_grxstsr;
#[doc = "OTG_GRXSTSP (r) register accessor: This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grxstsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_grxstsp`]
module"]
pub type OTG_GRXSTSP = crate::Reg<otg_grxstsp::OTG_GRXSTSPrs>;
#[doc = "This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
pub mod otg_grxstsp;
#[doc = "OTG_GRXFSIZ (rw) register accessor: The application can program the RAM size that must be allocated to the Rx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_grxfsiz`]
module"]
pub type OTG_GRXFSIZ = crate::Reg<otg_grxfsiz::OTG_GRXFSIZrs>;
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO."]
pub mod otg_grxfsiz;
#[doc = "OTG_HNPTXFSIZ (rw) register accessor: Host mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hnptxfsiz`]
module"]
pub type OTG_HNPTXFSIZ = crate::Reg<otg_hnptxfsiz::OTG_HNPTXFSIZrs>;
#[doc = "Host mode"]
pub mod otg_hnptxfsiz;
#[doc = "OTG_HNPTXSTS (r) register accessor: In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hnptxsts`]
module"]
pub type OTG_HNPTXSTS = crate::Reg<otg_hnptxsts::OTG_HNPTXSTSrs>;
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
pub mod otg_hnptxsts;
#[doc = "OTG_GCCFG (rw) register accessor: OTG general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_gccfg`]
module"]
pub type OTG_GCCFG = crate::Reg<otg_gccfg::OTG_GCCFGrs>;
#[doc = "OTG general core configuration register"]
pub mod otg_gccfg;
#[doc = "OTG_CID (rw) register accessor: This is a register containing the Product ID as reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_cid`]
module"]
pub type OTG_CID = crate::Reg<otg_cid::OTG_CIDrs>;
#[doc = "This is a register containing the Product ID as reset value."]
pub mod otg_cid;
#[doc = "OTG_GLPMCFG (rw) register accessor: OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_glpmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_glpmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_glpmcfg`]
module"]
pub type OTG_GLPMCFG = crate::Reg<otg_glpmcfg::OTG_GLPMCFGrs>;
#[doc = "OTG core LPM configuration register"]
pub mod otg_glpmcfg;
#[doc = "OTG_HPTXFSIZ (rw) register accessor: OTG host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hptxfsiz`]
module"]
pub type OTG_HPTXFSIZ = crate::Reg<otg_hptxfsiz::OTG_HPTXFSIZrs>;
#[doc = "OTG host periodic transmit FIFO size register"]
pub mod otg_hptxfsiz;
#[doc = "OTG_DIEPTXF1 (rw) register accessor: OTG device IN endpoint transmit FIFO 1 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf1`]
module"]
pub type OTG_DIEPTXF1 = crate::Reg<otg_dieptxf1::OTG_DIEPTXF1rs>;
#[doc = "OTG device IN endpoint transmit FIFO 1 size register"]
pub mod otg_dieptxf1;
#[doc = "OTG_DIEPTXF2 (rw) register accessor: OTG device IN endpoint transmit FIFO 2 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf2`]
module"]
pub type OTG_DIEPTXF2 = crate::Reg<otg_dieptxf2::OTG_DIEPTXF2rs>;
#[doc = "OTG device IN endpoint transmit FIFO 2 size register"]
pub mod otg_dieptxf2;
#[doc = "OTG_DIEPTXF3 (rw) register accessor: OTG device IN endpoint transmit FIFO 3 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf3`]
module"]
pub type OTG_DIEPTXF3 = crate::Reg<otg_dieptxf3::OTG_DIEPTXF3rs>;
#[doc = "OTG device IN endpoint transmit FIFO 3 size register"]
pub mod otg_dieptxf3;
#[doc = "OTG_DIEPTXF4 (rw) register accessor: OTG device IN endpoint transmit FIFO 4 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf4`]
module"]
pub type OTG_DIEPTXF4 = crate::Reg<otg_dieptxf4::OTG_DIEPTXF4rs>;
#[doc = "OTG device IN endpoint transmit FIFO 4 size register"]
pub mod otg_dieptxf4;
#[doc = "OTG_DIEPTXF5 (rw) register accessor: OTG device IN endpoint transmit FIFO 5 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf5`]
module"]
pub type OTG_DIEPTXF5 = crate::Reg<otg_dieptxf5::OTG_DIEPTXF5rs>;
#[doc = "OTG device IN endpoint transmit FIFO 5 size register"]
pub mod otg_dieptxf5;
#[doc = "OTG_DIEPTXF6 (rw) register accessor: OTG device IN endpoint transmit FIFO 6 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf6`]
module"]
pub type OTG_DIEPTXF6 = crate::Reg<otg_dieptxf6::OTG_DIEPTXF6rs>;
#[doc = "OTG device IN endpoint transmit FIFO 6 size register"]
pub mod otg_dieptxf6;
#[doc = "OTG_DIEPTXF7 (rw) register accessor: OTG device IN endpoint transmit FIFO 7 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf7`]
module"]
pub type OTG_DIEPTXF7 = crate::Reg<otg_dieptxf7::OTG_DIEPTXF7rs>;
#[doc = "OTG device IN endpoint transmit FIFO 7 size register"]
pub mod otg_dieptxf7;
#[doc = "OTG_DIEPTXF8 (rw) register accessor: OTG device IN endpoint transmit FIFO 8 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptxf8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptxf8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptxf8`]
module"]
pub type OTG_DIEPTXF8 = crate::Reg<otg_dieptxf8::OTG_DIEPTXF8rs>;
#[doc = "OTG device IN endpoint transmit FIFO 8 size register"]
pub mod otg_dieptxf8;
#[doc = "OTG_HCFG (rw) register accessor: This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcfg`]
module"]
pub type OTG_HCFG = crate::Reg<otg_hcfg::OTG_HCFGrs>;
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host."]
pub mod otg_hcfg;
#[doc = "OTG_HFIR (rw) register accessor: This register stores the frame interval information for the current speed to which the OTG controller has enumerated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hfir`]
module"]
pub type OTG_HFIR = crate::Reg<otg_hfir::OTG_HFIRrs>;
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
pub mod otg_hfir;
#[doc = "OTG_HFNUM (r) register accessor: This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hfnum`]
module"]
pub type OTG_HFNUM = crate::Reg<otg_hfnum::OTG_HFNUMrs>;
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
pub mod otg_hfnum;
#[doc = "OTG_HPTXSTS (r) register accessor: This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hptxsts`]
module"]
pub type OTG_HPTXSTS = crate::Reg<otg_hptxsts::OTG_HPTXSTSrs>;
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
pub mod otg_hptxsts;
#[doc = "OTG_HAINT (r) register accessor: When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_haint`]
module"]
pub type OTG_HAINT = crate::Reg<otg_haint::OTG_HAINTrs>;
#[doc = "When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
pub mod otg_haint;
#[doc = "OTG_HAINTMSK (rw) register accessor: The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_haintmsk`]
module"]
pub type OTG_HAINTMSK = crate::Reg<otg_haintmsk::OTG_HAINTMSKrs>;
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
pub mod otg_haintmsk;
#[doc = "OTG_HFLBADDR (rw) register accessor: This register holds the starting address of the frame list information (scatter/gather mode).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hflbaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hflbaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hflbaddr`]
module"]
pub type OTG_HFLBADDR = crate::Reg<otg_hflbaddr::OTG_HFLBADDRrs>;
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode)."]
pub mod otg_hflbaddr;
#[doc = "OTG_HPRT (rw) register accessor: This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hprt`]
module"]
pub type OTG_HPRT = crate::Reg<otg_hprt::OTG_HPRTrs>;
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
pub mod otg_hprt;
#[doc = "OTG_HCCHAR0 (rw) register accessor: OTG host channel 0 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar0`]
module"]
pub type OTG_HCCHAR0 = crate::Reg<otg_hcchar0::OTG_HCCHAR0rs>;
#[doc = "OTG host channel 0 characteristics register"]
pub mod otg_hcchar0;
#[doc = "OTG_HCSPLT0 (rw) register accessor: OTG host channel 0 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt0`]
module"]
pub type OTG_HCSPLT0 = crate::Reg<otg_hcsplt0::OTG_HCSPLT0rs>;
#[doc = "OTG host channel 0 split control register"]
pub mod otg_hcsplt0;
#[doc = "OTG_HCINT0 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint0`]
module"]
pub type OTG_HCINT0 = crate::Reg<otg_hcint0::OTG_HCINT0rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint0;
#[doc = "OTG_HCINTMSK0 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk0`]
module"]
pub type OTG_HCINTMSK0 = crate::Reg<otg_hcintmsk0::OTG_HCINTMSK0rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk0;
#[doc = "OTG_HCTSIZ0 (rw) register accessor: OTG host channel 0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz0`]
module"]
pub type OTG_HCTSIZ0 = crate::Reg<otg_hctsiz0::OTG_HCTSIZ0rs>;
#[doc = "OTG host channel 0 transfer size register"]
pub mod otg_hctsiz0;
#[doc = "OTG_HCDMA0 (rw) register accessor: OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma0`]
module"]
pub type OTG_HCDMA0 = crate::Reg<otg_hcdma0::OTG_HCDMA0rs>;
#[doc = "OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma0;
#[doc = "OTG_HCDMAB0 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab0`]
module"]
pub type OTG_HCDMAB0 = crate::Reg<otg_hcdmab0::OTG_HCDMAB0rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab0;
#[doc = "OTG_HCCHAR1 (rw) register accessor: OTG host channel 1 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar1`]
module"]
pub type OTG_HCCHAR1 = crate::Reg<otg_hcchar1::OTG_HCCHAR1rs>;
#[doc = "OTG host channel 1 characteristics register"]
pub mod otg_hcchar1;
#[doc = "OTG_HCSPLT1 (rw) register accessor: OTG host channel 1 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt1`]
module"]
pub type OTG_HCSPLT1 = crate::Reg<otg_hcsplt1::OTG_HCSPLT1rs>;
#[doc = "OTG host channel 1 split control register"]
pub mod otg_hcsplt1;
#[doc = "OTG_HCINT1 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint1`]
module"]
pub type OTG_HCINT1 = crate::Reg<otg_hcint1::OTG_HCINT1rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint1;
#[doc = "OTG_HCINTMSK1 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk1`]
module"]
pub type OTG_HCINTMSK1 = crate::Reg<otg_hcintmsk1::OTG_HCINTMSK1rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk1;
#[doc = "OTG_HCTSIZ1 (rw) register accessor: OTG host channel 1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz1`]
module"]
pub type OTG_HCTSIZ1 = crate::Reg<otg_hctsiz1::OTG_HCTSIZ1rs>;
#[doc = "OTG host channel 1 transfer size register"]
pub mod otg_hctsiz1;
#[doc = "OTG_HCDMA1 (rw) register accessor: OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma1`]
module"]
pub type OTG_HCDMA1 = crate::Reg<otg_hcdma1::OTG_HCDMA1rs>;
#[doc = "OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma1;
#[doc = "OTG_HCDMAB1 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab1`]
module"]
pub type OTG_HCDMAB1 = crate::Reg<otg_hcdmab1::OTG_HCDMAB1rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab1;
#[doc = "OTG_HCCHAR2 (rw) register accessor: OTG host channel 2 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar2`]
module"]
pub type OTG_HCCHAR2 = crate::Reg<otg_hcchar2::OTG_HCCHAR2rs>;
#[doc = "OTG host channel 2 characteristics register"]
pub mod otg_hcchar2;
#[doc = "OTG_HCSPLT2 (rw) register accessor: OTG host channel 2 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt2`]
module"]
pub type OTG_HCSPLT2 = crate::Reg<otg_hcsplt2::OTG_HCSPLT2rs>;
#[doc = "OTG host channel 2 split control register"]
pub mod otg_hcsplt2;
#[doc = "OTG_HCINT2 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint2`]
module"]
pub type OTG_HCINT2 = crate::Reg<otg_hcint2::OTG_HCINT2rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint2;
#[doc = "OTG_HCINTMSK2 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk2`]
module"]
pub type OTG_HCINTMSK2 = crate::Reg<otg_hcintmsk2::OTG_HCINTMSK2rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk2;
#[doc = "OTG_HCTSIZ2 (rw) register accessor: OTG host channel 2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz2`]
module"]
pub type OTG_HCTSIZ2 = crate::Reg<otg_hctsiz2::OTG_HCTSIZ2rs>;
#[doc = "OTG host channel 2 transfer size register"]
pub mod otg_hctsiz2;
#[doc = "OTG_HCDMA2 (rw) register accessor: OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma2`]
module"]
pub type OTG_HCDMA2 = crate::Reg<otg_hcdma2::OTG_HCDMA2rs>;
#[doc = "OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma2;
#[doc = "OTG_HCDMAB2 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab2`]
module"]
pub type OTG_HCDMAB2 = crate::Reg<otg_hcdmab2::OTG_HCDMAB2rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab2;
#[doc = "OTG_HCCHAR3 (rw) register accessor: OTG host channel 3 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar3`]
module"]
pub type OTG_HCCHAR3 = crate::Reg<otg_hcchar3::OTG_HCCHAR3rs>;
#[doc = "OTG host channel 3 characteristics register"]
pub mod otg_hcchar3;
#[doc = "OTG_HCSPLT3 (rw) register accessor: OTG host channel 3 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt3`]
module"]
pub type OTG_HCSPLT3 = crate::Reg<otg_hcsplt3::OTG_HCSPLT3rs>;
#[doc = "OTG host channel 3 split control register"]
pub mod otg_hcsplt3;
#[doc = "OTG_HCINT3 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint3`]
module"]
pub type OTG_HCINT3 = crate::Reg<otg_hcint3::OTG_HCINT3rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint3;
#[doc = "OTG_HCINTMSK3 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk3`]
module"]
pub type OTG_HCINTMSK3 = crate::Reg<otg_hcintmsk3::OTG_HCINTMSK3rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk3;
#[doc = "OTG_HCTSIZ3 (rw) register accessor: OTG host channel 3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz3`]
module"]
pub type OTG_HCTSIZ3 = crate::Reg<otg_hctsiz3::OTG_HCTSIZ3rs>;
#[doc = "OTG host channel 3 transfer size register"]
pub mod otg_hctsiz3;
#[doc = "OTG_HCDMA3 (rw) register accessor: OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma3`]
module"]
pub type OTG_HCDMA3 = crate::Reg<otg_hcdma3::OTG_HCDMA3rs>;
#[doc = "OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma3;
#[doc = "OTG_HCDMAB3 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab3`]
module"]
pub type OTG_HCDMAB3 = crate::Reg<otg_hcdmab3::OTG_HCDMAB3rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab3;
#[doc = "OTG_HCCHAR4 (rw) register accessor: OTG host channel 4 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar4`]
module"]
pub type OTG_HCCHAR4 = crate::Reg<otg_hcchar4::OTG_HCCHAR4rs>;
#[doc = "OTG host channel 4 characteristics register"]
pub mod otg_hcchar4;
#[doc = "OTG_HCSPLT4 (rw) register accessor: OTG host channel 4 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt4`]
module"]
pub type OTG_HCSPLT4 = crate::Reg<otg_hcsplt4::OTG_HCSPLT4rs>;
#[doc = "OTG host channel 4 split control register"]
pub mod otg_hcsplt4;
#[doc = "OTG_HCINT4 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint4`]
module"]
pub type OTG_HCINT4 = crate::Reg<otg_hcint4::OTG_HCINT4rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint4;
#[doc = "OTG_HCINTMSK4 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk4`]
module"]
pub type OTG_HCINTMSK4 = crate::Reg<otg_hcintmsk4::OTG_HCINTMSK4rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk4;
#[doc = "OTG_HCTSIZ4 (rw) register accessor: OTG host channel 4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz4`]
module"]
pub type OTG_HCTSIZ4 = crate::Reg<otg_hctsiz4::OTG_HCTSIZ4rs>;
#[doc = "OTG host channel 4 transfer size register"]
pub mod otg_hctsiz4;
#[doc = "OTG_HCDMA4 (rw) register accessor: OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma4`]
module"]
pub type OTG_HCDMA4 = crate::Reg<otg_hcdma4::OTG_HCDMA4rs>;
#[doc = "OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma4;
#[doc = "OTG_HCDMAB4 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab4`]
module"]
pub type OTG_HCDMAB4 = crate::Reg<otg_hcdmab4::OTG_HCDMAB4rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab4;
#[doc = "OTG_HCCHAR5 (rw) register accessor: OTG host channel 5 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar5`]
module"]
pub type OTG_HCCHAR5 = crate::Reg<otg_hcchar5::OTG_HCCHAR5rs>;
#[doc = "OTG host channel 5 characteristics register"]
pub mod otg_hcchar5;
#[doc = "OTG_HCSPLT5 (rw) register accessor: OTG host channel 5 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt5`]
module"]
pub type OTG_HCSPLT5 = crate::Reg<otg_hcsplt5::OTG_HCSPLT5rs>;
#[doc = "OTG host channel 5 split control register"]
pub mod otg_hcsplt5;
#[doc = "OTG_HCINT5 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint5`]
module"]
pub type OTG_HCINT5 = crate::Reg<otg_hcint5::OTG_HCINT5rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint5;
#[doc = "OTG_HCINTMSK5 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk5`]
module"]
pub type OTG_HCINTMSK5 = crate::Reg<otg_hcintmsk5::OTG_HCINTMSK5rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk5;
#[doc = "OTG_HCTSIZ5 (rw) register accessor: OTG host channel 5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz5`]
module"]
pub type OTG_HCTSIZ5 = crate::Reg<otg_hctsiz5::OTG_HCTSIZ5rs>;
#[doc = "OTG host channel 5 transfer size register"]
pub mod otg_hctsiz5;
#[doc = "OTG_HCDMA5 (rw) register accessor: OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma5`]
module"]
pub type OTG_HCDMA5 = crate::Reg<otg_hcdma5::OTG_HCDMA5rs>;
#[doc = "OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma5;
#[doc = "OTG_HCDMAB5 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab5`]
module"]
pub type OTG_HCDMAB5 = crate::Reg<otg_hcdmab5::OTG_HCDMAB5rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab5;
#[doc = "OTG_HCCHAR6 (rw) register accessor: OTG host channel 6 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar6`]
module"]
pub type OTG_HCCHAR6 = crate::Reg<otg_hcchar6::OTG_HCCHAR6rs>;
#[doc = "OTG host channel 6 characteristics register"]
pub mod otg_hcchar6;
#[doc = "OTG_HCSPLT6 (rw) register accessor: OTG host channel 6 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt6`]
module"]
pub type OTG_HCSPLT6 = crate::Reg<otg_hcsplt6::OTG_HCSPLT6rs>;
#[doc = "OTG host channel 6 split control register"]
pub mod otg_hcsplt6;
#[doc = "OTG_HCINT6 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint6`]
module"]
pub type OTG_HCINT6 = crate::Reg<otg_hcint6::OTG_HCINT6rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint6;
#[doc = "OTG_HCINTMSK6 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk6`]
module"]
pub type OTG_HCINTMSK6 = crate::Reg<otg_hcintmsk6::OTG_HCINTMSK6rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk6;
#[doc = "OTG_HCTSIZ6 (rw) register accessor: OTG host channel 6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz6`]
module"]
pub type OTG_HCTSIZ6 = crate::Reg<otg_hctsiz6::OTG_HCTSIZ6rs>;
#[doc = "OTG host channel 6 transfer size register"]
pub mod otg_hctsiz6;
#[doc = "OTG_HCDMA6 (rw) register accessor: OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma6`]
module"]
pub type OTG_HCDMA6 = crate::Reg<otg_hcdma6::OTG_HCDMA6rs>;
#[doc = "OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma6;
#[doc = "OTG_HCDMAB6 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab6`]
module"]
pub type OTG_HCDMAB6 = crate::Reg<otg_hcdmab6::OTG_HCDMAB6rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab6;
#[doc = "OTG_HCCHAR7 (rw) register accessor: OTG host channel 7 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar7`]
module"]
pub type OTG_HCCHAR7 = crate::Reg<otg_hcchar7::OTG_HCCHAR7rs>;
#[doc = "OTG host channel 7 characteristics register"]
pub mod otg_hcchar7;
#[doc = "OTG_HCSPLT7 (rw) register accessor: OTG host channel 7 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt7`]
module"]
pub type OTG_HCSPLT7 = crate::Reg<otg_hcsplt7::OTG_HCSPLT7rs>;
#[doc = "OTG host channel 7 split control register"]
pub mod otg_hcsplt7;
#[doc = "OTG_HCINT7 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint7`]
module"]
pub type OTG_HCINT7 = crate::Reg<otg_hcint7::OTG_HCINT7rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint7;
#[doc = "OTG_HCINTMSK7 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk7`]
module"]
pub type OTG_HCINTMSK7 = crate::Reg<otg_hcintmsk7::OTG_HCINTMSK7rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk7;
#[doc = "OTG_HCTSIZ7 (rw) register accessor: OTG host channel 7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz7`]
module"]
pub type OTG_HCTSIZ7 = crate::Reg<otg_hctsiz7::OTG_HCTSIZ7rs>;
#[doc = "OTG host channel 7 transfer size register"]
pub mod otg_hctsiz7;
#[doc = "OTG_HCDMA7 (rw) register accessor: OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma7`]
module"]
pub type OTG_HCDMA7 = crate::Reg<otg_hcdma7::OTG_HCDMA7rs>;
#[doc = "OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma7;
#[doc = "OTG_HCDMAB7 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab7`]
module"]
pub type OTG_HCDMAB7 = crate::Reg<otg_hcdmab7::OTG_HCDMAB7rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab7;
#[doc = "OTG_HCCHAR8 (rw) register accessor: OTG host channel 8 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar8`]
module"]
pub type OTG_HCCHAR8 = crate::Reg<otg_hcchar8::OTG_HCCHAR8rs>;
#[doc = "OTG host channel 8 characteristics register"]
pub mod otg_hcchar8;
#[doc = "OTG_HCSPLT8 (rw) register accessor: OTG host channel 8 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt8`]
module"]
pub type OTG_HCSPLT8 = crate::Reg<otg_hcsplt8::OTG_HCSPLT8rs>;
#[doc = "OTG host channel 8 split control register"]
pub mod otg_hcsplt8;
#[doc = "OTG_HCINT8 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint8`]
module"]
pub type OTG_HCINT8 = crate::Reg<otg_hcint8::OTG_HCINT8rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint8;
#[doc = "OTG_HCINTMSK8 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk8`]
module"]
pub type OTG_HCINTMSK8 = crate::Reg<otg_hcintmsk8::OTG_HCINTMSK8rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk8;
#[doc = "OTG_HCTSIZ8 (rw) register accessor: OTG host channel 8 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz8`]
module"]
pub type OTG_HCTSIZ8 = crate::Reg<otg_hctsiz8::OTG_HCTSIZ8rs>;
#[doc = "OTG host channel 8 transfer size register"]
pub mod otg_hctsiz8;
#[doc = "OTG_HCDMA8 (rw) register accessor: OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma8`]
module"]
pub type OTG_HCDMA8 = crate::Reg<otg_hcdma8::OTG_HCDMA8rs>;
#[doc = "OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma8;
#[doc = "OTG_HCDMAB8 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab8`]
module"]
pub type OTG_HCDMAB8 = crate::Reg<otg_hcdmab8::OTG_HCDMAB8rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab8;
#[doc = "OTG_HCCHAR9 (rw) register accessor: OTG host channel 9 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar9`]
module"]
pub type OTG_HCCHAR9 = crate::Reg<otg_hcchar9::OTG_HCCHAR9rs>;
#[doc = "OTG host channel 9 characteristics register"]
pub mod otg_hcchar9;
#[doc = "OTG_HCSPLT9 (rw) register accessor: OTG host channel 9 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt9`]
module"]
pub type OTG_HCSPLT9 = crate::Reg<otg_hcsplt9::OTG_HCSPLT9rs>;
#[doc = "OTG host channel 9 split control register"]
pub mod otg_hcsplt9;
#[doc = "OTG_HCINT9 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint9`]
module"]
pub type OTG_HCINT9 = crate::Reg<otg_hcint9::OTG_HCINT9rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint9;
#[doc = "OTG_HCINTMSK9 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk9`]
module"]
pub type OTG_HCINTMSK9 = crate::Reg<otg_hcintmsk9::OTG_HCINTMSK9rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk9;
#[doc = "OTG_HCTSIZ9 (rw) register accessor: OTG host channel 9 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz9`]
module"]
pub type OTG_HCTSIZ9 = crate::Reg<otg_hctsiz9::OTG_HCTSIZ9rs>;
#[doc = "OTG host channel 9 transfer size register"]
pub mod otg_hctsiz9;
#[doc = "OTG_HCDMA9 (rw) register accessor: OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma9`]
module"]
pub type OTG_HCDMA9 = crate::Reg<otg_hcdma9::OTG_HCDMA9rs>;
#[doc = "OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma9;
#[doc = "OTG_HCDMAB9 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab9`]
module"]
pub type OTG_HCDMAB9 = crate::Reg<otg_hcdmab9::OTG_HCDMAB9rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab9;
#[doc = "OTG_HCCHAR10 (rw) register accessor: OTG host channel 10 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar10`]
module"]
pub type OTG_HCCHAR10 = crate::Reg<otg_hcchar10::OTG_HCCHAR10rs>;
#[doc = "OTG host channel 10 characteristics register"]
pub mod otg_hcchar10;
#[doc = "OTG_HCSPLT10 (rw) register accessor: OTG host channel 10 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt10`]
module"]
pub type OTG_HCSPLT10 = crate::Reg<otg_hcsplt10::OTG_HCSPLT10rs>;
#[doc = "OTG host channel 10 split control register"]
pub mod otg_hcsplt10;
#[doc = "OTG_HCINT10 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint10`]
module"]
pub type OTG_HCINT10 = crate::Reg<otg_hcint10::OTG_HCINT10rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint10;
#[doc = "OTG_HCINTMSK10 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk10`]
module"]
pub type OTG_HCINTMSK10 = crate::Reg<otg_hcintmsk10::OTG_HCINTMSK10rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk10;
#[doc = "OTG_HCTSIZ10 (rw) register accessor: OTG host channel 10 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz10`]
module"]
pub type OTG_HCTSIZ10 = crate::Reg<otg_hctsiz10::OTG_HCTSIZ10rs>;
#[doc = "OTG host channel 10 transfer size register"]
pub mod otg_hctsiz10;
#[doc = "OTG_HCDMA10 (rw) register accessor: OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma10`]
module"]
pub type OTG_HCDMA10 = crate::Reg<otg_hcdma10::OTG_HCDMA10rs>;
#[doc = "OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma10;
#[doc = "OTG_HCDMAB10 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab10`]
module"]
pub type OTG_HCDMAB10 = crate::Reg<otg_hcdmab10::OTG_HCDMAB10rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab10;
#[doc = "OTG_HCCHAR11 (rw) register accessor: OTG host channel 11 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar11`]
module"]
pub type OTG_HCCHAR11 = crate::Reg<otg_hcchar11::OTG_HCCHAR11rs>;
#[doc = "OTG host channel 11 characteristics register"]
pub mod otg_hcchar11;
#[doc = "OTG_HCSPLT11 (rw) register accessor: OTG host channel 11 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt11`]
module"]
pub type OTG_HCSPLT11 = crate::Reg<otg_hcsplt11::OTG_HCSPLT11rs>;
#[doc = "OTG host channel 11 split control register"]
pub mod otg_hcsplt11;
#[doc = "OTG_HCINT11 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint11`]
module"]
pub type OTG_HCINT11 = crate::Reg<otg_hcint11::OTG_HCINT11rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint11;
#[doc = "OTG_HCINTMSK11 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk11`]
module"]
pub type OTG_HCINTMSK11 = crate::Reg<otg_hcintmsk11::OTG_HCINTMSK11rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk11;
#[doc = "OTG_HCTSIZ11 (rw) register accessor: OTG host channel 11 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz11`]
module"]
pub type OTG_HCTSIZ11 = crate::Reg<otg_hctsiz11::OTG_HCTSIZ11rs>;
#[doc = "OTG host channel 11 transfer size register"]
pub mod otg_hctsiz11;
#[doc = "OTG_HCDMA11 (rw) register accessor: OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma11`]
module"]
pub type OTG_HCDMA11 = crate::Reg<otg_hcdma11::OTG_HCDMA11rs>;
#[doc = "OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma11;
#[doc = "OTG_HCDMAB11 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab11`]
module"]
pub type OTG_HCDMAB11 = crate::Reg<otg_hcdmab11::OTG_HCDMAB11rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab11;
#[doc = "OTG_HCCHAR12 (rw) register accessor: OTG host channel 12 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar12`]
module"]
pub type OTG_HCCHAR12 = crate::Reg<otg_hcchar12::OTG_HCCHAR12rs>;
#[doc = "OTG host channel 12 characteristics register"]
pub mod otg_hcchar12;
#[doc = "OTG_HCSPLT12 (rw) register accessor: OTG host channel 12 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt12`]
module"]
pub type OTG_HCSPLT12 = crate::Reg<otg_hcsplt12::OTG_HCSPLT12rs>;
#[doc = "OTG host channel 12 split control register"]
pub mod otg_hcsplt12;
#[doc = "OTG_HCINT12 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint12`]
module"]
pub type OTG_HCINT12 = crate::Reg<otg_hcint12::OTG_HCINT12rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint12;
#[doc = "OTG_HCINTMSK12 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk12`]
module"]
pub type OTG_HCINTMSK12 = crate::Reg<otg_hcintmsk12::OTG_HCINTMSK12rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk12;
#[doc = "OTG_HCTSIZ12 (rw) register accessor: OTG host channel 12 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz12`]
module"]
pub type OTG_HCTSIZ12 = crate::Reg<otg_hctsiz12::OTG_HCTSIZ12rs>;
#[doc = "OTG host channel 12 transfer size register"]
pub mod otg_hctsiz12;
#[doc = "OTG_HCDMA12 (rw) register accessor: OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma12`]
module"]
pub type OTG_HCDMA12 = crate::Reg<otg_hcdma12::OTG_HCDMA12rs>;
#[doc = "OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma12;
#[doc = "OTG_HCDMAB12 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab12`]
module"]
pub type OTG_HCDMAB12 = crate::Reg<otg_hcdmab12::OTG_HCDMAB12rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab12;
#[doc = "OTG_HCCHAR13 (rw) register accessor: OTG host channel 13 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar13`]
module"]
pub type OTG_HCCHAR13 = crate::Reg<otg_hcchar13::OTG_HCCHAR13rs>;
#[doc = "OTG host channel 13 characteristics register"]
pub mod otg_hcchar13;
#[doc = "OTG_HCSPLT13 (rw) register accessor: OTG host channel 13 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt13`]
module"]
pub type OTG_HCSPLT13 = crate::Reg<otg_hcsplt13::OTG_HCSPLT13rs>;
#[doc = "OTG host channel 13 split control register"]
pub mod otg_hcsplt13;
#[doc = "OTG_HCINT13 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint13`]
module"]
pub type OTG_HCINT13 = crate::Reg<otg_hcint13::OTG_HCINT13rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint13;
#[doc = "OTG_HCINTMSK13 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk13`]
module"]
pub type OTG_HCINTMSK13 = crate::Reg<otg_hcintmsk13::OTG_HCINTMSK13rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk13;
#[doc = "OTG_HCTSIZ13 (rw) register accessor: OTG host channel 13 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz13`]
module"]
pub type OTG_HCTSIZ13 = crate::Reg<otg_hctsiz13::OTG_HCTSIZ13rs>;
#[doc = "OTG host channel 13 transfer size register"]
pub mod otg_hctsiz13;
#[doc = "OTG_HCDMA13 (rw) register accessor: OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma13`]
module"]
pub type OTG_HCDMA13 = crate::Reg<otg_hcdma13::OTG_HCDMA13rs>;
#[doc = "OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma13;
#[doc = "OTG_HCDMAB13 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab13`]
module"]
pub type OTG_HCDMAB13 = crate::Reg<otg_hcdmab13::OTG_HCDMAB13rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab13;
#[doc = "OTG_HCCHAR14 (rw) register accessor: OTG host channel 14 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar14`]
module"]
pub type OTG_HCCHAR14 = crate::Reg<otg_hcchar14::OTG_HCCHAR14rs>;
#[doc = "OTG host channel 14 characteristics register"]
pub mod otg_hcchar14;
#[doc = "OTG_HCSPLT14 (rw) register accessor: OTG host channel 14 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt14`]
module"]
pub type OTG_HCSPLT14 = crate::Reg<otg_hcsplt14::OTG_HCSPLT14rs>;
#[doc = "OTG host channel 14 split control register"]
pub mod otg_hcsplt14;
#[doc = "OTG_HCINT14 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint14`]
module"]
pub type OTG_HCINT14 = crate::Reg<otg_hcint14::OTG_HCINT14rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint14;
#[doc = "OTG_HCINTMSK14 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk14`]
module"]
pub type OTG_HCINTMSK14 = crate::Reg<otg_hcintmsk14::OTG_HCINTMSK14rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk14;
#[doc = "OTG_HCTSIZ14 (rw) register accessor: OTG host channel 14 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz14`]
module"]
pub type OTG_HCTSIZ14 = crate::Reg<otg_hctsiz14::OTG_HCTSIZ14rs>;
#[doc = "OTG host channel 14 transfer size register"]
pub mod otg_hctsiz14;
#[doc = "OTG_HCDMA14 (rw) register accessor: OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma14`]
module"]
pub type OTG_HCDMA14 = crate::Reg<otg_hcdma14::OTG_HCDMA14rs>;
#[doc = "OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma14;
#[doc = "OTG_HCDMAB14 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab14`]
module"]
pub type OTG_HCDMAB14 = crate::Reg<otg_hcdmab14::OTG_HCDMAB14rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab14;
#[doc = "OTG_HCCHAR15 (rw) register accessor: OTG host channel 15 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcchar15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcchar15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcchar15`]
module"]
pub type OTG_HCCHAR15 = crate::Reg<otg_hcchar15::OTG_HCCHAR15rs>;
#[doc = "OTG host channel 15 characteristics register"]
pub mod otg_hcchar15;
#[doc = "OTG_HCSPLT15 (rw) register accessor: OTG host channel 15 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcsplt15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcsplt15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcsplt15`]
module"]
pub type OTG_HCSPLT15 = crate::Reg<otg_hcsplt15::OTG_HCSPLT15rs>;
#[doc = "OTG host channel 15 split control register"]
pub mod otg_hcsplt15;
#[doc = "OTG_HCINT15 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcint15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcint15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcint15`]
module"]
pub type OTG_HCINT15 = crate::Reg<otg_hcint15::OTG_HCINT15rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint15;
#[doc = "OTG_HCINTMSK15 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcintmsk15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcintmsk15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcintmsk15`]
module"]
pub type OTG_HCINTMSK15 = crate::Reg<otg_hcintmsk15::OTG_HCINTMSK15rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk15;
#[doc = "OTG_HCTSIZ15 (rw) register accessor: OTG host channel 15 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hctsiz15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hctsiz15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hctsiz15`]
module"]
pub type OTG_HCTSIZ15 = crate::Reg<otg_hctsiz15::OTG_HCTSIZ15rs>;
#[doc = "OTG host channel 15 transfer size register"]
pub mod otg_hctsiz15;
#[doc = "OTG_HCDMA15 (rw) register accessor: OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdma15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hcdma15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdma15`]
module"]
pub type OTG_HCDMA15 = crate::Reg<otg_hcdma15::OTG_HCDMA15rs>;
#[doc = "OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma15;
#[doc = "OTG_HCDMAB15 (r) register accessor: OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hcdmab15`]
module"]
pub type OTG_HCDMAB15 = crate::Reg<otg_hcdmab15::OTG_HCDMAB15rs>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab15;
#[doc = "OTG_DCFG (rw) register accessor: This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dcfg`]
module"]
pub type OTG_DCFG = crate::Reg<otg_dcfg::OTG_DCFGrs>;
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
pub mod otg_dcfg;
#[doc = "OTG_DCTL (rw) register accessor: OTG device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dctl`]
module"]
pub type OTG_DCTL = crate::Reg<otg_dctl::OTG_DCTLrs>;
#[doc = "OTG device control register"]
pub mod otg_dctl;
#[doc = "OTG_DSTS (r) register accessor: This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dsts`]
module"]
pub type OTG_DSTS = crate::Reg<otg_dsts::OTG_DSTSrs>;
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
pub mod otg_dsts;
#[doc = "OTG_DIEPMSK (rw) register accessor: This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepmsk`]
module"]
pub type OTG_DIEPMSK = crate::Reg<otg_diepmsk::OTG_DIEPMSKrs>;
#[doc = "This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_diepmsk;
#[doc = "OTG_DOEPMSK (rw) register accessor: This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepmsk`]
module"]
pub type OTG_DOEPMSK = crate::Reg<otg_doepmsk::OTG_DOEPMSKrs>;
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_doepmsk;
#[doc = "OTG_DAINT (r) register accessor: When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_daint`]
module"]
pub type OTG_DAINT = crate::Reg<otg_daint::OTG_DAINTrs>;
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
pub mod otg_daint;
#[doc = "OTG_DAINTMSK (rw) register accessor: The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_daintmsk`]
module"]
pub type OTG_DAINTMSK = crate::Reg<otg_daintmsk::OTG_DAINTMSKrs>;
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
pub mod otg_daintmsk;
#[doc = "OTG_DVBUSDIS (rw) register accessor: This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dvbusdis`]
module"]
pub type OTG_DVBUSDIS = crate::Reg<otg_dvbusdis::OTG_DVBUSDISrs>;
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
pub mod otg_dvbusdis;
#[doc = "OTG_DVBUSPULSE (rw) register accessor: This register specifies the VBUS pulsing time during SRP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dvbuspulse`]
module"]
pub type OTG_DVBUSPULSE = crate::Reg<otg_dvbuspulse::OTG_DVBUSPULSErs>;
#[doc = "This register specifies the VBUS pulsing time during SRP."]
pub mod otg_dvbuspulse;
#[doc = "OTG_DTHRCTL (rw) register accessor: OTG device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dthrctl`]
module"]
pub type OTG_DTHRCTL = crate::Reg<otg_dthrctl::OTG_DTHRCTLrs>;
#[doc = "OTG device threshold control register"]
pub mod otg_dthrctl;
#[doc = "OTG_DIEPEMPMSK (rw) register accessor: This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepempmsk`]
module"]
pub type OTG_DIEPEMPMSK = crate::Reg<otg_diepempmsk::OTG_DIEPEMPMSKrs>;
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
pub mod otg_diepempmsk;
#[doc = "OTG_DEACHINT (r) register accessor: OTG device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_deachint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_deachint`]
module"]
pub type OTG_DEACHINT = crate::Reg<otg_deachint::OTG_DEACHINTrs>;
#[doc = "OTG device each endpoint interrupt register"]
pub mod otg_deachint;
#[doc = "OTG_DEACHINTMSK (rw) register accessor: There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_deachintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_deachintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_deachintmsk`]
module"]
pub type OTG_DEACHINTMSK = crate::Reg<otg_deachintmsk::OTG_DEACHINTMSKrs>;
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
pub mod otg_deachintmsk;
#[doc = "OTG_HS_DIEPEACHMSK1 (rw) register accessor: This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_diepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_diepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hs_diepeachmsk1`]
module"]
pub type OTG_HS_DIEPEACHMSK1 = crate::Reg<otg_hs_diepeachmsk1::OTG_HS_DIEPEACHMSK1rs>;
#[doc = "This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_diepeachmsk1;
#[doc = "OTG_HS_DOEPEACHMSK1 (rw) register accessor: This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hs_doepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_hs_doepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hs_doepeachmsk1`]
module"]
pub type OTG_HS_DOEPEACHMSK1 = crate::Reg<otg_hs_doepeachmsk1::OTG_HS_DOEPEACHMSK1rs>;
#[doc = "This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_doepeachmsk1;
#[doc = "OTG_DIEPCTL0 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl0`]
module"]
pub type OTG_DIEPCTL0 = crate::Reg<otg_diepctl0::OTG_DIEPCTL0rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl0;
#[doc = "OTG_DIEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint0`]
module"]
pub type OTG_DIEPINT0 = crate::Reg<otg_diepint0::OTG_DIEPINT0rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint0;
#[doc = "OTG_DIEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz0`]
module"]
pub type OTG_DIEPTSIZ0 = crate::Reg<otg_dieptsiz0::OTG_DIEPTSIZ0rs>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_dieptsiz0;
#[doc = "OTG_DIEPDMA0 (rw) register accessor: OTG device IN endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma0`]
module"]
pub type OTG_DIEPDMA0 = crate::Reg<otg_diepdma0::OTG_DIEPDMA0rs>;
#[doc = "OTG device IN endpoint 0 DMA address register"]
pub mod otg_diepdma0;
#[doc = "OTG_DTXFSTS0 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts0`]
module"]
pub type OTG_DTXFSTS0 = crate::Reg<otg_dtxfsts0::OTG_DTXFSTS0rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts0;
#[doc = "OTG_DIEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl1`]
module"]
pub type OTG_DIEPCTL1 = crate::Reg<otg_diepctl1::OTG_DIEPCTL1rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl1;
#[doc = "OTG_DIEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint1`]
module"]
pub type OTG_DIEPINT1 = crate::Reg<otg_diepint1::OTG_DIEPINT1rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint1;
#[doc = "OTG_DIEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz1`]
module"]
pub type OTG_DIEPTSIZ1 = crate::Reg<otg_dieptsiz1::OTG_DIEPTSIZ1rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz1;
#[doc = "OTG_DIEPDMA1 (rw) register accessor: OTG device IN endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma1`]
module"]
pub type OTG_DIEPDMA1 = crate::Reg<otg_diepdma1::OTG_DIEPDMA1rs>;
#[doc = "OTG device IN endpoint 1 DMA address register"]
pub mod otg_diepdma1;
#[doc = "OTG_DTXFSTS1 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts1`]
module"]
pub type OTG_DTXFSTS1 = crate::Reg<otg_dtxfsts1::OTG_DTXFSTS1rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts1;
#[doc = "OTG_DIEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl2`]
module"]
pub type OTG_DIEPCTL2 = crate::Reg<otg_diepctl2::OTG_DIEPCTL2rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl2;
#[doc = "OTG_DIEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint2`]
module"]
pub type OTG_DIEPINT2 = crate::Reg<otg_diepint2::OTG_DIEPINT2rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint2;
#[doc = "OTG_DIEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz2`]
module"]
pub type OTG_DIEPTSIZ2 = crate::Reg<otg_dieptsiz2::OTG_DIEPTSIZ2rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz2;
#[doc = "OTG_DIEPDMA2 (rw) register accessor: OTG device IN endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma2`]
module"]
pub type OTG_DIEPDMA2 = crate::Reg<otg_diepdma2::OTG_DIEPDMA2rs>;
#[doc = "OTG device IN endpoint 2 DMA address register"]
pub mod otg_diepdma2;
#[doc = "OTG_DTXFSTS2 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts2`]
module"]
pub type OTG_DTXFSTS2 = crate::Reg<otg_dtxfsts2::OTG_DTXFSTS2rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts2;
#[doc = "OTG_DIEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl3`]
module"]
pub type OTG_DIEPCTL3 = crate::Reg<otg_diepctl3::OTG_DIEPCTL3rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl3;
#[doc = "OTG_DIEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint3`]
module"]
pub type OTG_DIEPINT3 = crate::Reg<otg_diepint3::OTG_DIEPINT3rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint3;
#[doc = "OTG_DIEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz3`]
module"]
pub type OTG_DIEPTSIZ3 = crate::Reg<otg_dieptsiz3::OTG_DIEPTSIZ3rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz3;
#[doc = "OTG_DIEPDMA3 (rw) register accessor: OTG device IN endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma3`]
module"]
pub type OTG_DIEPDMA3 = crate::Reg<otg_diepdma3::OTG_DIEPDMA3rs>;
#[doc = "OTG device IN endpoint 3 DMA address register"]
pub mod otg_diepdma3;
#[doc = "OTG_DTXFSTS3 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts3`]
module"]
pub type OTG_DTXFSTS3 = crate::Reg<otg_dtxfsts3::OTG_DTXFSTS3rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts3;
#[doc = "OTG_DIEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl4`]
module"]
pub type OTG_DIEPCTL4 = crate::Reg<otg_diepctl4::OTG_DIEPCTL4rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl4;
#[doc = "OTG_DIEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint4`]
module"]
pub type OTG_DIEPINT4 = crate::Reg<otg_diepint4::OTG_DIEPINT4rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint4;
#[doc = "OTG_DIEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz4`]
module"]
pub type OTG_DIEPTSIZ4 = crate::Reg<otg_dieptsiz4::OTG_DIEPTSIZ4rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz4;
#[doc = "OTG_DIEPDMA4 (rw) register accessor: OTG device IN endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma4`]
module"]
pub type OTG_DIEPDMA4 = crate::Reg<otg_diepdma4::OTG_DIEPDMA4rs>;
#[doc = "OTG device IN endpoint 4 DMA address register"]
pub mod otg_diepdma4;
#[doc = "OTG_DTXFSTS4 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts4`]
module"]
pub type OTG_DTXFSTS4 = crate::Reg<otg_dtxfsts4::OTG_DTXFSTS4rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts4;
#[doc = "OTG_DIEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl5`]
module"]
pub type OTG_DIEPCTL5 = crate::Reg<otg_diepctl5::OTG_DIEPCTL5rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl5;
#[doc = "OTG_DIEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint5`]
module"]
pub type OTG_DIEPINT5 = crate::Reg<otg_diepint5::OTG_DIEPINT5rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint5;
#[doc = "OTG_DIEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz5`]
module"]
pub type OTG_DIEPTSIZ5 = crate::Reg<otg_dieptsiz5::OTG_DIEPTSIZ5rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz5;
#[doc = "OTG_DIEPDMA5 (rw) register accessor: OTG device IN endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma5`]
module"]
pub type OTG_DIEPDMA5 = crate::Reg<otg_diepdma5::OTG_DIEPDMA5rs>;
#[doc = "OTG device IN endpoint 5 DMA address register"]
pub mod otg_diepdma5;
#[doc = "OTG_DTXFSTS5 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts5`]
module"]
pub type OTG_DTXFSTS5 = crate::Reg<otg_dtxfsts5::OTG_DTXFSTS5rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts5;
#[doc = "OTG_DIEPCTL6 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl6`]
module"]
pub type OTG_DIEPCTL6 = crate::Reg<otg_diepctl6::OTG_DIEPCTL6rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl6;
#[doc = "OTG_DIEPINT6 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint6`]
module"]
pub type OTG_DIEPINT6 = crate::Reg<otg_diepint6::OTG_DIEPINT6rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint6;
#[doc = "OTG_DIEPTSIZ6 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz6`]
module"]
pub type OTG_DIEPTSIZ6 = crate::Reg<otg_dieptsiz6::OTG_DIEPTSIZ6rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz6;
#[doc = "OTG_DIEPDMA6 (rw) register accessor: OTG device IN endpoint 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma6`]
module"]
pub type OTG_DIEPDMA6 = crate::Reg<otg_diepdma6::OTG_DIEPDMA6rs>;
#[doc = "OTG device IN endpoint 6 DMA address register"]
pub mod otg_diepdma6;
#[doc = "OTG_DTXFSTS6 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts6`]
module"]
pub type OTG_DTXFSTS6 = crate::Reg<otg_dtxfsts6::OTG_DTXFSTS6rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts6;
#[doc = "OTG_DIEPCTL7 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl7`]
module"]
pub type OTG_DIEPCTL7 = crate::Reg<otg_diepctl7::OTG_DIEPCTL7rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl7;
#[doc = "OTG_DIEPINT7 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint7`]
module"]
pub type OTG_DIEPINT7 = crate::Reg<otg_diepint7::OTG_DIEPINT7rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint7;
#[doc = "OTG_DIEPTSIZ7 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz7`]
module"]
pub type OTG_DIEPTSIZ7 = crate::Reg<otg_dieptsiz7::OTG_DIEPTSIZ7rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz7;
#[doc = "OTG_DIEPDMA7 (rw) register accessor: OTG device IN endpoint 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma7`]
module"]
pub type OTG_DIEPDMA7 = crate::Reg<otg_diepdma7::OTG_DIEPDMA7rs>;
#[doc = "OTG device IN endpoint 7 DMA address register"]
pub mod otg_diepdma7;
#[doc = "OTG_DTXFSTS7 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts7`]
module"]
pub type OTG_DTXFSTS7 = crate::Reg<otg_dtxfsts7::OTG_DTXFSTS7rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts7;
#[doc = "OTG_DIEPCTL8 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepctl8`]
module"]
pub type OTG_DIEPCTL8 = crate::Reg<otg_diepctl8::OTG_DIEPCTL8rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl8;
#[doc = "OTG_DIEPINT8 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepint8`]
module"]
pub type OTG_DIEPINT8 = crate::Reg<otg_diepint8::OTG_DIEPINT8rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint8;
#[doc = "OTG_DIEPTSIZ8 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dieptsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_dieptsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dieptsiz8`]
module"]
pub type OTG_DIEPTSIZ8 = crate::Reg<otg_dieptsiz8::OTG_DIEPTSIZ8rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz8;
#[doc = "OTG_DIEPDMA8 (rw) register accessor: OTG device IN endpoint 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_diepdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_diepdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_diepdma8`]
module"]
pub type OTG_DIEPDMA8 = crate::Reg<otg_diepdma8::OTG_DIEPDMA8rs>;
#[doc = "OTG device IN endpoint 8 DMA address register"]
pub mod otg_diepdma8;
#[doc = "OTG_DTXFSTS8 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_dtxfsts8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_dtxfsts8`]
module"]
pub type OTG_DTXFSTS8 = crate::Reg<otg_dtxfsts8::OTG_DTXFSTS8rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts8;
#[doc = "OTG_DOEPCTL0 (rw) register accessor: This section describes the OTG_DOEPCTL0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl0`]
module"]
pub type OTG_DOEPCTL0 = crate::Reg<otg_doepctl0::OTG_DOEPCTL0rs>;
#[doc = "This section describes the OTG_DOEPCTL0 register."]
pub mod otg_doepctl0;
#[doc = "OTG_DOEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint0`]
module"]
pub type OTG_DOEPINT0 = crate::Reg<otg_doepint0::OTG_DOEPINT0rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint0;
#[doc = "OTG_DOEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz0`]
module"]
pub type OTG_DOEPTSIZ0 = crate::Reg<otg_doeptsiz0::OTG_DOEPTSIZ0rs>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_doeptsiz0;
#[doc = "OTG_DOEPDMA0 (rw) register accessor: OTG device OUT endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma0`]
module"]
pub type OTG_DOEPDMA0 = crate::Reg<otg_doepdma0::OTG_DOEPDMA0rs>;
#[doc = "OTG device OUT endpoint 0 DMA address register"]
pub mod otg_doepdma0;
#[doc = "OTG_DOEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl1`]
module"]
pub type OTG_DOEPCTL1 = crate::Reg<otg_doepctl1::OTG_DOEPCTL1rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl1;
#[doc = "OTG_DOEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint1`]
module"]
pub type OTG_DOEPINT1 = crate::Reg<otg_doepint1::OTG_DOEPINT1rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint1;
#[doc = "OTG_DOEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz1`]
module"]
pub type OTG_DOEPTSIZ1 = crate::Reg<otg_doeptsiz1::OTG_DOEPTSIZ1rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz1;
#[doc = "OTG_DOEPDMA1 (rw) register accessor: OTG device OUT endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma1`]
module"]
pub type OTG_DOEPDMA1 = crate::Reg<otg_doepdma1::OTG_DOEPDMA1rs>;
#[doc = "OTG device OUT endpoint 1 DMA address register"]
pub mod otg_doepdma1;
#[doc = "OTG_DOEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl2`]
module"]
pub type OTG_DOEPCTL2 = crate::Reg<otg_doepctl2::OTG_DOEPCTL2rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl2;
#[doc = "OTG_DOEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint2`]
module"]
pub type OTG_DOEPINT2 = crate::Reg<otg_doepint2::OTG_DOEPINT2rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint2;
#[doc = "OTG_DOEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz2`]
module"]
pub type OTG_DOEPTSIZ2 = crate::Reg<otg_doeptsiz2::OTG_DOEPTSIZ2rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz2;
#[doc = "OTG_DOEPDMA2 (rw) register accessor: OTG device OUT endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma2`]
module"]
pub type OTG_DOEPDMA2 = crate::Reg<otg_doepdma2::OTG_DOEPDMA2rs>;
#[doc = "OTG device OUT endpoint 2 DMA address register"]
pub mod otg_doepdma2;
#[doc = "OTG_DOEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl3`]
module"]
pub type OTG_DOEPCTL3 = crate::Reg<otg_doepctl3::OTG_DOEPCTL3rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl3;
#[doc = "OTG_DOEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint3`]
module"]
pub type OTG_DOEPINT3 = crate::Reg<otg_doepint3::OTG_DOEPINT3rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint3;
#[doc = "OTG_DOEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz3`]
module"]
pub type OTG_DOEPTSIZ3 = crate::Reg<otg_doeptsiz3::OTG_DOEPTSIZ3rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz3;
#[doc = "OTG_DOEPDMA3 (rw) register accessor: OTG device OUT endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma3`]
module"]
pub type OTG_DOEPDMA3 = crate::Reg<otg_doepdma3::OTG_DOEPDMA3rs>;
#[doc = "OTG device OUT endpoint 3 DMA address register"]
pub mod otg_doepdma3;
#[doc = "OTG_DOEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl4`]
module"]
pub type OTG_DOEPCTL4 = crate::Reg<otg_doepctl4::OTG_DOEPCTL4rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl4;
#[doc = "OTG_DOEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint4`]
module"]
pub type OTG_DOEPINT4 = crate::Reg<otg_doepint4::OTG_DOEPINT4rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint4;
#[doc = "OTG_DOEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz4`]
module"]
pub type OTG_DOEPTSIZ4 = crate::Reg<otg_doeptsiz4::OTG_DOEPTSIZ4rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz4;
#[doc = "OTG_DOEPDMA4 (rw) register accessor: OTG device OUT endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma4`]
module"]
pub type OTG_DOEPDMA4 = crate::Reg<otg_doepdma4::OTG_DOEPDMA4rs>;
#[doc = "OTG device OUT endpoint 4 DMA address register"]
pub mod otg_doepdma4;
#[doc = "OTG_DOEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl5`]
module"]
pub type OTG_DOEPCTL5 = crate::Reg<otg_doepctl5::OTG_DOEPCTL5rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl5;
#[doc = "OTG_DOEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint5`]
module"]
pub type OTG_DOEPINT5 = crate::Reg<otg_doepint5::OTG_DOEPINT5rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint5;
#[doc = "OTG_DOEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz5`]
module"]
pub type OTG_DOEPTSIZ5 = crate::Reg<otg_doeptsiz5::OTG_DOEPTSIZ5rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz5;
#[doc = "OTG_DOEPDMA5 (rw) register accessor: OTG device OUT endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma5`]
module"]
pub type OTG_DOEPDMA5 = crate::Reg<otg_doepdma5::OTG_DOEPDMA5rs>;
#[doc = "OTG device OUT endpoint 5 DMA address register"]
pub mod otg_doepdma5;
#[doc = "OTG_DOEPCTL6 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl6`]
module"]
pub type OTG_DOEPCTL6 = crate::Reg<otg_doepctl6::OTG_DOEPCTL6rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl6;
#[doc = "OTG_DOEPINT6 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint6`]
module"]
pub type OTG_DOEPINT6 = crate::Reg<otg_doepint6::OTG_DOEPINT6rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint6;
#[doc = "OTG_DOEPTSIZ6 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz6`]
module"]
pub type OTG_DOEPTSIZ6 = crate::Reg<otg_doeptsiz6::OTG_DOEPTSIZ6rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz6;
#[doc = "OTG_DOEPDMA6 (rw) register accessor: OTG device OUT endpoint 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma6`]
module"]
pub type OTG_DOEPDMA6 = crate::Reg<otg_doepdma6::OTG_DOEPDMA6rs>;
#[doc = "OTG device OUT endpoint 6 DMA address register"]
pub mod otg_doepdma6;
#[doc = "OTG_DOEPCTL7 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl7`]
module"]
pub type OTG_DOEPCTL7 = crate::Reg<otg_doepctl7::OTG_DOEPCTL7rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl7;
#[doc = "OTG_DOEPINT7 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint7`]
module"]
pub type OTG_DOEPINT7 = crate::Reg<otg_doepint7::OTG_DOEPINT7rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint7;
#[doc = "OTG_DOEPTSIZ7 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz7`]
module"]
pub type OTG_DOEPTSIZ7 = crate::Reg<otg_doeptsiz7::OTG_DOEPTSIZ7rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz7;
#[doc = "OTG_DOEPDMA7 (rw) register accessor: OTG device OUT endpoint 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma7`]
module"]
pub type OTG_DOEPDMA7 = crate::Reg<otg_doepdma7::OTG_DOEPDMA7rs>;
#[doc = "OTG device OUT endpoint 7 DMA address register"]
pub mod otg_doepdma7;
#[doc = "OTG_DOEPCTL8 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepctl8`]
module"]
pub type OTG_DOEPCTL8 = crate::Reg<otg_doepctl8::OTG_DOEPCTL8rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl8;
#[doc = "OTG_DOEPINT8 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepint8`]
module"]
pub type OTG_DOEPINT8 = crate::Reg<otg_doepint8::OTG_DOEPINT8rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint8;
#[doc = "OTG_DOEPTSIZ8 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doeptsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doeptsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doeptsiz8`]
module"]
pub type OTG_DOEPTSIZ8 = crate::Reg<otg_doeptsiz8::OTG_DOEPTSIZ8rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz8;
#[doc = "OTG_DOEPDMA8 (rw) register accessor: OTG device OUT endpoint 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_doepdma8`]
module"]
pub type OTG_DOEPDMA8 = crate::Reg<otg_doepdma8::OTG_DOEPDMA8rs>;
#[doc = "OTG device OUT endpoint 8 DMA address register"]
pub mod otg_doepdma8;
#[doc = "OTG_PCGCCTL (rw) register accessor: This register is available in host and device modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_pcgcctl`]
module"]
pub type OTG_PCGCCTL = crate::Reg<otg_pcgcctl::OTG_PCGCCTLrs>;
#[doc = "This register is available in host and device modes."]
pub mod otg_pcgcctl;
