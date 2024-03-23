#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    _reserved_7_grxstsr: [u8; 0x04],
    _reserved_8_grxstsp: [u8; 0x04],
    grxfsiz: GRXFSIZ,
    hnptxfsiz: HNPTXFSIZ,
    hnptxsts: HNPTXSTS,
    _reserved12: [u8; 0x08],
    gccfg: GCCFG,
    cid: CID,
    _reserved14: [u8; 0x14],
    glpmcfg: GLPMCFG,
    _reserved15: [u8; 0xa8],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
    dieptxf6: DIEPTXF6,
    dieptxf7: DIEPTXF7,
    dieptxf8: DIEPTXF8,
    _reserved24: [u8; 0x02dc],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved27: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved30: [u8; 0x24],
    hprt: HPRT,
    _reserved31: [u8; 0xbc],
    hcchar0: HCCHAR0,
    hcsplt0: HCSPLT0,
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    hcdma0: HCDMA0,
    _reserved37: [u8; 0x08],
    hcchar1: HCCHAR1,
    hcsplt1: HCSPLT1,
    hcint1_device: HCINT1_DEVICE,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    hcdma1: HCDMA1,
    _reserved43: [u8; 0x08],
    hcchar2: HCCHAR2,
    hcsplt2: HCSPLT2,
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    hcdma2: HCDMA2,
    _reserved49: [u8; 0x08],
    hcchar3: HCCHAR3,
    hcsplt3: HCSPLT3,
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    hcdma3: HCDMA3,
    _reserved55: [u8; 0x08],
    hcchar4: HCCHAR4,
    hcsplt4: HCSPLT4,
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    hcdma4: HCDMA4,
    _reserved61: [u8; 0x08],
    hcchar5: HCCHAR5,
    hcsplt5: HCSPLT5,
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    hcdma5: HCDMA5,
    _reserved67: [u8; 0x08],
    hcchar6: HCCHAR6,
    hcsplt6: HCSPLT6,
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    hcdma6: HCDMA6,
    _reserved73: [u8; 0x08],
    hcchar7: HCCHAR7,
    hcsplt7: HCSPLT7,
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    hcdma7: HCDMA7,
    _reserved79: [u8; 0x08],
    hcchar8: HCCHAR8,
    hcsplt8: HCSPLT8,
    hcint8: HCINT8,
    hcintmsk8: HCINTMSK8,
    hctsiz8: HCTSIZ8,
    hcdma8: HCDMA8,
    _reserved85: [u8; 0x08],
    hcchar9: HCCHAR9,
    hcsplt9: HCSPLT9,
    hcint9: HCINT9,
    hcintmsk9: HCINTMSK9,
    hctsiz9: HCTSIZ9,
    hcdma9: HCDMA9,
    _reserved91: [u8; 0x08],
    hcchar10: HCCHAR10,
    hcsplt10: HCSPLT10,
    hcint10: HCINT10,
    hcintmsk10: HCINTMSK10,
    hctsiz10: HCTSIZ10,
    hcdma10: HCDMA10,
    _reserved97: [u8; 0x08],
    hcchar11: HCCHAR11,
    hcsplt11: HCSPLT11,
    hcint11: HCINT11,
    hcintmsk11: HCINTMSK11,
    hctsiz11: HCTSIZ11,
    hcdma11: HCDMA11,
    _reserved103: [u8; 0x08],
    hcchar12: HCCHAR12,
    hcsplt12: HCSPLT12,
    hcint12: HCINT12,
    hcintmsk12: HCINTMSK12,
    hctsiz12: HCTSIZ12,
    hcdma12: HCDMA12,
    _reserved109: [u8; 0x08],
    hcchar13: HCCHAR13,
    hcsplt13: HCSPLT13,
    hcint13: HCINT13,
    hcintmsk13: HCINTMSK13,
    hctsiz13: HCTSIZ13,
    hcdma13: HCDMA13,
    _reserved115: [u8; 0x08],
    hcchar14: HCCHAR14,
    hcsplt14: HCSPLT14,
    hcint14: HCINT14,
    hcintmsk14: HCINTMSK14,
    hctsiz14: HCTSIZ14,
    hcdma14: HCDMA14,
    _reserved121: [u8; 0x08],
    hcchar15: HCCHAR15,
    hcsplt15: HCSPLT15,
    hcint15: HCINT15,
    hcintmsk15: HCINTMSK15,
    hctsiz15: HCTSIZ15,
    hcdma15: HCDMA15,
    _reserved127: [u8; 0x0108],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved130: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved134: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    _reserved138: [u8; 0x4c],
    hs_doepeachmsk1: HS_DOEPEACHMSK1,
    _reserved139: [u8; 0x78],
    diepctl0: DIEPCTL0,
    _reserved140: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved141: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    _reserved142: [u8; 0x04],
    dtxfsts0: DTXFSTS0,
    _reserved143: [u8; 0x04],
    diepctl1: DIEPCTL1,
    _reserved144: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved145: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    diepdma1: DIEPDMA1,
    dtxfsts1: DTXFSTS1,
    _reserved148: [u8; 0x04],
    diepctl2: DIEPCTL2,
    _reserved149: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved150: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    diepdma2: DIEPDMA2,
    dtxfsts2: DTXFSTS2,
    _reserved153: [u8; 0x04],
    diepctl3: DIEPCTL3,
    _reserved154: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved155: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    diepdma3: DIEPDMA3,
    dtxfsts3: DTXFSTS3,
    _reserved158: [u8; 0x04],
    diepctl4: DIEPCTL4,
    _reserved159: [u8; 0x04],
    diepint4: DIEPINT4,
    _reserved160: [u8; 0x04],
    dieptsiz4: DIEPTSIZ4,
    diepdma4: DIEPDMA4,
    dtxfsts4: DTXFSTS4,
    _reserved163: [u8; 0x04],
    diepctl5: DIEPCTL5,
    _reserved164: [u8; 0x04],
    diepint5: DIEPINT5,
    _reserved165: [u8; 0x04],
    dieptsiz5: DIEPTSIZ5,
    diepdma5: DIEPDMA5,
    dtxfsts5: DTXFSTS5,
    _reserved168: [u8; 0x0c],
    diepint6: DIEPINT6,
    _reserved169: [u8; 0x04],
    dieptsiz6: DIEPTSIZ6,
    diepdma6: DIEPDMA6,
    _reserved171: [u8; 0x10],
    diepint7: DIEPINT7,
    _reserved172: [u8; 0x04],
    dieptsiz7: DIEPTSIZ7,
    diepdma7: DIEPDMA7,
    _reserved174: [u8; 0x10],
    diepint8: DIEPINT8,
    _reserved175: [u8; 0x04],
    dieptsiz8: DIEPTSIZ8,
    diepdma8: DIEPDMA8,
    _reserved177: [u8; 0xe8],
    doepctl0: DOEPCTL0,
    _reserved178: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved179: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    doepdma0: DOEPDMA0,
    _reserved181: [u8; 0x08],
    doepctl1: DOEPCTL1,
    _reserved182: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved183: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    doepdma1: DOEPDMA1,
    _reserved185: [u8; 0x08],
    doepctl2: DOEPCTL2,
    _reserved186: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved187: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    doepdma2: DOEPDMA2,
    _reserved189: [u8; 0x08],
    doepctl3: DOEPCTL3,
    _reserved190: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved191: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
    doepdma3: DOEPDMA3,
    _reserved193: [u8; 0x08],
    doepctl4: DOEPCTL4,
    _reserved194: [u8; 0x04],
    doepint4: DOEPINT4,
    _reserved195: [u8; 0x04],
    doeptsiz4: DOEPTSIZ4,
    doepdma4: DOEPDMA4,
    _reserved197: [u8; 0x08],
    doepctl5: DOEPCTL5,
    _reserved198: [u8; 0x04],
    doepint5: DOEPINT5,
    _reserved199: [u8; 0x04],
    doeptsiz5: DOEPTSIZ5,
    doepdma5: DOEPDMA5,
    _reserved201: [u8; 0x08],
    doepctl6: DOEPCTL6,
    _reserved202: [u8; 0x04],
    doepint6: DOEPINT6,
    _reserved203: [u8; 0x04],
    doeptsiz6: DOEPTSIZ6,
    doepdma6: DOEPDMA6,
    _reserved205: [u8; 0x08],
    doepctl7: DOEPCTL7,
    _reserved206: [u8; 0x04],
    doepint7: DOEPINT7,
    _reserved207: [u8; 0x04],
    doeptsiz7: DOEPTSIZ7,
    doepdma7: DOEPDMA7,
    _reserved209: [u8; 0x08],
    doepctl8: DOEPCTL8,
    _reserved210: [u8; 0x04],
    doepint8: DOEPINT8,
    _reserved211: [u8; 0x04],
    doeptsiz8: DOEPTSIZ8,
    doepdma8: DOEPDMA8,
    _reserved213: [u8; 0x01e8],
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - The application uses this register to reset various hardware features inside the core."]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set."]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x1c - This description is for register GRXSTSR in Host mode"]
    #[inline(always)]
    pub const fn grxstsr_host(&self) -> &GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
    #[inline(always)]
    pub const fn grxstsr_device(&self) -> &GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - This description is for register GRXSTSP in HOST mode"]
    #[inline(always)]
    pub const fn grxstsp_host(&self) -> &GRXSTSP_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted."]
    #[inline(always)]
    pub const fn grxstsp_device(&self) -> &GRXSTSP_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - The application can program the RAM size that must be allocated to the Rx FIFO."]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - Host mode"]
    #[inline(always)]
    pub const fn hnptxfsiz(&self) -> &HNPTXFSIZ {
        &self.hnptxfsiz
    }
    #[doc = "0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
    #[inline(always)]
    pub const fn hnptxsts(&self) -> &HNPTXSTS {
        &self.hnptxsts
    }
    #[doc = "0x38 - OTG general core configuration register"]
    #[inline(always)]
    pub const fn gccfg(&self) -> &GCCFG {
        &self.gccfg
    }
    #[doc = "0x3c - This is a register containing the Product ID as reset value."]
    #[inline(always)]
    pub const fn cid(&self) -> &CID {
        &self.cid
    }
    #[doc = "0x54 - OTG core LPM configuration register"]
    #[inline(always)]
    pub const fn glpmcfg(&self) -> &GLPMCFG {
        &self.glpmcfg
    }
    #[doc = "0x100 - OTG host periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104 - OTG device IN endpoint transmit FIFO 1 size register"]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - OTG device IN endpoint transmit FIFO 2 size register"]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - OTG device IN endpoint transmit FIFO 3 size register"]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - OTG device IN endpoint transmit FIFO 4 size register"]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - OTG device IN endpoint transmit FIFO 5 size register"]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - OTG device IN endpoint transmit FIFO 6 size register"]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF6 {
        &self.dieptxf6
    }
    #[doc = "0x11c - OTG device IN endpoint transmit FIFO 7 size register"]
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &DIEPTXF7 {
        &self.dieptxf7
    }
    #[doc = "0x120 - OTG device IN endpoint transmit FIFO 8 size register"]
    #[inline(always)]
    pub const fn dieptxf8(&self) -> &DIEPTXF8 {
        &self.dieptxf8
    }
    #[doc = "0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host."]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x500 - OTG host channel 0 characteristics register"]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    #[doc = "0x504 - OTG host channel 0 split control register"]
    #[inline(always)]
    pub const fn hcsplt0(&self) -> &HCSPLT0 {
        &self.hcsplt0
    }
    #[doc = "0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    #[doc = "0x50c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    #[doc = "0x510 - OTG host channel 0 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    #[doc = "0x514 - OTG host channel 0 DMA address register"]
    #[inline(always)]
    pub const fn hcdma0(&self) -> &HCDMA0 {
        &self.hcdma0
    }
    #[doc = "0x520 - OTG host channel 1 characteristics register"]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    #[doc = "0x524 - OTG host channel 1 split control register"]
    #[inline(always)]
    pub const fn hcsplt1(&self) -> &HCSPLT1 {
        &self.hcsplt1
    }
    #[doc = "0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint1_device(&self) -> &HCINT1_DEVICE {
        &self.hcint1_device
    }
    #[doc = "0x52c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    #[doc = "0x530 - OTG host channel 1 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    #[doc = "0x534 - OTG host channel 1 DMA address register"]
    #[inline(always)]
    pub const fn hcdma1(&self) -> &HCDMA1 {
        &self.hcdma1
    }
    #[doc = "0x540 - OTG host channel 2 characteristics register"]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    #[doc = "0x544 - OTG host channel 2 split control register"]
    #[inline(always)]
    pub const fn hcsplt2(&self) -> &HCSPLT2 {
        &self.hcsplt2
    }
    #[doc = "0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    #[doc = "0x54c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    #[doc = "0x550 - OTG host channel 2 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    #[doc = "0x554 - OTG host channel 2 DMA address register"]
    #[inline(always)]
    pub const fn hcdma2(&self) -> &HCDMA2 {
        &self.hcdma2
    }
    #[doc = "0x560 - OTG host channel 3 characteristics register"]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    #[doc = "0x564 - OTG host channel 3 split control register"]
    #[inline(always)]
    pub const fn hcsplt3(&self) -> &HCSPLT3 {
        &self.hcsplt3
    }
    #[doc = "0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    #[doc = "0x56c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    #[doc = "0x570 - OTG host channel 3 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    #[doc = "0x574 - OTG host channel 3 DMA address register"]
    #[inline(always)]
    pub const fn hcdma3(&self) -> &HCDMA3 {
        &self.hcdma3
    }
    #[doc = "0x580 - OTG host channel 4 characteristics register"]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    #[doc = "0x584 - OTG host channel 4 split control register"]
    #[inline(always)]
    pub const fn hcsplt4(&self) -> &HCSPLT4 {
        &self.hcsplt4
    }
    #[doc = "0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    #[doc = "0x58c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    #[doc = "0x590 - OTG host channel 4 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    #[doc = "0x594 - OTG host channel 4 DMA address register"]
    #[inline(always)]
    pub const fn hcdma4(&self) -> &HCDMA4 {
        &self.hcdma4
    }
    #[doc = "0x5a0 - OTG host channel 5 characteristics register"]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    #[doc = "0x5a4 - OTG host channel 5 split control register"]
    #[inline(always)]
    pub const fn hcsplt5(&self) -> &HCSPLT5 {
        &self.hcsplt5
    }
    #[doc = "0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    #[doc = "0x5ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    #[doc = "0x5b0 - OTG host channel 5 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    #[doc = "0x5b4 - OTG host channel 5 DMA address register"]
    #[inline(always)]
    pub const fn hcdma5(&self) -> &HCDMA5 {
        &self.hcdma5
    }
    #[doc = "0x5c0 - OTG host channel 6 characteristics register"]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    #[doc = "0x5c4 - OTG host channel 6 split control register"]
    #[inline(always)]
    pub const fn hcsplt6(&self) -> &HCSPLT6 {
        &self.hcsplt6
    }
    #[doc = "0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    #[doc = "0x5cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    #[doc = "0x5d0 - OTG host channel 6 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    #[doc = "0x5d4 - OTG host channel 6 DMA address register"]
    #[inline(always)]
    pub const fn hcdma6(&self) -> &HCDMA6 {
        &self.hcdma6
    }
    #[doc = "0x5e0 - OTG host channel 7 characteristics register"]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    #[doc = "0x5e4 - OTG host channel 7 split control register"]
    #[inline(always)]
    pub const fn hcsplt7(&self) -> &HCSPLT7 {
        &self.hcsplt7
    }
    #[doc = "0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    #[doc = "0x5ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    #[doc = "0x5f0 - OTG host channel 7 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
    #[doc = "0x5f4 - OTG host channel 7 DMA address register"]
    #[inline(always)]
    pub const fn hcdma7(&self) -> &HCDMA7 {
        &self.hcdma7
    }
    #[doc = "0x600 - OTG host channel 8 characteristics register"]
    #[inline(always)]
    pub const fn hcchar8(&self) -> &HCCHAR8 {
        &self.hcchar8
    }
    #[doc = "0x604 - OTG host channel 8 split control register"]
    #[inline(always)]
    pub const fn hcsplt8(&self) -> &HCSPLT8 {
        &self.hcsplt8
    }
    #[doc = "0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint8(&self) -> &HCINT8 {
        &self.hcint8
    }
    #[doc = "0x60c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &HCINTMSK8 {
        &self.hcintmsk8
    }
    #[doc = "0x610 - OTG host channel 8 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &HCTSIZ8 {
        &self.hctsiz8
    }
    #[doc = "0x614 - OTG host channel 8 DMA address register"]
    #[inline(always)]
    pub const fn hcdma8(&self) -> &HCDMA8 {
        &self.hcdma8
    }
    #[doc = "0x620 - OTG host channel 9 characteristics register"]
    #[inline(always)]
    pub const fn hcchar9(&self) -> &HCCHAR9 {
        &self.hcchar9
    }
    #[doc = "0x624 - OTG host channel 9 split control register"]
    #[inline(always)]
    pub const fn hcsplt9(&self) -> &HCSPLT9 {
        &self.hcsplt9
    }
    #[doc = "0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint9(&self) -> &HCINT9 {
        &self.hcint9
    }
    #[doc = "0x62c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &HCINTMSK9 {
        &self.hcintmsk9
    }
    #[doc = "0x630 - OTG host channel 9 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &HCTSIZ9 {
        &self.hctsiz9
    }
    #[doc = "0x634 - OTG host channel 9 DMA address register"]
    #[inline(always)]
    pub const fn hcdma9(&self) -> &HCDMA9 {
        &self.hcdma9
    }
    #[doc = "0x640 - OTG host channel 10 characteristics register"]
    #[inline(always)]
    pub const fn hcchar10(&self) -> &HCCHAR10 {
        &self.hcchar10
    }
    #[doc = "0x644 - OTG host channel 10 split control register"]
    #[inline(always)]
    pub const fn hcsplt10(&self) -> &HCSPLT10 {
        &self.hcsplt10
    }
    #[doc = "0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint10(&self) -> &HCINT10 {
        &self.hcint10
    }
    #[doc = "0x64c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &HCINTMSK10 {
        &self.hcintmsk10
    }
    #[doc = "0x650 - OTG host channel 10 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &HCTSIZ10 {
        &self.hctsiz10
    }
    #[doc = "0x654 - OTG host channel 10 DMA address register"]
    #[inline(always)]
    pub const fn hcdma10(&self) -> &HCDMA10 {
        &self.hcdma10
    }
    #[doc = "0x660 - OTG host channel 11 characteristics register"]
    #[inline(always)]
    pub const fn hcchar11(&self) -> &HCCHAR11 {
        &self.hcchar11
    }
    #[doc = "0x664 - OTG host channel 11 split control register"]
    #[inline(always)]
    pub const fn hcsplt11(&self) -> &HCSPLT11 {
        &self.hcsplt11
    }
    #[doc = "0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint11(&self) -> &HCINT11 {
        &self.hcint11
    }
    #[doc = "0x66c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &HCINTMSK11 {
        &self.hcintmsk11
    }
    #[doc = "0x670 - OTG host channel 11 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &HCTSIZ11 {
        &self.hctsiz11
    }
    #[doc = "0x674 - OTG host channel 11 DMA address register"]
    #[inline(always)]
    pub const fn hcdma11(&self) -> &HCDMA11 {
        &self.hcdma11
    }
    #[doc = "0x680 - OTG host channel 12 characteristics register"]
    #[inline(always)]
    pub const fn hcchar12(&self) -> &HCCHAR12 {
        &self.hcchar12
    }
    #[doc = "0x684 - OTG host channel 0 split control register"]
    #[inline(always)]
    pub const fn hcsplt12(&self) -> &HCSPLT12 {
        &self.hcsplt12
    }
    #[doc = "0x688 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint12(&self) -> &HCINT12 {
        &self.hcint12
    }
    #[doc = "0x68c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk12(&self) -> &HCINTMSK12 {
        &self.hcintmsk12
    }
    #[doc = "0x690 - OTG host channel 12 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz12(&self) -> &HCTSIZ12 {
        &self.hctsiz12
    }
    #[doc = "0x694 - OTG host channel 12 DMA address register"]
    #[inline(always)]
    pub const fn hcdma12(&self) -> &HCDMA12 {
        &self.hcdma12
    }
    #[doc = "0x6a0 - OTG host channel 13 characteristics register"]
    #[inline(always)]
    pub const fn hcchar13(&self) -> &HCCHAR13 {
        &self.hcchar13
    }
    #[doc = "0x6a4 - OTG host channel 13 split control register"]
    #[inline(always)]
    pub const fn hcsplt13(&self) -> &HCSPLT13 {
        &self.hcsplt13
    }
    #[doc = "0x6a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint13(&self) -> &HCINT13 {
        &self.hcint13
    }
    #[doc = "0x6ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk13(&self) -> &HCINTMSK13 {
        &self.hcintmsk13
    }
    #[doc = "0x6b0 - OTG host channel 13 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz13(&self) -> &HCTSIZ13 {
        &self.hctsiz13
    }
    #[doc = "0x6b4 - OTG host channel 13 DMA address register"]
    #[inline(always)]
    pub const fn hcdma13(&self) -> &HCDMA13 {
        &self.hcdma13
    }
    #[doc = "0x6c0 - OTG host channel 14 characteristics register"]
    #[inline(always)]
    pub const fn hcchar14(&self) -> &HCCHAR14 {
        &self.hcchar14
    }
    #[doc = "0x6c4 - OTG host channel 14 split control register"]
    #[inline(always)]
    pub const fn hcsplt14(&self) -> &HCSPLT14 {
        &self.hcsplt14
    }
    #[doc = "0x6c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint14(&self) -> &HCINT14 {
        &self.hcint14
    }
    #[doc = "0x6cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk14(&self) -> &HCINTMSK14 {
        &self.hcintmsk14
    }
    #[doc = "0x6d0 - OTG host channel 14 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz14(&self) -> &HCTSIZ14 {
        &self.hctsiz14
    }
    #[doc = "0x6d4 - OTG host channel 14 DMA address register"]
    #[inline(always)]
    pub const fn hcdma14(&self) -> &HCDMA14 {
        &self.hcdma14
    }
    #[doc = "0x6e0 - OTG host channel 15 characteristics register"]
    #[inline(always)]
    pub const fn hcchar15(&self) -> &HCCHAR15 {
        &self.hcchar15
    }
    #[doc = "0x6e4 - OTG host channel 15 split control register"]
    #[inline(always)]
    pub const fn hcsplt15(&self) -> &HCSPLT15 {
        &self.hcsplt15
    }
    #[doc = "0x6e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint15(&self) -> &HCINT15 {
        &self.hcint15
    }
    #[doc = "0x6ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk15(&self) -> &HCINTMSK15 {
        &self.hcintmsk15
    }
    #[doc = "0x6f0 - OTG host channel 15 transfer size register"]
    #[inline(always)]
    pub const fn hctsiz15(&self) -> &HCTSIZ15 {
        &self.hctsiz15
    }
    #[doc = "0x6f4 - OTG host channel 15 DMA address register"]
    #[inline(always)]
    pub const fn hcdma15(&self) -> &HCDMA15 {
        &self.hcdma15
    }
    #[doc = "0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x804 - OTG device control register"]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register."]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x810 - This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x814 - This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x818 - When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx)."]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x81c - The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set."]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x82c - This register specifies the VBUS pulsing time during SRP."]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x830 - OTG device threshold control register"]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx)."]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x884 - OTG device each OUT endpoint-1 interrupt mask register"]
    #[inline(always)]
    pub const fn hs_doepeachmsk1(&self) -> &HS_DOEPEACHMSK1 {
        &self.hs_doepeachmsk1
    }
    #[doc = "0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    #[doc = "0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    #[doc = "0x910 - The application must modify this register before enabling endpoint 0."]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    #[doc = "0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    #[doc = "0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    #[doc = "0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    #[doc = "0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    #[doc = "0x934 - OTG device IN endpoint 1 DMA address register"]
    #[inline(always)]
    pub const fn diepdma1(&self) -> &DIEPDMA1 {
        &self.diepdma1
    }
    #[doc = "0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    #[doc = "0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    #[doc = "0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    #[doc = "0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    #[doc = "0x954 - OTG device IN endpoint 2 DMA address register"]
    #[inline(always)]
    pub const fn diepdma2(&self) -> &DIEPDMA2 {
        &self.diepdma2
    }
    #[doc = "0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    #[doc = "0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    #[doc = "0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    #[doc = "0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    #[doc = "0x974 - OTG device IN endpoint 3 DMA address register"]
    #[inline(always)]
    pub const fn diepdma3(&self) -> &DIEPDMA3 {
        &self.diepdma3
    }
    #[doc = "0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    #[doc = "0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &DIEPCTL4 {
        &self.diepctl4
    }
    #[doc = "0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint4(&self) -> &DIEPINT4 {
        &self.diepint4
    }
    #[doc = "0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &DIEPTSIZ4 {
        &self.dieptsiz4
    }
    #[doc = "0x994 - OTG device IN endpoint 4 DMA address register"]
    #[inline(always)]
    pub const fn diepdma4(&self) -> &DIEPDMA4 {
        &self.diepdma4
    }
    #[doc = "0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &DTXFSTS4 {
        &self.dtxfsts4
    }
    #[doc = "0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &DIEPCTL5 {
        &self.diepctl5
    }
    #[doc = "0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint5(&self) -> &DIEPINT5 {
        &self.diepint5
    }
    #[doc = "0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &DIEPTSIZ5 {
        &self.dieptsiz5
    }
    #[doc = "0x9b4 - OTG device IN endpoint 5 DMA address register"]
    #[inline(always)]
    pub const fn diepdma5(&self) -> &DIEPDMA5 {
        &self.diepdma5
    }
    #[doc = "0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &DTXFSTS5 {
        &self.dtxfsts5
    }
    #[doc = "0x9c8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint6(&self) -> &DIEPINT6 {
        &self.diepint6
    }
    #[doc = "0x9d0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz6(&self) -> &DIEPTSIZ6 {
        &self.dieptsiz6
    }
    #[doc = "0x9d4 - OTG device IN endpoint 6 DMA address register"]
    #[inline(always)]
    pub const fn diepdma6(&self) -> &DIEPDMA6 {
        &self.diepdma6
    }
    #[doc = "0x9e8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint7(&self) -> &DIEPINT7 {
        &self.diepint7
    }
    #[doc = "0x9f0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz7(&self) -> &DIEPTSIZ7 {
        &self.dieptsiz7
    }
    #[doc = "0x9f4 - OTG device IN endpoint 7 DMA address register"]
    #[inline(always)]
    pub const fn diepdma7(&self) -> &DIEPDMA7 {
        &self.diepdma7
    }
    #[doc = "0xa08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn diepint8(&self) -> &DIEPINT8 {
        &self.diepint8
    }
    #[doc = "0xa10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn dieptsiz8(&self) -> &DIEPTSIZ8 {
        &self.dieptsiz8
    }
    #[doc = "0xa14 - OTG device IN endpoint 8 DMA address register"]
    #[inline(always)]
    pub const fn diepdma8(&self) -> &DIEPDMA8 {
        &self.diepdma8
    }
    #[doc = "0xb00 - This section describes the DOEPCTL0 register."]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    #[doc = "0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    #[doc = "0xb10 - The application must modify this register before enabling endpoint 0."]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    #[doc = "0xb14 - OTG device OUT endpoint 0 DMA address register"]
    #[inline(always)]
    pub const fn doepdma0(&self) -> &DOEPDMA0 {
        &self.doepdma0
    }
    #[doc = "0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    #[doc = "0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    #[doc = "0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    #[doc = "0xb34 - OTG device OUT endpoint 1 DMA address register"]
    #[inline(always)]
    pub const fn doepdma1(&self) -> &DOEPDMA1 {
        &self.doepdma1
    }
    #[doc = "0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    #[doc = "0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    #[doc = "0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    #[doc = "0xb54 - OTG device OUT endpoint 2 DMA address register"]
    #[inline(always)]
    pub const fn doepdma2(&self) -> &DOEPDMA2 {
        &self.doepdma2
    }
    #[doc = "0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    #[doc = "0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    #[doc = "0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
    #[doc = "0xb74 - OTG device OUT endpoint 3 DMA address register"]
    #[inline(always)]
    pub const fn doepdma3(&self) -> &DOEPDMA3 {
        &self.doepdma3
    }
    #[doc = "0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &DOEPCTL4 {
        &self.doepctl4
    }
    #[doc = "0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint4(&self) -> &DOEPINT4 {
        &self.doepint4
    }
    #[doc = "0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &DOEPTSIZ4 {
        &self.doeptsiz4
    }
    #[doc = "0xb94 - OTG device OUT endpoint 4 DMA address register"]
    #[inline(always)]
    pub const fn doepdma4(&self) -> &DOEPDMA4 {
        &self.doepdma4
    }
    #[doc = "0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &DOEPCTL5 {
        &self.doepctl5
    }
    #[doc = "0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint5(&self) -> &DOEPINT5 {
        &self.doepint5
    }
    #[doc = "0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &DOEPTSIZ5 {
        &self.doeptsiz5
    }
    #[doc = "0xbb4 - OTG device OUT endpoint 5 DMA address register"]
    #[inline(always)]
    pub const fn doepdma5(&self) -> &DOEPDMA5 {
        &self.doepdma5
    }
    #[doc = "0xbc0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl6(&self) -> &DOEPCTL6 {
        &self.doepctl6
    }
    #[doc = "0xbc8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint6(&self) -> &DOEPINT6 {
        &self.doepint6
    }
    #[doc = "0xbd0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz6(&self) -> &DOEPTSIZ6 {
        &self.doeptsiz6
    }
    #[doc = "0xbd4 - OTG device OUT endpoint 6 DMA address register"]
    #[inline(always)]
    pub const fn doepdma6(&self) -> &DOEPDMA6 {
        &self.doepdma6
    }
    #[doc = "0xbe0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl7(&self) -> &DOEPCTL7 {
        &self.doepctl7
    }
    #[doc = "0xbe8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint7(&self) -> &DOEPINT7 {
        &self.doepint7
    }
    #[doc = "0xbf0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz7(&self) -> &DOEPTSIZ7 {
        &self.doeptsiz7
    }
    #[doc = "0xbf4 - OTG device OUT endpoint 7 DMA address register"]
    #[inline(always)]
    pub const fn doepdma7(&self) -> &DOEPDMA7 {
        &self.doepdma7
    }
    #[doc = "0xc00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    #[inline(always)]
    pub const fn doepctl8(&self) -> &DOEPCTL8 {
        &self.doepctl8
    }
    #[doc = "0xc08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn doepint8(&self) -> &DOEPINT8 {
        &self.doepint8
    }
    #[doc = "0xc10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    #[inline(always)]
    pub const fn doeptsiz8(&self) -> &DOEPTSIZ8 {
        &self.doeptsiz8
    }
    #[doc = "0xc14 - OTG device OUT endpoint 8 DMA address register"]
    #[inline(always)]
    pub const fn doepdma8(&self) -> &DOEPDMA8 {
        &self.doepdma8
    }
    #[doc = "0xe00 - This register is available in host and device modes."]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "GOTGCTL (rw) register accessor: The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`]
