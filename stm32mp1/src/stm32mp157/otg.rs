#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
    pub otg_gotgctl: OTG_GOTGCTL,
    #[doc = "0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
    pub otg_gotgint: OTG_GOTGINT,
    #[doc = "0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
    pub otg_gahbcfg: OTG_GAHBCFG,
    #[doc = "0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
    pub otg_gusbcfg: OTG_GUSBCFG,
    #[doc = "0x10 - The application uses this register to reset various hardware features inside the core."]
    pub otg_grstctl: OTG_GRSTCTL,
    #[doc = "0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
    pub otg_gintsts: OTG_GINTSTS,
    #[doc = "0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
    pub otg_gintmsk: OTG_GINTMSK,
    #[doc = "0x1c - This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
    pub otg_grxstsr: OTG_GRXSTSR,
    #[doc = "0x20 - This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
    pub otg_grxstsp: OTG_GRXSTSP,
    #[doc = "0x24 - The application can program the RAM size that must be allocated to the Rx FIFO."]
    pub otg_grxfsiz: OTG_GRXFSIZ,
    #[doc = "0x28 - Host mode"]
    pub otg_hnptxfsiz: OTG_HNPTXFSIZ,
    #[doc = "0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
    pub otg_hnptxsts: OTG_HNPTXSTS,
    _reserved12: [u8; 8usize],
    #[doc = "0x38 - OTG general core configuration register"]
    pub otg_gccfg: OTG_GCCFG,
    #[doc = "0x3c - This is a register containing the Product ID as reset value."]
    pub otg_cid: OTG_CID,
    _reserved14: [u8; 20usize],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_glpmcfg: OTG_GLPMCFG,
    _reserved15: [u8; 168usize],
    #[doc = "0x100 - OTG host periodic transmit FIFO size register"]
    pub otg_hptxfsiz: OTG_HPTXFSIZ,
    #[doc = "0x104 - OTG device IN endpoint transmit FIFO 1 size register"]
    pub otg_dieptxf1: OTG_DIEPTXF1,
    #[doc = "0x108 - OTG device IN endpoint transmit FIFO 2 size register"]
    pub otg_dieptxf2: OTG_DIEPTXF2,
    #[doc = "0x10c - OTG device IN endpoint transmit FIFO 3 size register"]
    pub otg_dieptxf3: OTG_DIEPTXF3,
    #[doc = "0x110 - OTG device IN endpoint transmit FIFO 4 size register"]
    pub otg_dieptxf4: OTG_DIEPTXF4,
    #[doc = "0x114 - OTG device IN endpoint transmit FIFO 5 size register"]
    pub otg_dieptxf5: OTG_DIEPTXF5,
    #[doc = "0x118 - OTG device IN endpoint transmit FIFO 6 size register"]
    pub otg_dieptxf6: OTG_DIEPTXF6,
    #[doc = "0x11c - OTG device IN endpoint transmit FIFO 7 size register"]
    pub otg_dieptxf7: OTG_DIEPTXF7,
    #[doc = "0x120 - OTG device IN endpoint transmit FIFO 8 size register"]
    pub otg_dieptxf8: OTG_DIEPTXF8,
    _reserved24: [u8; 732usize],
    #[doc = "0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host."]
    pub otg_hcfg: OTG_HCFG,
    #[doc = "0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
    pub otg_hfir: OTG_HFIR,
    #[doc = "0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
    pub otg_hfnum: OTG_HFNUM,
    _reserved27: [u8; 4usize],
    #[doc = "0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
    pub otg_hptxsts: OTG_HPTXSTS,
    #[doc = "0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
    pub otg_haint: OTG_HAINT,
    #[doc = "0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
    pub otg_haintmsk: OTG_HAINTMSK,
    #[doc = "0x41c - This register holds the starting address of the frame list information (scatter/gather mode)."]
    pub otg_hflbaddr: OTG_HFLBADDR,
    _reserved31: [u8; 32usize],
    #[doc = "0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
    pub otg_hprt: OTG_HPRT,
    _reserved32: [u8; 188usize],
    #[doc = "0x500 - OTG host channel 0 characteristics register"]
    pub otg_hcchar0: OTG_HCCHAR0,
    #[doc = "0x504 - OTG host channel 0 split control register"]
    pub otg_hcsplt0: OTG_HCSPLT0,
    #[doc = "0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint0: OTG_HCINT0,
    #[doc = "0x50c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk0: OTG_HCINTMSK0,
    #[doc = "0x510 - OTG host channel 0 transfer size register"]
    pub otg_hctsiz0: OTG_HCTSIZ0,
    #[doc = "0x514 - OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma0: OTG_HCDMA0,
    _reserved38: [u8; 4usize],
    #[doc = "0x51c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab0: OTG_HCDMAB0,
    #[doc = "0x520 - OTG host channel 1 characteristics register"]
    pub otg_hcchar1: OTG_HCCHAR1,
    #[doc = "0x524 - OTG host channel 1 split control register"]
    pub otg_hcsplt1: OTG_HCSPLT1,
    #[doc = "0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint1: OTG_HCINT1,
    #[doc = "0x52c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk1: OTG_HCINTMSK1,
    #[doc = "0x530 - OTG host channel 1 transfer size register"]
    pub otg_hctsiz1: OTG_HCTSIZ1,
    #[doc = "0x534 - OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma1: OTG_HCDMA1,
    _reserved45: [u8; 4usize],
    #[doc = "0x53c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab1: OTG_HCDMAB1,
    #[doc = "0x540 - OTG host channel 2 characteristics register"]
    pub otg_hcchar2: OTG_HCCHAR2,
    #[doc = "0x544 - OTG host channel 2 split control register"]
    pub otg_hcsplt2: OTG_HCSPLT2,
    #[doc = "0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint2: OTG_HCINT2,
    #[doc = "0x54c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk2: OTG_HCINTMSK2,
    #[doc = "0x550 - OTG host channel 2 transfer size register"]
    pub otg_hctsiz2: OTG_HCTSIZ2,
    #[doc = "0x554 - OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma2: OTG_HCDMA2,
    _reserved52: [u8; 4usize],
    #[doc = "0x55c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab2: OTG_HCDMAB2,
    #[doc = "0x560 - OTG host channel 3 characteristics register"]
    pub otg_hcchar3: OTG_HCCHAR3,
    #[doc = "0x564 - OTG host channel 3 split control register"]
    pub otg_hcsplt3: OTG_HCSPLT3,
    #[doc = "0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint3: OTG_HCINT3,
    #[doc = "0x56c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk3: OTG_HCINTMSK3,
    #[doc = "0x570 - OTG host channel 3 transfer size register"]
    pub otg_hctsiz3: OTG_HCTSIZ3,
    #[doc = "0x574 - OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma3: OTG_HCDMA3,
    _reserved59: [u8; 4usize],
    #[doc = "0x57c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab3: OTG_HCDMAB3,
    #[doc = "0x580 - OTG host channel 4 characteristics register"]
    pub otg_hcchar4: OTG_HCCHAR4,
    #[doc = "0x584 - OTG host channel 4 split control register"]
    pub otg_hcsplt4: OTG_HCSPLT4,
    #[doc = "0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint4: OTG_HCINT4,
    #[doc = "0x58c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk4: OTG_HCINTMSK4,
    #[doc = "0x590 - OTG host channel 4 transfer size register"]
    pub otg_hctsiz4: OTG_HCTSIZ4,
    #[doc = "0x594 - OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma4: OTG_HCDMA4,
    _reserved66: [u8; 4usize],
    #[doc = "0x59c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab4: OTG_HCDMAB4,
    #[doc = "0x5a0 - OTG host channel 5 characteristics register"]
    pub otg_hcchar5: OTG_HCCHAR5,
    #[doc = "0x5a4 - OTG host channel 5 split control register"]
    pub otg_hcsplt5: OTG_HCSPLT5,
    #[doc = "0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint5: OTG_HCINT5,
    #[doc = "0x5ac - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk5: OTG_HCINTMSK5,
    #[doc = "0x5b0 - OTG host channel 5 transfer size register"]
    pub otg_hctsiz5: OTG_HCTSIZ5,
    #[doc = "0x5b4 - OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma5: OTG_HCDMA5,
    _reserved73: [u8; 4usize],
    #[doc = "0x5bc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab5: OTG_HCDMAB5,
    #[doc = "0x5c0 - OTG host channel 6 characteristics register"]
    pub otg_hcchar6: OTG_HCCHAR6,
    #[doc = "0x5c4 - OTG host channel 6 split control register"]
    pub otg_hcsplt6: OTG_HCSPLT6,
    #[doc = "0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint6: OTG_HCINT6,
    #[doc = "0x5cc - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk6: OTG_HCINTMSK6,
    #[doc = "0x5d0 - OTG host channel 6 transfer size register"]
    pub otg_hctsiz6: OTG_HCTSIZ6,
    #[doc = "0x5d4 - OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma6: OTG_HCDMA6,
    _reserved80: [u8; 4usize],
    #[doc = "0x5dc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab6: OTG_HCDMAB6,
    #[doc = "0x5e0 - OTG host channel 7 characteristics register"]
    pub otg_hcchar7: OTG_HCCHAR7,
    #[doc = "0x5e4 - OTG host channel 7 split control register"]
    pub otg_hcsplt7: OTG_HCSPLT7,
    #[doc = "0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint7: OTG_HCINT7,
    #[doc = "0x5ec - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk7: OTG_HCINTMSK7,
    #[doc = "0x5f0 - OTG host channel 7 transfer size register"]
    pub otg_hctsiz7: OTG_HCTSIZ7,
    #[doc = "0x5f4 - OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma7: OTG_HCDMA7,
    _reserved87: [u8; 4usize],
    #[doc = "0x5fc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab7: OTG_HCDMAB7,
    #[doc = "0x600 - OTG host channel 8 characteristics register"]
    pub otg_hcchar8: OTG_HCCHAR8,
    #[doc = "0x604 - OTG host channel 8 split control register"]
    pub otg_hcsplt8: OTG_HCSPLT8,
    #[doc = "0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint8: OTG_HCINT8,
    #[doc = "0x60c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk8: OTG_HCINTMSK8,
    #[doc = "0x610 - OTG host channel 8 transfer size register"]
    pub otg_hctsiz8: OTG_HCTSIZ8,
    #[doc = "0x614 - OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma8: OTG_HCDMA8,
    _reserved94: [u8; 4usize],
    #[doc = "0x61c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab8: OTG_HCDMAB8,
    #[doc = "0x620 - OTG host channel 9 characteristics register"]
    pub otg_hcchar9: OTG_HCCHAR9,
    #[doc = "0x624 - OTG host channel 9 split control register"]
    pub otg_hcsplt9: OTG_HCSPLT9,
    #[doc = "0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint9: OTG_HCINT9,
    #[doc = "0x62c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk9: OTG_HCINTMSK9,
    #[doc = "0x630 - OTG host channel 9 transfer size register"]
    pub otg_hctsiz9: OTG_HCTSIZ9,
    #[doc = "0x634 - OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma9: OTG_HCDMA9,
    _reserved101: [u8; 4usize],
    #[doc = "0x63c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab9: OTG_HCDMAB9,
    #[doc = "0x640 - OTG host channel 10 characteristics register"]
    pub otg_hcchar10: OTG_HCCHAR10,
    #[doc = "0x644 - OTG host channel 10 split control register"]
    pub otg_hcsplt10: OTG_HCSPLT10,
    #[doc = "0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint10: OTG_HCINT10,
    #[doc = "0x64c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk10: OTG_HCINTMSK10,
    #[doc = "0x650 - OTG host channel 10 transfer size register"]
    pub otg_hctsiz10: OTG_HCTSIZ10,
    #[doc = "0x654 - OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma10: OTG_HCDMA10,
    _reserved108: [u8; 4usize],
    #[doc = "0x65c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab10: OTG_HCDMAB10,
    #[doc = "0x660 - OTG host channel 11 characteristics register"]
    pub otg_hcchar11: OTG_HCCHAR11,
    #[doc = "0x664 - OTG host channel 11 split control register"]
    pub otg_hcsplt11: OTG_HCSPLT11,
    #[doc = "0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint11: OTG_HCINT11,
    #[doc = "0x66c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk11: OTG_HCINTMSK11,
    #[doc = "0x670 - OTG host channel 11 transfer size register"]
    pub otg_hctsiz11: OTG_HCTSIZ11,
    #[doc = "0x674 - OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma11: OTG_HCDMA11,
    _reserved115: [u8; 4usize],
    #[doc = "0x67c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab11: OTG_HCDMAB11,
    #[doc = "0x680 - OTG host channel 12 characteristics register"]
    pub otg_hcchar12: OTG_HCCHAR12,
    #[doc = "0x684 - OTG host channel 12 split control register"]
    pub otg_hcsplt12: OTG_HCSPLT12,
    #[doc = "0x688 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint12: OTG_HCINT12,
    #[doc = "0x68c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk12: OTG_HCINTMSK12,
    #[doc = "0x690 - OTG host channel 12 transfer size register"]
    pub otg_hctsiz12: OTG_HCTSIZ12,
    #[doc = "0x694 - OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma12: OTG_HCDMA12,
    _reserved122: [u8; 4usize],
    #[doc = "0x69c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab12: OTG_HCDMAB12,
    #[doc = "0x6a0 - OTG host channel 13 characteristics register"]
    pub otg_hcchar13: OTG_HCCHAR13,
    #[doc = "0x6a4 - OTG host channel 13 split control register"]
    pub otg_hcsplt13: OTG_HCSPLT13,
    #[doc = "0x6a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint13: OTG_HCINT13,
    #[doc = "0x6ac - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk13: OTG_HCINTMSK13,
    #[doc = "0x6b0 - OTG host channel 13 transfer size register"]
    pub otg_hctsiz13: OTG_HCTSIZ13,
    #[doc = "0x6b4 - OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma13: OTG_HCDMA13,
    _reserved129: [u8; 4usize],
    #[doc = "0x6bc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab13: OTG_HCDMAB13,
    #[doc = "0x6c0 - OTG host channel 14 characteristics register"]
    pub otg_hcchar14: OTG_HCCHAR14,
    #[doc = "0x6c4 - OTG host channel 14 split control register"]
    pub otg_hcsplt14: OTG_HCSPLT14,
    #[doc = "0x6c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint14: OTG_HCINT14,
    #[doc = "0x6cc - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk14: OTG_HCINTMSK14,
    #[doc = "0x6d0 - OTG host channel 14 transfer size register"]
    pub otg_hctsiz14: OTG_HCTSIZ14,
    #[doc = "0x6d4 - OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma14: OTG_HCDMA14,
    _reserved136: [u8; 4usize],
    #[doc = "0x6dc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab14: OTG_HCDMAB14,
    #[doc = "0x6e0 - OTG host channel 15 characteristics register"]
    pub otg_hcchar15: OTG_HCCHAR15,
    #[doc = "0x6e4 - OTG host channel 15 split control register"]
    pub otg_hcsplt15: OTG_HCSPLT15,
    #[doc = "0x6e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint15: OTG_HCINT15,
    #[doc = "0x6ec - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk15: OTG_HCINTMSK15,
    #[doc = "0x6f0 - OTG host channel 15 transfer size register"]
    pub otg_hctsiz15: OTG_HCTSIZ15,
    #[doc = "0x6f4 - OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma15: OTG_HCDMA15,
    _reserved143: [u8; 4usize],
    #[doc = "0x6fc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab15: OTG_HCDMAB15,
    _reserved144: [u8; 256usize],
    #[doc = "0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
    pub otg_dcfg: OTG_DCFG,
    #[doc = "0x804 - OTG device control register"]
    pub otg_dctl: OTG_DCTL,
    #[doc = "0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
    pub otg_dsts: OTG_DSTS,
    _reserved147: [u8; 4usize],
    #[doc = "0x810 - This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
    pub otg_diepmsk: OTG_DIEPMSK,
    #[doc = "0x814 - This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_doepmsk: OTG_DOEPMSK,
    #[doc = "0x818 - When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
    pub otg_daint: OTG_DAINT,
    #[doc = "0x81c - The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
    pub otg_daintmsk: OTG_DAINTMSK,
    _reserved151: [u8; 8usize],
    #[doc = "0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
    pub otg_dvbusdis: OTG_DVBUSDIS,
    #[doc = "0x82c - This register specifies the VBUS pulsing time during SRP."]
    pub otg_dvbuspulse: OTG_DVBUSPULSE,
    #[doc = "0x830 - OTG device threshold control register"]
    pub otg_dthrctl: OTG_DTHRCTL,
    #[doc = "0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
    pub otg_diepempmsk: OTG_DIEPEMPMSK,
    #[doc = "0x838 - OTG device each endpoint interrupt register"]
    pub otg_deachint: OTG_DEACHINT,
    #[doc = "0x83c - There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
    pub otg_deachintmsk: OTG_DEACHINTMSK,
    _reserved157: [u8; 4usize],
    #[doc = "0x844 - This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_hs_diepeachmsk1: OTG_HS_DIEPEACHMSK1,
    _reserved158: [u8; 60usize],
    #[doc = "0x884 - This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_hs_doepeachmsk1: OTG_HS_DOEPEACHMSK1,
    _reserved159: [u8; 120usize],
    #[doc = "0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl0: OTG_DIEPCTL0,
    _reserved160: [u8; 4usize],
    #[doc = "0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint0: OTG_DIEPINT0,
    _reserved161: [u8; 4usize],
    #[doc = "0x910 - The application must modify this register before enabling endpoint 0."]
    pub otg_dieptsiz0: OTG_DIEPTSIZ0,
    #[doc = "0x914 - OTG device IN endpoint 0 DMA address register"]
    pub otg_diepdma0: OTG_DIEPDMA0,
    #[doc = "0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts0: OTG_DTXFSTS0,
    _reserved164: [u8; 4usize],
    #[doc = "0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl1: OTG_DIEPCTL1,
    _reserved165: [u8; 4usize],
    #[doc = "0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint1: OTG_DIEPINT1,
    _reserved166: [u8; 4usize],
    #[doc = "0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz1: OTG_DIEPTSIZ1,
    #[doc = "0x934 - OTG device IN endpoint 1 DMA address register"]
    pub otg_diepdma1: OTG_DIEPDMA1,
    #[doc = "0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts1: OTG_DTXFSTS1,
    _reserved169: [u8; 4usize],
    #[doc = "0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl2: OTG_DIEPCTL2,
    _reserved170: [u8; 4usize],
    #[doc = "0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint2: OTG_DIEPINT2,
    _reserved171: [u8; 4usize],
    #[doc = "0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz2: OTG_DIEPTSIZ2,
    #[doc = "0x954 - OTG device IN endpoint 2 DMA address register"]
    pub otg_diepdma2: OTG_DIEPDMA2,
    #[doc = "0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts2: OTG_DTXFSTS2,
    _reserved174: [u8; 4usize],
    #[doc = "0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl3: OTG_DIEPCTL3,
    _reserved175: [u8; 4usize],
    #[doc = "0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint3: OTG_DIEPINT3,
    _reserved176: [u8; 4usize],
    #[doc = "0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz3: OTG_DIEPTSIZ3,
    #[doc = "0x974 - OTG device IN endpoint 3 DMA address register"]
    pub otg_diepdma3: OTG_DIEPDMA3,
    #[doc = "0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts3: OTG_DTXFSTS3,
    _reserved179: [u8; 4usize],
    #[doc = "0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl4: OTG_DIEPCTL4,
    _reserved180: [u8; 4usize],
    #[doc = "0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint4: OTG_DIEPINT4,
    _reserved181: [u8; 4usize],
    #[doc = "0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz4: OTG_DIEPTSIZ4,
    #[doc = "0x994 - OTG device IN endpoint 4 DMA address register"]
    pub otg_diepdma4: OTG_DIEPDMA4,
    #[doc = "0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts4: OTG_DTXFSTS4,
    _reserved184: [u8; 4usize],
    #[doc = "0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl5: OTG_DIEPCTL5,
    _reserved185: [u8; 4usize],
    #[doc = "0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint5: OTG_DIEPINT5,
    _reserved186: [u8; 4usize],
    #[doc = "0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz5: OTG_DIEPTSIZ5,
    #[doc = "0x9b4 - OTG device IN endpoint 5 DMA address register"]
    pub otg_diepdma5: OTG_DIEPDMA5,
    #[doc = "0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts5: OTG_DTXFSTS5,
    _reserved189: [u8; 4usize],
    #[doc = "0x9c0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl6: OTG_DIEPCTL6,
    _reserved190: [u8; 4usize],
    #[doc = "0x9c8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint6: OTG_DIEPINT6,
    _reserved191: [u8; 4usize],
    #[doc = "0x9d0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz6: OTG_DIEPTSIZ6,
    #[doc = "0x9d4 - OTG device IN endpoint 6 DMA address register"]
    pub otg_diepdma6: OTG_DIEPDMA6,
    #[doc = "0x9d8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts6: OTG_DTXFSTS6,
    _reserved194: [u8; 4usize],
    #[doc = "0x9e0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl7: OTG_DIEPCTL7,
    _reserved195: [u8; 4usize],
    #[doc = "0x9e8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint7: OTG_DIEPINT7,
    _reserved196: [u8; 4usize],
    #[doc = "0x9f0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz7: OTG_DIEPTSIZ7,
    #[doc = "0x9f4 - OTG device IN endpoint 7 DMA address register"]
    pub otg_diepdma7: OTG_DIEPDMA7,
    #[doc = "0x9f8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts7: OTG_DTXFSTS7,
    _reserved199: [u8; 4usize],
    #[doc = "0xa00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl8: OTG_DIEPCTL8,
    _reserved200: [u8; 4usize],
    #[doc = "0xa08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint8: OTG_DIEPINT8,
    _reserved201: [u8; 4usize],
    #[doc = "0xa10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz8: OTG_DIEPTSIZ8,
    #[doc = "0xa14 - OTG device IN endpoint 8 DMA address register"]
    pub otg_diepdma8: OTG_DIEPDMA8,
    #[doc = "0xa18 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts8: OTG_DTXFSTS8,
    _reserved204: [u8; 228usize],
    #[doc = "0xb00 - This section describes the OTG_DOEPCTL0 register."]
    pub otg_doepctl0: OTG_DOEPCTL0,
    _reserved205: [u8; 4usize],
    #[doc = "0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint0: OTG_DOEPINT0,
    _reserved206: [u8; 4usize],
    #[doc = "0xb10 - The application must modify this register before enabling endpoint 0."]
    pub otg_doeptsiz0: OTG_DOEPTSIZ0,
    #[doc = "0xb14 - OTG device OUT endpoint 0 DMA address register"]
    pub otg_doepdma0: OTG_DOEPDMA0,
    _reserved208: [u8; 8usize],
    #[doc = "0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl1: OTG_DOEPCTL1,
    _reserved209: [u8; 4usize],
    #[doc = "0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint1: OTG_DOEPINT1,
    _reserved210: [u8; 4usize],
    #[doc = "0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz1: OTG_DOEPTSIZ1,
    #[doc = "0xb34 - OTG device OUT endpoint 1 DMA address register"]
    pub otg_doepdma1: OTG_DOEPDMA1,
    _reserved212: [u8; 8usize],
    #[doc = "0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl2: OTG_DOEPCTL2,
    _reserved213: [u8; 4usize],
    #[doc = "0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint2: OTG_DOEPINT2,
    _reserved214: [u8; 4usize],
    #[doc = "0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz2: OTG_DOEPTSIZ2,
    #[doc = "0xb54 - OTG device OUT endpoint 2 DMA address register"]
    pub otg_doepdma2: OTG_DOEPDMA2,
    _reserved216: [u8; 8usize],
    #[doc = "0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl3: OTG_DOEPCTL3,
    _reserved217: [u8; 4usize],
    #[doc = "0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint3: OTG_DOEPINT3,
    _reserved218: [u8; 4usize],
    #[doc = "0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz3: OTG_DOEPTSIZ3,
    #[doc = "0xb74 - OTG device OUT endpoint 3 DMA address register"]
    pub otg_doepdma3: OTG_DOEPDMA3,
    _reserved220: [u8; 8usize],
    #[doc = "0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl4: OTG_DOEPCTL4,
    _reserved221: [u8; 4usize],
    #[doc = "0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint4: OTG_DOEPINT4,
    _reserved222: [u8; 4usize],
    #[doc = "0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz4: OTG_DOEPTSIZ4,
    #[doc = "0xb94 - OTG device OUT endpoint 4 DMA address register"]
    pub otg_doepdma4: OTG_DOEPDMA4,
    _reserved224: [u8; 8usize],
    #[doc = "0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl5: OTG_DOEPCTL5,
    _reserved225: [u8; 4usize],
    #[doc = "0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint5: OTG_DOEPINT5,
    _reserved226: [u8; 4usize],
    #[doc = "0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz5: OTG_DOEPTSIZ5,
    #[doc = "0xbb4 - OTG device OUT endpoint 5 DMA address register"]
    pub otg_doepdma5: OTG_DOEPDMA5,
    _reserved228: [u8; 8usize],
    #[doc = "0xbc0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl6: OTG_DOEPCTL6,
    _reserved229: [u8; 4usize],
    #[doc = "0xbc8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint6: OTG_DOEPINT6,
    _reserved230: [u8; 4usize],
    #[doc = "0xbd0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz6: OTG_DOEPTSIZ6,
    #[doc = "0xbd4 - OTG device OUT endpoint 6 DMA address register"]
    pub otg_doepdma6: OTG_DOEPDMA6,
    _reserved232: [u8; 8usize],
    #[doc = "0xbe0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl7: OTG_DOEPCTL7,
    _reserved233: [u8; 4usize],
    #[doc = "0xbe8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint7: OTG_DOEPINT7,
    _reserved234: [u8; 4usize],
    #[doc = "0xbf0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz7: OTG_DOEPTSIZ7,
    #[doc = "0xbf4 - OTG device OUT endpoint 7 DMA address register"]
    pub otg_doepdma7: OTG_DOEPDMA7,
    _reserved236: [u8; 8usize],
    #[doc = "0xc00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl8: OTG_DOEPCTL8,
    _reserved237: [u8; 4usize],
    #[doc = "0xc08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint8: OTG_DOEPINT8,
    _reserved238: [u8; 4usize],
    #[doc = "0xc10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz8: OTG_DOEPTSIZ8,
    #[doc = "0xc14 - OTG device OUT endpoint 8 DMA address register"]
    pub otg_doepdma8: OTG_DOEPDMA8,
    _reserved240: [u8; 488usize],
    #[doc = "0xe00 - This register is available in host and device modes."]
    pub otg_pcgcctl: OTG_PCGCCTL,
}
#[doc = "The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gotgctl](otg_gotgctl) module"]
pub type OTG_GOTGCTL = crate::Reg<u32, _OTG_GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GOTGCTL;
#[doc = "`read()` method returns [otg_gotgctl::R](otg_gotgctl::R) reader structure"]
impl crate::Readable for OTG_GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [otg_gotgctl::W](otg_gotgctl::W) writer structure"]
impl crate::Writable for OTG_GOTGCTL {}
#[doc = "The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
pub mod otg_gotgctl;
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gotgint](otg_gotgint) module"]
pub type OTG_GOTGINT = crate::Reg<u32, _OTG_GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GOTGINT;
#[doc = "`read()` method returns [otg_gotgint::R](otg_gotgint::R) reader structure"]
impl crate::Readable for OTG_GOTGINT {}
#[doc = "`write(|w| ..)` method takes [otg_gotgint::W](otg_gotgint::W) writer structure"]
impl crate::Writable for OTG_GOTGINT {}
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
pub mod otg_gotgint;
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gahbcfg](otg_gahbcfg) module"]
pub type OTG_GAHBCFG = crate::Reg<u32, _OTG_GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GAHBCFG;
#[doc = "`read()` method returns [otg_gahbcfg::R](otg_gahbcfg::R) reader structure"]
impl crate::Readable for OTG_GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_gahbcfg::W](otg_gahbcfg::W) writer structure"]
impl crate::Writable for OTG_GAHBCFG {}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
pub mod otg_gahbcfg;
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gusbcfg](otg_gusbcfg) module"]
pub type OTG_GUSBCFG = crate::Reg<u32, _OTG_GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GUSBCFG;
#[doc = "`read()` method returns [otg_gusbcfg::R](otg_gusbcfg::R) reader structure"]
impl crate::Readable for OTG_GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [otg_gusbcfg::W](otg_gusbcfg::W) writer structure"]
impl crate::Writable for OTG_GUSBCFG {}
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
pub mod otg_gusbcfg;
#[doc = "The application uses this register to reset various hardware features inside the core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grstctl](otg_grstctl) module"]
pub type OTG_GRSTCTL = crate::Reg<u32, _OTG_GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GRSTCTL;
#[doc = "`read()` method returns [otg_grstctl::R](otg_grstctl::R) reader structure"]
impl crate::Readable for OTG_GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [otg_grstctl::W](otg_grstctl::W) writer structure"]
impl crate::Writable for OTG_GRSTCTL {}
#[doc = "The application uses this register to reset various hardware features inside the core."]
pub mod otg_grstctl;
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gintsts](otg_gintsts) module"]
pub type OTG_GINTSTS = crate::Reg<u32, _OTG_GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GINTSTS;
#[doc = "`read()` method returns [otg_gintsts::R](otg_gintsts::R) reader structure"]
impl crate::Readable for OTG_GINTSTS {}
#[doc = "`write(|w| ..)` method takes [otg_gintsts::W](otg_gintsts::W) writer structure"]
impl crate::Writable for OTG_GINTSTS {}
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
pub mod otg_gintsts;
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gintmsk](otg_gintmsk) module"]
pub type OTG_GINTMSK = crate::Reg<u32, _OTG_GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GINTMSK;
#[doc = "`read()` method returns [otg_gintmsk::R](otg_gintmsk::R) reader structure"]
impl crate::Readable for OTG_GINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_gintmsk::W](otg_gintmsk::W) writer structure"]
impl crate::Writable for OTG_GINTMSK {}
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
pub mod otg_gintmsk;
#[doc = "This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grxstsr](otg_grxstsr) module"]
pub type OTG_GRXSTSR = crate::Reg<u32, _OTG_GRXSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GRXSTSR;
#[doc = "`read()` method returns [otg_grxstsr::R](otg_grxstsr::R) reader structure"]
impl crate::Readable for OTG_GRXSTSR {}
#[doc = "This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
pub mod otg_grxstsr;
#[doc = "This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grxstsp](otg_grxstsp) module"]
pub type OTG_GRXSTSP = crate::Reg<u32, _OTG_GRXSTSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GRXSTSP;
#[doc = "`read()` method returns [otg_grxstsp::R](otg_grxstsp::R) reader structure"]
impl crate::Readable for OTG_GRXSTSP {}
#[doc = "This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
pub mod otg_grxstsp;
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grxfsiz](otg_grxfsiz) module"]
pub type OTG_GRXFSIZ = crate::Reg<u32, _OTG_GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GRXFSIZ;
#[doc = "`read()` method returns [otg_grxfsiz::R](otg_grxfsiz::R) reader structure"]
impl crate::Readable for OTG_GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_grxfsiz::W](otg_grxfsiz::W) writer structure"]
impl crate::Writable for OTG_GRXFSIZ {}
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO."]
pub mod otg_grxfsiz;
#[doc = "Host mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hnptxfsiz](otg_hnptxfsiz) module"]
pub type OTG_HNPTXFSIZ = crate::Reg<u32, _OTG_HNPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HNPTXFSIZ;
#[doc = "`read()` method returns [otg_hnptxfsiz::R](otg_hnptxfsiz::R) reader structure"]
impl crate::Readable for OTG_HNPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_hnptxfsiz::W](otg_hnptxfsiz::W) writer structure"]
impl crate::Writable for OTG_HNPTXFSIZ {}
#[doc = "Host mode"]
pub mod otg_hnptxfsiz;
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hnptxsts](otg_hnptxsts) module"]
pub type OTG_HNPTXSTS = crate::Reg<u32, _OTG_HNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HNPTXSTS;
#[doc = "`read()` method returns [otg_hnptxsts::R](otg_hnptxsts::R) reader structure"]
impl crate::Readable for OTG_HNPTXSTS {}
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
pub mod otg_hnptxsts;
#[doc = "OTG general core configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gccfg](otg_gccfg) module"]
pub type OTG_GCCFG = crate::Reg<u32, _OTG_GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GCCFG;
#[doc = "`read()` method returns [otg_gccfg::R](otg_gccfg::R) reader structure"]
impl crate::Readable for OTG_GCCFG {}
#[doc = "`write(|w| ..)` method takes [otg_gccfg::W](otg_gccfg::W) writer structure"]
impl crate::Writable for OTG_GCCFG {}
#[doc = "OTG general core configuration register"]
pub mod otg_gccfg;
#[doc = "This is a register containing the Product ID as reset value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_cid](otg_cid) module"]
pub type OTG_CID = crate::Reg<u32, _OTG_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_CID;
#[doc = "`read()` method returns [otg_cid::R](otg_cid::R) reader structure"]
impl crate::Readable for OTG_CID {}
#[doc = "`write(|w| ..)` method takes [otg_cid::W](otg_cid::W) writer structure"]
impl crate::Writable for OTG_CID {}
#[doc = "This is a register containing the Product ID as reset value."]
pub mod otg_cid;
#[doc = "OTG core LPM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_glpmcfg](otg_glpmcfg) module"]
pub type OTG_GLPMCFG = crate::Reg<u32, _OTG_GLPMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_GLPMCFG;
#[doc = "`read()` method returns [otg_glpmcfg::R](otg_glpmcfg::R) reader structure"]
impl crate::Readable for OTG_GLPMCFG {}
#[doc = "`write(|w| ..)` method takes [otg_glpmcfg::W](otg_glpmcfg::W) writer structure"]
impl crate::Writable for OTG_GLPMCFG {}
#[doc = "OTG core LPM configuration register"]
pub mod otg_glpmcfg;
#[doc = "OTG host periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hptxfsiz](otg_hptxfsiz) module"]
pub type OTG_HPTXFSIZ = crate::Reg<u32, _OTG_HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HPTXFSIZ;
#[doc = "`read()` method returns [otg_hptxfsiz::R](otg_hptxfsiz::R) reader structure"]
impl crate::Readable for OTG_HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [otg_hptxfsiz::W](otg_hptxfsiz::W) writer structure"]
impl crate::Writable for OTG_HPTXFSIZ {}
#[doc = "OTG host periodic transmit FIFO size register"]
pub mod otg_hptxfsiz;
#[doc = "OTG device IN endpoint transmit FIFO 1 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf1](otg_dieptxf1) module"]
pub type OTG_DIEPTXF1 = crate::Reg<u32, _OTG_DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF1;
#[doc = "`read()` method returns [otg_dieptxf1::R](otg_dieptxf1::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf1::W](otg_dieptxf1::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF1 {}
#[doc = "OTG device IN endpoint transmit FIFO 1 size register"]
pub mod otg_dieptxf1;
#[doc = "OTG device IN endpoint transmit FIFO 2 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf2](otg_dieptxf2) module"]
pub type OTG_DIEPTXF2 = crate::Reg<u32, _OTG_DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF2;
#[doc = "`read()` method returns [otg_dieptxf2::R](otg_dieptxf2::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf2::W](otg_dieptxf2::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF2 {}
#[doc = "OTG device IN endpoint transmit FIFO 2 size register"]
pub mod otg_dieptxf2;
#[doc = "OTG device IN endpoint transmit FIFO 3 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf3](otg_dieptxf3) module"]
pub type OTG_DIEPTXF3 = crate::Reg<u32, _OTG_DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF3;
#[doc = "`read()` method returns [otg_dieptxf3::R](otg_dieptxf3::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf3::W](otg_dieptxf3::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF3 {}
#[doc = "OTG device IN endpoint transmit FIFO 3 size register"]
pub mod otg_dieptxf3;
#[doc = "OTG device IN endpoint transmit FIFO 4 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf4](otg_dieptxf4) module"]
pub type OTG_DIEPTXF4 = crate::Reg<u32, _OTG_DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF4;
#[doc = "`read()` method returns [otg_dieptxf4::R](otg_dieptxf4::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf4::W](otg_dieptxf4::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF4 {}
#[doc = "OTG device IN endpoint transmit FIFO 4 size register"]
pub mod otg_dieptxf4;
#[doc = "OTG device IN endpoint transmit FIFO 5 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf5](otg_dieptxf5) module"]
pub type OTG_DIEPTXF5 = crate::Reg<u32, _OTG_DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF5;
#[doc = "`read()` method returns [otg_dieptxf5::R](otg_dieptxf5::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf5::W](otg_dieptxf5::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF5 {}
#[doc = "OTG device IN endpoint transmit FIFO 5 size register"]
pub mod otg_dieptxf5;
#[doc = "OTG device IN endpoint transmit FIFO 6 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf6](otg_dieptxf6) module"]
pub type OTG_DIEPTXF6 = crate::Reg<u32, _OTG_DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF6;
#[doc = "`read()` method returns [otg_dieptxf6::R](otg_dieptxf6::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf6::W](otg_dieptxf6::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF6 {}
#[doc = "OTG device IN endpoint transmit FIFO 6 size register"]
pub mod otg_dieptxf6;
#[doc = "OTG device IN endpoint transmit FIFO 7 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf7](otg_dieptxf7) module"]
pub type OTG_DIEPTXF7 = crate::Reg<u32, _OTG_DIEPTXF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF7;
#[doc = "`read()` method returns [otg_dieptxf7::R](otg_dieptxf7::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF7 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf7::W](otg_dieptxf7::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF7 {}
#[doc = "OTG device IN endpoint transmit FIFO 7 size register"]
pub mod otg_dieptxf7;
#[doc = "OTG device IN endpoint transmit FIFO 8 size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptxf8](otg_dieptxf8) module"]
pub type OTG_DIEPTXF8 = crate::Reg<u32, _OTG_DIEPTXF8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTXF8;
#[doc = "`read()` method returns [otg_dieptxf8::R](otg_dieptxf8::R) reader structure"]
impl crate::Readable for OTG_DIEPTXF8 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptxf8::W](otg_dieptxf8::W) writer structure"]
impl crate::Writable for OTG_DIEPTXF8 {}
#[doc = "OTG device IN endpoint transmit FIFO 8 size register"]
pub mod otg_dieptxf8;
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcfg](otg_hcfg) module"]
pub type OTG_HCFG = crate::Reg<u32, _OTG_HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCFG;
#[doc = "`read()` method returns [otg_hcfg::R](otg_hcfg::R) reader structure"]
impl crate::Readable for OTG_HCFG {}
#[doc = "`write(|w| ..)` method takes [otg_hcfg::W](otg_hcfg::W) writer structure"]
impl crate::Writable for OTG_HCFG {}
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host."]
pub mod otg_hcfg;
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hfir](otg_hfir) module"]
pub type OTG_HFIR = crate::Reg<u32, _OTG_HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HFIR;
#[doc = "`read()` method returns [otg_hfir::R](otg_hfir::R) reader structure"]
impl crate::Readable for OTG_HFIR {}
#[doc = "`write(|w| ..)` method takes [otg_hfir::W](otg_hfir::W) writer structure"]
impl crate::Writable for OTG_HFIR {}
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
pub mod otg_hfir;
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hfnum](otg_hfnum) module"]
pub type OTG_HFNUM = crate::Reg<u32, _OTG_HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HFNUM;
#[doc = "`read()` method returns [otg_hfnum::R](otg_hfnum::R) reader structure"]
impl crate::Readable for OTG_HFNUM {}
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
pub mod otg_hfnum;
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hptxsts](otg_hptxsts) module"]
pub type OTG_HPTXSTS = crate::Reg<u32, _OTG_HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HPTXSTS;
#[doc = "`read()` method returns [otg_hptxsts::R](otg_hptxsts::R) reader structure"]
impl crate::Readable for OTG_HPTXSTS {}
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
pub mod otg_hptxsts;
#[doc = "When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_haint](otg_haint) module"]
pub type OTG_HAINT = crate::Reg<u32, _OTG_HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HAINT;
#[doc = "`read()` method returns [otg_haint::R](otg_haint::R) reader structure"]
impl crate::Readable for OTG_HAINT {}
#[doc = "When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
pub mod otg_haint;
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_haintmsk](otg_haintmsk) module"]
pub type OTG_HAINTMSK = crate::Reg<u32, _OTG_HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HAINTMSK;
#[doc = "`read()` method returns [otg_haintmsk::R](otg_haintmsk::R) reader structure"]
impl crate::Readable for OTG_HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_haintmsk::W](otg_haintmsk::W) writer structure"]
impl crate::Writable for OTG_HAINTMSK {}
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
pub mod otg_haintmsk;
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hflbaddr](otg_hflbaddr) module"]
pub type OTG_HFLBADDR = crate::Reg<u32, _OTG_HFLBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HFLBADDR;
#[doc = "`read()` method returns [otg_hflbaddr::R](otg_hflbaddr::R) reader structure"]
impl crate::Readable for OTG_HFLBADDR {}
#[doc = "`write(|w| ..)` method takes [otg_hflbaddr::W](otg_hflbaddr::W) writer structure"]
impl crate::Writable for OTG_HFLBADDR {}
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode)."]
pub mod otg_hflbaddr;
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hprt](otg_hprt) module"]
pub type OTG_HPRT = crate::Reg<u32, _OTG_HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HPRT;
#[doc = "`read()` method returns [otg_hprt::R](otg_hprt::R) reader structure"]
impl crate::Readable for OTG_HPRT {}
#[doc = "`write(|w| ..)` method takes [otg_hprt::W](otg_hprt::W) writer structure"]
impl crate::Writable for OTG_HPRT {}
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
pub mod otg_hprt;
#[doc = "OTG host channel 0 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar0](otg_hcchar0) module"]
pub type OTG_HCCHAR0 = crate::Reg<u32, _OTG_HCCHAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR0;
#[doc = "`read()` method returns [otg_hcchar0::R](otg_hcchar0::R) reader structure"]
impl crate::Readable for OTG_HCCHAR0 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar0::W](otg_hcchar0::W) writer structure"]
impl crate::Writable for OTG_HCCHAR0 {}
#[doc = "OTG host channel 0 characteristics register"]
pub mod otg_hcchar0;
#[doc = "OTG host channel 0 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt0](otg_hcsplt0) module"]
pub type OTG_HCSPLT0 = crate::Reg<u32, _OTG_HCSPLT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT0;
#[doc = "`read()` method returns [otg_hcsplt0::R](otg_hcsplt0::R) reader structure"]
impl crate::Readable for OTG_HCSPLT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt0::W](otg_hcsplt0::W) writer structure"]
impl crate::Writable for OTG_HCSPLT0 {}
#[doc = "OTG host channel 0 split control register"]
pub mod otg_hcsplt0;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint0](otg_hcint0) module"]
pub type OTG_HCINT0 = crate::Reg<u32, _OTG_HCINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT0;
#[doc = "`read()` method returns [otg_hcint0::R](otg_hcint0::R) reader structure"]
impl crate::Readable for OTG_HCINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint0::W](otg_hcint0::W) writer structure"]
impl crate::Writable for OTG_HCINT0 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint0;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk0](otg_hcintmsk0) module"]
pub type OTG_HCINTMSK0 = crate::Reg<u32, _OTG_HCINTMSK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK0;
#[doc = "`read()` method returns [otg_hcintmsk0::R](otg_hcintmsk0::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK0 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk0::W](otg_hcintmsk0::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK0 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk0;
#[doc = "OTG host channel 0 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz0](otg_hctsiz0) module"]
pub type OTG_HCTSIZ0 = crate::Reg<u32, _OTG_HCTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ0;
#[doc = "`read()` method returns [otg_hctsiz0::R](otg_hctsiz0::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz0::W](otg_hctsiz0::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ0 {}
#[doc = "OTG host channel 0 transfer size register"]
pub mod otg_hctsiz0;
#[doc = "OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma0](otg_hcdma0) module"]
pub type OTG_HCDMA0 = crate::Reg<u32, _OTG_HCDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA0;
#[doc = "`read()` method returns [otg_hcdma0::R](otg_hcdma0::R) reader structure"]
impl crate::Readable for OTG_HCDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma0::W](otg_hcdma0::W) writer structure"]
impl crate::Writable for OTG_HCDMA0 {}
#[doc = "OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma0;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab0](otg_hcdmab0) module"]
pub type OTG_HCDMAB0 = crate::Reg<u32, _OTG_HCDMAB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB0;
#[doc = "`read()` method returns [otg_hcdmab0::R](otg_hcdmab0::R) reader structure"]
impl crate::Readable for OTG_HCDMAB0 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab0;
#[doc = "OTG host channel 1 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar1](otg_hcchar1) module"]
pub type OTG_HCCHAR1 = crate::Reg<u32, _OTG_HCCHAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR1;
#[doc = "`read()` method returns [otg_hcchar1::R](otg_hcchar1::R) reader structure"]
impl crate::Readable for OTG_HCCHAR1 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar1::W](otg_hcchar1::W) writer structure"]
impl crate::Writable for OTG_HCCHAR1 {}
#[doc = "OTG host channel 1 characteristics register"]
pub mod otg_hcchar1;
#[doc = "OTG host channel 1 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt1](otg_hcsplt1) module"]
pub type OTG_HCSPLT1 = crate::Reg<u32, _OTG_HCSPLT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT1;
#[doc = "`read()` method returns [otg_hcsplt1::R](otg_hcsplt1::R) reader structure"]
impl crate::Readable for OTG_HCSPLT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt1::W](otg_hcsplt1::W) writer structure"]
impl crate::Writable for OTG_HCSPLT1 {}
#[doc = "OTG host channel 1 split control register"]
pub mod otg_hcsplt1;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint1](otg_hcint1) module"]
pub type OTG_HCINT1 = crate::Reg<u32, _OTG_HCINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT1;
#[doc = "`read()` method returns [otg_hcint1::R](otg_hcint1::R) reader structure"]
impl crate::Readable for OTG_HCINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint1::W](otg_hcint1::W) writer structure"]
impl crate::Writable for OTG_HCINT1 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint1;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk1](otg_hcintmsk1) module"]
pub type OTG_HCINTMSK1 = crate::Reg<u32, _OTG_HCINTMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK1;
#[doc = "`read()` method returns [otg_hcintmsk1::R](otg_hcintmsk1::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK1 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk1::W](otg_hcintmsk1::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK1 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk1;
#[doc = "OTG host channel 1 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz1](otg_hctsiz1) module"]
pub type OTG_HCTSIZ1 = crate::Reg<u32, _OTG_HCTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ1;
#[doc = "`read()` method returns [otg_hctsiz1::R](otg_hctsiz1::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz1::W](otg_hctsiz1::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ1 {}
#[doc = "OTG host channel 1 transfer size register"]
pub mod otg_hctsiz1;
#[doc = "OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma1](otg_hcdma1) module"]
pub type OTG_HCDMA1 = crate::Reg<u32, _OTG_HCDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA1;
#[doc = "`read()` method returns [otg_hcdma1::R](otg_hcdma1::R) reader structure"]
impl crate::Readable for OTG_HCDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma1::W](otg_hcdma1::W) writer structure"]
impl crate::Writable for OTG_HCDMA1 {}
#[doc = "OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma1;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab1](otg_hcdmab1) module"]
pub type OTG_HCDMAB1 = crate::Reg<u32, _OTG_HCDMAB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB1;
#[doc = "`read()` method returns [otg_hcdmab1::R](otg_hcdmab1::R) reader structure"]
impl crate::Readable for OTG_HCDMAB1 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab1;
#[doc = "OTG host channel 2 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar2](otg_hcchar2) module"]
pub type OTG_HCCHAR2 = crate::Reg<u32, _OTG_HCCHAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR2;
#[doc = "`read()` method returns [otg_hcchar2::R](otg_hcchar2::R) reader structure"]
impl crate::Readable for OTG_HCCHAR2 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar2::W](otg_hcchar2::W) writer structure"]
impl crate::Writable for OTG_HCCHAR2 {}
#[doc = "OTG host channel 2 characteristics register"]
pub mod otg_hcchar2;
#[doc = "OTG host channel 2 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt2](otg_hcsplt2) module"]
pub type OTG_HCSPLT2 = crate::Reg<u32, _OTG_HCSPLT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT2;
#[doc = "`read()` method returns [otg_hcsplt2::R](otg_hcsplt2::R) reader structure"]
impl crate::Readable for OTG_HCSPLT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt2::W](otg_hcsplt2::W) writer structure"]
impl crate::Writable for OTG_HCSPLT2 {}
#[doc = "OTG host channel 2 split control register"]
pub mod otg_hcsplt2;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint2](otg_hcint2) module"]
pub type OTG_HCINT2 = crate::Reg<u32, _OTG_HCINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT2;
#[doc = "`read()` method returns [otg_hcint2::R](otg_hcint2::R) reader structure"]
impl crate::Readable for OTG_HCINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint2::W](otg_hcint2::W) writer structure"]
impl crate::Writable for OTG_HCINT2 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint2;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk2](otg_hcintmsk2) module"]
pub type OTG_HCINTMSK2 = crate::Reg<u32, _OTG_HCINTMSK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK2;
#[doc = "`read()` method returns [otg_hcintmsk2::R](otg_hcintmsk2::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK2 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk2::W](otg_hcintmsk2::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK2 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk2;
#[doc = "OTG host channel 2 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz2](otg_hctsiz2) module"]
pub type OTG_HCTSIZ2 = crate::Reg<u32, _OTG_HCTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ2;
#[doc = "`read()` method returns [otg_hctsiz2::R](otg_hctsiz2::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz2::W](otg_hctsiz2::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ2 {}
#[doc = "OTG host channel 2 transfer size register"]
pub mod otg_hctsiz2;
#[doc = "OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma2](otg_hcdma2) module"]
pub type OTG_HCDMA2 = crate::Reg<u32, _OTG_HCDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA2;
#[doc = "`read()` method returns [otg_hcdma2::R](otg_hcdma2::R) reader structure"]
impl crate::Readable for OTG_HCDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma2::W](otg_hcdma2::W) writer structure"]
impl crate::Writable for OTG_HCDMA2 {}
#[doc = "OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma2;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab2](otg_hcdmab2) module"]
pub type OTG_HCDMAB2 = crate::Reg<u32, _OTG_HCDMAB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB2;
#[doc = "`read()` method returns [otg_hcdmab2::R](otg_hcdmab2::R) reader structure"]
impl crate::Readable for OTG_HCDMAB2 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab2;
#[doc = "OTG host channel 3 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar3](otg_hcchar3) module"]
pub type OTG_HCCHAR3 = crate::Reg<u32, _OTG_HCCHAR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR3;
#[doc = "`read()` method returns [otg_hcchar3::R](otg_hcchar3::R) reader structure"]
impl crate::Readable for OTG_HCCHAR3 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar3::W](otg_hcchar3::W) writer structure"]
impl crate::Writable for OTG_HCCHAR3 {}
#[doc = "OTG host channel 3 characteristics register"]
pub mod otg_hcchar3;
#[doc = "OTG host channel 3 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt3](otg_hcsplt3) module"]
pub type OTG_HCSPLT3 = crate::Reg<u32, _OTG_HCSPLT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT3;
#[doc = "`read()` method returns [otg_hcsplt3::R](otg_hcsplt3::R) reader structure"]
impl crate::Readable for OTG_HCSPLT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt3::W](otg_hcsplt3::W) writer structure"]
impl crate::Writable for OTG_HCSPLT3 {}
#[doc = "OTG host channel 3 split control register"]
pub mod otg_hcsplt3;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint3](otg_hcint3) module"]
pub type OTG_HCINT3 = crate::Reg<u32, _OTG_HCINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT3;
#[doc = "`read()` method returns [otg_hcint3::R](otg_hcint3::R) reader structure"]
impl crate::Readable for OTG_HCINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint3::W](otg_hcint3::W) writer structure"]
impl crate::Writable for OTG_HCINT3 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint3;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk3](otg_hcintmsk3) module"]
pub type OTG_HCINTMSK3 = crate::Reg<u32, _OTG_HCINTMSK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK3;
#[doc = "`read()` method returns [otg_hcintmsk3::R](otg_hcintmsk3::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK3 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk3::W](otg_hcintmsk3::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK3 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk3;
#[doc = "OTG host channel 3 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz3](otg_hctsiz3) module"]
pub type OTG_HCTSIZ3 = crate::Reg<u32, _OTG_HCTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ3;
#[doc = "`read()` method returns [otg_hctsiz3::R](otg_hctsiz3::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz3::W](otg_hctsiz3::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ3 {}
#[doc = "OTG host channel 3 transfer size register"]
pub mod otg_hctsiz3;
#[doc = "OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma3](otg_hcdma3) module"]
pub type OTG_HCDMA3 = crate::Reg<u32, _OTG_HCDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA3;
#[doc = "`read()` method returns [otg_hcdma3::R](otg_hcdma3::R) reader structure"]
impl crate::Readable for OTG_HCDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma3::W](otg_hcdma3::W) writer structure"]
impl crate::Writable for OTG_HCDMA3 {}
#[doc = "OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma3;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab3](otg_hcdmab3) module"]
pub type OTG_HCDMAB3 = crate::Reg<u32, _OTG_HCDMAB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB3;
#[doc = "`read()` method returns [otg_hcdmab3::R](otg_hcdmab3::R) reader structure"]
impl crate::Readable for OTG_HCDMAB3 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab3;
#[doc = "OTG host channel 4 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar4](otg_hcchar4) module"]
pub type OTG_HCCHAR4 = crate::Reg<u32, _OTG_HCCHAR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR4;
#[doc = "`read()` method returns [otg_hcchar4::R](otg_hcchar4::R) reader structure"]
impl crate::Readable for OTG_HCCHAR4 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar4::W](otg_hcchar4::W) writer structure"]
impl crate::Writable for OTG_HCCHAR4 {}
#[doc = "OTG host channel 4 characteristics register"]
pub mod otg_hcchar4;
#[doc = "OTG host channel 4 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt4](otg_hcsplt4) module"]
pub type OTG_HCSPLT4 = crate::Reg<u32, _OTG_HCSPLT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT4;
#[doc = "`read()` method returns [otg_hcsplt4::R](otg_hcsplt4::R) reader structure"]
impl crate::Readable for OTG_HCSPLT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt4::W](otg_hcsplt4::W) writer structure"]
impl crate::Writable for OTG_HCSPLT4 {}
#[doc = "OTG host channel 4 split control register"]
pub mod otg_hcsplt4;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint4](otg_hcint4) module"]
pub type OTG_HCINT4 = crate::Reg<u32, _OTG_HCINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT4;
#[doc = "`read()` method returns [otg_hcint4::R](otg_hcint4::R) reader structure"]
impl crate::Readable for OTG_HCINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint4::W](otg_hcint4::W) writer structure"]
impl crate::Writable for OTG_HCINT4 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint4;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk4](otg_hcintmsk4) module"]
pub type OTG_HCINTMSK4 = crate::Reg<u32, _OTG_HCINTMSK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK4;
#[doc = "`read()` method returns [otg_hcintmsk4::R](otg_hcintmsk4::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK4 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk4::W](otg_hcintmsk4::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK4 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk4;
#[doc = "OTG host channel 4 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz4](otg_hctsiz4) module"]
pub type OTG_HCTSIZ4 = crate::Reg<u32, _OTG_HCTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ4;
#[doc = "`read()` method returns [otg_hctsiz4::R](otg_hctsiz4::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz4::W](otg_hctsiz4::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ4 {}
#[doc = "OTG host channel 4 transfer size register"]
pub mod otg_hctsiz4;
#[doc = "OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma4](otg_hcdma4) module"]
pub type OTG_HCDMA4 = crate::Reg<u32, _OTG_HCDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA4;
#[doc = "`read()` method returns [otg_hcdma4::R](otg_hcdma4::R) reader structure"]
impl crate::Readable for OTG_HCDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma4::W](otg_hcdma4::W) writer structure"]
impl crate::Writable for OTG_HCDMA4 {}
#[doc = "OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma4;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab4](otg_hcdmab4) module"]
pub type OTG_HCDMAB4 = crate::Reg<u32, _OTG_HCDMAB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB4;
#[doc = "`read()` method returns [otg_hcdmab4::R](otg_hcdmab4::R) reader structure"]
impl crate::Readable for OTG_HCDMAB4 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab4;
#[doc = "OTG host channel 5 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar5](otg_hcchar5) module"]
pub type OTG_HCCHAR5 = crate::Reg<u32, _OTG_HCCHAR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR5;
#[doc = "`read()` method returns [otg_hcchar5::R](otg_hcchar5::R) reader structure"]
impl crate::Readable for OTG_HCCHAR5 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar5::W](otg_hcchar5::W) writer structure"]
impl crate::Writable for OTG_HCCHAR5 {}
#[doc = "OTG host channel 5 characteristics register"]
pub mod otg_hcchar5;
#[doc = "OTG host channel 5 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt5](otg_hcsplt5) module"]
pub type OTG_HCSPLT5 = crate::Reg<u32, _OTG_HCSPLT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT5;
#[doc = "`read()` method returns [otg_hcsplt5::R](otg_hcsplt5::R) reader structure"]
impl crate::Readable for OTG_HCSPLT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt5::W](otg_hcsplt5::W) writer structure"]
impl crate::Writable for OTG_HCSPLT5 {}
#[doc = "OTG host channel 5 split control register"]
pub mod otg_hcsplt5;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint5](otg_hcint5) module"]
pub type OTG_HCINT5 = crate::Reg<u32, _OTG_HCINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT5;
#[doc = "`read()` method returns [otg_hcint5::R](otg_hcint5::R) reader structure"]
impl crate::Readable for OTG_HCINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint5::W](otg_hcint5::W) writer structure"]
impl crate::Writable for OTG_HCINT5 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint5;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk5](otg_hcintmsk5) module"]
pub type OTG_HCINTMSK5 = crate::Reg<u32, _OTG_HCINTMSK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK5;
#[doc = "`read()` method returns [otg_hcintmsk5::R](otg_hcintmsk5::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK5 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk5::W](otg_hcintmsk5::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK5 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk5;
#[doc = "OTG host channel 5 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz5](otg_hctsiz5) module"]
pub type OTG_HCTSIZ5 = crate::Reg<u32, _OTG_HCTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ5;
#[doc = "`read()` method returns [otg_hctsiz5::R](otg_hctsiz5::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz5::W](otg_hctsiz5::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ5 {}
#[doc = "OTG host channel 5 transfer size register"]
pub mod otg_hctsiz5;
#[doc = "OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma5](otg_hcdma5) module"]
pub type OTG_HCDMA5 = crate::Reg<u32, _OTG_HCDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA5;
#[doc = "`read()` method returns [otg_hcdma5::R](otg_hcdma5::R) reader structure"]
impl crate::Readable for OTG_HCDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma5::W](otg_hcdma5::W) writer structure"]
impl crate::Writable for OTG_HCDMA5 {}
#[doc = "OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma5;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab5](otg_hcdmab5) module"]
pub type OTG_HCDMAB5 = crate::Reg<u32, _OTG_HCDMAB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB5;
#[doc = "`read()` method returns [otg_hcdmab5::R](otg_hcdmab5::R) reader structure"]
impl crate::Readable for OTG_HCDMAB5 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab5;
#[doc = "OTG host channel 6 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar6](otg_hcchar6) module"]
pub type OTG_HCCHAR6 = crate::Reg<u32, _OTG_HCCHAR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR6;
#[doc = "`read()` method returns [otg_hcchar6::R](otg_hcchar6::R) reader structure"]
impl crate::Readable for OTG_HCCHAR6 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar6::W](otg_hcchar6::W) writer structure"]
impl crate::Writable for OTG_HCCHAR6 {}
#[doc = "OTG host channel 6 characteristics register"]
pub mod otg_hcchar6;
#[doc = "OTG host channel 6 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt6](otg_hcsplt6) module"]
pub type OTG_HCSPLT6 = crate::Reg<u32, _OTG_HCSPLT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT6;
#[doc = "`read()` method returns [otg_hcsplt6::R](otg_hcsplt6::R) reader structure"]
impl crate::Readable for OTG_HCSPLT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt6::W](otg_hcsplt6::W) writer structure"]
impl crate::Writable for OTG_HCSPLT6 {}
#[doc = "OTG host channel 6 split control register"]
pub mod otg_hcsplt6;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint6](otg_hcint6) module"]
pub type OTG_HCINT6 = crate::Reg<u32, _OTG_HCINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT6;
#[doc = "`read()` method returns [otg_hcint6::R](otg_hcint6::R) reader structure"]
impl crate::Readable for OTG_HCINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint6::W](otg_hcint6::W) writer structure"]
impl crate::Writable for OTG_HCINT6 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint6;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk6](otg_hcintmsk6) module"]
pub type OTG_HCINTMSK6 = crate::Reg<u32, _OTG_HCINTMSK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK6;
#[doc = "`read()` method returns [otg_hcintmsk6::R](otg_hcintmsk6::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK6 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk6::W](otg_hcintmsk6::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK6 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk6;
#[doc = "OTG host channel 6 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz6](otg_hctsiz6) module"]
pub type OTG_HCTSIZ6 = crate::Reg<u32, _OTG_HCTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ6;
#[doc = "`read()` method returns [otg_hctsiz6::R](otg_hctsiz6::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz6::W](otg_hctsiz6::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ6 {}
#[doc = "OTG host channel 6 transfer size register"]
pub mod otg_hctsiz6;
#[doc = "OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma6](otg_hcdma6) module"]
pub type OTG_HCDMA6 = crate::Reg<u32, _OTG_HCDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA6;
#[doc = "`read()` method returns [otg_hcdma6::R](otg_hcdma6::R) reader structure"]
impl crate::Readable for OTG_HCDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma6::W](otg_hcdma6::W) writer structure"]
impl crate::Writable for OTG_HCDMA6 {}
#[doc = "OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma6;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab6](otg_hcdmab6) module"]
pub type OTG_HCDMAB6 = crate::Reg<u32, _OTG_HCDMAB6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB6;
#[doc = "`read()` method returns [otg_hcdmab6::R](otg_hcdmab6::R) reader structure"]
impl crate::Readable for OTG_HCDMAB6 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab6;
#[doc = "OTG host channel 7 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar7](otg_hcchar7) module"]
pub type OTG_HCCHAR7 = crate::Reg<u32, _OTG_HCCHAR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR7;
#[doc = "`read()` method returns [otg_hcchar7::R](otg_hcchar7::R) reader structure"]
impl crate::Readable for OTG_HCCHAR7 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar7::W](otg_hcchar7::W) writer structure"]
impl crate::Writable for OTG_HCCHAR7 {}
#[doc = "OTG host channel 7 characteristics register"]
pub mod otg_hcchar7;
#[doc = "OTG host channel 7 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt7](otg_hcsplt7) module"]
pub type OTG_HCSPLT7 = crate::Reg<u32, _OTG_HCSPLT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT7;
#[doc = "`read()` method returns [otg_hcsplt7::R](otg_hcsplt7::R) reader structure"]
impl crate::Readable for OTG_HCSPLT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt7::W](otg_hcsplt7::W) writer structure"]
impl crate::Writable for OTG_HCSPLT7 {}
#[doc = "OTG host channel 7 split control register"]
pub mod otg_hcsplt7;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint7](otg_hcint7) module"]
pub type OTG_HCINT7 = crate::Reg<u32, _OTG_HCINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT7;
#[doc = "`read()` method returns [otg_hcint7::R](otg_hcint7::R) reader structure"]
impl crate::Readable for OTG_HCINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint7::W](otg_hcint7::W) writer structure"]
impl crate::Writable for OTG_HCINT7 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint7;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk7](otg_hcintmsk7) module"]
pub type OTG_HCINTMSK7 = crate::Reg<u32, _OTG_HCINTMSK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK7;
#[doc = "`read()` method returns [otg_hcintmsk7::R](otg_hcintmsk7::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK7 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk7::W](otg_hcintmsk7::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK7 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk7;
#[doc = "OTG host channel 7 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz7](otg_hctsiz7) module"]
pub type OTG_HCTSIZ7 = crate::Reg<u32, _OTG_HCTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ7;
#[doc = "`read()` method returns [otg_hctsiz7::R](otg_hctsiz7::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz7::W](otg_hctsiz7::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ7 {}
#[doc = "OTG host channel 7 transfer size register"]
pub mod otg_hctsiz7;
#[doc = "OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma7](otg_hcdma7) module"]
pub type OTG_HCDMA7 = crate::Reg<u32, _OTG_HCDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA7;
#[doc = "`read()` method returns [otg_hcdma7::R](otg_hcdma7::R) reader structure"]
impl crate::Readable for OTG_HCDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma7::W](otg_hcdma7::W) writer structure"]
impl crate::Writable for OTG_HCDMA7 {}
#[doc = "OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma7;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab7](otg_hcdmab7) module"]
pub type OTG_HCDMAB7 = crate::Reg<u32, _OTG_HCDMAB7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB7;
#[doc = "`read()` method returns [otg_hcdmab7::R](otg_hcdmab7::R) reader structure"]
impl crate::Readable for OTG_HCDMAB7 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab7;
#[doc = "OTG host channel 8 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar8](otg_hcchar8) module"]
pub type OTG_HCCHAR8 = crate::Reg<u32, _OTG_HCCHAR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR8;
#[doc = "`read()` method returns [otg_hcchar8::R](otg_hcchar8::R) reader structure"]
impl crate::Readable for OTG_HCCHAR8 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar8::W](otg_hcchar8::W) writer structure"]
impl crate::Writable for OTG_HCCHAR8 {}
#[doc = "OTG host channel 8 characteristics register"]
pub mod otg_hcchar8;
#[doc = "OTG host channel 8 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt8](otg_hcsplt8) module"]
pub type OTG_HCSPLT8 = crate::Reg<u32, _OTG_HCSPLT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT8;
#[doc = "`read()` method returns [otg_hcsplt8::R](otg_hcsplt8::R) reader structure"]
impl crate::Readable for OTG_HCSPLT8 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt8::W](otg_hcsplt8::W) writer structure"]
impl crate::Writable for OTG_HCSPLT8 {}
#[doc = "OTG host channel 8 split control register"]
pub mod otg_hcsplt8;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint8](otg_hcint8) module"]
pub type OTG_HCINT8 = crate::Reg<u32, _OTG_HCINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT8;
#[doc = "`read()` method returns [otg_hcint8::R](otg_hcint8::R) reader structure"]
impl crate::Readable for OTG_HCINT8 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint8::W](otg_hcint8::W) writer structure"]
impl crate::Writable for OTG_HCINT8 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint8;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk8](otg_hcintmsk8) module"]
pub type OTG_HCINTMSK8 = crate::Reg<u32, _OTG_HCINTMSK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK8;
#[doc = "`read()` method returns [otg_hcintmsk8::R](otg_hcintmsk8::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK8 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk8::W](otg_hcintmsk8::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK8 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk8;
#[doc = "OTG host channel 8 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz8](otg_hctsiz8) module"]
pub type OTG_HCTSIZ8 = crate::Reg<u32, _OTG_HCTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ8;
#[doc = "`read()` method returns [otg_hctsiz8::R](otg_hctsiz8::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz8::W](otg_hctsiz8::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ8 {}
#[doc = "OTG host channel 8 transfer size register"]
pub mod otg_hctsiz8;
#[doc = "OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma8](otg_hcdma8) module"]
pub type OTG_HCDMA8 = crate::Reg<u32, _OTG_HCDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA8;
#[doc = "`read()` method returns [otg_hcdma8::R](otg_hcdma8::R) reader structure"]
impl crate::Readable for OTG_HCDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma8::W](otg_hcdma8::W) writer structure"]
impl crate::Writable for OTG_HCDMA8 {}
#[doc = "OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma8;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab8](otg_hcdmab8) module"]
pub type OTG_HCDMAB8 = crate::Reg<u32, _OTG_HCDMAB8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB8;
#[doc = "`read()` method returns [otg_hcdmab8::R](otg_hcdmab8::R) reader structure"]
impl crate::Readable for OTG_HCDMAB8 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab8;
#[doc = "OTG host channel 9 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar9](otg_hcchar9) module"]
pub type OTG_HCCHAR9 = crate::Reg<u32, _OTG_HCCHAR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR9;
#[doc = "`read()` method returns [otg_hcchar9::R](otg_hcchar9::R) reader structure"]
impl crate::Readable for OTG_HCCHAR9 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar9::W](otg_hcchar9::W) writer structure"]
impl crate::Writable for OTG_HCCHAR9 {}
#[doc = "OTG host channel 9 characteristics register"]
pub mod otg_hcchar9;
#[doc = "OTG host channel 9 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt9](otg_hcsplt9) module"]
pub type OTG_HCSPLT9 = crate::Reg<u32, _OTG_HCSPLT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT9;
#[doc = "`read()` method returns [otg_hcsplt9::R](otg_hcsplt9::R) reader structure"]
impl crate::Readable for OTG_HCSPLT9 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt9::W](otg_hcsplt9::W) writer structure"]
impl crate::Writable for OTG_HCSPLT9 {}
#[doc = "OTG host channel 9 split control register"]
pub mod otg_hcsplt9;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint9](otg_hcint9) module"]
pub type OTG_HCINT9 = crate::Reg<u32, _OTG_HCINT9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT9;
#[doc = "`read()` method returns [otg_hcint9::R](otg_hcint9::R) reader structure"]
impl crate::Readable for OTG_HCINT9 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint9::W](otg_hcint9::W) writer structure"]
impl crate::Writable for OTG_HCINT9 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint9;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk9](otg_hcintmsk9) module"]
pub type OTG_HCINTMSK9 = crate::Reg<u32, _OTG_HCINTMSK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK9;
#[doc = "`read()` method returns [otg_hcintmsk9::R](otg_hcintmsk9::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK9 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk9::W](otg_hcintmsk9::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK9 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk9;
#[doc = "OTG host channel 9 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz9](otg_hctsiz9) module"]
pub type OTG_HCTSIZ9 = crate::Reg<u32, _OTG_HCTSIZ9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ9;
#[doc = "`read()` method returns [otg_hctsiz9::R](otg_hctsiz9::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ9 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz9::W](otg_hctsiz9::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ9 {}
#[doc = "OTG host channel 9 transfer size register"]
pub mod otg_hctsiz9;
#[doc = "OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma9](otg_hcdma9) module"]
pub type OTG_HCDMA9 = crate::Reg<u32, _OTG_HCDMA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA9;
#[doc = "`read()` method returns [otg_hcdma9::R](otg_hcdma9::R) reader structure"]
impl crate::Readable for OTG_HCDMA9 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma9::W](otg_hcdma9::W) writer structure"]
impl crate::Writable for OTG_HCDMA9 {}
#[doc = "OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma9;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab9](otg_hcdmab9) module"]
pub type OTG_HCDMAB9 = crate::Reg<u32, _OTG_HCDMAB9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB9;
#[doc = "`read()` method returns [otg_hcdmab9::R](otg_hcdmab9::R) reader structure"]
impl crate::Readable for OTG_HCDMAB9 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab9;
#[doc = "OTG host channel 10 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar10](otg_hcchar10) module"]
pub type OTG_HCCHAR10 = crate::Reg<u32, _OTG_HCCHAR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR10;
#[doc = "`read()` method returns [otg_hcchar10::R](otg_hcchar10::R) reader structure"]
impl crate::Readable for OTG_HCCHAR10 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar10::W](otg_hcchar10::W) writer structure"]
impl crate::Writable for OTG_HCCHAR10 {}
#[doc = "OTG host channel 10 characteristics register"]
pub mod otg_hcchar10;
#[doc = "OTG host channel 10 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt10](otg_hcsplt10) module"]
pub type OTG_HCSPLT10 = crate::Reg<u32, _OTG_HCSPLT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT10;
#[doc = "`read()` method returns [otg_hcsplt10::R](otg_hcsplt10::R) reader structure"]
impl crate::Readable for OTG_HCSPLT10 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt10::W](otg_hcsplt10::W) writer structure"]
impl crate::Writable for OTG_HCSPLT10 {}
#[doc = "OTG host channel 10 split control register"]
pub mod otg_hcsplt10;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint10](otg_hcint10) module"]
pub type OTG_HCINT10 = crate::Reg<u32, _OTG_HCINT10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT10;
#[doc = "`read()` method returns [otg_hcint10::R](otg_hcint10::R) reader structure"]
impl crate::Readable for OTG_HCINT10 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint10::W](otg_hcint10::W) writer structure"]
impl crate::Writable for OTG_HCINT10 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint10;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk10](otg_hcintmsk10) module"]
pub type OTG_HCINTMSK10 = crate::Reg<u32, _OTG_HCINTMSK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK10;
#[doc = "`read()` method returns [otg_hcintmsk10::R](otg_hcintmsk10::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK10 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk10::W](otg_hcintmsk10::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK10 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk10;
#[doc = "OTG host channel 10 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz10](otg_hctsiz10) module"]
pub type OTG_HCTSIZ10 = crate::Reg<u32, _OTG_HCTSIZ10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ10;
#[doc = "`read()` method returns [otg_hctsiz10::R](otg_hctsiz10::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ10 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz10::W](otg_hctsiz10::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ10 {}
#[doc = "OTG host channel 10 transfer size register"]
pub mod otg_hctsiz10;
#[doc = "OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma10](otg_hcdma10) module"]
pub type OTG_HCDMA10 = crate::Reg<u32, _OTG_HCDMA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA10;
#[doc = "`read()` method returns [otg_hcdma10::R](otg_hcdma10::R) reader structure"]
impl crate::Readable for OTG_HCDMA10 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma10::W](otg_hcdma10::W) writer structure"]
impl crate::Writable for OTG_HCDMA10 {}
#[doc = "OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma10;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab10](otg_hcdmab10) module"]
pub type OTG_HCDMAB10 = crate::Reg<u32, _OTG_HCDMAB10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB10;
#[doc = "`read()` method returns [otg_hcdmab10::R](otg_hcdmab10::R) reader structure"]
impl crate::Readable for OTG_HCDMAB10 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab10;
#[doc = "OTG host channel 11 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar11](otg_hcchar11) module"]
pub type OTG_HCCHAR11 = crate::Reg<u32, _OTG_HCCHAR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR11;
#[doc = "`read()` method returns [otg_hcchar11::R](otg_hcchar11::R) reader structure"]
impl crate::Readable for OTG_HCCHAR11 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar11::W](otg_hcchar11::W) writer structure"]
impl crate::Writable for OTG_HCCHAR11 {}
#[doc = "OTG host channel 11 characteristics register"]
pub mod otg_hcchar11;
#[doc = "OTG host channel 11 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt11](otg_hcsplt11) module"]
pub type OTG_HCSPLT11 = crate::Reg<u32, _OTG_HCSPLT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT11;
#[doc = "`read()` method returns [otg_hcsplt11::R](otg_hcsplt11::R) reader structure"]
impl crate::Readable for OTG_HCSPLT11 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt11::W](otg_hcsplt11::W) writer structure"]
impl crate::Writable for OTG_HCSPLT11 {}
#[doc = "OTG host channel 11 split control register"]
pub mod otg_hcsplt11;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint11](otg_hcint11) module"]
pub type OTG_HCINT11 = crate::Reg<u32, _OTG_HCINT11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT11;
#[doc = "`read()` method returns [otg_hcint11::R](otg_hcint11::R) reader structure"]
impl crate::Readable for OTG_HCINT11 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint11::W](otg_hcint11::W) writer structure"]
impl crate::Writable for OTG_HCINT11 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint11;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk11](otg_hcintmsk11) module"]
pub type OTG_HCINTMSK11 = crate::Reg<u32, _OTG_HCINTMSK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK11;
#[doc = "`read()` method returns [otg_hcintmsk11::R](otg_hcintmsk11::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK11 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk11::W](otg_hcintmsk11::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK11 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk11;
#[doc = "OTG host channel 11 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz11](otg_hctsiz11) module"]
pub type OTG_HCTSIZ11 = crate::Reg<u32, _OTG_HCTSIZ11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ11;
#[doc = "`read()` method returns [otg_hctsiz11::R](otg_hctsiz11::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ11 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz11::W](otg_hctsiz11::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ11 {}
#[doc = "OTG host channel 11 transfer size register"]
pub mod otg_hctsiz11;
#[doc = "OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma11](otg_hcdma11) module"]
pub type OTG_HCDMA11 = crate::Reg<u32, _OTG_HCDMA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA11;
#[doc = "`read()` method returns [otg_hcdma11::R](otg_hcdma11::R) reader structure"]
impl crate::Readable for OTG_HCDMA11 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma11::W](otg_hcdma11::W) writer structure"]
impl crate::Writable for OTG_HCDMA11 {}
#[doc = "OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma11;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab11](otg_hcdmab11) module"]
pub type OTG_HCDMAB11 = crate::Reg<u32, _OTG_HCDMAB11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB11;
#[doc = "`read()` method returns [otg_hcdmab11::R](otg_hcdmab11::R) reader structure"]
impl crate::Readable for OTG_HCDMAB11 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab11;
#[doc = "OTG host channel 12 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar12](otg_hcchar12) module"]
pub type OTG_HCCHAR12 = crate::Reg<u32, _OTG_HCCHAR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR12;
#[doc = "`read()` method returns [otg_hcchar12::R](otg_hcchar12::R) reader structure"]
impl crate::Readable for OTG_HCCHAR12 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar12::W](otg_hcchar12::W) writer structure"]
impl crate::Writable for OTG_HCCHAR12 {}
#[doc = "OTG host channel 12 characteristics register"]
pub mod otg_hcchar12;
#[doc = "OTG host channel 12 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt12](otg_hcsplt12) module"]
pub type OTG_HCSPLT12 = crate::Reg<u32, _OTG_HCSPLT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT12;
#[doc = "`read()` method returns [otg_hcsplt12::R](otg_hcsplt12::R) reader structure"]
impl crate::Readable for OTG_HCSPLT12 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt12::W](otg_hcsplt12::W) writer structure"]
impl crate::Writable for OTG_HCSPLT12 {}
#[doc = "OTG host channel 12 split control register"]
pub mod otg_hcsplt12;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint12](otg_hcint12) module"]
pub type OTG_HCINT12 = crate::Reg<u32, _OTG_HCINT12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT12;
#[doc = "`read()` method returns [otg_hcint12::R](otg_hcint12::R) reader structure"]
impl crate::Readable for OTG_HCINT12 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint12::W](otg_hcint12::W) writer structure"]
impl crate::Writable for OTG_HCINT12 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint12;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk12](otg_hcintmsk12) module"]
pub type OTG_HCINTMSK12 = crate::Reg<u32, _OTG_HCINTMSK12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK12;
#[doc = "`read()` method returns [otg_hcintmsk12::R](otg_hcintmsk12::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK12 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk12::W](otg_hcintmsk12::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK12 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk12;
#[doc = "OTG host channel 12 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz12](otg_hctsiz12) module"]
pub type OTG_HCTSIZ12 = crate::Reg<u32, _OTG_HCTSIZ12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ12;
#[doc = "`read()` method returns [otg_hctsiz12::R](otg_hctsiz12::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ12 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz12::W](otg_hctsiz12::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ12 {}
#[doc = "OTG host channel 12 transfer size register"]
pub mod otg_hctsiz12;
#[doc = "OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma12](otg_hcdma12) module"]
pub type OTG_HCDMA12 = crate::Reg<u32, _OTG_HCDMA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA12;
#[doc = "`read()` method returns [otg_hcdma12::R](otg_hcdma12::R) reader structure"]
impl crate::Readable for OTG_HCDMA12 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma12::W](otg_hcdma12::W) writer structure"]
impl crate::Writable for OTG_HCDMA12 {}
#[doc = "OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma12;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab12](otg_hcdmab12) module"]
pub type OTG_HCDMAB12 = crate::Reg<u32, _OTG_HCDMAB12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB12;
#[doc = "`read()` method returns [otg_hcdmab12::R](otg_hcdmab12::R) reader structure"]
impl crate::Readable for OTG_HCDMAB12 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab12;
#[doc = "OTG host channel 13 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar13](otg_hcchar13) module"]
pub type OTG_HCCHAR13 = crate::Reg<u32, _OTG_HCCHAR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR13;
#[doc = "`read()` method returns [otg_hcchar13::R](otg_hcchar13::R) reader structure"]
impl crate::Readable for OTG_HCCHAR13 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar13::W](otg_hcchar13::W) writer structure"]
impl crate::Writable for OTG_HCCHAR13 {}
#[doc = "OTG host channel 13 characteristics register"]
pub mod otg_hcchar13;
#[doc = "OTG host channel 13 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt13](otg_hcsplt13) module"]
pub type OTG_HCSPLT13 = crate::Reg<u32, _OTG_HCSPLT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT13;
#[doc = "`read()` method returns [otg_hcsplt13::R](otg_hcsplt13::R) reader structure"]
impl crate::Readable for OTG_HCSPLT13 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt13::W](otg_hcsplt13::W) writer structure"]
impl crate::Writable for OTG_HCSPLT13 {}
#[doc = "OTG host channel 13 split control register"]
pub mod otg_hcsplt13;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint13](otg_hcint13) module"]
pub type OTG_HCINT13 = crate::Reg<u32, _OTG_HCINT13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT13;
#[doc = "`read()` method returns [otg_hcint13::R](otg_hcint13::R) reader structure"]
impl crate::Readable for OTG_HCINT13 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint13::W](otg_hcint13::W) writer structure"]
impl crate::Writable for OTG_HCINT13 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint13;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk13](otg_hcintmsk13) module"]
pub type OTG_HCINTMSK13 = crate::Reg<u32, _OTG_HCINTMSK13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK13;
#[doc = "`read()` method returns [otg_hcintmsk13::R](otg_hcintmsk13::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK13 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk13::W](otg_hcintmsk13::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK13 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk13;
#[doc = "OTG host channel 13 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz13](otg_hctsiz13) module"]
pub type OTG_HCTSIZ13 = crate::Reg<u32, _OTG_HCTSIZ13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ13;
#[doc = "`read()` method returns [otg_hctsiz13::R](otg_hctsiz13::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ13 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz13::W](otg_hctsiz13::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ13 {}
#[doc = "OTG host channel 13 transfer size register"]
pub mod otg_hctsiz13;
#[doc = "OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma13](otg_hcdma13) module"]
pub type OTG_HCDMA13 = crate::Reg<u32, _OTG_HCDMA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA13;
#[doc = "`read()` method returns [otg_hcdma13::R](otg_hcdma13::R) reader structure"]
impl crate::Readable for OTG_HCDMA13 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma13::W](otg_hcdma13::W) writer structure"]
impl crate::Writable for OTG_HCDMA13 {}
#[doc = "OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma13;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab13](otg_hcdmab13) module"]
pub type OTG_HCDMAB13 = crate::Reg<u32, _OTG_HCDMAB13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB13;
#[doc = "`read()` method returns [otg_hcdmab13::R](otg_hcdmab13::R) reader structure"]
impl crate::Readable for OTG_HCDMAB13 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab13;
#[doc = "OTG host channel 14 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar14](otg_hcchar14) module"]
pub type OTG_HCCHAR14 = crate::Reg<u32, _OTG_HCCHAR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR14;
#[doc = "`read()` method returns [otg_hcchar14::R](otg_hcchar14::R) reader structure"]
impl crate::Readable for OTG_HCCHAR14 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar14::W](otg_hcchar14::W) writer structure"]
impl crate::Writable for OTG_HCCHAR14 {}
#[doc = "OTG host channel 14 characteristics register"]
pub mod otg_hcchar14;
#[doc = "OTG host channel 14 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt14](otg_hcsplt14) module"]
pub type OTG_HCSPLT14 = crate::Reg<u32, _OTG_HCSPLT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT14;
#[doc = "`read()` method returns [otg_hcsplt14::R](otg_hcsplt14::R) reader structure"]
impl crate::Readable for OTG_HCSPLT14 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt14::W](otg_hcsplt14::W) writer structure"]
impl crate::Writable for OTG_HCSPLT14 {}
#[doc = "OTG host channel 14 split control register"]
pub mod otg_hcsplt14;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint14](otg_hcint14) module"]
pub type OTG_HCINT14 = crate::Reg<u32, _OTG_HCINT14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT14;
#[doc = "`read()` method returns [otg_hcint14::R](otg_hcint14::R) reader structure"]
impl crate::Readable for OTG_HCINT14 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint14::W](otg_hcint14::W) writer structure"]
impl crate::Writable for OTG_HCINT14 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint14;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk14](otg_hcintmsk14) module"]
pub type OTG_HCINTMSK14 = crate::Reg<u32, _OTG_HCINTMSK14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK14;
#[doc = "`read()` method returns [otg_hcintmsk14::R](otg_hcintmsk14::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK14 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk14::W](otg_hcintmsk14::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK14 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk14;
#[doc = "OTG host channel 14 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz14](otg_hctsiz14) module"]
pub type OTG_HCTSIZ14 = crate::Reg<u32, _OTG_HCTSIZ14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ14;
#[doc = "`read()` method returns [otg_hctsiz14::R](otg_hctsiz14::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ14 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz14::W](otg_hctsiz14::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ14 {}
#[doc = "OTG host channel 14 transfer size register"]
pub mod otg_hctsiz14;
#[doc = "OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma14](otg_hcdma14) module"]
pub type OTG_HCDMA14 = crate::Reg<u32, _OTG_HCDMA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA14;
#[doc = "`read()` method returns [otg_hcdma14::R](otg_hcdma14::R) reader structure"]
impl crate::Readable for OTG_HCDMA14 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma14::W](otg_hcdma14::W) writer structure"]
impl crate::Writable for OTG_HCDMA14 {}
#[doc = "OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma14;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab14](otg_hcdmab14) module"]
pub type OTG_HCDMAB14 = crate::Reg<u32, _OTG_HCDMAB14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB14;
#[doc = "`read()` method returns [otg_hcdmab14::R](otg_hcdmab14::R) reader structure"]
impl crate::Readable for OTG_HCDMAB14 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab14;
#[doc = "OTG host channel 15 characteristics register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcchar15](otg_hcchar15) module"]
pub type OTG_HCCHAR15 = crate::Reg<u32, _OTG_HCCHAR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCCHAR15;
#[doc = "`read()` method returns [otg_hcchar15::R](otg_hcchar15::R) reader structure"]
impl crate::Readable for OTG_HCCHAR15 {}
#[doc = "`write(|w| ..)` method takes [otg_hcchar15::W](otg_hcchar15::W) writer structure"]
impl crate::Writable for OTG_HCCHAR15 {}
#[doc = "OTG host channel 15 characteristics register"]
pub mod otg_hcchar15;
#[doc = "OTG host channel 15 split control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcsplt15](otg_hcsplt15) module"]
pub type OTG_HCSPLT15 = crate::Reg<u32, _OTG_HCSPLT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCSPLT15;
#[doc = "`read()` method returns [otg_hcsplt15::R](otg_hcsplt15::R) reader structure"]
impl crate::Readable for OTG_HCSPLT15 {}
#[doc = "`write(|w| ..)` method takes [otg_hcsplt15::W](otg_hcsplt15::W) writer structure"]
impl crate::Writable for OTG_HCSPLT15 {}
#[doc = "OTG host channel 15 split control register"]
pub mod otg_hcsplt15;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcint15](otg_hcint15) module"]
pub type OTG_HCINT15 = crate::Reg<u32, _OTG_HCINT15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINT15;
#[doc = "`read()` method returns [otg_hcint15::R](otg_hcint15::R) reader structure"]
impl crate::Readable for OTG_HCINT15 {}
#[doc = "`write(|w| ..)` method takes [otg_hcint15::W](otg_hcint15::W) writer structure"]
impl crate::Writable for OTG_HCINT15 {}
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint15;
#[doc = "This register reflects the mask for each channel status described in the previous section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcintmsk15](otg_hcintmsk15) module"]
pub type OTG_HCINTMSK15 = crate::Reg<u32, _OTG_HCINTMSK15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCINTMSK15;
#[doc = "`read()` method returns [otg_hcintmsk15::R](otg_hcintmsk15::R) reader structure"]
impl crate::Readable for OTG_HCINTMSK15 {}
#[doc = "`write(|w| ..)` method takes [otg_hcintmsk15::W](otg_hcintmsk15::W) writer structure"]
impl crate::Writable for OTG_HCINTMSK15 {}
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk15;
#[doc = "OTG host channel 15 transfer size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hctsiz15](otg_hctsiz15) module"]
pub type OTG_HCTSIZ15 = crate::Reg<u32, _OTG_HCTSIZ15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCTSIZ15;
#[doc = "`read()` method returns [otg_hctsiz15::R](otg_hctsiz15::R) reader structure"]
impl crate::Readable for OTG_HCTSIZ15 {}
#[doc = "`write(|w| ..)` method takes [otg_hctsiz15::W](otg_hctsiz15::W) writer structure"]
impl crate::Writable for OTG_HCTSIZ15 {}
#[doc = "OTG host channel 15 transfer size register"]
pub mod otg_hctsiz15;
#[doc = "OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma15](otg_hcdma15) module"]
pub type OTG_HCDMA15 = crate::Reg<u32, _OTG_HCDMA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMA15;
#[doc = "`read()` method returns [otg_hcdma15::R](otg_hcdma15::R) reader structure"]
impl crate::Readable for OTG_HCDMA15 {}
#[doc = "`write(|w| ..)` method takes [otg_hcdma15::W](otg_hcdma15::W) writer structure"]
impl crate::Writable for OTG_HCDMA15 {}
#[doc = "OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma15;
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab15](otg_hcdmab15) module"]
pub type OTG_HCDMAB15 = crate::Reg<u32, _OTG_HCDMAB15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HCDMAB15;
#[doc = "`read()` method returns [otg_hcdmab15::R](otg_hcdmab15::R) reader structure"]
impl crate::Readable for OTG_HCDMAB15 {}
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab15;
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dcfg](otg_dcfg) module"]
pub type OTG_DCFG = crate::Reg<u32, _OTG_DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DCFG;
#[doc = "`read()` method returns [otg_dcfg::R](otg_dcfg::R) reader structure"]
impl crate::Readable for OTG_DCFG {}
#[doc = "`write(|w| ..)` method takes [otg_dcfg::W](otg_dcfg::W) writer structure"]
impl crate::Writable for OTG_DCFG {}
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
pub mod otg_dcfg;
#[doc = "OTG device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dctl](otg_dctl) module"]
pub type OTG_DCTL = crate::Reg<u32, _OTG_DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DCTL;
#[doc = "`read()` method returns [otg_dctl::R](otg_dctl::R) reader structure"]
impl crate::Readable for OTG_DCTL {}
#[doc = "`write(|w| ..)` method takes [otg_dctl::W](otg_dctl::W) writer structure"]
impl crate::Writable for OTG_DCTL {}
#[doc = "OTG device control register"]
pub mod otg_dctl;
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dsts](otg_dsts) module"]
pub type OTG_DSTS = crate::Reg<u32, _OTG_DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DSTS;
#[doc = "`read()` method returns [otg_dsts::R](otg_dsts::R) reader structure"]
impl crate::Readable for OTG_DSTS {}
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
pub mod otg_dsts;
#[doc = "This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepmsk](otg_diepmsk) module"]
pub type OTG_DIEPMSK = crate::Reg<u32, _OTG_DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPMSK;
#[doc = "`read()` method returns [otg_diepmsk::R](otg_diepmsk::R) reader structure"]
impl crate::Readable for OTG_DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_diepmsk::W](otg_diepmsk::W) writer structure"]
impl crate::Writable for OTG_DIEPMSK {}
#[doc = "This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_diepmsk;
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepmsk](otg_doepmsk) module"]
pub type OTG_DOEPMSK = crate::Reg<u32, _OTG_DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPMSK;
#[doc = "`read()` method returns [otg_doepmsk::R](otg_doepmsk::R) reader structure"]
impl crate::Readable for OTG_DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_doepmsk::W](otg_doepmsk::W) writer structure"]
impl crate::Writable for OTG_DOEPMSK {}
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_doepmsk;
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_daint](otg_daint) module"]
pub type OTG_DAINT = crate::Reg<u32, _OTG_DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DAINT;
#[doc = "`read()` method returns [otg_daint::R](otg_daint::R) reader structure"]
impl crate::Readable for OTG_DAINT {}
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
pub mod otg_daint;
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_daintmsk](otg_daintmsk) module"]
pub type OTG_DAINTMSK = crate::Reg<u32, _OTG_DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DAINTMSK;
#[doc = "`read()` method returns [otg_daintmsk::R](otg_daintmsk::R) reader structure"]
impl crate::Readable for OTG_DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_daintmsk::W](otg_daintmsk::W) writer structure"]
impl crate::Writable for OTG_DAINTMSK {}
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
pub mod otg_daintmsk;
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dvbusdis](otg_dvbusdis) module"]
pub type OTG_DVBUSDIS = crate::Reg<u32, _OTG_DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DVBUSDIS;
#[doc = "`read()` method returns [otg_dvbusdis::R](otg_dvbusdis::R) reader structure"]
impl crate::Readable for OTG_DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [otg_dvbusdis::W](otg_dvbusdis::W) writer structure"]
impl crate::Writable for OTG_DVBUSDIS {}
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
pub mod otg_dvbusdis;
#[doc = "This register specifies the VBUS pulsing time during SRP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dvbuspulse](otg_dvbuspulse) module"]
pub type OTG_DVBUSPULSE = crate::Reg<u32, _OTG_DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DVBUSPULSE;
#[doc = "`read()` method returns [otg_dvbuspulse::R](otg_dvbuspulse::R) reader structure"]
impl crate::Readable for OTG_DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [otg_dvbuspulse::W](otg_dvbuspulse::W) writer structure"]
impl crate::Writable for OTG_DVBUSPULSE {}
#[doc = "This register specifies the VBUS pulsing time during SRP."]
pub mod otg_dvbuspulse;
#[doc = "OTG device threshold control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dthrctl](otg_dthrctl) module"]
pub type OTG_DTHRCTL = crate::Reg<u32, _OTG_DTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTHRCTL;
#[doc = "`read()` method returns [otg_dthrctl::R](otg_dthrctl::R) reader structure"]
impl crate::Readable for OTG_DTHRCTL {}
#[doc = "`write(|w| ..)` method takes [otg_dthrctl::W](otg_dthrctl::W) writer structure"]
impl crate::Writable for OTG_DTHRCTL {}
#[doc = "OTG device threshold control register"]
pub mod otg_dthrctl;
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepempmsk](otg_diepempmsk) module"]
pub type OTG_DIEPEMPMSK = crate::Reg<u32, _OTG_DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPEMPMSK;
#[doc = "`read()` method returns [otg_diepempmsk::R](otg_diepempmsk::R) reader structure"]
impl crate::Readable for OTG_DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [otg_diepempmsk::W](otg_diepempmsk::W) writer structure"]
impl crate::Writable for OTG_DIEPEMPMSK {}
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
pub mod otg_diepempmsk;
#[doc = "OTG device each endpoint interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_deachint](otg_deachint) module"]
pub type OTG_DEACHINT = crate::Reg<u32, _OTG_DEACHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DEACHINT;
#[doc = "`read()` method returns [otg_deachint::R](otg_deachint::R) reader structure"]
impl crate::Readable for OTG_DEACHINT {}
#[doc = "OTG device each endpoint interrupt register"]
pub mod otg_deachint;
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_deachintmsk](otg_deachintmsk) module"]
pub type OTG_DEACHINTMSK = crate::Reg<u32, _OTG_DEACHINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DEACHINTMSK;
#[doc = "`read()` method returns [otg_deachintmsk::R](otg_deachintmsk::R) reader structure"]
impl crate::Readable for OTG_DEACHINTMSK {}
#[doc = "`write(|w| ..)` method takes [otg_deachintmsk::W](otg_deachintmsk::W) writer structure"]
impl crate::Writable for OTG_DEACHINTMSK {}
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
pub mod otg_deachintmsk;
#[doc = "This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_diepeachmsk1](otg_hs_diepeachmsk1) module"]
pub type OTG_HS_DIEPEACHMSK1 = crate::Reg<u32, _OTG_HS_DIEPEACHMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DIEPEACHMSK1;
#[doc = "`read()` method returns [otg_hs_diepeachmsk1::R](otg_hs_diepeachmsk1::R) reader structure"]
impl crate::Readable for OTG_HS_DIEPEACHMSK1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_diepeachmsk1::W](otg_hs_diepeachmsk1::W) writer structure"]
impl crate::Writable for OTG_HS_DIEPEACHMSK1 {}
#[doc = "This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_diepeachmsk1;
#[doc = "This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepeachmsk1](otg_hs_doepeachmsk1) module"]
pub type OTG_HS_DOEPEACHMSK1 = crate::Reg<u32, _OTG_HS_DOEPEACHMSK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_HS_DOEPEACHMSK1;
#[doc = "`read()` method returns [otg_hs_doepeachmsk1::R](otg_hs_doepeachmsk1::R) reader structure"]
impl crate::Readable for OTG_HS_DOEPEACHMSK1 {}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepeachmsk1::W](otg_hs_doepeachmsk1::W) writer structure"]
impl crate::Writable for OTG_HS_DOEPEACHMSK1 {}
#[doc = "This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_doepeachmsk1;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl0](otg_diepctl0) module"]
pub type OTG_DIEPCTL0 = crate::Reg<u32, _OTG_DIEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL0;
#[doc = "`read()` method returns [otg_diepctl0::R](otg_diepctl0::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl0::W](otg_diepctl0::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL0 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl0;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint0](otg_diepint0) module"]
pub type OTG_DIEPINT0 = crate::Reg<u32, _OTG_DIEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT0;
#[doc = "`read()` method returns [otg_diepint0::R](otg_diepint0::R) reader structure"]
impl crate::Readable for OTG_DIEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint0::W](otg_diepint0::W) writer structure"]
impl crate::Writable for OTG_DIEPINT0 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint0;
#[doc = "The application must modify this register before enabling endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz0](otg_dieptsiz0) module"]
pub type OTG_DIEPTSIZ0 = crate::Reg<u32, _OTG_DIEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ0;
#[doc = "`read()` method returns [otg_dieptsiz0::R](otg_dieptsiz0::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz0::W](otg_dieptsiz0::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ0 {}
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_dieptsiz0;
#[doc = "OTG device IN endpoint 0 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma0](otg_diepdma0) module"]
pub type OTG_DIEPDMA0 = crate::Reg<u32, _OTG_DIEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA0;
#[doc = "`read()` method returns [otg_diepdma0::R](otg_diepdma0::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma0::W](otg_diepdma0::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA0 {}
#[doc = "OTG device IN endpoint 0 DMA address register"]
pub mod otg_diepdma0;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts0](otg_dtxfsts0) module"]
pub type OTG_DTXFSTS0 = crate::Reg<u32, _OTG_DTXFSTS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS0;
#[doc = "`read()` method returns [otg_dtxfsts0::R](otg_dtxfsts0::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS0 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts0;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl1](otg_diepctl1) module"]
pub type OTG_DIEPCTL1 = crate::Reg<u32, _OTG_DIEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL1;
#[doc = "`read()` method returns [otg_diepctl1::R](otg_diepctl1::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl1::W](otg_diepctl1::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL1 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl1;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint1](otg_diepint1) module"]
pub type OTG_DIEPINT1 = crate::Reg<u32, _OTG_DIEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT1;
#[doc = "`read()` method returns [otg_diepint1::R](otg_diepint1::R) reader structure"]
impl crate::Readable for OTG_DIEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint1::W](otg_diepint1::W) writer structure"]
impl crate::Writable for OTG_DIEPINT1 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint1;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz1](otg_dieptsiz1) module"]
pub type OTG_DIEPTSIZ1 = crate::Reg<u32, _OTG_DIEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ1;
#[doc = "`read()` method returns [otg_dieptsiz1::R](otg_dieptsiz1::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz1::W](otg_dieptsiz1::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ1 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz1;
#[doc = "OTG device IN endpoint 1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma1](otg_diepdma1) module"]
pub type OTG_DIEPDMA1 = crate::Reg<u32, _OTG_DIEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA1;
#[doc = "`read()` method returns [otg_diepdma1::R](otg_diepdma1::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma1::W](otg_diepdma1::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA1 {}
#[doc = "OTG device IN endpoint 1 DMA address register"]
pub mod otg_diepdma1;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts1](otg_dtxfsts1) module"]
pub type OTG_DTXFSTS1 = crate::Reg<u32, _OTG_DTXFSTS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS1;
#[doc = "`read()` method returns [otg_dtxfsts1::R](otg_dtxfsts1::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS1 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts1;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl2](otg_diepctl2) module"]
pub type OTG_DIEPCTL2 = crate::Reg<u32, _OTG_DIEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL2;
#[doc = "`read()` method returns [otg_diepctl2::R](otg_diepctl2::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl2::W](otg_diepctl2::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL2 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl2;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint2](otg_diepint2) module"]
pub type OTG_DIEPINT2 = crate::Reg<u32, _OTG_DIEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT2;
#[doc = "`read()` method returns [otg_diepint2::R](otg_diepint2::R) reader structure"]
impl crate::Readable for OTG_DIEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint2::W](otg_diepint2::W) writer structure"]
impl crate::Writable for OTG_DIEPINT2 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint2;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz2](otg_dieptsiz2) module"]
pub type OTG_DIEPTSIZ2 = crate::Reg<u32, _OTG_DIEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ2;
#[doc = "`read()` method returns [otg_dieptsiz2::R](otg_dieptsiz2::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz2::W](otg_dieptsiz2::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ2 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz2;
#[doc = "OTG device IN endpoint 2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma2](otg_diepdma2) module"]
pub type OTG_DIEPDMA2 = crate::Reg<u32, _OTG_DIEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA2;
#[doc = "`read()` method returns [otg_diepdma2::R](otg_diepdma2::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma2::W](otg_diepdma2::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA2 {}
#[doc = "OTG device IN endpoint 2 DMA address register"]
pub mod otg_diepdma2;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts2](otg_dtxfsts2) module"]
pub type OTG_DTXFSTS2 = crate::Reg<u32, _OTG_DTXFSTS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS2;
#[doc = "`read()` method returns [otg_dtxfsts2::R](otg_dtxfsts2::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS2 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts2;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl3](otg_diepctl3) module"]
pub type OTG_DIEPCTL3 = crate::Reg<u32, _OTG_DIEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL3;
#[doc = "`read()` method returns [otg_diepctl3::R](otg_diepctl3::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl3::W](otg_diepctl3::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL3 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl3;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint3](otg_diepint3) module"]
pub type OTG_DIEPINT3 = crate::Reg<u32, _OTG_DIEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT3;
#[doc = "`read()` method returns [otg_diepint3::R](otg_diepint3::R) reader structure"]
impl crate::Readable for OTG_DIEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint3::W](otg_diepint3::W) writer structure"]
impl crate::Writable for OTG_DIEPINT3 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint3;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz3](otg_dieptsiz3) module"]
pub type OTG_DIEPTSIZ3 = crate::Reg<u32, _OTG_DIEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ3;
#[doc = "`read()` method returns [otg_dieptsiz3::R](otg_dieptsiz3::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz3::W](otg_dieptsiz3::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ3 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz3;
#[doc = "OTG device IN endpoint 3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma3](otg_diepdma3) module"]
pub type OTG_DIEPDMA3 = crate::Reg<u32, _OTG_DIEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA3;
#[doc = "`read()` method returns [otg_diepdma3::R](otg_diepdma3::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma3::W](otg_diepdma3::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA3 {}
#[doc = "OTG device IN endpoint 3 DMA address register"]
pub mod otg_diepdma3;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts3](otg_dtxfsts3) module"]
pub type OTG_DTXFSTS3 = crate::Reg<u32, _OTG_DTXFSTS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS3;
#[doc = "`read()` method returns [otg_dtxfsts3::R](otg_dtxfsts3::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS3 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts3;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl4](otg_diepctl4) module"]
pub type OTG_DIEPCTL4 = crate::Reg<u32, _OTG_DIEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL4;
#[doc = "`read()` method returns [otg_diepctl4::R](otg_diepctl4::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl4::W](otg_diepctl4::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL4 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl4;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint4](otg_diepint4) module"]
pub type OTG_DIEPINT4 = crate::Reg<u32, _OTG_DIEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT4;
#[doc = "`read()` method returns [otg_diepint4::R](otg_diepint4::R) reader structure"]
impl crate::Readable for OTG_DIEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint4::W](otg_diepint4::W) writer structure"]
impl crate::Writable for OTG_DIEPINT4 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint4;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz4](otg_dieptsiz4) module"]
pub type OTG_DIEPTSIZ4 = crate::Reg<u32, _OTG_DIEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ4;
#[doc = "`read()` method returns [otg_dieptsiz4::R](otg_dieptsiz4::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz4::W](otg_dieptsiz4::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ4 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz4;
#[doc = "OTG device IN endpoint 4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma4](otg_diepdma4) module"]
pub type OTG_DIEPDMA4 = crate::Reg<u32, _OTG_DIEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA4;
#[doc = "`read()` method returns [otg_diepdma4::R](otg_diepdma4::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma4::W](otg_diepdma4::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA4 {}
#[doc = "OTG device IN endpoint 4 DMA address register"]
pub mod otg_diepdma4;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts4](otg_dtxfsts4) module"]
pub type OTG_DTXFSTS4 = crate::Reg<u32, _OTG_DTXFSTS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS4;
#[doc = "`read()` method returns [otg_dtxfsts4::R](otg_dtxfsts4::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS4 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts4;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl5](otg_diepctl5) module"]
pub type OTG_DIEPCTL5 = crate::Reg<u32, _OTG_DIEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL5;
#[doc = "`read()` method returns [otg_diepctl5::R](otg_diepctl5::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl5::W](otg_diepctl5::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL5 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl5;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint5](otg_diepint5) module"]
pub type OTG_DIEPINT5 = crate::Reg<u32, _OTG_DIEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT5;
#[doc = "`read()` method returns [otg_diepint5::R](otg_diepint5::R) reader structure"]
impl crate::Readable for OTG_DIEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint5::W](otg_diepint5::W) writer structure"]
impl crate::Writable for OTG_DIEPINT5 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint5;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz5](otg_dieptsiz5) module"]
pub type OTG_DIEPTSIZ5 = crate::Reg<u32, _OTG_DIEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ5;
#[doc = "`read()` method returns [otg_dieptsiz5::R](otg_dieptsiz5::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz5::W](otg_dieptsiz5::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ5 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz5;
#[doc = "OTG device IN endpoint 5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma5](otg_diepdma5) module"]
pub type OTG_DIEPDMA5 = crate::Reg<u32, _OTG_DIEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA5;
#[doc = "`read()` method returns [otg_diepdma5::R](otg_diepdma5::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma5::W](otg_diepdma5::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA5 {}
#[doc = "OTG device IN endpoint 5 DMA address register"]
pub mod otg_diepdma5;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts5](otg_dtxfsts5) module"]
pub type OTG_DTXFSTS5 = crate::Reg<u32, _OTG_DTXFSTS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS5;
#[doc = "`read()` method returns [otg_dtxfsts5::R](otg_dtxfsts5::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS5 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts5;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl6](otg_diepctl6) module"]
pub type OTG_DIEPCTL6 = crate::Reg<u32, _OTG_DIEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL6;
#[doc = "`read()` method returns [otg_diepctl6::R](otg_diepctl6::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL6 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl6::W](otg_diepctl6::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL6 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl6;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint6](otg_diepint6) module"]
pub type OTG_DIEPINT6 = crate::Reg<u32, _OTG_DIEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT6;
#[doc = "`read()` method returns [otg_diepint6::R](otg_diepint6::R) reader structure"]
impl crate::Readable for OTG_DIEPINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint6::W](otg_diepint6::W) writer structure"]
impl crate::Writable for OTG_DIEPINT6 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint6;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz6](otg_dieptsiz6) module"]
pub type OTG_DIEPTSIZ6 = crate::Reg<u32, _OTG_DIEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ6;
#[doc = "`read()` method returns [otg_dieptsiz6::R](otg_dieptsiz6::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz6::W](otg_dieptsiz6::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ6 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz6;
#[doc = "OTG device IN endpoint 6 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma6](otg_diepdma6) module"]
pub type OTG_DIEPDMA6 = crate::Reg<u32, _OTG_DIEPDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA6;
#[doc = "`read()` method returns [otg_diepdma6::R](otg_diepdma6::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma6::W](otg_diepdma6::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA6 {}
#[doc = "OTG device IN endpoint 6 DMA address register"]
pub mod otg_diepdma6;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts6](otg_dtxfsts6) module"]
pub type OTG_DTXFSTS6 = crate::Reg<u32, _OTG_DTXFSTS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS6;
#[doc = "`read()` method returns [otg_dtxfsts6::R](otg_dtxfsts6::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS6 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts6;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl7](otg_diepctl7) module"]
pub type OTG_DIEPCTL7 = crate::Reg<u32, _OTG_DIEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL7;
#[doc = "`read()` method returns [otg_diepctl7::R](otg_diepctl7::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL7 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl7::W](otg_diepctl7::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL7 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl7;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint7](otg_diepint7) module"]
pub type OTG_DIEPINT7 = crate::Reg<u32, _OTG_DIEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT7;
#[doc = "`read()` method returns [otg_diepint7::R](otg_diepint7::R) reader structure"]
impl crate::Readable for OTG_DIEPINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint7::W](otg_diepint7::W) writer structure"]
impl crate::Writable for OTG_DIEPINT7 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint7;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz7](otg_dieptsiz7) module"]
pub type OTG_DIEPTSIZ7 = crate::Reg<u32, _OTG_DIEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ7;
#[doc = "`read()` method returns [otg_dieptsiz7::R](otg_dieptsiz7::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz7::W](otg_dieptsiz7::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ7 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz7;
#[doc = "OTG device IN endpoint 7 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma7](otg_diepdma7) module"]
pub type OTG_DIEPDMA7 = crate::Reg<u32, _OTG_DIEPDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA7;
#[doc = "`read()` method returns [otg_diepdma7::R](otg_diepdma7::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma7::W](otg_diepdma7::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA7 {}
#[doc = "OTG device IN endpoint 7 DMA address register"]
pub mod otg_diepdma7;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts7](otg_dtxfsts7) module"]
pub type OTG_DTXFSTS7 = crate::Reg<u32, _OTG_DTXFSTS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS7;
#[doc = "`read()` method returns [otg_dtxfsts7::R](otg_dtxfsts7::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS7 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts7;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepctl8](otg_diepctl8) module"]
pub type OTG_DIEPCTL8 = crate::Reg<u32, _OTG_DIEPCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPCTL8;
#[doc = "`read()` method returns [otg_diepctl8::R](otg_diepctl8::R) reader structure"]
impl crate::Readable for OTG_DIEPCTL8 {}
#[doc = "`write(|w| ..)` method takes [otg_diepctl8::W](otg_diepctl8::W) writer structure"]
impl crate::Writable for OTG_DIEPCTL8 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl8;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepint8](otg_diepint8) module"]
pub type OTG_DIEPINT8 = crate::Reg<u32, _OTG_DIEPINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPINT8;
#[doc = "`read()` method returns [otg_diepint8::R](otg_diepint8::R) reader structure"]
impl crate::Readable for OTG_DIEPINT8 {}
#[doc = "`write(|w| ..)` method takes [otg_diepint8::W](otg_diepint8::W) writer structure"]
impl crate::Writable for OTG_DIEPINT8 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint8;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dieptsiz8](otg_dieptsiz8) module"]
pub type OTG_DIEPTSIZ8 = crate::Reg<u32, _OTG_DIEPTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPTSIZ8;
#[doc = "`read()` method returns [otg_dieptsiz8::R](otg_dieptsiz8::R) reader structure"]
impl crate::Readable for OTG_DIEPTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [otg_dieptsiz8::W](otg_dieptsiz8::W) writer structure"]
impl crate::Writable for OTG_DIEPTSIZ8 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz8;
#[doc = "OTG device IN endpoint 8 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepdma8](otg_diepdma8) module"]
pub type OTG_DIEPDMA8 = crate::Reg<u32, _OTG_DIEPDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DIEPDMA8;
#[doc = "`read()` method returns [otg_diepdma8::R](otg_diepdma8::R) reader structure"]
impl crate::Readable for OTG_DIEPDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_diepdma8::W](otg_diepdma8::W) writer structure"]
impl crate::Writable for OTG_DIEPDMA8 {}
#[doc = "OTG device IN endpoint 8 DMA address register"]
pub mod otg_diepdma8;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts8](otg_dtxfsts8) module"]
pub type OTG_DTXFSTS8 = crate::Reg<u32, _OTG_DTXFSTS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DTXFSTS8;
#[doc = "`read()` method returns [otg_dtxfsts8::R](otg_dtxfsts8::R) reader structure"]
impl crate::Readable for OTG_DTXFSTS8 {}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts8;
#[doc = "This section describes the OTG_DOEPCTL0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl0](otg_doepctl0) module"]
pub type OTG_DOEPCTL0 = crate::Reg<u32, _OTG_DOEPCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL0;
#[doc = "`read()` method returns [otg_doepctl0::R](otg_doepctl0::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL0 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl0::W](otg_doepctl0::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL0 {}
#[doc = "This section describes the OTG_DOEPCTL0 register."]
pub mod otg_doepctl0;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint0](otg_doepint0) module"]
pub type OTG_DOEPINT0 = crate::Reg<u32, _OTG_DOEPINT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT0;
#[doc = "`read()` method returns [otg_doepint0::R](otg_doepint0::R) reader structure"]
impl crate::Readable for OTG_DOEPINT0 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint0::W](otg_doepint0::W) writer structure"]
impl crate::Writable for OTG_DOEPINT0 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint0;
#[doc = "The application must modify this register before enabling endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz0](otg_doeptsiz0) module"]
pub type OTG_DOEPTSIZ0 = crate::Reg<u32, _OTG_DOEPTSIZ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ0;
#[doc = "`read()` method returns [otg_doeptsiz0::R](otg_doeptsiz0::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ0 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz0::W](otg_doeptsiz0::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ0 {}
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_doeptsiz0;
#[doc = "OTG device OUT endpoint 0 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma0](otg_doepdma0) module"]
pub type OTG_DOEPDMA0 = crate::Reg<u32, _OTG_DOEPDMA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA0;
#[doc = "`read()` method returns [otg_doepdma0::R](otg_doepdma0::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA0 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma0::W](otg_doepdma0::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA0 {}
#[doc = "OTG device OUT endpoint 0 DMA address register"]
pub mod otg_doepdma0;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl1](otg_doepctl1) module"]
pub type OTG_DOEPCTL1 = crate::Reg<u32, _OTG_DOEPCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL1;
#[doc = "`read()` method returns [otg_doepctl1::R](otg_doepctl1::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL1 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl1::W](otg_doepctl1::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL1 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl1;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint1](otg_doepint1) module"]
pub type OTG_DOEPINT1 = crate::Reg<u32, _OTG_DOEPINT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT1;
#[doc = "`read()` method returns [otg_doepint1::R](otg_doepint1::R) reader structure"]
impl crate::Readable for OTG_DOEPINT1 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint1::W](otg_doepint1::W) writer structure"]
impl crate::Writable for OTG_DOEPINT1 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint1;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz1](otg_doeptsiz1) module"]
pub type OTG_DOEPTSIZ1 = crate::Reg<u32, _OTG_DOEPTSIZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ1;
#[doc = "`read()` method returns [otg_doeptsiz1::R](otg_doeptsiz1::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ1 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz1::W](otg_doeptsiz1::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ1 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz1;
#[doc = "OTG device OUT endpoint 1 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma1](otg_doepdma1) module"]
pub type OTG_DOEPDMA1 = crate::Reg<u32, _OTG_DOEPDMA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA1;
#[doc = "`read()` method returns [otg_doepdma1::R](otg_doepdma1::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA1 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma1::W](otg_doepdma1::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA1 {}
#[doc = "OTG device OUT endpoint 1 DMA address register"]
pub mod otg_doepdma1;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl2](otg_doepctl2) module"]
pub type OTG_DOEPCTL2 = crate::Reg<u32, _OTG_DOEPCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL2;
#[doc = "`read()` method returns [otg_doepctl2::R](otg_doepctl2::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL2 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl2::W](otg_doepctl2::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL2 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl2;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint2](otg_doepint2) module"]
pub type OTG_DOEPINT2 = crate::Reg<u32, _OTG_DOEPINT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT2;
#[doc = "`read()` method returns [otg_doepint2::R](otg_doepint2::R) reader structure"]
impl crate::Readable for OTG_DOEPINT2 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint2::W](otg_doepint2::W) writer structure"]
impl crate::Writable for OTG_DOEPINT2 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint2;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz2](otg_doeptsiz2) module"]
pub type OTG_DOEPTSIZ2 = crate::Reg<u32, _OTG_DOEPTSIZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ2;
#[doc = "`read()` method returns [otg_doeptsiz2::R](otg_doeptsiz2::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ2 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz2::W](otg_doeptsiz2::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ2 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz2;
#[doc = "OTG device OUT endpoint 2 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma2](otg_doepdma2) module"]
pub type OTG_DOEPDMA2 = crate::Reg<u32, _OTG_DOEPDMA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA2;
#[doc = "`read()` method returns [otg_doepdma2::R](otg_doepdma2::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA2 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma2::W](otg_doepdma2::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA2 {}
#[doc = "OTG device OUT endpoint 2 DMA address register"]
pub mod otg_doepdma2;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl3](otg_doepctl3) module"]
pub type OTG_DOEPCTL3 = crate::Reg<u32, _OTG_DOEPCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL3;
#[doc = "`read()` method returns [otg_doepctl3::R](otg_doepctl3::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL3 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl3::W](otg_doepctl3::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL3 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl3;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint3](otg_doepint3) module"]
pub type OTG_DOEPINT3 = crate::Reg<u32, _OTG_DOEPINT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT3;
#[doc = "`read()` method returns [otg_doepint3::R](otg_doepint3::R) reader structure"]
impl crate::Readable for OTG_DOEPINT3 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint3::W](otg_doepint3::W) writer structure"]
impl crate::Writable for OTG_DOEPINT3 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint3;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz3](otg_doeptsiz3) module"]
pub type OTG_DOEPTSIZ3 = crate::Reg<u32, _OTG_DOEPTSIZ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ3;
#[doc = "`read()` method returns [otg_doeptsiz3::R](otg_doeptsiz3::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ3 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz3::W](otg_doeptsiz3::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ3 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz3;
#[doc = "OTG device OUT endpoint 3 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma3](otg_doepdma3) module"]
pub type OTG_DOEPDMA3 = crate::Reg<u32, _OTG_DOEPDMA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA3;
#[doc = "`read()` method returns [otg_doepdma3::R](otg_doepdma3::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA3 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma3::W](otg_doepdma3::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA3 {}
#[doc = "OTG device OUT endpoint 3 DMA address register"]
pub mod otg_doepdma3;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl4](otg_doepctl4) module"]
pub type OTG_DOEPCTL4 = crate::Reg<u32, _OTG_DOEPCTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL4;
#[doc = "`read()` method returns [otg_doepctl4::R](otg_doepctl4::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL4 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl4::W](otg_doepctl4::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL4 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl4;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint4](otg_doepint4) module"]
pub type OTG_DOEPINT4 = crate::Reg<u32, _OTG_DOEPINT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT4;
#[doc = "`read()` method returns [otg_doepint4::R](otg_doepint4::R) reader structure"]
impl crate::Readable for OTG_DOEPINT4 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint4::W](otg_doepint4::W) writer structure"]
impl crate::Writable for OTG_DOEPINT4 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint4;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz4](otg_doeptsiz4) module"]
pub type OTG_DOEPTSIZ4 = crate::Reg<u32, _OTG_DOEPTSIZ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ4;
#[doc = "`read()` method returns [otg_doeptsiz4::R](otg_doeptsiz4::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ4 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz4::W](otg_doeptsiz4::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ4 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz4;
#[doc = "OTG device OUT endpoint 4 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma4](otg_doepdma4) module"]
pub type OTG_DOEPDMA4 = crate::Reg<u32, _OTG_DOEPDMA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA4;
#[doc = "`read()` method returns [otg_doepdma4::R](otg_doepdma4::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA4 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma4::W](otg_doepdma4::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA4 {}
#[doc = "OTG device OUT endpoint 4 DMA address register"]
pub mod otg_doepdma4;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl5](otg_doepctl5) module"]
pub type OTG_DOEPCTL5 = crate::Reg<u32, _OTG_DOEPCTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL5;
#[doc = "`read()` method returns [otg_doepctl5::R](otg_doepctl5::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL5 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl5::W](otg_doepctl5::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL5 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl5;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint5](otg_doepint5) module"]
pub type OTG_DOEPINT5 = crate::Reg<u32, _OTG_DOEPINT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT5;
#[doc = "`read()` method returns [otg_doepint5::R](otg_doepint5::R) reader structure"]
impl crate::Readable for OTG_DOEPINT5 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint5::W](otg_doepint5::W) writer structure"]
impl crate::Writable for OTG_DOEPINT5 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint5;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz5](otg_doeptsiz5) module"]
pub type OTG_DOEPTSIZ5 = crate::Reg<u32, _OTG_DOEPTSIZ5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ5;
#[doc = "`read()` method returns [otg_doeptsiz5::R](otg_doeptsiz5::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ5 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz5::W](otg_doeptsiz5::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ5 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz5;
#[doc = "OTG device OUT endpoint 5 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma5](otg_doepdma5) module"]
pub type OTG_DOEPDMA5 = crate::Reg<u32, _OTG_DOEPDMA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA5;
#[doc = "`read()` method returns [otg_doepdma5::R](otg_doepdma5::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA5 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma5::W](otg_doepdma5::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA5 {}
#[doc = "OTG device OUT endpoint 5 DMA address register"]
pub mod otg_doepdma5;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl6](otg_doepctl6) module"]
pub type OTG_DOEPCTL6 = crate::Reg<u32, _OTG_DOEPCTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL6;
#[doc = "`read()` method returns [otg_doepctl6::R](otg_doepctl6::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL6 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl6::W](otg_doepctl6::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL6 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl6;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint6](otg_doepint6) module"]
pub type OTG_DOEPINT6 = crate::Reg<u32, _OTG_DOEPINT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT6;
#[doc = "`read()` method returns [otg_doepint6::R](otg_doepint6::R) reader structure"]
impl crate::Readable for OTG_DOEPINT6 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint6::W](otg_doepint6::W) writer structure"]
impl crate::Writable for OTG_DOEPINT6 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint6;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz6](otg_doeptsiz6) module"]
pub type OTG_DOEPTSIZ6 = crate::Reg<u32, _OTG_DOEPTSIZ6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ6;
#[doc = "`read()` method returns [otg_doeptsiz6::R](otg_doeptsiz6::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ6 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz6::W](otg_doeptsiz6::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ6 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz6;
#[doc = "OTG device OUT endpoint 6 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma6](otg_doepdma6) module"]
pub type OTG_DOEPDMA6 = crate::Reg<u32, _OTG_DOEPDMA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA6;
#[doc = "`read()` method returns [otg_doepdma6::R](otg_doepdma6::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA6 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma6::W](otg_doepdma6::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA6 {}
#[doc = "OTG device OUT endpoint 6 DMA address register"]
pub mod otg_doepdma6;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl7](otg_doepctl7) module"]
pub type OTG_DOEPCTL7 = crate::Reg<u32, _OTG_DOEPCTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL7;
#[doc = "`read()` method returns [otg_doepctl7::R](otg_doepctl7::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL7 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl7::W](otg_doepctl7::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL7 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl7;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint7](otg_doepint7) module"]
pub type OTG_DOEPINT7 = crate::Reg<u32, _OTG_DOEPINT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT7;
#[doc = "`read()` method returns [otg_doepint7::R](otg_doepint7::R) reader structure"]
impl crate::Readable for OTG_DOEPINT7 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint7::W](otg_doepint7::W) writer structure"]
impl crate::Writable for OTG_DOEPINT7 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint7;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz7](otg_doeptsiz7) module"]
pub type OTG_DOEPTSIZ7 = crate::Reg<u32, _OTG_DOEPTSIZ7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ7;
#[doc = "`read()` method returns [otg_doeptsiz7::R](otg_doeptsiz7::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ7 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz7::W](otg_doeptsiz7::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ7 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz7;
#[doc = "OTG device OUT endpoint 7 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma7](otg_doepdma7) module"]
pub type OTG_DOEPDMA7 = crate::Reg<u32, _OTG_DOEPDMA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA7;
#[doc = "`read()` method returns [otg_doepdma7::R](otg_doepdma7::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA7 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma7::W](otg_doepdma7::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA7 {}
#[doc = "OTG device OUT endpoint 7 DMA address register"]
pub mod otg_doepdma7;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl8](otg_doepctl8) module"]
pub type OTG_DOEPCTL8 = crate::Reg<u32, _OTG_DOEPCTL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPCTL8;
#[doc = "`read()` method returns [otg_doepctl8::R](otg_doepctl8::R) reader structure"]
impl crate::Readable for OTG_DOEPCTL8 {}
#[doc = "`write(|w| ..)` method takes [otg_doepctl8::W](otg_doepctl8::W) writer structure"]
impl crate::Writable for OTG_DOEPCTL8 {}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl8;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepint8](otg_doepint8) module"]
pub type OTG_DOEPINT8 = crate::Reg<u32, _OTG_DOEPINT8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPINT8;
#[doc = "`read()` method returns [otg_doepint8::R](otg_doepint8::R) reader structure"]
impl crate::Readable for OTG_DOEPINT8 {}
#[doc = "`write(|w| ..)` method takes [otg_doepint8::W](otg_doepint8::W) writer structure"]
impl crate::Writable for OTG_DOEPINT8 {}
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint8;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doeptsiz8](otg_doeptsiz8) module"]
pub type OTG_DOEPTSIZ8 = crate::Reg<u32, _OTG_DOEPTSIZ8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPTSIZ8;
#[doc = "`read()` method returns [otg_doeptsiz8::R](otg_doeptsiz8::R) reader structure"]
impl crate::Readable for OTG_DOEPTSIZ8 {}
#[doc = "`write(|w| ..)` method takes [otg_doeptsiz8::W](otg_doeptsiz8::W) writer structure"]
impl crate::Writable for OTG_DOEPTSIZ8 {}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz8;
#[doc = "OTG device OUT endpoint 8 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepdma8](otg_doepdma8) module"]
pub type OTG_DOEPDMA8 = crate::Reg<u32, _OTG_DOEPDMA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_DOEPDMA8;
#[doc = "`read()` method returns [otg_doepdma8::R](otg_doepdma8::R) reader structure"]
impl crate::Readable for OTG_DOEPDMA8 {}
#[doc = "`write(|w| ..)` method takes [otg_doepdma8::W](otg_doepdma8::W) writer structure"]
impl crate::Writable for OTG_DOEPDMA8 {}
#[doc = "OTG device OUT endpoint 8 DMA address register"]
pub mod otg_doepdma8;
#[doc = "This register is available in host and device modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_pcgcctl](otg_pcgcctl) module"]
pub type OTG_PCGCCTL = crate::Reg<u32, _OTG_PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTG_PCGCCTL;
#[doc = "`read()` method returns [otg_pcgcctl::R](otg_pcgcctl::R) reader structure"]
impl crate::Readable for OTG_PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [otg_pcgcctl::W](otg_pcgcctl::W) writer structure"]
impl crate::Writable for OTG_PCGCCTL {}
#[doc = "This register is available in host and device modes."]
pub mod otg_pcgcctl;