module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTLrs>;
#[doc = "The GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`]
module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINTrs>;
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`]
module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFGrs>;
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`]
module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFGrs>;
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: The application uses this register to reset various hardware features inside the core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`]
module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTLrs>;
#[doc = "The application uses this register to reset various hardware features inside the core."]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`]
module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTSrs>;
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`]
module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSKrs>;
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (GINTSTS) register bit corresponding to that interrupt is still set."]
pub mod gintmsk;
#[doc = "GRXSTSR_DEVICE (r) register accessor: This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_device`]
module"]
pub type GRXSTSR_DEVICE = crate::Reg<grxstsr_device::GRXSTSR_DEVICErs>;
#[doc = "This description is for register GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
pub mod grxstsr_device;
#[doc = "GRXSTSR_HOST (r) register accessor: This description is for register GRXSTSR in Host mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`]
module"]
pub type GRXSTSR_HOST = crate::Reg<grxstsr_host::GRXSTSR_HOSTrs>;
#[doc = "This description is for register GRXSTSR in Host mode"]
pub mod grxstsr_host;
#[doc = "GRXSTSP_DEVICE (r) register accessor: This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_device`]
module"]
pub type GRXSTSP_DEVICE = crate::Reg<grxstsp_device::GRXSTSP_DEVICErs>;
#[doc = "This description is for register GRXSTSP in Device mode. Similarly to GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in GINTSTS) is asserted."]
pub mod grxstsp_device;
#[doc = "GRXSTSP_HOST (r) register accessor: This description is for register GRXSTSP in HOST mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_host`]
module"]
pub type GRXSTSP_HOST = crate::Reg<grxstsp_host::GRXSTSP_HOSTrs>;
#[doc = "This description is for register GRXSTSP in HOST mode"]
pub mod grxstsp_host;
#[doc = "GRXFSIZ (rw) register accessor: The application can program the RAM size that must be allocated to the Rx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`]
module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZrs>;
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO."]
pub mod grxfsiz;
#[doc = "HNPTXFSIZ (rw) register accessor: Host mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxfsiz`]
module"]
pub type HNPTXFSIZ = crate::Reg<hnptxfsiz::HNPTXFSIZrs>;
#[doc = "Host mode"]
pub mod hnptxfsiz;
#[doc = "HNPTXSTS (r) register accessor: In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxsts`]
module"]
pub type HNPTXSTS = crate::Reg<hnptxsts::HNPTXSTSrs>;
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
pub mod hnptxsts;
#[doc = "GCCFG (rw) register accessor: OTG general core configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`]
module"]
pub type GCCFG = crate::Reg<gccfg::GCCFGrs>;
#[doc = "OTG general core configuration register"]
pub mod gccfg;
#[doc = "CID (rw) register accessor: This is a register containing the Product ID as reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`]
module"]
pub type CID = crate::Reg<cid::CIDrs>;
#[doc = "This is a register containing the Product ID as reset value."]
pub mod cid;
#[doc = "GLPMCFG (rw) register accessor: OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glpmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glpmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glpmcfg`]
module"]
pub type GLPMCFG = crate::Reg<glpmcfg::GLPMCFGrs>;
#[doc = "OTG core LPM configuration register"]
pub mod glpmcfg;
#[doc = "HPTXFSIZ (rw) register accessor: OTG host periodic transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`]
module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZrs>;
#[doc = "OTG host periodic transmit FIFO size register"]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: OTG device IN endpoint transmit FIFO 1 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`]
module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1rs>;
#[doc = "OTG device IN endpoint transmit FIFO 1 size register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: OTG device IN endpoint transmit FIFO 2 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`]
module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2rs>;
#[doc = "OTG device IN endpoint transmit FIFO 2 size register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: OTG device IN endpoint transmit FIFO 3 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`]
module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3rs>;
#[doc = "OTG device IN endpoint transmit FIFO 3 size register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: OTG device IN endpoint transmit FIFO 4 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`]
module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4rs>;
#[doc = "OTG device IN endpoint transmit FIFO 4 size register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: OTG device IN endpoint transmit FIFO 5 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`]
module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5rs>;
#[doc = "OTG device IN endpoint transmit FIFO 5 size register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: OTG device IN endpoint transmit FIFO 6 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`]
module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6rs>;
#[doc = "OTG device IN endpoint transmit FIFO 6 size register"]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: OTG device IN endpoint transmit FIFO 7 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf7`]
module"]
pub type DIEPTXF7 = crate::Reg<dieptxf7::DIEPTXF7rs>;
#[doc = "OTG device IN endpoint transmit FIFO 7 size register"]
pub mod dieptxf7;
#[doc = "DIEPTXF8 (rw) register accessor: OTG device IN endpoint transmit FIFO 8 size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf8`]
module"]
pub type DIEPTXF8 = crate::Reg<dieptxf8::DIEPTXF8rs>;
#[doc = "OTG device IN endpoint transmit FIFO 8 size register"]
pub mod dieptxf8;
#[doc = "HCFG (rw) register accessor: This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`]
module"]
pub type HCFG = crate::Reg<hcfg::HCFGrs>;
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host."]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: This register stores the frame interval information for the current speed to which the OTG controller has enumerated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIRrs>;
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`]
module"]
pub type HFNUM = crate::Reg<hfnum::HFNUMrs>;
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`]
module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTSrs>;
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINTrs>;
#[doc = "When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSKrs>;
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
pub mod haintmsk;
#[doc = "HPRT (rw) register accessor: This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`]
module"]
pub type HPRT = crate::Reg<hprt::HPRTrs>;
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: OTG host channel 0 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`]
module"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0rs>;
#[doc = "OTG host channel 0 characteristics register"]
pub mod hcchar0;
#[doc = "HCSPLT0 (rw) register accessor: OTG host channel 0 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt0`]
module"]
pub type HCSPLT0 = crate::Reg<hcsplt0::HCSPLT0rs>;
#[doc = "OTG host channel 0 split control register"]
pub mod hcsplt0;
#[doc = "HCSPLT1 (rw) register accessor: OTG host channel 1 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt1`]
module"]
pub type HCSPLT1 = crate::Reg<hcsplt1::HCSPLT1rs>;
#[doc = "OTG host channel 1 split control register"]
pub mod hcsplt1;
#[doc = "HCSPLT2 (rw) register accessor: OTG host channel 2 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt2`]
module"]
pub type HCSPLT2 = crate::Reg<hcsplt2::HCSPLT2rs>;
#[doc = "OTG host channel 2 split control register"]
pub mod hcsplt2;
#[doc = "HCSPLT3 (rw) register accessor: OTG host channel 3 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt3`]
module"]
pub type HCSPLT3 = crate::Reg<hcsplt3::HCSPLT3rs>;
#[doc = "OTG host channel 3 split control register"]
pub mod hcsplt3;
#[doc = "HCSPLT4 (rw) register accessor: OTG host channel 4 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt4`]
module"]
pub type HCSPLT4 = crate::Reg<hcsplt4::HCSPLT4rs>;
#[doc = "OTG host channel 4 split control register"]
pub mod hcsplt4;
#[doc = "HCSPLT5 (rw) register accessor: OTG host channel 5 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt5`]
module"]
pub type HCSPLT5 = crate::Reg<hcsplt5::HCSPLT5rs>;
#[doc = "OTG host channel 5 split control register"]
pub mod hcsplt5;
#[doc = "HCSPLT6 (rw) register accessor: OTG host channel 6 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt6`]
module"]
pub type HCSPLT6 = crate::Reg<hcsplt6::HCSPLT6rs>;
#[doc = "OTG host channel 6 split control register"]
pub mod hcsplt6;
#[doc = "HCSPLT7 (rw) register accessor: OTG host channel 7 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt7`]
module"]
pub type HCSPLT7 = crate::Reg<hcsplt7::HCSPLT7rs>;
#[doc = "OTG host channel 7 split control register"]
pub mod hcsplt7;
#[doc = "HCSPLT8 (rw) register accessor: OTG host channel 8 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt8`]
module"]
pub type HCSPLT8 = crate::Reg<hcsplt8::HCSPLT8rs>;
#[doc = "OTG host channel 8 split control register"]
pub mod hcsplt8;
#[doc = "HCSPLT9 (rw) register accessor: OTG host channel 9 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt9`]
module"]
pub type HCSPLT9 = crate::Reg<hcsplt9::HCSPLT9rs>;
#[doc = "OTG host channel 9 split control register"]
pub mod hcsplt9;
#[doc = "HCSPLT10 (rw) register accessor: OTG host channel 10 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt10`]
module"]
pub type HCSPLT10 = crate::Reg<hcsplt10::HCSPLT10rs>;
#[doc = "OTG host channel 10 split control register"]
pub mod hcsplt10;
#[doc = "HCSPLT11 (rw) register accessor: OTG host channel 11 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt11`]
module"]
pub type HCSPLT11 = crate::Reg<hcsplt11::HCSPLT11rs>;
#[doc = "OTG host channel 11 split control register"]
pub mod hcsplt11;
#[doc = "HCSPLT12 (rw) register accessor: OTG host channel 0 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt12`]
module"]
pub type HCSPLT12 = crate::Reg<hcsplt12::HCSPLT12rs>;
#[doc = "OTG host channel 0 split control register"]
pub mod hcsplt12;
#[doc = "HCSPLT13 (rw) register accessor: OTG host channel 13 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt13`]
module"]
pub type HCSPLT13 = crate::Reg<hcsplt13::HCSPLT13rs>;
#[doc = "OTG host channel 13 split control register"]
pub mod hcsplt13;
#[doc = "HCSPLT14 (rw) register accessor: OTG host channel 14 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt14`]
module"]
pub type HCSPLT14 = crate::Reg<hcsplt14::HCSPLT14rs>;
#[doc = "OTG host channel 14 split control register"]
pub mod hcsplt14;
#[doc = "HCSPLT15 (rw) register accessor: OTG host channel 15 split control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcsplt15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcsplt15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt15`]
module"]
pub type HCSPLT15 = crate::Reg<hcsplt15::HCSPLT15rs>;
#[doc = "OTG host channel 15 split control register"]
pub mod hcsplt15;
#[doc = "HCINT0 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`]
module"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint0;
#[doc = "HCINTMSK0 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`]
module"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk0;
#[doc = "HCTSIZ0 (rw) register accessor: OTG host channel 0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`]
module"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0rs>;
#[doc = "OTG host channel 0 transfer size register"]
pub mod hctsiz0;
#[doc = "HCDMA0 (rw) register accessor: OTG host channel 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma0`]
module"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0rs>;
#[doc = "OTG host channel 0 DMA address register"]
pub mod hcdma0;
#[doc = "HCDMA1 (rw) register accessor: OTG host channel 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma1`]
module"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1rs>;
#[doc = "OTG host channel 1 DMA address register"]
pub mod hcdma1;
#[doc = "HCDMA2 (rw) register accessor: OTG host channel 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma2`]
module"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2rs>;
#[doc = "OTG host channel 2 DMA address register"]
pub mod hcdma2;
#[doc = "HCDMA3 (rw) register accessor: OTG host channel 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma3`]
module"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3rs>;
#[doc = "OTG host channel 3 DMA address register"]
pub mod hcdma3;
#[doc = "HCDMA4 (rw) register accessor: OTG host channel 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma4`]
module"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4rs>;
#[doc = "OTG host channel 4 DMA address register"]
pub mod hcdma4;
#[doc = "HCDMA5 (rw) register accessor: OTG host channel 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma5`]
module"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5rs>;
#[doc = "OTG host channel 5 DMA address register"]
pub mod hcdma5;
#[doc = "HCDMA6 (rw) register accessor: OTG host channel 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma6`]
module"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6rs>;
#[doc = "OTG host channel 6 DMA address register"]
pub mod hcdma6;
#[doc = "HCDMA7 (rw) register accessor: OTG host channel 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma7`]
module"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7rs>;
#[doc = "OTG host channel 7 DMA address register"]
pub mod hcdma7;
#[doc = "HCDMA8 (rw) register accessor: OTG host channel 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma8`]
module"]
pub type HCDMA8 = crate::Reg<hcdma8::HCDMA8rs>;
#[doc = "OTG host channel 8 DMA address register"]
pub mod hcdma8;
#[doc = "HCDMA9 (rw) register accessor: OTG host channel 9 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma9`]
module"]
pub type HCDMA9 = crate::Reg<hcdma9::HCDMA9rs>;
#[doc = "OTG host channel 9 DMA address register"]
pub mod hcdma9;
#[doc = "HCDMA10 (rw) register accessor: OTG host channel 10 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma10`]
module"]
pub type HCDMA10 = crate::Reg<hcdma10::HCDMA10rs>;
#[doc = "OTG host channel 10 DMA address register"]
pub mod hcdma10;
#[doc = "HCDMA11 (rw) register accessor: OTG host channel 11 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma11`]
module"]
pub type HCDMA11 = crate::Reg<hcdma11::HCDMA11rs>;
#[doc = "OTG host channel 11 DMA address register"]
pub mod hcdma11;
#[doc = "HCDMA12 (rw) register accessor: OTG host channel 12 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma12`]
module"]
pub type HCDMA12 = crate::Reg<hcdma12::HCDMA12rs>;
#[doc = "OTG host channel 12 DMA address register"]
pub mod hcdma12;
#[doc = "HCDMA13 (rw) register accessor: OTG host channel 13 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma13`]
module"]
pub type HCDMA13 = crate::Reg<hcdma13::HCDMA13rs>;
#[doc = "OTG host channel 13 DMA address register"]
pub mod hcdma13;
#[doc = "HCDMA14 (rw) register accessor: OTG host channel 14 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma14`]
module"]
pub type HCDMA14 = crate::Reg<hcdma14::HCDMA14rs>;
#[doc = "OTG host channel 14 DMA address register"]
pub mod hcdma14;
#[doc = "HCDMA15 (rw) register accessor: OTG host channel 15 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcdma15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcdma15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma15`]
module"]
pub type HCDMA15 = crate::Reg<hcdma15::HCDMA15rs>;
#[doc = "OTG host channel 15 DMA address register"]
pub mod hcdma15;
#[doc = "HCCHAR1 (rw) register accessor: OTG host channel 1 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`]
module"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1rs>;
#[doc = "OTG host channel 1 characteristics register"]
pub mod hcchar1;
#[doc = "HCINT1_DEVICE (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint1_device::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint1_device::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1_device`]
module"]
pub type HCINT1_DEVICE = crate::Reg<hcint1_device::HCINT1_DEVICErs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint1_device;
#[doc = "HCINTMSK1 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`]
module"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk1;
#[doc = "HCTSIZ1 (rw) register accessor: OTG host channel 1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`]
module"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1rs>;
#[doc = "OTG host channel 1 transfer size register"]
pub mod hctsiz1;
#[doc = "HCCHAR2 (rw) register accessor: OTG host channel 2 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`]
module"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2rs>;
#[doc = "OTG host channel 2 characteristics register"]
pub mod hcchar2;
#[doc = "HCINT2 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`]
module"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint2;
#[doc = "HCINTMSK2 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`]
module"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk2;
#[doc = "HCTSIZ2 (rw) register accessor: OTG host channel 2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`]
module"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2rs>;
#[doc = "OTG host channel 2 transfer size register"]
pub mod hctsiz2;
#[doc = "HCCHAR3 (rw) register accessor: OTG host channel 3 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`]
module"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3rs>;
#[doc = "OTG host channel 3 characteristics register"]
pub mod hcchar3;
#[doc = "HCINT3 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`]
module"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint3;
#[doc = "HCINTMSK3 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`]
module"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk3;
#[doc = "HCTSIZ3 (rw) register accessor: OTG host channel 3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`]
module"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3rs>;
#[doc = "OTG host channel 3 transfer size register"]
pub mod hctsiz3;
#[doc = "HCCHAR4 (rw) register accessor: OTG host channel 4 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`]
module"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4rs>;
#[doc = "OTG host channel 4 characteristics register"]
pub mod hcchar4;
#[doc = "HCINT4 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`]
module"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint4;
#[doc = "HCINTMSK4 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`]
module"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk4;
#[doc = "HCTSIZ4 (rw) register accessor: OTG host channel 4 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`]
module"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4rs>;
#[doc = "OTG host channel 4 transfer size register"]
pub mod hctsiz4;
#[doc = "HCCHAR5 (rw) register accessor: OTG host channel 5 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`]
module"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5rs>;
#[doc = "OTG host channel 5 characteristics register"]
pub mod hcchar5;
#[doc = "HCINT5 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`]
module"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint5;
#[doc = "HCINTMSK5 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`]
module"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk5;
#[doc = "HCTSIZ5 (rw) register accessor: OTG host channel 5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`]
module"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5rs>;
#[doc = "OTG host channel 5 transfer size register"]
pub mod hctsiz5;
#[doc = "HCCHAR6 (rw) register accessor: OTG host channel 6 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`]
module"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6rs>;
#[doc = "OTG host channel 6 characteristics register"]
pub mod hcchar6;
#[doc = "HCINT6 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`]
module"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint6;
#[doc = "HCINTMSK6 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`]
module"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk6;
#[doc = "HCTSIZ6 (rw) register accessor: OTG host channel 6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`]
module"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6rs>;
#[doc = "OTG host channel 6 transfer size register"]
pub mod hctsiz6;
#[doc = "HCCHAR7 (rw) register accessor: OTG host channel 7 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`]
module"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7rs>;
#[doc = "OTG host channel 7 characteristics register"]
pub mod hcchar7;
#[doc = "HCINT7 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`]
module"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint7;
#[doc = "HCINTMSK7 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`]
module"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk7;
#[doc = "HCTSIZ7 (rw) register accessor: OTG host channel 7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`]
module"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7rs>;
#[doc = "OTG host channel 7 transfer size register"]
pub mod hctsiz7;
#[doc = "HCCHAR8 (rw) register accessor: OTG host channel 8 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar8`]
module"]
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8rs>;
#[doc = "OTG host channel 8 characteristics register"]
pub mod hcchar8;
#[doc = "HCINT8 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint8`]
module"]
pub type HCINT8 = crate::Reg<hcint8::HCINT8rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint8;
#[doc = "HCINTMSK8 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk8`]
module"]
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk8;
#[doc = "HCTSIZ8 (rw) register accessor: OTG host channel 8 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz8`]
module"]
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8rs>;
#[doc = "OTG host channel 8 transfer size register"]
pub mod hctsiz8;
#[doc = "HCCHAR9 (rw) register accessor: OTG host channel 9 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar9`]
module"]
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9rs>;
#[doc = "OTG host channel 9 characteristics register"]
pub mod hcchar9;
#[doc = "HCINT9 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint9`]
module"]
pub type HCINT9 = crate::Reg<hcint9::HCINT9rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint9;
#[doc = "HCINTMSK9 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk9`]
module"]
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk9;
#[doc = "HCTSIZ9 (rw) register accessor: OTG host channel 9 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz9`]
module"]
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9rs>;
#[doc = "OTG host channel 9 transfer size register"]
pub mod hctsiz9;
#[doc = "HCCHAR10 (rw) register accessor: OTG host channel 10 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar10`]
module"]
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10rs>;
#[doc = "OTG host channel 10 characteristics register"]
pub mod hcchar10;
#[doc = "HCINT10 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint10`]
module"]
pub type HCINT10 = crate::Reg<hcint10::HCINT10rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint10;
#[doc = "HCINTMSK10 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk10`]
module"]
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk10;
#[doc = "HCTSIZ10 (rw) register accessor: OTG host channel 10 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz10`]
module"]
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10rs>;
#[doc = "OTG host channel 10 transfer size register"]
pub mod hctsiz10;
#[doc = "HCCHAR11 (rw) register accessor: OTG host channel 11 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar11`]
module"]
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11rs>;
#[doc = "OTG host channel 11 characteristics register"]
pub mod hcchar11;
#[doc = "HCCHAR12 (rw) register accessor: OTG host channel 12 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar12`]
module"]
pub type HCCHAR12 = crate::Reg<hcchar12::HCCHAR12rs>;
#[doc = "OTG host channel 12 characteristics register"]
pub mod hcchar12;
#[doc = "HCCHAR13 (rw) register accessor: OTG host channel 13 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar13`]
module"]
pub type HCCHAR13 = crate::Reg<hcchar13::HCCHAR13rs>;
#[doc = "OTG host channel 13 characteristics register"]
pub mod hcchar13;
#[doc = "HCCHAR14 (rw) register accessor: OTG host channel 14 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar14`]
module"]
pub type HCCHAR14 = crate::Reg<hcchar14::HCCHAR14rs>;
#[doc = "OTG host channel 14 characteristics register"]
pub mod hcchar14;
#[doc = "HCCHAR15 (rw) register accessor: OTG host channel 15 characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcchar15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcchar15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar15`]
module"]
pub type HCCHAR15 = crate::Reg<hcchar15::HCCHAR15rs>;
#[doc = "OTG host channel 15 characteristics register"]
pub mod hcchar15;
#[doc = "HCINT11 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint11`]
module"]
pub type HCINT11 = crate::Reg<hcint11::HCINT11rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint11;
#[doc = "HCINT12 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint12`]
module"]
pub type HCINT12 = crate::Reg<hcint12::HCINT12rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint12;
#[doc = "HCINT13 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint13`]
module"]
pub type HCINT13 = crate::Reg<hcint13::HCINT13rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint13;
#[doc = "HCINT14 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint14`]
module"]
pub type HCINT14 = crate::Reg<hcint14::HCINT14rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint14;
#[doc = "HCINT15 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint15`]
module"]
pub type HCINT15 = crate::Reg<hcint15::HCINT15rs>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint15;
#[doc = "HCINTMSK11 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk11`]
module"]
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk11;
#[doc = "HCINTMSK12 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk12`]
module"]
pub type HCINTMSK12 = crate::Reg<hcintmsk12::HCINTMSK12rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk12;
#[doc = "HCINTMSK13 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk13`]
module"]
pub type HCINTMSK13 = crate::Reg<hcintmsk13::HCINTMSK13rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk13;
#[doc = "HCINTMSK14 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk14`]
module"]
pub type HCINTMSK14 = crate::Reg<hcintmsk14::HCINTMSK14rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk14;
#[doc = "HCINTMSK15 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk15`]
module"]
pub type HCINTMSK15 = crate::Reg<hcintmsk15::HCINTMSK15rs>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk15;
#[doc = "HCTSIZ11 (rw) register accessor: OTG host channel 11 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz11`]
module"]
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11rs>;
#[doc = "OTG host channel 11 transfer size register"]
pub mod hctsiz11;
#[doc = "HCTSIZ12 (rw) register accessor: OTG host channel 12 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz12`]
module"]
pub type HCTSIZ12 = crate::Reg<hctsiz12::HCTSIZ12rs>;
#[doc = "OTG host channel 12 transfer size register"]
pub mod hctsiz12;
#[doc = "HCTSIZ13 (rw) register accessor: OTG host channel 13 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz13`]
module"]
pub type HCTSIZ13 = crate::Reg<hctsiz13::HCTSIZ13rs>;
#[doc = "OTG host channel 13 transfer size register"]
pub mod hctsiz13;
#[doc = "HCTSIZ14 (rw) register accessor: OTG host channel 14 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz14`]
module"]
pub type HCTSIZ14 = crate::Reg<hctsiz14::HCTSIZ14rs>;
#[doc = "OTG host channel 14 transfer size register"]
pub mod hctsiz14;
#[doc = "HCTSIZ15 (rw) register accessor: OTG host channel 15 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz15`]
module"]
pub type HCTSIZ15 = crate::Reg<hctsiz15::HCTSIZ15rs>;
#[doc = "OTG host channel 15 transfer size register"]
pub mod hctsiz15;
#[doc = "DCFG (rw) register accessor: This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`]
module"]
pub type DCFG = crate::Reg<dcfg::DCFGrs>;
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: OTG device control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`]
module"]
pub type DCTL = crate::Reg<dctl::DCTLrs>;
#[doc = "OTG device control register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`]
module"]
pub type DSTS = crate::Reg<dsts::DSTSrs>;
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (DAINT) register."]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`]
module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSKrs>;
#[doc = "This register works with each of the DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`]
module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSKrs>;
#[doc = "This register works with each of the DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`]
module"]
pub type DAINT = crate::Reg<daint::DAINTrs>;
#[doc = "When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx)."]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`]
module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSKrs>;
#[doc = "The DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the DAINT register bit corresponding to that interrupt is still set."]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`]
module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDISrs>;
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: This register specifies the VBUS pulsing time during SRP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbuspulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`]
module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSErs>;
#[doc = "This register specifies the VBUS pulsing time during SRP."]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: OTG device threshold control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dthrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`]
module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTLrs>;
#[doc = "OTG device threshold control register"]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepempmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`]
module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSKrs>;
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_DIEPINTx)."]
pub mod diepempmsk;
#[doc = "HS_DOEPEACHMSK1 (rw) register accessor: OTG device each OUT endpoint-1 interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_doepeachmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_doepeachmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_doepeachmsk1`]
module"]
pub type HS_DOEPEACHMSK1 = crate::Reg<hs_doepeachmsk1::HS_DOEPEACHMSK1rs>;
#[doc = "OTG device each OUT endpoint-1 interrupt mask register"]
pub mod hs_doepeachmsk1;
#[doc = "DIEPCTL0 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`]
module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`]
module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`]
module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0rs>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod dieptsiz0;
#[doc = "DTXFSTS0 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`]
module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts0;
#[doc = "DIEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`]
module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl1;
#[doc = "DIEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`]
module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint1;
#[doc = "DIEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`]
module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 (rw) register accessor: OTG device IN endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma1`]
module"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1rs>;
#[doc = "OTG device IN endpoint 1 DMA address register"]
pub mod diepdma1;
#[doc = "DTXFSTS1 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`]
module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts1;
#[doc = "DIEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`]
module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl2;
#[doc = "DIEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`]
module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint2;
#[doc = "DIEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`]
module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 (rw) register accessor: OTG device IN endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma2`]
module"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2rs>;
#[doc = "OTG device IN endpoint 2 DMA address register"]
pub mod diepdma2;
#[doc = "DTXFSTS2 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`]
module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts2;
#[doc = "DIEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`]
module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl3;
#[doc = "DIEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`]
module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint3;
#[doc = "DIEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`]
module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 (rw) register accessor: OTG device IN endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma3`]
module"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3rs>;
#[doc = "OTG device IN endpoint 3 DMA address register"]
pub mod diepdma3;
#[doc = "DTXFSTS3 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`]
module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts3;
#[doc = "DIEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`]
module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl4;
#[doc = "DIEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`]
module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint4;
#[doc = "DIEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`]
module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 (rw) register accessor: OTG device IN endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma4`]
module"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4rs>;
#[doc = "OTG device IN endpoint 4 DMA address register"]
pub mod diepdma4;
#[doc = "DTXFSTS4 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`]
module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts4;
#[doc = "DIEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`]
module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod diepctl5;
#[doc = "DIEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`]
module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint5;
#[doc = "DIEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`]
module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz5;
#[doc = "DIEPDMA5 (rw) register accessor: OTG device IN endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma5`]
module"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5rs>;
#[doc = "OTG device IN endpoint 5 DMA address register"]
pub mod diepdma5;
#[doc = "DTXFSTS5 (r) register accessor: This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`]
module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5rs>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod dtxfsts5;
#[doc = "DIEPINT6 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint6`]
module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint6;
#[doc = "DIEPTSIZ6 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz6`]
module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz6;
#[doc = "DIEPDMA6 (rw) register accessor: OTG device IN endpoint 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma6`]
module"]
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6rs>;
#[doc = "OTG device IN endpoint 6 DMA address register"]
pub mod diepdma6;
#[doc = "DIEPINT7 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint7`]
module"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint7;
#[doc = "DIEPTSIZ7 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz7`]
module"]
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz7;
#[doc = "DIEPDMA7 (rw) register accessor: OTG device IN endpoint 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma7`]
module"]
pub type DIEPDMA7 = crate::Reg<diepdma7::DIEPDMA7rs>;
#[doc = "OTG device IN endpoint 7 DMA address register"]
pub mod diepdma7;
#[doc = "DIEPINT8 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint8`]
module"]
pub type DIEPINT8 = crate::Reg<diepint8::DIEPINT8rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod diepint8;
#[doc = "DIEPTSIZ8 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz8`]
module"]
pub type DIEPTSIZ8 = crate::Reg<dieptsiz8::DIEPTSIZ8rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod dieptsiz8;
#[doc = "DIEPDMA8 (rw) register accessor: OTG device IN endpoint 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma8`]
module"]
pub type DIEPDMA8 = crate::Reg<diepdma8::DIEPDMA8rs>;
#[doc = "OTG device IN endpoint 8 DMA address register"]
pub mod diepdma8;
#[doc = "DOEPCTL0 (rw) register accessor: This section describes the DOEPCTL0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`]
module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0rs>;
#[doc = "This section describes the DOEPCTL0 register."]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`]
module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`]
module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0rs>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: OTG device OUT endpoint 0 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma0`]
module"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0rs>;
#[doc = "OTG device OUT endpoint 0 DMA address register"]
pub mod doepdma0;
#[doc = "DOEPCTL1 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`]
module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl1;
#[doc = "DOEPINT1 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`]
module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint1;
#[doc = "DOEPTSIZ1 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`]
module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 (rw) register accessor: OTG device OUT endpoint 1 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma1`]
module"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1rs>;
#[doc = "OTG device OUT endpoint 1 DMA address register"]
pub mod doepdma1;
#[doc = "DOEPCTL2 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`]
module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl2;
#[doc = "DOEPINT2 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`]
module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint2;
#[doc = "DOEPTSIZ2 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`]
module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 (rw) register accessor: OTG device OUT endpoint 2 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma2`]
module"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2rs>;
#[doc = "OTG device OUT endpoint 2 DMA address register"]
pub mod doepdma2;
#[doc = "DOEPCTL3 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`]
module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl3;
#[doc = "DOEPINT3 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`]
module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint3;
#[doc = "DOEPTSIZ3 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`]
module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 (rw) register accessor: OTG device OUT endpoint 3 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma3`]
module"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3rs>;
#[doc = "OTG device OUT endpoint 3 DMA address register"]
pub mod doepdma3;
#[doc = "DOEPCTL4 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`]
module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl4;
#[doc = "DOEPINT4 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`]
module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint4;
#[doc = "DOEPTSIZ4 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`]
module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 (rw) register accessor: OTG device OUT endpoint 4 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma4`]
module"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4rs>;
#[doc = "OTG device OUT endpoint 4 DMA address register"]
pub mod doepdma4;
#[doc = "DOEPCTL5 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`]
module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl5;
#[doc = "DOEPINT5 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`]
module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint5;
#[doc = "DOEPTSIZ5 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`]
module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz5;
#[doc = "DOEPDMA5 (rw) register accessor: OTG device OUT endpoint 5 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma5`]
module"]
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5rs>;
#[doc = "OTG device OUT endpoint 5 DMA address register"]
pub mod doepdma5;
#[doc = "DOEPCTL6 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl6`]
module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl6;
#[doc = "DOEPINT6 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint6`]
module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint6;
#[doc = "DOEPTSIZ6 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz6`]
module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz6;
#[doc = "DOEPDMA6 (rw) register accessor: OTG device OUT endpoint 6 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma6`]
module"]
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6rs>;
#[doc = "OTG device OUT endpoint 6 DMA address register"]
pub mod doepdma6;
#[doc = "DOEPCTL7 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl7`]
module"]
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl7;
#[doc = "DOEPINT7 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint7`]
module"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint7;
#[doc = "DOEPTSIZ7 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz7`]
module"]
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz7;
#[doc = "DOEPDMA7 (rw) register accessor: OTG device OUT endpoint 7 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma7`]
module"]
pub type DOEPDMA7 = crate::Reg<doepdma7::DOEPDMA7rs>;
#[doc = "OTG device OUT endpoint 7 DMA address register"]
pub mod doepdma7;
#[doc = "DOEPCTL8 (rw) register accessor: The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl8`]
module"]
pub type DOEPCTL8 = crate::Reg<doepctl8::DOEPCTL8rs>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod doepctl8;
#[doc = "DOEPINT8 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint8`]
module"]
pub type DOEPINT8 = crate::Reg<doepint8::DOEPINT8rs>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the GINTSTS register (OEPINT bit in GINTSTS) is set. Before the application can read this register, it must first read the DAINT register to get the exact endpoint number for the DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers."]
pub mod doepint8;
#[doc = "DOEPTSIZ8 (rw) register accessor: The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz8`]
module"]
pub type DOEPTSIZ8 = crate::Reg<doeptsiz8::DOEPTSIZ8rs>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the DOEPCTLx registers (EPENA bit in DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod doeptsiz8;
#[doc = "DOEPDMA8 (rw) register accessor: OTG device OUT endpoint 8 DMA address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdma8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdma8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma8`]
module"]
pub type DOEPDMA8 = crate::Reg<doepdma8::DOEPDMA8rs>;
#[doc = "OTG device OUT endpoint 8 DMA address register"]
pub mod doepdma8;
#[doc = "PCGCCTL (rw) register accessor: This register is available in host and device modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTLrs>;
#[doc = "This register is available in host and device modes."]
pub mod pcgcctl;
