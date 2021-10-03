#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
    pub otg_gotgctl: crate::Reg<otg_gotgctl::OTG_GOTGCTL_SPEC>,
    #[doc = "0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
    pub otg_gotgint: crate::Reg<otg_gotgint::OTG_GOTGINT_SPEC>,
    #[doc = "0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
    pub otg_gahbcfg: crate::Reg<otg_gahbcfg::OTG_GAHBCFG_SPEC>,
    #[doc = "0x0c - This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
    pub otg_gusbcfg: crate::Reg<otg_gusbcfg::OTG_GUSBCFG_SPEC>,
    #[doc = "0x10 - The application uses this register to reset various hardware features inside the core."]
    pub otg_grstctl: crate::Reg<otg_grstctl::OTG_GRSTCTL_SPEC>,
    #[doc = "0x14 - This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
    pub otg_gintsts: crate::Reg<otg_gintsts::OTG_GINTSTS_SPEC>,
    #[doc = "0x18 - This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
    pub otg_gintmsk: crate::Reg<otg_gintmsk::OTG_GINTMSK_SPEC>,
    #[doc = "0x1c - This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
    pub otg_grxstsr: crate::Reg<otg_grxstsr::OTG_GRXSTSR_SPEC>,
    #[doc = "0x20 - This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
    pub otg_grxstsp: crate::Reg<otg_grxstsp::OTG_GRXSTSP_SPEC>,
    #[doc = "0x24 - The application can program the RAM size that must be allocated to the Rx FIFO."]
    pub otg_grxfsiz: crate::Reg<otg_grxfsiz::OTG_GRXFSIZ_SPEC>,
    #[doc = "0x28 - Host mode"]
    pub otg_hnptxfsiz: crate::Reg<otg_hnptxfsiz::OTG_HNPTXFSIZ_SPEC>,
    #[doc = "0x2c - In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
    pub otg_hnptxsts: crate::Reg<otg_hnptxsts::OTG_HNPTXSTS_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x38 - OTG general core configuration register"]
    pub otg_gccfg: crate::Reg<otg_gccfg::OTG_GCCFG_SPEC>,
    #[doc = "0x3c - This is a register containing the Product ID as reset value."]
    pub otg_cid: crate::Reg<otg_cid::OTG_CID_SPEC>,
    _reserved14: [u8; 0x14],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_glpmcfg: crate::Reg<otg_glpmcfg::OTG_GLPMCFG_SPEC>,
    _reserved15: [u8; 0xa8],
    #[doc = "0x100 - OTG host periodic transmit FIFO size register"]
    pub otg_hptxfsiz: crate::Reg<otg_hptxfsiz::OTG_HPTXFSIZ_SPEC>,
    #[doc = "0x104 - OTG device IN endpoint transmit FIFO 1 size register"]
    pub otg_dieptxf1: crate::Reg<otg_dieptxf1::OTG_DIEPTXF1_SPEC>,
    #[doc = "0x108 - OTG device IN endpoint transmit FIFO 2 size register"]
    pub otg_dieptxf2: crate::Reg<otg_dieptxf2::OTG_DIEPTXF2_SPEC>,
    #[doc = "0x10c - OTG device IN endpoint transmit FIFO 3 size register"]
    pub otg_dieptxf3: crate::Reg<otg_dieptxf3::OTG_DIEPTXF3_SPEC>,
    #[doc = "0x110 - OTG device IN endpoint transmit FIFO 4 size register"]
    pub otg_dieptxf4: crate::Reg<otg_dieptxf4::OTG_DIEPTXF4_SPEC>,
    #[doc = "0x114 - OTG device IN endpoint transmit FIFO 5 size register"]
    pub otg_dieptxf5: crate::Reg<otg_dieptxf5::OTG_DIEPTXF5_SPEC>,
    #[doc = "0x118 - OTG device IN endpoint transmit FIFO 6 size register"]
    pub otg_dieptxf6: crate::Reg<otg_dieptxf6::OTG_DIEPTXF6_SPEC>,
    #[doc = "0x11c - OTG device IN endpoint transmit FIFO 7 size register"]
    pub otg_dieptxf7: crate::Reg<otg_dieptxf7::OTG_DIEPTXF7_SPEC>,
    #[doc = "0x120 - OTG device IN endpoint transmit FIFO 8 size register"]
    pub otg_dieptxf8: crate::Reg<otg_dieptxf8::OTG_DIEPTXF8_SPEC>,
    _reserved24: [u8; 0x02dc],
    #[doc = "0x400 - This register configures the core after power-on. Do not make changes to this register after initializing the host."]
    pub otg_hcfg: crate::Reg<otg_hcfg::OTG_HCFG_SPEC>,
    #[doc = "0x404 - This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
    pub otg_hfir: crate::Reg<otg_hfir::OTG_HFIR_SPEC>,
    #[doc = "0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
    pub otg_hfnum: crate::Reg<otg_hfnum::OTG_HFNUM_SPEC>,
    _reserved27: [u8; 0x04],
    #[doc = "0x410 - This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
    pub otg_hptxsts: crate::Reg<otg_hptxsts::OTG_HPTXSTS_SPEC>,
    #[doc = "0x414 - When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
    pub otg_haint: crate::Reg<otg_haint::OTG_HAINT_SPEC>,
    #[doc = "0x418 - The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
    pub otg_haintmsk: crate::Reg<otg_haintmsk::OTG_HAINTMSK_SPEC>,
    #[doc = "0x41c - This register holds the starting address of the frame list information (scatter/gather mode)."]
    pub otg_hflbaddr: crate::Reg<otg_hflbaddr::OTG_HFLBADDR_SPEC>,
    _reserved31: [u8; 0x20],
    #[doc = "0x440 - This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
    pub otg_hprt: crate::Reg<otg_hprt::OTG_HPRT_SPEC>,
    _reserved32: [u8; 0xbc],
    #[doc = "0x500 - OTG host channel 0 characteristics register"]
    pub otg_hcchar0: crate::Reg<otg_hcchar0::OTG_HCCHAR0_SPEC>,
    #[doc = "0x504 - OTG host channel 0 split control register"]
    pub otg_hcsplt0: crate::Reg<otg_hcsplt0::OTG_HCSPLT0_SPEC>,
    #[doc = "0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint0: crate::Reg<otg_hcint0::OTG_HCINT0_SPEC>,
    #[doc = "0x50c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk0: crate::Reg<otg_hcintmsk0::OTG_HCINTMSK0_SPEC>,
    #[doc = "0x510 - OTG host channel 0 transfer size register"]
    pub otg_hctsiz0: crate::Reg<otg_hctsiz0::OTG_HCTSIZ0_SPEC>,
    #[doc = "0x514 - OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma0: crate::Reg<otg_hcdma0::OTG_HCDMA0_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x51c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab0: crate::Reg<otg_hcdmab0::OTG_HCDMAB0_SPEC>,
    #[doc = "0x520 - OTG host channel 1 characteristics register"]
    pub otg_hcchar1: crate::Reg<otg_hcchar1::OTG_HCCHAR1_SPEC>,
    #[doc = "0x524 - OTG host channel 1 split control register"]
    pub otg_hcsplt1: crate::Reg<otg_hcsplt1::OTG_HCSPLT1_SPEC>,
    #[doc = "0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint1: crate::Reg<otg_hcint1::OTG_HCINT1_SPEC>,
    #[doc = "0x52c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk1: crate::Reg<otg_hcintmsk1::OTG_HCINTMSK1_SPEC>,
    #[doc = "0x530 - OTG host channel 1 transfer size register"]
    pub otg_hctsiz1: crate::Reg<otg_hctsiz1::OTG_HCTSIZ1_SPEC>,
    #[doc = "0x534 - OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma1: crate::Reg<otg_hcdma1::OTG_HCDMA1_SPEC>,
    _reserved45: [u8; 0x04],
    #[doc = "0x53c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab1: crate::Reg<otg_hcdmab1::OTG_HCDMAB1_SPEC>,
    #[doc = "0x540 - OTG host channel 2 characteristics register"]
    pub otg_hcchar2: crate::Reg<otg_hcchar2::OTG_HCCHAR2_SPEC>,
    #[doc = "0x544 - OTG host channel 2 split control register"]
    pub otg_hcsplt2: crate::Reg<otg_hcsplt2::OTG_HCSPLT2_SPEC>,
    #[doc = "0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint2: crate::Reg<otg_hcint2::OTG_HCINT2_SPEC>,
    #[doc = "0x54c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk2: crate::Reg<otg_hcintmsk2::OTG_HCINTMSK2_SPEC>,
    #[doc = "0x550 - OTG host channel 2 transfer size register"]
    pub otg_hctsiz2: crate::Reg<otg_hctsiz2::OTG_HCTSIZ2_SPEC>,
    #[doc = "0x554 - OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma2: crate::Reg<otg_hcdma2::OTG_HCDMA2_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0x55c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab2: crate::Reg<otg_hcdmab2::OTG_HCDMAB2_SPEC>,
    #[doc = "0x560 - OTG host channel 3 characteristics register"]
    pub otg_hcchar3: crate::Reg<otg_hcchar3::OTG_HCCHAR3_SPEC>,
    #[doc = "0x564 - OTG host channel 3 split control register"]
    pub otg_hcsplt3: crate::Reg<otg_hcsplt3::OTG_HCSPLT3_SPEC>,
    #[doc = "0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint3: crate::Reg<otg_hcint3::OTG_HCINT3_SPEC>,
    #[doc = "0x56c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk3: crate::Reg<otg_hcintmsk3::OTG_HCINTMSK3_SPEC>,
    #[doc = "0x570 - OTG host channel 3 transfer size register"]
    pub otg_hctsiz3: crate::Reg<otg_hctsiz3::OTG_HCTSIZ3_SPEC>,
    #[doc = "0x574 - OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma3: crate::Reg<otg_hcdma3::OTG_HCDMA3_SPEC>,
    _reserved59: [u8; 0x04],
    #[doc = "0x57c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab3: crate::Reg<otg_hcdmab3::OTG_HCDMAB3_SPEC>,
    #[doc = "0x580 - OTG host channel 4 characteristics register"]
    pub otg_hcchar4: crate::Reg<otg_hcchar4::OTG_HCCHAR4_SPEC>,
    #[doc = "0x584 - OTG host channel 4 split control register"]
    pub otg_hcsplt4: crate::Reg<otg_hcsplt4::OTG_HCSPLT4_SPEC>,
    #[doc = "0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint4: crate::Reg<otg_hcint4::OTG_HCINT4_SPEC>,
    #[doc = "0x58c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk4: crate::Reg<otg_hcintmsk4::OTG_HCINTMSK4_SPEC>,
    #[doc = "0x590 - OTG host channel 4 transfer size register"]
    pub otg_hctsiz4: crate::Reg<otg_hctsiz4::OTG_HCTSIZ4_SPEC>,
    #[doc = "0x594 - OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma4: crate::Reg<otg_hcdma4::OTG_HCDMA4_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x59c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab4: crate::Reg<otg_hcdmab4::OTG_HCDMAB4_SPEC>,
    #[doc = "0x5a0 - OTG host channel 5 characteristics register"]
    pub otg_hcchar5: crate::Reg<otg_hcchar5::OTG_HCCHAR5_SPEC>,
    #[doc = "0x5a4 - OTG host channel 5 split control register"]
    pub otg_hcsplt5: crate::Reg<otg_hcsplt5::OTG_HCSPLT5_SPEC>,
    #[doc = "0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint5: crate::Reg<otg_hcint5::OTG_HCINT5_SPEC>,
    #[doc = "0x5ac - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk5: crate::Reg<otg_hcintmsk5::OTG_HCINTMSK5_SPEC>,
    #[doc = "0x5b0 - OTG host channel 5 transfer size register"]
    pub otg_hctsiz5: crate::Reg<otg_hctsiz5::OTG_HCTSIZ5_SPEC>,
    #[doc = "0x5b4 - OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma5: crate::Reg<otg_hcdma5::OTG_HCDMA5_SPEC>,
    _reserved73: [u8; 0x04],
    #[doc = "0x5bc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab5: crate::Reg<otg_hcdmab5::OTG_HCDMAB5_SPEC>,
    #[doc = "0x5c0 - OTG host channel 6 characteristics register"]
    pub otg_hcchar6: crate::Reg<otg_hcchar6::OTG_HCCHAR6_SPEC>,
    #[doc = "0x5c4 - OTG host channel 6 split control register"]
    pub otg_hcsplt6: crate::Reg<otg_hcsplt6::OTG_HCSPLT6_SPEC>,
    #[doc = "0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint6: crate::Reg<otg_hcint6::OTG_HCINT6_SPEC>,
    #[doc = "0x5cc - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk6: crate::Reg<otg_hcintmsk6::OTG_HCINTMSK6_SPEC>,
    #[doc = "0x5d0 - OTG host channel 6 transfer size register"]
    pub otg_hctsiz6: crate::Reg<otg_hctsiz6::OTG_HCTSIZ6_SPEC>,
    #[doc = "0x5d4 - OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma6: crate::Reg<otg_hcdma6::OTG_HCDMA6_SPEC>,
    _reserved80: [u8; 0x04],
    #[doc = "0x5dc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab6: crate::Reg<otg_hcdmab6::OTG_HCDMAB6_SPEC>,
    #[doc = "0x5e0 - OTG host channel 7 characteristics register"]
    pub otg_hcchar7: crate::Reg<otg_hcchar7::OTG_HCCHAR7_SPEC>,
    #[doc = "0x5e4 - OTG host channel 7 split control register"]
    pub otg_hcsplt7: crate::Reg<otg_hcsplt7::OTG_HCSPLT7_SPEC>,
    #[doc = "0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint7: crate::Reg<otg_hcint7::OTG_HCINT7_SPEC>,
    #[doc = "0x5ec - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk7: crate::Reg<otg_hcintmsk7::OTG_HCINTMSK7_SPEC>,
    #[doc = "0x5f0 - OTG host channel 7 transfer size register"]
    pub otg_hctsiz7: crate::Reg<otg_hctsiz7::OTG_HCTSIZ7_SPEC>,
    #[doc = "0x5f4 - OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma7: crate::Reg<otg_hcdma7::OTG_HCDMA7_SPEC>,
    _reserved87: [u8; 0x04],
    #[doc = "0x5fc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab7: crate::Reg<otg_hcdmab7::OTG_HCDMAB7_SPEC>,
    #[doc = "0x600 - OTG host channel 8 characteristics register"]
    pub otg_hcchar8: crate::Reg<otg_hcchar8::OTG_HCCHAR8_SPEC>,
    #[doc = "0x604 - OTG host channel 8 split control register"]
    pub otg_hcsplt8: crate::Reg<otg_hcsplt8::OTG_HCSPLT8_SPEC>,
    #[doc = "0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint8: crate::Reg<otg_hcint8::OTG_HCINT8_SPEC>,
    #[doc = "0x60c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk8: crate::Reg<otg_hcintmsk8::OTG_HCINTMSK8_SPEC>,
    #[doc = "0x610 - OTG host channel 8 transfer size register"]
    pub otg_hctsiz8: crate::Reg<otg_hctsiz8::OTG_HCTSIZ8_SPEC>,
    #[doc = "0x614 - OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma8: crate::Reg<otg_hcdma8::OTG_HCDMA8_SPEC>,
    _reserved94: [u8; 0x04],
    #[doc = "0x61c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab8: crate::Reg<otg_hcdmab8::OTG_HCDMAB8_SPEC>,
    #[doc = "0x620 - OTG host channel 9 characteristics register"]
    pub otg_hcchar9: crate::Reg<otg_hcchar9::OTG_HCCHAR9_SPEC>,
    #[doc = "0x624 - OTG host channel 9 split control register"]
    pub otg_hcsplt9: crate::Reg<otg_hcsplt9::OTG_HCSPLT9_SPEC>,
    #[doc = "0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint9: crate::Reg<otg_hcint9::OTG_HCINT9_SPEC>,
    #[doc = "0x62c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk9: crate::Reg<otg_hcintmsk9::OTG_HCINTMSK9_SPEC>,
    #[doc = "0x630 - OTG host channel 9 transfer size register"]
    pub otg_hctsiz9: crate::Reg<otg_hctsiz9::OTG_HCTSIZ9_SPEC>,
    #[doc = "0x634 - OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma9: crate::Reg<otg_hcdma9::OTG_HCDMA9_SPEC>,
    _reserved101: [u8; 0x04],
    #[doc = "0x63c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab9: crate::Reg<otg_hcdmab9::OTG_HCDMAB9_SPEC>,
    #[doc = "0x640 - OTG host channel 10 characteristics register"]
    pub otg_hcchar10: crate::Reg<otg_hcchar10::OTG_HCCHAR10_SPEC>,
    #[doc = "0x644 - OTG host channel 10 split control register"]
    pub otg_hcsplt10: crate::Reg<otg_hcsplt10::OTG_HCSPLT10_SPEC>,
    #[doc = "0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint10: crate::Reg<otg_hcint10::OTG_HCINT10_SPEC>,
    #[doc = "0x64c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk10: crate::Reg<otg_hcintmsk10::OTG_HCINTMSK10_SPEC>,
    #[doc = "0x650 - OTG host channel 10 transfer size register"]
    pub otg_hctsiz10: crate::Reg<otg_hctsiz10::OTG_HCTSIZ10_SPEC>,
    #[doc = "0x654 - OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma10: crate::Reg<otg_hcdma10::OTG_HCDMA10_SPEC>,
    _reserved108: [u8; 0x04],
    #[doc = "0x65c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab10: crate::Reg<otg_hcdmab10::OTG_HCDMAB10_SPEC>,
    #[doc = "0x660 - OTG host channel 11 characteristics register"]
    pub otg_hcchar11: crate::Reg<otg_hcchar11::OTG_HCCHAR11_SPEC>,
    #[doc = "0x664 - OTG host channel 11 split control register"]
    pub otg_hcsplt11: crate::Reg<otg_hcsplt11::OTG_HCSPLT11_SPEC>,
    #[doc = "0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint11: crate::Reg<otg_hcint11::OTG_HCINT11_SPEC>,
    #[doc = "0x66c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk11: crate::Reg<otg_hcintmsk11::OTG_HCINTMSK11_SPEC>,
    #[doc = "0x670 - OTG host channel 11 transfer size register"]
    pub otg_hctsiz11: crate::Reg<otg_hctsiz11::OTG_HCTSIZ11_SPEC>,
    #[doc = "0x674 - OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma11: crate::Reg<otg_hcdma11::OTG_HCDMA11_SPEC>,
    _reserved115: [u8; 0x04],
    #[doc = "0x67c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab11: crate::Reg<otg_hcdmab11::OTG_HCDMAB11_SPEC>,
    #[doc = "0x680 - OTG host channel 12 characteristics register"]
    pub otg_hcchar12: crate::Reg<otg_hcchar12::OTG_HCCHAR12_SPEC>,
    #[doc = "0x684 - OTG host channel 12 split control register"]
    pub otg_hcsplt12: crate::Reg<otg_hcsplt12::OTG_HCSPLT12_SPEC>,
    #[doc = "0x688 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint12: crate::Reg<otg_hcint12::OTG_HCINT12_SPEC>,
    #[doc = "0x68c - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk12: crate::Reg<otg_hcintmsk12::OTG_HCINTMSK12_SPEC>,
    #[doc = "0x690 - OTG host channel 12 transfer size register"]
    pub otg_hctsiz12: crate::Reg<otg_hctsiz12::OTG_HCTSIZ12_SPEC>,
    #[doc = "0x694 - OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma12: crate::Reg<otg_hcdma12::OTG_HCDMA12_SPEC>,
    _reserved122: [u8; 0x04],
    #[doc = "0x69c - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab12: crate::Reg<otg_hcdmab12::OTG_HCDMAB12_SPEC>,
    #[doc = "0x6a0 - OTG host channel 13 characteristics register"]
    pub otg_hcchar13: crate::Reg<otg_hcchar13::OTG_HCCHAR13_SPEC>,
    #[doc = "0x6a4 - OTG host channel 13 split control register"]
    pub otg_hcsplt13: crate::Reg<otg_hcsplt13::OTG_HCSPLT13_SPEC>,
    #[doc = "0x6a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint13: crate::Reg<otg_hcint13::OTG_HCINT13_SPEC>,
    #[doc = "0x6ac - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk13: crate::Reg<otg_hcintmsk13::OTG_HCINTMSK13_SPEC>,
    #[doc = "0x6b0 - OTG host channel 13 transfer size register"]
    pub otg_hctsiz13: crate::Reg<otg_hctsiz13::OTG_HCTSIZ13_SPEC>,
    #[doc = "0x6b4 - OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma13: crate::Reg<otg_hcdma13::OTG_HCDMA13_SPEC>,
    _reserved129: [u8; 0x04],
    #[doc = "0x6bc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab13: crate::Reg<otg_hcdmab13::OTG_HCDMAB13_SPEC>,
    #[doc = "0x6c0 - OTG host channel 14 characteristics register"]
    pub otg_hcchar14: crate::Reg<otg_hcchar14::OTG_HCCHAR14_SPEC>,
    #[doc = "0x6c4 - OTG host channel 14 split control register"]
    pub otg_hcsplt14: crate::Reg<otg_hcsplt14::OTG_HCSPLT14_SPEC>,
    #[doc = "0x6c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint14: crate::Reg<otg_hcint14::OTG_HCINT14_SPEC>,
    #[doc = "0x6cc - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk14: crate::Reg<otg_hcintmsk14::OTG_HCINTMSK14_SPEC>,
    #[doc = "0x6d0 - OTG host channel 14 transfer size register"]
    pub otg_hctsiz14: crate::Reg<otg_hctsiz14::OTG_HCTSIZ14_SPEC>,
    #[doc = "0x6d4 - OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma14: crate::Reg<otg_hcdma14::OTG_HCDMA14_SPEC>,
    _reserved136: [u8; 0x04],
    #[doc = "0x6dc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab14: crate::Reg<otg_hcdmab14::OTG_HCDMAB14_SPEC>,
    #[doc = "0x6e0 - OTG host channel 15 characteristics register"]
    pub otg_hcchar15: crate::Reg<otg_hcchar15::OTG_HCCHAR15_SPEC>,
    #[doc = "0x6e4 - OTG host channel 15 split control register"]
    pub otg_hcsplt15: crate::Reg<otg_hcsplt15::OTG_HCSPLT15_SPEC>,
    #[doc = "0x6e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
    pub otg_hcint15: crate::Reg<otg_hcint15::OTG_HCINT15_SPEC>,
    #[doc = "0x6ec - This register reflects the mask for each channel status described in the previous section."]
    pub otg_hcintmsk15: crate::Reg<otg_hcintmsk15::OTG_HCINTMSK15_SPEC>,
    #[doc = "0x6f0 - OTG host channel 15 transfer size register"]
    pub otg_hctsiz15: crate::Reg<otg_hctsiz15::OTG_HCTSIZ15_SPEC>,
    #[doc = "0x6f4 - OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
    pub otg_hcdma15: crate::Reg<otg_hcdma15::OTG_HCDMA15_SPEC>,
    _reserved143: [u8; 0x04],
    #[doc = "0x6fc - OTG host channel-n DMA address buffer register"]
    pub otg_hcdmab15: crate::Reg<otg_hcdmab15::OTG_HCDMAB15_SPEC>,
    _reserved144: [u8; 0x0100],
    #[doc = "0x800 - This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
    pub otg_dcfg: crate::Reg<otg_dcfg::OTG_DCFG_SPEC>,
    #[doc = "0x804 - OTG device control register"]
    pub otg_dctl: crate::Reg<otg_dctl::OTG_DCTL_SPEC>,
    #[doc = "0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
    pub otg_dsts: crate::Reg<otg_dsts::OTG_DSTS_SPEC>,
    _reserved147: [u8; 0x04],
    #[doc = "0x810 - This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
    pub otg_diepmsk: crate::Reg<otg_diepmsk::OTG_DIEPMSK_SPEC>,
    #[doc = "0x814 - This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_doepmsk: crate::Reg<otg_doepmsk::OTG_DOEPMSK_SPEC>,
    #[doc = "0x818 - When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
    pub otg_daint: crate::Reg<otg_daint::OTG_DAINT_SPEC>,
    #[doc = "0x81c - The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
    pub otg_daintmsk: crate::Reg<otg_daintmsk::OTG_DAINTMSK_SPEC>,
    _reserved151: [u8; 0x08],
    #[doc = "0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
    pub otg_dvbusdis: crate::Reg<otg_dvbusdis::OTG_DVBUSDIS_SPEC>,
    #[doc = "0x82c - This register specifies the VBUS pulsing time during SRP."]
    pub otg_dvbuspulse: crate::Reg<otg_dvbuspulse::OTG_DVBUSPULSE_SPEC>,
    #[doc = "0x830 - OTG device threshold control register"]
    pub otg_dthrctl: crate::Reg<otg_dthrctl::OTG_DTHRCTL_SPEC>,
    #[doc = "0x834 - This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
    pub otg_diepempmsk: crate::Reg<otg_diepempmsk::OTG_DIEPEMPMSK_SPEC>,
    #[doc = "0x838 - OTG device each endpoint interrupt register"]
    pub otg_deachint: crate::Reg<otg_deachint::OTG_DEACHINT_SPEC>,
    #[doc = "0x83c - There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
    pub otg_deachintmsk: crate::Reg<otg_deachintmsk::OTG_DEACHINTMSK_SPEC>,
    _reserved157: [u8; 0x04],
    #[doc = "0x844 - This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_hs_diepeachmsk1: crate::Reg<otg_hs_diepeachmsk1::OTG_HS_DIEPEACHMSK1_SPEC>,
    _reserved158: [u8; 0x3c],
    #[doc = "0x884 - This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    pub otg_hs_doepeachmsk1: crate::Reg<otg_hs_doepeachmsk1::OTG_HS_DOEPEACHMSK1_SPEC>,
    _reserved159: [u8; 0x78],
    #[doc = "0x900 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl0: crate::Reg<otg_diepctl0::OTG_DIEPCTL0_SPEC>,
    _reserved160: [u8; 0x04],
    #[doc = "0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint0: crate::Reg<otg_diepint0::OTG_DIEPINT0_SPEC>,
    _reserved161: [u8; 0x04],
    #[doc = "0x910 - The application must modify this register before enabling endpoint 0."]
    pub otg_dieptsiz0: crate::Reg<otg_dieptsiz0::OTG_DIEPTSIZ0_SPEC>,
    #[doc = "0x914 - OTG device IN endpoint 0 DMA address register"]
    pub otg_diepdma0: crate::Reg<otg_diepdma0::OTG_DIEPDMA0_SPEC>,
    #[doc = "0x918 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts0: crate::Reg<otg_dtxfsts0::OTG_DTXFSTS0_SPEC>,
    _reserved164: [u8; 0x04],
    #[doc = "0x920 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl1: crate::Reg<otg_diepctl1::OTG_DIEPCTL1_SPEC>,
    _reserved165: [u8; 0x04],
    #[doc = "0x928 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint1: crate::Reg<otg_diepint1::OTG_DIEPINT1_SPEC>,
    _reserved166: [u8; 0x04],
    #[doc = "0x930 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz1: crate::Reg<otg_dieptsiz1::OTG_DIEPTSIZ1_SPEC>,
    #[doc = "0x934 - OTG device IN endpoint 1 DMA address register"]
    pub otg_diepdma1: crate::Reg<otg_diepdma1::OTG_DIEPDMA1_SPEC>,
    #[doc = "0x938 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts1: crate::Reg<otg_dtxfsts1::OTG_DTXFSTS1_SPEC>,
    _reserved169: [u8; 0x04],
    #[doc = "0x940 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl2: crate::Reg<otg_diepctl2::OTG_DIEPCTL2_SPEC>,
    _reserved170: [u8; 0x04],
    #[doc = "0x948 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint2: crate::Reg<otg_diepint2::OTG_DIEPINT2_SPEC>,
    _reserved171: [u8; 0x04],
    #[doc = "0x950 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz2: crate::Reg<otg_dieptsiz2::OTG_DIEPTSIZ2_SPEC>,
    #[doc = "0x954 - OTG device IN endpoint 2 DMA address register"]
    pub otg_diepdma2: crate::Reg<otg_diepdma2::OTG_DIEPDMA2_SPEC>,
    #[doc = "0x958 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts2: crate::Reg<otg_dtxfsts2::OTG_DTXFSTS2_SPEC>,
    _reserved174: [u8; 0x04],
    #[doc = "0x960 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl3: crate::Reg<otg_diepctl3::OTG_DIEPCTL3_SPEC>,
    _reserved175: [u8; 0x04],
    #[doc = "0x968 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint3: crate::Reg<otg_diepint3::OTG_DIEPINT3_SPEC>,
    _reserved176: [u8; 0x04],
    #[doc = "0x970 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz3: crate::Reg<otg_dieptsiz3::OTG_DIEPTSIZ3_SPEC>,
    #[doc = "0x974 - OTG device IN endpoint 3 DMA address register"]
    pub otg_diepdma3: crate::Reg<otg_diepdma3::OTG_DIEPDMA3_SPEC>,
    #[doc = "0x978 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts3: crate::Reg<otg_dtxfsts3::OTG_DTXFSTS3_SPEC>,
    _reserved179: [u8; 0x04],
    #[doc = "0x980 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl4: crate::Reg<otg_diepctl4::OTG_DIEPCTL4_SPEC>,
    _reserved180: [u8; 0x04],
    #[doc = "0x988 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint4: crate::Reg<otg_diepint4::OTG_DIEPINT4_SPEC>,
    _reserved181: [u8; 0x04],
    #[doc = "0x990 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz4: crate::Reg<otg_dieptsiz4::OTG_DIEPTSIZ4_SPEC>,
    #[doc = "0x994 - OTG device IN endpoint 4 DMA address register"]
    pub otg_diepdma4: crate::Reg<otg_diepdma4::OTG_DIEPDMA4_SPEC>,
    #[doc = "0x998 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts4: crate::Reg<otg_dtxfsts4::OTG_DTXFSTS4_SPEC>,
    _reserved184: [u8; 0x04],
    #[doc = "0x9a0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl5: crate::Reg<otg_diepctl5::OTG_DIEPCTL5_SPEC>,
    _reserved185: [u8; 0x04],
    #[doc = "0x9a8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint5: crate::Reg<otg_diepint5::OTG_DIEPINT5_SPEC>,
    _reserved186: [u8; 0x04],
    #[doc = "0x9b0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz5: crate::Reg<otg_dieptsiz5::OTG_DIEPTSIZ5_SPEC>,
    #[doc = "0x9b4 - OTG device IN endpoint 5 DMA address register"]
    pub otg_diepdma5: crate::Reg<otg_diepdma5::OTG_DIEPDMA5_SPEC>,
    #[doc = "0x9b8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts5: crate::Reg<otg_dtxfsts5::OTG_DTXFSTS5_SPEC>,
    _reserved189: [u8; 0x04],
    #[doc = "0x9c0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl6: crate::Reg<otg_diepctl6::OTG_DIEPCTL6_SPEC>,
    _reserved190: [u8; 0x04],
    #[doc = "0x9c8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint6: crate::Reg<otg_diepint6::OTG_DIEPINT6_SPEC>,
    _reserved191: [u8; 0x04],
    #[doc = "0x9d0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz6: crate::Reg<otg_dieptsiz6::OTG_DIEPTSIZ6_SPEC>,
    #[doc = "0x9d4 - OTG device IN endpoint 6 DMA address register"]
    pub otg_diepdma6: crate::Reg<otg_diepdma6::OTG_DIEPDMA6_SPEC>,
    #[doc = "0x9d8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts6: crate::Reg<otg_dtxfsts6::OTG_DTXFSTS6_SPEC>,
    _reserved194: [u8; 0x04],
    #[doc = "0x9e0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl7: crate::Reg<otg_diepctl7::OTG_DIEPCTL7_SPEC>,
    _reserved195: [u8; 0x04],
    #[doc = "0x9e8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint7: crate::Reg<otg_diepint7::OTG_DIEPINT7_SPEC>,
    _reserved196: [u8; 0x04],
    #[doc = "0x9f0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz7: crate::Reg<otg_dieptsiz7::OTG_DIEPTSIZ7_SPEC>,
    #[doc = "0x9f4 - OTG device IN endpoint 7 DMA address register"]
    pub otg_diepdma7: crate::Reg<otg_diepdma7::OTG_DIEPDMA7_SPEC>,
    #[doc = "0x9f8 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts7: crate::Reg<otg_dtxfsts7::OTG_DTXFSTS7_SPEC>,
    _reserved199: [u8; 0x04],
    #[doc = "0xa00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_diepctl8: crate::Reg<otg_diepctl8::OTG_DIEPCTL8_SPEC>,
    _reserved200: [u8; 0x04],
    #[doc = "0xa08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_diepint8: crate::Reg<otg_diepint8::OTG_DIEPINT8_SPEC>,
    _reserved201: [u8; 0x04],
    #[doc = "0xa10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_dieptsiz8: crate::Reg<otg_dieptsiz8::OTG_DIEPTSIZ8_SPEC>,
    #[doc = "0xa14 - OTG device IN endpoint 8 DMA address register"]
    pub otg_diepdma8: crate::Reg<otg_diepdma8::OTG_DIEPDMA8_SPEC>,
    #[doc = "0xa18 - This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
    pub otg_dtxfsts8: crate::Reg<otg_dtxfsts8::OTG_DTXFSTS8_SPEC>,
    _reserved204: [u8; 0xe4],
    #[doc = "0xb00 - This section describes the OTG_DOEPCTL0 register."]
    pub otg_doepctl0: crate::Reg<otg_doepctl0::OTG_DOEPCTL0_SPEC>,
    _reserved205: [u8; 0x04],
    #[doc = "0xb08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint0: crate::Reg<otg_doepint0::OTG_DOEPINT0_SPEC>,
    _reserved206: [u8; 0x04],
    #[doc = "0xb10 - The application must modify this register before enabling endpoint 0."]
    pub otg_doeptsiz0: crate::Reg<otg_doeptsiz0::OTG_DOEPTSIZ0_SPEC>,
    #[doc = "0xb14 - OTG device OUT endpoint 0 DMA address register"]
    pub otg_doepdma0: crate::Reg<otg_doepdma0::OTG_DOEPDMA0_SPEC>,
    _reserved208: [u8; 0x08],
    #[doc = "0xb20 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl1: crate::Reg<otg_doepctl1::OTG_DOEPCTL1_SPEC>,
    _reserved209: [u8; 0x04],
    #[doc = "0xb28 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint1: crate::Reg<otg_doepint1::OTG_DOEPINT1_SPEC>,
    _reserved210: [u8; 0x04],
    #[doc = "0xb30 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz1: crate::Reg<otg_doeptsiz1::OTG_DOEPTSIZ1_SPEC>,
    #[doc = "0xb34 - OTG device OUT endpoint 1 DMA address register"]
    pub otg_doepdma1: crate::Reg<otg_doepdma1::OTG_DOEPDMA1_SPEC>,
    _reserved212: [u8; 0x08],
    #[doc = "0xb40 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl2: crate::Reg<otg_doepctl2::OTG_DOEPCTL2_SPEC>,
    _reserved213: [u8; 0x04],
    #[doc = "0xb48 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint2: crate::Reg<otg_doepint2::OTG_DOEPINT2_SPEC>,
    _reserved214: [u8; 0x04],
    #[doc = "0xb50 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz2: crate::Reg<otg_doeptsiz2::OTG_DOEPTSIZ2_SPEC>,
    #[doc = "0xb54 - OTG device OUT endpoint 2 DMA address register"]
    pub otg_doepdma2: crate::Reg<otg_doepdma2::OTG_DOEPDMA2_SPEC>,
    _reserved216: [u8; 0x08],
    #[doc = "0xb60 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl3: crate::Reg<otg_doepctl3::OTG_DOEPCTL3_SPEC>,
    _reserved217: [u8; 0x04],
    #[doc = "0xb68 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint3: crate::Reg<otg_doepint3::OTG_DOEPINT3_SPEC>,
    _reserved218: [u8; 0x04],
    #[doc = "0xb70 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz3: crate::Reg<otg_doeptsiz3::OTG_DOEPTSIZ3_SPEC>,
    #[doc = "0xb74 - OTG device OUT endpoint 3 DMA address register"]
    pub otg_doepdma3: crate::Reg<otg_doepdma3::OTG_DOEPDMA3_SPEC>,
    _reserved220: [u8; 0x08],
    #[doc = "0xb80 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl4: crate::Reg<otg_doepctl4::OTG_DOEPCTL4_SPEC>,
    _reserved221: [u8; 0x04],
    #[doc = "0xb88 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint4: crate::Reg<otg_doepint4::OTG_DOEPINT4_SPEC>,
    _reserved222: [u8; 0x04],
    #[doc = "0xb90 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz4: crate::Reg<otg_doeptsiz4::OTG_DOEPTSIZ4_SPEC>,
    #[doc = "0xb94 - OTG device OUT endpoint 4 DMA address register"]
    pub otg_doepdma4: crate::Reg<otg_doepdma4::OTG_DOEPDMA4_SPEC>,
    _reserved224: [u8; 0x08],
    #[doc = "0xba0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl5: crate::Reg<otg_doepctl5::OTG_DOEPCTL5_SPEC>,
    _reserved225: [u8; 0x04],
    #[doc = "0xba8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint5: crate::Reg<otg_doepint5::OTG_DOEPINT5_SPEC>,
    _reserved226: [u8; 0x04],
    #[doc = "0xbb0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz5: crate::Reg<otg_doeptsiz5::OTG_DOEPTSIZ5_SPEC>,
    #[doc = "0xbb4 - OTG device OUT endpoint 5 DMA address register"]
    pub otg_doepdma5: crate::Reg<otg_doepdma5::OTG_DOEPDMA5_SPEC>,
    _reserved228: [u8; 0x08],
    #[doc = "0xbc0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl6: crate::Reg<otg_doepctl6::OTG_DOEPCTL6_SPEC>,
    _reserved229: [u8; 0x04],
    #[doc = "0xbc8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint6: crate::Reg<otg_doepint6::OTG_DOEPINT6_SPEC>,
    _reserved230: [u8; 0x04],
    #[doc = "0xbd0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz6: crate::Reg<otg_doeptsiz6::OTG_DOEPTSIZ6_SPEC>,
    #[doc = "0xbd4 - OTG device OUT endpoint 6 DMA address register"]
    pub otg_doepdma6: crate::Reg<otg_doepdma6::OTG_DOEPDMA6_SPEC>,
    _reserved232: [u8; 0x08],
    #[doc = "0xbe0 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl7: crate::Reg<otg_doepctl7::OTG_DOEPCTL7_SPEC>,
    _reserved233: [u8; 0x04],
    #[doc = "0xbe8 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint7: crate::Reg<otg_doepint7::OTG_DOEPINT7_SPEC>,
    _reserved234: [u8; 0x04],
    #[doc = "0xbf0 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz7: crate::Reg<otg_doeptsiz7::OTG_DOEPTSIZ7_SPEC>,
    #[doc = "0xbf4 - OTG device OUT endpoint 7 DMA address register"]
    pub otg_doepdma7: crate::Reg<otg_doepdma7::OTG_DOEPDMA7_SPEC>,
    _reserved236: [u8; 0x08],
    #[doc = "0xc00 - The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
    pub otg_doepctl8: crate::Reg<otg_doepctl8::OTG_DOEPCTL8_SPEC>,
    _reserved237: [u8; 0x04],
    #[doc = "0xc08 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
    pub otg_doepint8: crate::Reg<otg_doepint8::OTG_DOEPINT8_SPEC>,
    _reserved238: [u8; 0x04],
    #[doc = "0xc10 - The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
    pub otg_doeptsiz8: crate::Reg<otg_doeptsiz8::OTG_DOEPTSIZ8_SPEC>,
    #[doc = "0xc14 - OTG device OUT endpoint 8 DMA address register"]
    pub otg_doepdma8: crate::Reg<otg_doepdma8::OTG_DOEPDMA8_SPEC>,
    _reserved240: [u8; 0x01e8],
    #[doc = "0xe00 - This register is available in host and device modes."]
    pub otg_pcgcctl: crate::Reg<otg_pcgcctl::OTG_PCGCCTL_SPEC>,
}
#[doc = "OTG_GOTGCTL register accessor: an alias for `Reg<OTG_GOTGCTL_SPEC>`"]
pub type OTG_GOTGCTL = crate::Reg<otg_gotgctl::OTG_GOTGCTL_SPEC>;
#[doc = "The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core."]
pub mod otg_gotgctl;
#[doc = "OTG_GOTGINT register accessor: an alias for `Reg<OTG_GOTGINT_SPEC>`"]
pub type OTG_GOTGINT = crate::Reg<otg_gotgint::OTG_GOTGINT_SPEC>;
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
pub mod otg_gotgint;
#[doc = "OTG_GAHBCFG register accessor: an alias for `Reg<OTG_GAHBCFG_SPEC>`"]
pub type OTG_GAHBCFG = crate::Reg<otg_gahbcfg::OTG_GAHBCFG_SPEC>;
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
pub mod otg_gahbcfg;
#[doc = "OTG_GUSBCFG register accessor: an alias for `Reg<OTG_GUSBCFG_SPEC>`"]
pub type OTG_GUSBCFG = crate::Reg<otg_gusbcfg::OTG_GUSBCFG_SPEC>;
#[doc = "This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming."]
pub mod otg_gusbcfg;
#[doc = "OTG_GRSTCTL register accessor: an alias for `Reg<OTG_GRSTCTL_SPEC>`"]
pub type OTG_GRSTCTL = crate::Reg<otg_grstctl::OTG_GRSTCTL_SPEC>;
#[doc = "The application uses this register to reset various hardware features inside the core."]
pub mod otg_grstctl;
#[doc = "OTG_GINTSTS register accessor: an alias for `Reg<OTG_GINTSTS_SPEC>`"]
pub type OTG_GINTSTS = crate::Reg<otg_gintsts::OTG_GINTSTS_SPEC>;
#[doc = "This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
pub mod otg_gintsts;
#[doc = "OTG_GINTMSK register accessor: an alias for `Reg<OTG_GINTMSK_SPEC>`"]
pub type OTG_GINTMSK = crate::Reg<otg_gintmsk::OTG_GINTMSK_SPEC>;
#[doc = "This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set."]
pub mod otg_gintmsk;
#[doc = "OTG_GRXSTSR register accessor: an alias for `Reg<OTG_GRXSTSR_SPEC>`"]
pub type OTG_GRXSTSR = crate::Reg<otg_grxstsr::OTG_GRXSTSR_SPEC>;
#[doc = "This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000."]
pub mod otg_grxstsr;
#[doc = "OTG_GRXSTSP register accessor: an alias for `Reg<OTG_GRXSTSP_SPEC>`"]
pub type OTG_GRXSTSP = crate::Reg<otg_grxstsp::OTG_GRXSTSP_SPEC>;
#[doc = "This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted."]
pub mod otg_grxstsp;
#[doc = "OTG_GRXFSIZ register accessor: an alias for `Reg<OTG_GRXFSIZ_SPEC>`"]
pub type OTG_GRXFSIZ = crate::Reg<otg_grxfsiz::OTG_GRXFSIZ_SPEC>;
#[doc = "The application can program the RAM size that must be allocated to the Rx FIFO."]
pub mod otg_grxfsiz;
#[doc = "OTG_HNPTXFSIZ register accessor: an alias for `Reg<OTG_HNPTXFSIZ_SPEC>`"]
pub type OTG_HNPTXFSIZ = crate::Reg<otg_hnptxfsiz::OTG_HNPTXFSIZ_SPEC>;
#[doc = "Host mode"]
pub mod otg_hnptxfsiz;
#[doc = "OTG_HNPTXSTS register accessor: an alias for `Reg<OTG_HNPTXSTS_SPEC>`"]
pub type OTG_HNPTXSTS = crate::Reg<otg_hnptxsts::OTG_HNPTXSTS_SPEC>;
#[doc = "In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue."]
pub mod otg_hnptxsts;
#[doc = "OTG_GCCFG register accessor: an alias for `Reg<OTG_GCCFG_SPEC>`"]
pub type OTG_GCCFG = crate::Reg<otg_gccfg::OTG_GCCFG_SPEC>;
#[doc = "OTG general core configuration register"]
pub mod otg_gccfg;
#[doc = "OTG_CID register accessor: an alias for `Reg<OTG_CID_SPEC>`"]
pub type OTG_CID = crate::Reg<otg_cid::OTG_CID_SPEC>;
#[doc = "This is a register containing the Product ID as reset value."]
pub mod otg_cid;
#[doc = "OTG_GLPMCFG register accessor: an alias for `Reg<OTG_GLPMCFG_SPEC>`"]
pub type OTG_GLPMCFG = crate::Reg<otg_glpmcfg::OTG_GLPMCFG_SPEC>;
#[doc = "OTG core LPM configuration register"]
pub mod otg_glpmcfg;
#[doc = "OTG_HPTXFSIZ register accessor: an alias for `Reg<OTG_HPTXFSIZ_SPEC>`"]
pub type OTG_HPTXFSIZ = crate::Reg<otg_hptxfsiz::OTG_HPTXFSIZ_SPEC>;
#[doc = "OTG host periodic transmit FIFO size register"]
pub mod otg_hptxfsiz;
#[doc = "OTG_DIEPTXF1 register accessor: an alias for `Reg<OTG_DIEPTXF1_SPEC>`"]
pub type OTG_DIEPTXF1 = crate::Reg<otg_dieptxf1::OTG_DIEPTXF1_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 1 size register"]
pub mod otg_dieptxf1;
#[doc = "OTG_DIEPTXF2 register accessor: an alias for `Reg<OTG_DIEPTXF2_SPEC>`"]
pub type OTG_DIEPTXF2 = crate::Reg<otg_dieptxf2::OTG_DIEPTXF2_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 2 size register"]
pub mod otg_dieptxf2;
#[doc = "OTG_DIEPTXF3 register accessor: an alias for `Reg<OTG_DIEPTXF3_SPEC>`"]
pub type OTG_DIEPTXF3 = crate::Reg<otg_dieptxf3::OTG_DIEPTXF3_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 3 size register"]
pub mod otg_dieptxf3;
#[doc = "OTG_DIEPTXF4 register accessor: an alias for `Reg<OTG_DIEPTXF4_SPEC>`"]
pub type OTG_DIEPTXF4 = crate::Reg<otg_dieptxf4::OTG_DIEPTXF4_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 4 size register"]
pub mod otg_dieptxf4;
#[doc = "OTG_DIEPTXF5 register accessor: an alias for `Reg<OTG_DIEPTXF5_SPEC>`"]
pub type OTG_DIEPTXF5 = crate::Reg<otg_dieptxf5::OTG_DIEPTXF5_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 5 size register"]
pub mod otg_dieptxf5;
#[doc = "OTG_DIEPTXF6 register accessor: an alias for `Reg<OTG_DIEPTXF6_SPEC>`"]
pub type OTG_DIEPTXF6 = crate::Reg<otg_dieptxf6::OTG_DIEPTXF6_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 6 size register"]
pub mod otg_dieptxf6;
#[doc = "OTG_DIEPTXF7 register accessor: an alias for `Reg<OTG_DIEPTXF7_SPEC>`"]
pub type OTG_DIEPTXF7 = crate::Reg<otg_dieptxf7::OTG_DIEPTXF7_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 7 size register"]
pub mod otg_dieptxf7;
#[doc = "OTG_DIEPTXF8 register accessor: an alias for `Reg<OTG_DIEPTXF8_SPEC>`"]
pub type OTG_DIEPTXF8 = crate::Reg<otg_dieptxf8::OTG_DIEPTXF8_SPEC>;
#[doc = "OTG device IN endpoint transmit FIFO 8 size register"]
pub mod otg_dieptxf8;
#[doc = "OTG_HCFG register accessor: an alias for `Reg<OTG_HCFG_SPEC>`"]
pub type OTG_HCFG = crate::Reg<otg_hcfg::OTG_HCFG_SPEC>;
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host."]
pub mod otg_hcfg;
#[doc = "OTG_HFIR register accessor: an alias for `Reg<OTG_HFIR_SPEC>`"]
pub type OTG_HFIR = crate::Reg<otg_hfir::OTG_HFIR_SPEC>;
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated."]
pub mod otg_hfir;
#[doc = "OTG_HFNUM register accessor: an alias for `Reg<OTG_HFNUM_SPEC>`"]
pub type OTG_HFNUM = crate::Reg<otg_hfnum::OTG_HFNUM_SPEC>;
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame."]
pub mod otg_hfnum;
#[doc = "OTG_HPTXSTS register accessor: an alias for `Reg<OTG_HPTXSTS_SPEC>`"]
pub type OTG_HPTXSTS = crate::Reg<otg_hptxsts::OTG_HPTXSTS_SPEC>;
#[doc = "This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue."]
pub mod otg_hptxsts;
#[doc = "OTG_HAINT register accessor: an alias for `Reg<OTG_HAINT_SPEC>`"]
pub type OTG_HAINT = crate::Reg<otg_haint::OTG_HAINT_SPEC>;
#[doc = "When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register."]
pub mod otg_haint;
#[doc = "OTG_HAINTMSK register accessor: an alias for `Reg<OTG_HAINTMSK_SPEC>`"]
pub type OTG_HAINTMSK = crate::Reg<otg_haintmsk::OTG_HAINTMSK_SPEC>;
#[doc = "The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
pub mod otg_haintmsk;
#[doc = "OTG_HFLBADDR register accessor: an alias for `Reg<OTG_HFLBADDR_SPEC>`"]
pub type OTG_HFLBADDR = crate::Reg<otg_hflbaddr::OTG_HFLBADDR_SPEC>;
#[doc = "This register holds the starting address of the frame list information (scatter/gather mode)."]
pub mod otg_hflbaddr;
#[doc = "OTG_HPRT register accessor: an alias for `Reg<OTG_HPRT_SPEC>`"]
pub type OTG_HPRT = crate::Reg<otg_hprt::OTG_HPRT_SPEC>;
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt."]
pub mod otg_hprt;
#[doc = "OTG_HCCHAR0 register accessor: an alias for `Reg<OTG_HCCHAR0_SPEC>`"]
pub type OTG_HCCHAR0 = crate::Reg<otg_hcchar0::OTG_HCCHAR0_SPEC>;
#[doc = "OTG host channel 0 characteristics register"]
pub mod otg_hcchar0;
#[doc = "OTG_HCSPLT0 register accessor: an alias for `Reg<OTG_HCSPLT0_SPEC>`"]
pub type OTG_HCSPLT0 = crate::Reg<otg_hcsplt0::OTG_HCSPLT0_SPEC>;
#[doc = "OTG host channel 0 split control register"]
pub mod otg_hcsplt0;
#[doc = "OTG_HCINT0 register accessor: an alias for `Reg<OTG_HCINT0_SPEC>`"]
pub type OTG_HCINT0 = crate::Reg<otg_hcint0::OTG_HCINT0_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint0;
#[doc = "OTG_HCINTMSK0 register accessor: an alias for `Reg<OTG_HCINTMSK0_SPEC>`"]
pub type OTG_HCINTMSK0 = crate::Reg<otg_hcintmsk0::OTG_HCINTMSK0_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk0;
#[doc = "OTG_HCTSIZ0 register accessor: an alias for `Reg<OTG_HCTSIZ0_SPEC>`"]
pub type OTG_HCTSIZ0 = crate::Reg<otg_hctsiz0::OTG_HCTSIZ0_SPEC>;
#[doc = "OTG host channel 0 transfer size register"]
pub mod otg_hctsiz0;
#[doc = "OTG_HCDMA0 register accessor: an alias for `Reg<OTG_HCDMA0_SPEC>`"]
pub type OTG_HCDMA0 = crate::Reg<otg_hcdma0::OTG_HCDMA0_SPEC>;
#[doc = "OTG host channel 0 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma0;
#[doc = "OTG_HCDMAB0 register accessor: an alias for `Reg<OTG_HCDMAB0_SPEC>`"]
pub type OTG_HCDMAB0 = crate::Reg<otg_hcdmab0::OTG_HCDMAB0_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab0;
#[doc = "OTG_HCCHAR1 register accessor: an alias for `Reg<OTG_HCCHAR1_SPEC>`"]
pub type OTG_HCCHAR1 = crate::Reg<otg_hcchar1::OTG_HCCHAR1_SPEC>;
#[doc = "OTG host channel 1 characteristics register"]
pub mod otg_hcchar1;
#[doc = "OTG_HCSPLT1 register accessor: an alias for `Reg<OTG_HCSPLT1_SPEC>`"]
pub type OTG_HCSPLT1 = crate::Reg<otg_hcsplt1::OTG_HCSPLT1_SPEC>;
#[doc = "OTG host channel 1 split control register"]
pub mod otg_hcsplt1;
#[doc = "OTG_HCINT1 register accessor: an alias for `Reg<OTG_HCINT1_SPEC>`"]
pub type OTG_HCINT1 = crate::Reg<otg_hcint1::OTG_HCINT1_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint1;
#[doc = "OTG_HCINTMSK1 register accessor: an alias for `Reg<OTG_HCINTMSK1_SPEC>`"]
pub type OTG_HCINTMSK1 = crate::Reg<otg_hcintmsk1::OTG_HCINTMSK1_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk1;
#[doc = "OTG_HCTSIZ1 register accessor: an alias for `Reg<OTG_HCTSIZ1_SPEC>`"]
pub type OTG_HCTSIZ1 = crate::Reg<otg_hctsiz1::OTG_HCTSIZ1_SPEC>;
#[doc = "OTG host channel 1 transfer size register"]
pub mod otg_hctsiz1;
#[doc = "OTG_HCDMA1 register accessor: an alias for `Reg<OTG_HCDMA1_SPEC>`"]
pub type OTG_HCDMA1 = crate::Reg<otg_hcdma1::OTG_HCDMA1_SPEC>;
#[doc = "OTG host channel 1 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma1;
#[doc = "OTG_HCDMAB1 register accessor: an alias for `Reg<OTG_HCDMAB1_SPEC>`"]
pub type OTG_HCDMAB1 = crate::Reg<otg_hcdmab1::OTG_HCDMAB1_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab1;
#[doc = "OTG_HCCHAR2 register accessor: an alias for `Reg<OTG_HCCHAR2_SPEC>`"]
pub type OTG_HCCHAR2 = crate::Reg<otg_hcchar2::OTG_HCCHAR2_SPEC>;
#[doc = "OTG host channel 2 characteristics register"]
pub mod otg_hcchar2;
#[doc = "OTG_HCSPLT2 register accessor: an alias for `Reg<OTG_HCSPLT2_SPEC>`"]
pub type OTG_HCSPLT2 = crate::Reg<otg_hcsplt2::OTG_HCSPLT2_SPEC>;
#[doc = "OTG host channel 2 split control register"]
pub mod otg_hcsplt2;
#[doc = "OTG_HCINT2 register accessor: an alias for `Reg<OTG_HCINT2_SPEC>`"]
pub type OTG_HCINT2 = crate::Reg<otg_hcint2::OTG_HCINT2_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint2;
#[doc = "OTG_HCINTMSK2 register accessor: an alias for `Reg<OTG_HCINTMSK2_SPEC>`"]
pub type OTG_HCINTMSK2 = crate::Reg<otg_hcintmsk2::OTG_HCINTMSK2_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk2;
#[doc = "OTG_HCTSIZ2 register accessor: an alias for `Reg<OTG_HCTSIZ2_SPEC>`"]
pub type OTG_HCTSIZ2 = crate::Reg<otg_hctsiz2::OTG_HCTSIZ2_SPEC>;
#[doc = "OTG host channel 2 transfer size register"]
pub mod otg_hctsiz2;
#[doc = "OTG_HCDMA2 register accessor: an alias for `Reg<OTG_HCDMA2_SPEC>`"]
pub type OTG_HCDMA2 = crate::Reg<otg_hcdma2::OTG_HCDMA2_SPEC>;
#[doc = "OTG host channel 2 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma2;
#[doc = "OTG_HCDMAB2 register accessor: an alias for `Reg<OTG_HCDMAB2_SPEC>`"]
pub type OTG_HCDMAB2 = crate::Reg<otg_hcdmab2::OTG_HCDMAB2_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab2;
#[doc = "OTG_HCCHAR3 register accessor: an alias for `Reg<OTG_HCCHAR3_SPEC>`"]
pub type OTG_HCCHAR3 = crate::Reg<otg_hcchar3::OTG_HCCHAR3_SPEC>;
#[doc = "OTG host channel 3 characteristics register"]
pub mod otg_hcchar3;
#[doc = "OTG_HCSPLT3 register accessor: an alias for `Reg<OTG_HCSPLT3_SPEC>`"]
pub type OTG_HCSPLT3 = crate::Reg<otg_hcsplt3::OTG_HCSPLT3_SPEC>;
#[doc = "OTG host channel 3 split control register"]
pub mod otg_hcsplt3;
#[doc = "OTG_HCINT3 register accessor: an alias for `Reg<OTG_HCINT3_SPEC>`"]
pub type OTG_HCINT3 = crate::Reg<otg_hcint3::OTG_HCINT3_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint3;
#[doc = "OTG_HCINTMSK3 register accessor: an alias for `Reg<OTG_HCINTMSK3_SPEC>`"]
pub type OTG_HCINTMSK3 = crate::Reg<otg_hcintmsk3::OTG_HCINTMSK3_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk3;
#[doc = "OTG_HCTSIZ3 register accessor: an alias for `Reg<OTG_HCTSIZ3_SPEC>`"]
pub type OTG_HCTSIZ3 = crate::Reg<otg_hctsiz3::OTG_HCTSIZ3_SPEC>;
#[doc = "OTG host channel 3 transfer size register"]
pub mod otg_hctsiz3;
#[doc = "OTG_HCDMA3 register accessor: an alias for `Reg<OTG_HCDMA3_SPEC>`"]
pub type OTG_HCDMA3 = crate::Reg<otg_hcdma3::OTG_HCDMA3_SPEC>;
#[doc = "OTG host channel 3 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma3;
#[doc = "OTG_HCDMAB3 register accessor: an alias for `Reg<OTG_HCDMAB3_SPEC>`"]
pub type OTG_HCDMAB3 = crate::Reg<otg_hcdmab3::OTG_HCDMAB3_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab3;
#[doc = "OTG_HCCHAR4 register accessor: an alias for `Reg<OTG_HCCHAR4_SPEC>`"]
pub type OTG_HCCHAR4 = crate::Reg<otg_hcchar4::OTG_HCCHAR4_SPEC>;
#[doc = "OTG host channel 4 characteristics register"]
pub mod otg_hcchar4;
#[doc = "OTG_HCSPLT4 register accessor: an alias for `Reg<OTG_HCSPLT4_SPEC>`"]
pub type OTG_HCSPLT4 = crate::Reg<otg_hcsplt4::OTG_HCSPLT4_SPEC>;
#[doc = "OTG host channel 4 split control register"]
pub mod otg_hcsplt4;
#[doc = "OTG_HCINT4 register accessor: an alias for `Reg<OTG_HCINT4_SPEC>`"]
pub type OTG_HCINT4 = crate::Reg<otg_hcint4::OTG_HCINT4_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint4;
#[doc = "OTG_HCINTMSK4 register accessor: an alias for `Reg<OTG_HCINTMSK4_SPEC>`"]
pub type OTG_HCINTMSK4 = crate::Reg<otg_hcintmsk4::OTG_HCINTMSK4_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk4;
#[doc = "OTG_HCTSIZ4 register accessor: an alias for `Reg<OTG_HCTSIZ4_SPEC>`"]
pub type OTG_HCTSIZ4 = crate::Reg<otg_hctsiz4::OTG_HCTSIZ4_SPEC>;
#[doc = "OTG host channel 4 transfer size register"]
pub mod otg_hctsiz4;
#[doc = "OTG_HCDMA4 register accessor: an alias for `Reg<OTG_HCDMA4_SPEC>`"]
pub type OTG_HCDMA4 = crate::Reg<otg_hcdma4::OTG_HCDMA4_SPEC>;
#[doc = "OTG host channel 4 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma4;
#[doc = "OTG_HCDMAB4 register accessor: an alias for `Reg<OTG_HCDMAB4_SPEC>`"]
pub type OTG_HCDMAB4 = crate::Reg<otg_hcdmab4::OTG_HCDMAB4_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab4;
#[doc = "OTG_HCCHAR5 register accessor: an alias for `Reg<OTG_HCCHAR5_SPEC>`"]
pub type OTG_HCCHAR5 = crate::Reg<otg_hcchar5::OTG_HCCHAR5_SPEC>;
#[doc = "OTG host channel 5 characteristics register"]
pub mod otg_hcchar5;
#[doc = "OTG_HCSPLT5 register accessor: an alias for `Reg<OTG_HCSPLT5_SPEC>`"]
pub type OTG_HCSPLT5 = crate::Reg<otg_hcsplt5::OTG_HCSPLT5_SPEC>;
#[doc = "OTG host channel 5 split control register"]
pub mod otg_hcsplt5;
#[doc = "OTG_HCINT5 register accessor: an alias for `Reg<OTG_HCINT5_SPEC>`"]
pub type OTG_HCINT5 = crate::Reg<otg_hcint5::OTG_HCINT5_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint5;
#[doc = "OTG_HCINTMSK5 register accessor: an alias for `Reg<OTG_HCINTMSK5_SPEC>`"]
pub type OTG_HCINTMSK5 = crate::Reg<otg_hcintmsk5::OTG_HCINTMSK5_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk5;
#[doc = "OTG_HCTSIZ5 register accessor: an alias for `Reg<OTG_HCTSIZ5_SPEC>`"]
pub type OTG_HCTSIZ5 = crate::Reg<otg_hctsiz5::OTG_HCTSIZ5_SPEC>;
#[doc = "OTG host channel 5 transfer size register"]
pub mod otg_hctsiz5;
#[doc = "OTG_HCDMA5 register accessor: an alias for `Reg<OTG_HCDMA5_SPEC>`"]
pub type OTG_HCDMA5 = crate::Reg<otg_hcdma5::OTG_HCDMA5_SPEC>;
#[doc = "OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma5;
#[doc = "OTG_HCDMAB5 register accessor: an alias for `Reg<OTG_HCDMAB5_SPEC>`"]
pub type OTG_HCDMAB5 = crate::Reg<otg_hcdmab5::OTG_HCDMAB5_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab5;
#[doc = "OTG_HCCHAR6 register accessor: an alias for `Reg<OTG_HCCHAR6_SPEC>`"]
pub type OTG_HCCHAR6 = crate::Reg<otg_hcchar6::OTG_HCCHAR6_SPEC>;
#[doc = "OTG host channel 6 characteristics register"]
pub mod otg_hcchar6;
#[doc = "OTG_HCSPLT6 register accessor: an alias for `Reg<OTG_HCSPLT6_SPEC>`"]
pub type OTG_HCSPLT6 = crate::Reg<otg_hcsplt6::OTG_HCSPLT6_SPEC>;
#[doc = "OTG host channel 6 split control register"]
pub mod otg_hcsplt6;
#[doc = "OTG_HCINT6 register accessor: an alias for `Reg<OTG_HCINT6_SPEC>`"]
pub type OTG_HCINT6 = crate::Reg<otg_hcint6::OTG_HCINT6_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint6;
#[doc = "OTG_HCINTMSK6 register accessor: an alias for `Reg<OTG_HCINTMSK6_SPEC>`"]
pub type OTG_HCINTMSK6 = crate::Reg<otg_hcintmsk6::OTG_HCINTMSK6_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk6;
#[doc = "OTG_HCTSIZ6 register accessor: an alias for `Reg<OTG_HCTSIZ6_SPEC>`"]
pub type OTG_HCTSIZ6 = crate::Reg<otg_hctsiz6::OTG_HCTSIZ6_SPEC>;
#[doc = "OTG host channel 6 transfer size register"]
pub mod otg_hctsiz6;
#[doc = "OTG_HCDMA6 register accessor: an alias for `Reg<OTG_HCDMA6_SPEC>`"]
pub type OTG_HCDMA6 = crate::Reg<otg_hcdma6::OTG_HCDMA6_SPEC>;
#[doc = "OTG host channel 6 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma6;
#[doc = "OTG_HCDMAB6 register accessor: an alias for `Reg<OTG_HCDMAB6_SPEC>`"]
pub type OTG_HCDMAB6 = crate::Reg<otg_hcdmab6::OTG_HCDMAB6_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab6;
#[doc = "OTG_HCCHAR7 register accessor: an alias for `Reg<OTG_HCCHAR7_SPEC>`"]
pub type OTG_HCCHAR7 = crate::Reg<otg_hcchar7::OTG_HCCHAR7_SPEC>;
#[doc = "OTG host channel 7 characteristics register"]
pub mod otg_hcchar7;
#[doc = "OTG_HCSPLT7 register accessor: an alias for `Reg<OTG_HCSPLT7_SPEC>`"]
pub type OTG_HCSPLT7 = crate::Reg<otg_hcsplt7::OTG_HCSPLT7_SPEC>;
#[doc = "OTG host channel 7 split control register"]
pub mod otg_hcsplt7;
#[doc = "OTG_HCINT7 register accessor: an alias for `Reg<OTG_HCINT7_SPEC>`"]
pub type OTG_HCINT7 = crate::Reg<otg_hcint7::OTG_HCINT7_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint7;
#[doc = "OTG_HCINTMSK7 register accessor: an alias for `Reg<OTG_HCINTMSK7_SPEC>`"]
pub type OTG_HCINTMSK7 = crate::Reg<otg_hcintmsk7::OTG_HCINTMSK7_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk7;
#[doc = "OTG_HCTSIZ7 register accessor: an alias for `Reg<OTG_HCTSIZ7_SPEC>`"]
pub type OTG_HCTSIZ7 = crate::Reg<otg_hctsiz7::OTG_HCTSIZ7_SPEC>;
#[doc = "OTG host channel 7 transfer size register"]
pub mod otg_hctsiz7;
#[doc = "OTG_HCDMA7 register accessor: an alias for `Reg<OTG_HCDMA7_SPEC>`"]
pub type OTG_HCDMA7 = crate::Reg<otg_hcdma7::OTG_HCDMA7_SPEC>;
#[doc = "OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma7;
#[doc = "OTG_HCDMAB7 register accessor: an alias for `Reg<OTG_HCDMAB7_SPEC>`"]
pub type OTG_HCDMAB7 = crate::Reg<otg_hcdmab7::OTG_HCDMAB7_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab7;
#[doc = "OTG_HCCHAR8 register accessor: an alias for `Reg<OTG_HCCHAR8_SPEC>`"]
pub type OTG_HCCHAR8 = crate::Reg<otg_hcchar8::OTG_HCCHAR8_SPEC>;
#[doc = "OTG host channel 8 characteristics register"]
pub mod otg_hcchar8;
#[doc = "OTG_HCSPLT8 register accessor: an alias for `Reg<OTG_HCSPLT8_SPEC>`"]
pub type OTG_HCSPLT8 = crate::Reg<otg_hcsplt8::OTG_HCSPLT8_SPEC>;
#[doc = "OTG host channel 8 split control register"]
pub mod otg_hcsplt8;
#[doc = "OTG_HCINT8 register accessor: an alias for `Reg<OTG_HCINT8_SPEC>`"]
pub type OTG_HCINT8 = crate::Reg<otg_hcint8::OTG_HCINT8_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint8;
#[doc = "OTG_HCINTMSK8 register accessor: an alias for `Reg<OTG_HCINTMSK8_SPEC>`"]
pub type OTG_HCINTMSK8 = crate::Reg<otg_hcintmsk8::OTG_HCINTMSK8_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk8;
#[doc = "OTG_HCTSIZ8 register accessor: an alias for `Reg<OTG_HCTSIZ8_SPEC>`"]
pub type OTG_HCTSIZ8 = crate::Reg<otg_hctsiz8::OTG_HCTSIZ8_SPEC>;
#[doc = "OTG host channel 8 transfer size register"]
pub mod otg_hctsiz8;
#[doc = "OTG_HCDMA8 register accessor: an alias for `Reg<OTG_HCDMA8_SPEC>`"]
pub type OTG_HCDMA8 = crate::Reg<otg_hcdma8::OTG_HCDMA8_SPEC>;
#[doc = "OTG host channel 8 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma8;
#[doc = "OTG_HCDMAB8 register accessor: an alias for `Reg<OTG_HCDMAB8_SPEC>`"]
pub type OTG_HCDMAB8 = crate::Reg<otg_hcdmab8::OTG_HCDMAB8_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab8;
#[doc = "OTG_HCCHAR9 register accessor: an alias for `Reg<OTG_HCCHAR9_SPEC>`"]
pub type OTG_HCCHAR9 = crate::Reg<otg_hcchar9::OTG_HCCHAR9_SPEC>;
#[doc = "OTG host channel 9 characteristics register"]
pub mod otg_hcchar9;
#[doc = "OTG_HCSPLT9 register accessor: an alias for `Reg<OTG_HCSPLT9_SPEC>`"]
pub type OTG_HCSPLT9 = crate::Reg<otg_hcsplt9::OTG_HCSPLT9_SPEC>;
#[doc = "OTG host channel 9 split control register"]
pub mod otg_hcsplt9;
#[doc = "OTG_HCINT9 register accessor: an alias for `Reg<OTG_HCINT9_SPEC>`"]
pub type OTG_HCINT9 = crate::Reg<otg_hcint9::OTG_HCINT9_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint9;
#[doc = "OTG_HCINTMSK9 register accessor: an alias for `Reg<OTG_HCINTMSK9_SPEC>`"]
pub type OTG_HCINTMSK9 = crate::Reg<otg_hcintmsk9::OTG_HCINTMSK9_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk9;
#[doc = "OTG_HCTSIZ9 register accessor: an alias for `Reg<OTG_HCTSIZ9_SPEC>`"]
pub type OTG_HCTSIZ9 = crate::Reg<otg_hctsiz9::OTG_HCTSIZ9_SPEC>;
#[doc = "OTG host channel 9 transfer size register"]
pub mod otg_hctsiz9;
#[doc = "OTG_HCDMA9 register accessor: an alias for `Reg<OTG_HCDMA9_SPEC>`"]
pub type OTG_HCDMA9 = crate::Reg<otg_hcdma9::OTG_HCDMA9_SPEC>;
#[doc = "OTG host channel 9 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma9;
#[doc = "OTG_HCDMAB9 register accessor: an alias for `Reg<OTG_HCDMAB9_SPEC>`"]
pub type OTG_HCDMAB9 = crate::Reg<otg_hcdmab9::OTG_HCDMAB9_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab9;
#[doc = "OTG_HCCHAR10 register accessor: an alias for `Reg<OTG_HCCHAR10_SPEC>`"]
pub type OTG_HCCHAR10 = crate::Reg<otg_hcchar10::OTG_HCCHAR10_SPEC>;
#[doc = "OTG host channel 10 characteristics register"]
pub mod otg_hcchar10;
#[doc = "OTG_HCSPLT10 register accessor: an alias for `Reg<OTG_HCSPLT10_SPEC>`"]
pub type OTG_HCSPLT10 = crate::Reg<otg_hcsplt10::OTG_HCSPLT10_SPEC>;
#[doc = "OTG host channel 10 split control register"]
pub mod otg_hcsplt10;
#[doc = "OTG_HCINT10 register accessor: an alias for `Reg<OTG_HCINT10_SPEC>`"]
pub type OTG_HCINT10 = crate::Reg<otg_hcint10::OTG_HCINT10_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint10;
#[doc = "OTG_HCINTMSK10 register accessor: an alias for `Reg<OTG_HCINTMSK10_SPEC>`"]
pub type OTG_HCINTMSK10 = crate::Reg<otg_hcintmsk10::OTG_HCINTMSK10_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk10;
#[doc = "OTG_HCTSIZ10 register accessor: an alias for `Reg<OTG_HCTSIZ10_SPEC>`"]
pub type OTG_HCTSIZ10 = crate::Reg<otg_hctsiz10::OTG_HCTSIZ10_SPEC>;
#[doc = "OTG host channel 10 transfer size register"]
pub mod otg_hctsiz10;
#[doc = "OTG_HCDMA10 register accessor: an alias for `Reg<OTG_HCDMA10_SPEC>`"]
pub type OTG_HCDMA10 = crate::Reg<otg_hcdma10::OTG_HCDMA10_SPEC>;
#[doc = "OTG host channel 10 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma10;
#[doc = "OTG_HCDMAB10 register accessor: an alias for `Reg<OTG_HCDMAB10_SPEC>`"]
pub type OTG_HCDMAB10 = crate::Reg<otg_hcdmab10::OTG_HCDMAB10_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab10;
#[doc = "OTG_HCCHAR11 register accessor: an alias for `Reg<OTG_HCCHAR11_SPEC>`"]
pub type OTG_HCCHAR11 = crate::Reg<otg_hcchar11::OTG_HCCHAR11_SPEC>;
#[doc = "OTG host channel 11 characteristics register"]
pub mod otg_hcchar11;
#[doc = "OTG_HCSPLT11 register accessor: an alias for `Reg<OTG_HCSPLT11_SPEC>`"]
pub type OTG_HCSPLT11 = crate::Reg<otg_hcsplt11::OTG_HCSPLT11_SPEC>;
#[doc = "OTG host channel 11 split control register"]
pub mod otg_hcsplt11;
#[doc = "OTG_HCINT11 register accessor: an alias for `Reg<OTG_HCINT11_SPEC>`"]
pub type OTG_HCINT11 = crate::Reg<otg_hcint11::OTG_HCINT11_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint11;
#[doc = "OTG_HCINTMSK11 register accessor: an alias for `Reg<OTG_HCINTMSK11_SPEC>`"]
pub type OTG_HCINTMSK11 = crate::Reg<otg_hcintmsk11::OTG_HCINTMSK11_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk11;
#[doc = "OTG_HCTSIZ11 register accessor: an alias for `Reg<OTG_HCTSIZ11_SPEC>`"]
pub type OTG_HCTSIZ11 = crate::Reg<otg_hctsiz11::OTG_HCTSIZ11_SPEC>;
#[doc = "OTG host channel 11 transfer size register"]
pub mod otg_hctsiz11;
#[doc = "OTG_HCDMA11 register accessor: an alias for `Reg<OTG_HCDMA11_SPEC>`"]
pub type OTG_HCDMA11 = crate::Reg<otg_hcdma11::OTG_HCDMA11_SPEC>;
#[doc = "OTG host channel 11 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma11;
#[doc = "OTG_HCDMAB11 register accessor: an alias for `Reg<OTG_HCDMAB11_SPEC>`"]
pub type OTG_HCDMAB11 = crate::Reg<otg_hcdmab11::OTG_HCDMAB11_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab11;
#[doc = "OTG_HCCHAR12 register accessor: an alias for `Reg<OTG_HCCHAR12_SPEC>`"]
pub type OTG_HCCHAR12 = crate::Reg<otg_hcchar12::OTG_HCCHAR12_SPEC>;
#[doc = "OTG host channel 12 characteristics register"]
pub mod otg_hcchar12;
#[doc = "OTG_HCSPLT12 register accessor: an alias for `Reg<OTG_HCSPLT12_SPEC>`"]
pub type OTG_HCSPLT12 = crate::Reg<otg_hcsplt12::OTG_HCSPLT12_SPEC>;
#[doc = "OTG host channel 12 split control register"]
pub mod otg_hcsplt12;
#[doc = "OTG_HCINT12 register accessor: an alias for `Reg<OTG_HCINT12_SPEC>`"]
pub type OTG_HCINT12 = crate::Reg<otg_hcint12::OTG_HCINT12_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint12;
#[doc = "OTG_HCINTMSK12 register accessor: an alias for `Reg<OTG_HCINTMSK12_SPEC>`"]
pub type OTG_HCINTMSK12 = crate::Reg<otg_hcintmsk12::OTG_HCINTMSK12_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk12;
#[doc = "OTG_HCTSIZ12 register accessor: an alias for `Reg<OTG_HCTSIZ12_SPEC>`"]
pub type OTG_HCTSIZ12 = crate::Reg<otg_hctsiz12::OTG_HCTSIZ12_SPEC>;
#[doc = "OTG host channel 12 transfer size register"]
pub mod otg_hctsiz12;
#[doc = "OTG_HCDMA12 register accessor: an alias for `Reg<OTG_HCDMA12_SPEC>`"]
pub type OTG_HCDMA12 = crate::Reg<otg_hcdma12::OTG_HCDMA12_SPEC>;
#[doc = "OTG host channel 12 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma12;
#[doc = "OTG_HCDMAB12 register accessor: an alias for `Reg<OTG_HCDMAB12_SPEC>`"]
pub type OTG_HCDMAB12 = crate::Reg<otg_hcdmab12::OTG_HCDMAB12_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab12;
#[doc = "OTG_HCCHAR13 register accessor: an alias for `Reg<OTG_HCCHAR13_SPEC>`"]
pub type OTG_HCCHAR13 = crate::Reg<otg_hcchar13::OTG_HCCHAR13_SPEC>;
#[doc = "OTG host channel 13 characteristics register"]
pub mod otg_hcchar13;
#[doc = "OTG_HCSPLT13 register accessor: an alias for `Reg<OTG_HCSPLT13_SPEC>`"]
pub type OTG_HCSPLT13 = crate::Reg<otg_hcsplt13::OTG_HCSPLT13_SPEC>;
#[doc = "OTG host channel 13 split control register"]
pub mod otg_hcsplt13;
#[doc = "OTG_HCINT13 register accessor: an alias for `Reg<OTG_HCINT13_SPEC>`"]
pub type OTG_HCINT13 = crate::Reg<otg_hcint13::OTG_HCINT13_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint13;
#[doc = "OTG_HCINTMSK13 register accessor: an alias for `Reg<OTG_HCINTMSK13_SPEC>`"]
pub type OTG_HCINTMSK13 = crate::Reg<otg_hcintmsk13::OTG_HCINTMSK13_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk13;
#[doc = "OTG_HCTSIZ13 register accessor: an alias for `Reg<OTG_HCTSIZ13_SPEC>`"]
pub type OTG_HCTSIZ13 = crate::Reg<otg_hctsiz13::OTG_HCTSIZ13_SPEC>;
#[doc = "OTG host channel 13 transfer size register"]
pub mod otg_hctsiz13;
#[doc = "OTG_HCDMA13 register accessor: an alias for `Reg<OTG_HCDMA13_SPEC>`"]
pub type OTG_HCDMA13 = crate::Reg<otg_hcdma13::OTG_HCDMA13_SPEC>;
#[doc = "OTG host channel 13 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma13;
#[doc = "OTG_HCDMAB13 register accessor: an alias for `Reg<OTG_HCDMAB13_SPEC>`"]
pub type OTG_HCDMAB13 = crate::Reg<otg_hcdmab13::OTG_HCDMAB13_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab13;
#[doc = "OTG_HCCHAR14 register accessor: an alias for `Reg<OTG_HCCHAR14_SPEC>`"]
pub type OTG_HCCHAR14 = crate::Reg<otg_hcchar14::OTG_HCCHAR14_SPEC>;
#[doc = "OTG host channel 14 characteristics register"]
pub mod otg_hcchar14;
#[doc = "OTG_HCSPLT14 register accessor: an alias for `Reg<OTG_HCSPLT14_SPEC>`"]
pub type OTG_HCSPLT14 = crate::Reg<otg_hcsplt14::OTG_HCSPLT14_SPEC>;
#[doc = "OTG host channel 14 split control register"]
pub mod otg_hcsplt14;
#[doc = "OTG_HCINT14 register accessor: an alias for `Reg<OTG_HCINT14_SPEC>`"]
pub type OTG_HCINT14 = crate::Reg<otg_hcint14::OTG_HCINT14_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint14;
#[doc = "OTG_HCINTMSK14 register accessor: an alias for `Reg<OTG_HCINTMSK14_SPEC>`"]
pub type OTG_HCINTMSK14 = crate::Reg<otg_hcintmsk14::OTG_HCINTMSK14_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk14;
#[doc = "OTG_HCTSIZ14 register accessor: an alias for `Reg<OTG_HCTSIZ14_SPEC>`"]
pub type OTG_HCTSIZ14 = crate::Reg<otg_hctsiz14::OTG_HCTSIZ14_SPEC>;
#[doc = "OTG host channel 14 transfer size register"]
pub mod otg_hctsiz14;
#[doc = "OTG_HCDMA14 register accessor: an alias for `Reg<OTG_HCDMA14_SPEC>`"]
pub type OTG_HCDMA14 = crate::Reg<otg_hcdma14::OTG_HCDMA14_SPEC>;
#[doc = "OTG host channel 14 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma14;
#[doc = "OTG_HCDMAB14 register accessor: an alias for `Reg<OTG_HCDMAB14_SPEC>`"]
pub type OTG_HCDMAB14 = crate::Reg<otg_hcdmab14::OTG_HCDMAB14_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab14;
#[doc = "OTG_HCCHAR15 register accessor: an alias for `Reg<OTG_HCCHAR15_SPEC>`"]
pub type OTG_HCCHAR15 = crate::Reg<otg_hcchar15::OTG_HCCHAR15_SPEC>;
#[doc = "OTG host channel 15 characteristics register"]
pub mod otg_hcchar15;
#[doc = "OTG_HCSPLT15 register accessor: an alias for `Reg<OTG_HCSPLT15_SPEC>`"]
pub type OTG_HCSPLT15 = crate::Reg<otg_hcsplt15::OTG_HCSPLT15_SPEC>;
#[doc = "OTG host channel 15 split control register"]
pub mod otg_hcsplt15;
#[doc = "OTG_HCINT15 register accessor: an alias for `Reg<OTG_HCINT15_SPEC>`"]
pub type OTG_HCINT15 = crate::Reg<otg_hcint15::OTG_HCINT15_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers."]
pub mod otg_hcint15;
#[doc = "OTG_HCINTMSK15 register accessor: an alias for `Reg<OTG_HCINTMSK15_SPEC>`"]
pub type OTG_HCINTMSK15 = crate::Reg<otg_hcintmsk15::OTG_HCINTMSK15_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod otg_hcintmsk15;
#[doc = "OTG_HCTSIZ15 register accessor: an alias for `Reg<OTG_HCTSIZ15_SPEC>`"]
pub type OTG_HCTSIZ15 = crate::Reg<otg_hctsiz15::OTG_HCTSIZ15_SPEC>;
#[doc = "OTG host channel 15 transfer size register"]
pub mod otg_hctsiz15;
#[doc = "OTG_HCDMA15 register accessor: an alias for `Reg<OTG_HCDMA15_SPEC>`"]
pub type OTG_HCDMA15 = crate::Reg<otg_hcdma15::OTG_HCDMA15_SPEC>;
#[doc = "OTG host channel 15 DMA address register in buffer DMA \\[alternate\\]"]
pub mod otg_hcdma15;
#[doc = "OTG_HCDMAB15 register accessor: an alias for `Reg<OTG_HCDMAB15_SPEC>`"]
pub type OTG_HCDMAB15 = crate::Reg<otg_hcdmab15::OTG_HCDMAB15_SPEC>;
#[doc = "OTG host channel-n DMA address buffer register"]
pub mod otg_hcdmab15;
#[doc = "OTG_DCFG register accessor: an alias for `Reg<OTG_DCFG_SPEC>`"]
pub type OTG_DCFG = crate::Reg<otg_dcfg::OTG_DCFG_SPEC>;
#[doc = "This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
pub mod otg_dcfg;
#[doc = "OTG_DCTL register accessor: an alias for `Reg<OTG_DCTL_SPEC>`"]
pub type OTG_DCTL = crate::Reg<otg_dctl::OTG_DCTL_SPEC>;
#[doc = "OTG device control register"]
pub mod otg_dctl;
#[doc = "OTG_DSTS register accessor: an alias for `Reg<OTG_DSTS_SPEC>`"]
pub type OTG_DSTS = crate::Reg<otg_dsts::OTG_DSTS_SPEC>;
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register."]
pub mod otg_dsts;
#[doc = "OTG_DIEPMSK register accessor: an alias for `Reg<OTG_DIEPMSK_SPEC>`"]
pub type OTG_DIEPMSK = crate::Reg<otg_diepmsk::OTG_DIEPMSK_SPEC>;
#[doc = "This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_diepmsk;
#[doc = "OTG_DOEPMSK register accessor: an alias for `Reg<OTG_DOEPMSK_SPEC>`"]
pub type OTG_DOEPMSK = crate::Reg<otg_doepmsk::OTG_DOEPMSK_SPEC>;
#[doc = "This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_doepmsk;
#[doc = "OTG_DAINT register accessor: an alias for `Reg<OTG_DAINT_SPEC>`"]
pub type OTG_DAINT = crate::Reg<otg_daint::OTG_DAINT_SPEC>;
#[doc = "When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx)."]
pub mod otg_daint;
#[doc = "OTG_DAINTMSK register accessor: an alias for `Reg<OTG_DAINTMSK_SPEC>`"]
pub type OTG_DAINTMSK = crate::Reg<otg_daintmsk::OTG_DAINTMSK_SPEC>;
#[doc = "The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set."]
pub mod otg_daintmsk;
#[doc = "OTG_DVBUSDIS register accessor: an alias for `Reg<OTG_DVBUSDIS_SPEC>`"]
pub type OTG_DVBUSDIS = crate::Reg<otg_dvbusdis::OTG_DVBUSDIS_SPEC>;
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
pub mod otg_dvbusdis;
#[doc = "OTG_DVBUSPULSE register accessor: an alias for `Reg<OTG_DVBUSPULSE_SPEC>`"]
pub type OTG_DVBUSPULSE = crate::Reg<otg_dvbuspulse::OTG_DVBUSPULSE_SPEC>;
#[doc = "This register specifies the VBUS pulsing time during SRP."]
pub mod otg_dvbuspulse;
#[doc = "OTG_DTHRCTL register accessor: an alias for `Reg<OTG_DTHRCTL_SPEC>`"]
pub type OTG_DTHRCTL = crate::Reg<otg_dthrctl::OTG_DTHRCTL_SPEC>;
#[doc = "OTG device threshold control register"]
pub mod otg_dthrctl;
#[doc = "OTG_DIEPEMPMSK register accessor: an alias for `Reg<OTG_DIEPEMPMSK_SPEC>`"]
pub type OTG_DIEPEMPMSK = crate::Reg<otg_diepempmsk::OTG_DIEPEMPMSK_SPEC>;
#[doc = "This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx)."]
pub mod otg_diepempmsk;
#[doc = "OTG_DEACHINT register accessor: an alias for `Reg<OTG_DEACHINT_SPEC>`"]
pub type OTG_DEACHINT = crate::Reg<otg_deachint::OTG_DEACHINT_SPEC>;
#[doc = "OTG device each endpoint interrupt register"]
pub mod otg_deachint;
#[doc = "OTG_DEACHINTMSK register accessor: an alias for `Reg<OTG_DEACHINTMSK_SPEC>`"]
pub type OTG_DEACHINTMSK = crate::Reg<otg_deachintmsk::OTG_DEACHINTMSK_SPEC>;
#[doc = "There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT."]
pub mod otg_deachintmsk;
#[doc = "OTG_HS_DIEPEACHMSK1 register accessor: an alias for `Reg<OTG_HS_DIEPEACHMSK1_SPEC>`"]
pub type OTG_HS_DIEPEACHMSK1 = crate::Reg<otg_hs_diepeachmsk1::OTG_HS_DIEPEACHMSK1_SPEC>;
#[doc = "This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_diepeachmsk1;
#[doc = "OTG_HS_DOEPEACHMSK1 register accessor: an alias for `Reg<OTG_HS_DOEPEACHMSK1_SPEC>`"]
pub type OTG_HS_DOEPEACHMSK1 = crate::Reg<otg_hs_doepeachmsk1::OTG_HS_DOEPEACHMSK1_SPEC>;
#[doc = "This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod otg_hs_doepeachmsk1;
#[doc = "OTG_DIEPCTL0 register accessor: an alias for `Reg<OTG_DIEPCTL0_SPEC>`"]
pub type OTG_DIEPCTL0 = crate::Reg<otg_diepctl0::OTG_DIEPCTL0_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl0;
#[doc = "OTG_DIEPINT0 register accessor: an alias for `Reg<OTG_DIEPINT0_SPEC>`"]
pub type OTG_DIEPINT0 = crate::Reg<otg_diepint0::OTG_DIEPINT0_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint0;
#[doc = "OTG_DIEPTSIZ0 register accessor: an alias for `Reg<OTG_DIEPTSIZ0_SPEC>`"]
pub type OTG_DIEPTSIZ0 = crate::Reg<otg_dieptsiz0::OTG_DIEPTSIZ0_SPEC>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_dieptsiz0;
#[doc = "OTG_DIEPDMA0 register accessor: an alias for `Reg<OTG_DIEPDMA0_SPEC>`"]
pub type OTG_DIEPDMA0 = crate::Reg<otg_diepdma0::OTG_DIEPDMA0_SPEC>;
#[doc = "OTG device IN endpoint 0 DMA address register"]
pub mod otg_diepdma0;
#[doc = "OTG_DTXFSTS0 register accessor: an alias for `Reg<OTG_DTXFSTS0_SPEC>`"]
pub type OTG_DTXFSTS0 = crate::Reg<otg_dtxfsts0::OTG_DTXFSTS0_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts0;
#[doc = "OTG_DIEPCTL1 register accessor: an alias for `Reg<OTG_DIEPCTL1_SPEC>`"]
pub type OTG_DIEPCTL1 = crate::Reg<otg_diepctl1::OTG_DIEPCTL1_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl1;
#[doc = "OTG_DIEPINT1 register accessor: an alias for `Reg<OTG_DIEPINT1_SPEC>`"]
pub type OTG_DIEPINT1 = crate::Reg<otg_diepint1::OTG_DIEPINT1_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint1;
#[doc = "OTG_DIEPTSIZ1 register accessor: an alias for `Reg<OTG_DIEPTSIZ1_SPEC>`"]
pub type OTG_DIEPTSIZ1 = crate::Reg<otg_dieptsiz1::OTG_DIEPTSIZ1_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz1;
#[doc = "OTG_DIEPDMA1 register accessor: an alias for `Reg<OTG_DIEPDMA1_SPEC>`"]
pub type OTG_DIEPDMA1 = crate::Reg<otg_diepdma1::OTG_DIEPDMA1_SPEC>;
#[doc = "OTG device IN endpoint 1 DMA address register"]
pub mod otg_diepdma1;
#[doc = "OTG_DTXFSTS1 register accessor: an alias for `Reg<OTG_DTXFSTS1_SPEC>`"]
pub type OTG_DTXFSTS1 = crate::Reg<otg_dtxfsts1::OTG_DTXFSTS1_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts1;
#[doc = "OTG_DIEPCTL2 register accessor: an alias for `Reg<OTG_DIEPCTL2_SPEC>`"]
pub type OTG_DIEPCTL2 = crate::Reg<otg_diepctl2::OTG_DIEPCTL2_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl2;
#[doc = "OTG_DIEPINT2 register accessor: an alias for `Reg<OTG_DIEPINT2_SPEC>`"]
pub type OTG_DIEPINT2 = crate::Reg<otg_diepint2::OTG_DIEPINT2_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint2;
#[doc = "OTG_DIEPTSIZ2 register accessor: an alias for `Reg<OTG_DIEPTSIZ2_SPEC>`"]
pub type OTG_DIEPTSIZ2 = crate::Reg<otg_dieptsiz2::OTG_DIEPTSIZ2_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz2;
#[doc = "OTG_DIEPDMA2 register accessor: an alias for `Reg<OTG_DIEPDMA2_SPEC>`"]
pub type OTG_DIEPDMA2 = crate::Reg<otg_diepdma2::OTG_DIEPDMA2_SPEC>;
#[doc = "OTG device IN endpoint 2 DMA address register"]
pub mod otg_diepdma2;
#[doc = "OTG_DTXFSTS2 register accessor: an alias for `Reg<OTG_DTXFSTS2_SPEC>`"]
pub type OTG_DTXFSTS2 = crate::Reg<otg_dtxfsts2::OTG_DTXFSTS2_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts2;
#[doc = "OTG_DIEPCTL3 register accessor: an alias for `Reg<OTG_DIEPCTL3_SPEC>`"]
pub type OTG_DIEPCTL3 = crate::Reg<otg_diepctl3::OTG_DIEPCTL3_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl3;
#[doc = "OTG_DIEPINT3 register accessor: an alias for `Reg<OTG_DIEPINT3_SPEC>`"]
pub type OTG_DIEPINT3 = crate::Reg<otg_diepint3::OTG_DIEPINT3_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint3;
#[doc = "OTG_DIEPTSIZ3 register accessor: an alias for `Reg<OTG_DIEPTSIZ3_SPEC>`"]
pub type OTG_DIEPTSIZ3 = crate::Reg<otg_dieptsiz3::OTG_DIEPTSIZ3_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz3;
#[doc = "OTG_DIEPDMA3 register accessor: an alias for `Reg<OTG_DIEPDMA3_SPEC>`"]
pub type OTG_DIEPDMA3 = crate::Reg<otg_diepdma3::OTG_DIEPDMA3_SPEC>;
#[doc = "OTG device IN endpoint 3 DMA address register"]
pub mod otg_diepdma3;
#[doc = "OTG_DTXFSTS3 register accessor: an alias for `Reg<OTG_DTXFSTS3_SPEC>`"]
pub type OTG_DTXFSTS3 = crate::Reg<otg_dtxfsts3::OTG_DTXFSTS3_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts3;
#[doc = "OTG_DIEPCTL4 register accessor: an alias for `Reg<OTG_DIEPCTL4_SPEC>`"]
pub type OTG_DIEPCTL4 = crate::Reg<otg_diepctl4::OTG_DIEPCTL4_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl4;
#[doc = "OTG_DIEPINT4 register accessor: an alias for `Reg<OTG_DIEPINT4_SPEC>`"]
pub type OTG_DIEPINT4 = crate::Reg<otg_diepint4::OTG_DIEPINT4_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint4;
#[doc = "OTG_DIEPTSIZ4 register accessor: an alias for `Reg<OTG_DIEPTSIZ4_SPEC>`"]
pub type OTG_DIEPTSIZ4 = crate::Reg<otg_dieptsiz4::OTG_DIEPTSIZ4_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz4;
#[doc = "OTG_DIEPDMA4 register accessor: an alias for `Reg<OTG_DIEPDMA4_SPEC>`"]
pub type OTG_DIEPDMA4 = crate::Reg<otg_diepdma4::OTG_DIEPDMA4_SPEC>;
#[doc = "OTG device IN endpoint 4 DMA address register"]
pub mod otg_diepdma4;
#[doc = "OTG_DTXFSTS4 register accessor: an alias for `Reg<OTG_DTXFSTS4_SPEC>`"]
pub type OTG_DTXFSTS4 = crate::Reg<otg_dtxfsts4::OTG_DTXFSTS4_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts4;
#[doc = "OTG_DIEPCTL5 register accessor: an alias for `Reg<OTG_DIEPCTL5_SPEC>`"]
pub type OTG_DIEPCTL5 = crate::Reg<otg_diepctl5::OTG_DIEPCTL5_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl5;
#[doc = "OTG_DIEPINT5 register accessor: an alias for `Reg<OTG_DIEPINT5_SPEC>`"]
pub type OTG_DIEPINT5 = crate::Reg<otg_diepint5::OTG_DIEPINT5_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint5;
#[doc = "OTG_DIEPTSIZ5 register accessor: an alias for `Reg<OTG_DIEPTSIZ5_SPEC>`"]
pub type OTG_DIEPTSIZ5 = crate::Reg<otg_dieptsiz5::OTG_DIEPTSIZ5_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz5;
#[doc = "OTG_DIEPDMA5 register accessor: an alias for `Reg<OTG_DIEPDMA5_SPEC>`"]
pub type OTG_DIEPDMA5 = crate::Reg<otg_diepdma5::OTG_DIEPDMA5_SPEC>;
#[doc = "OTG device IN endpoint 5 DMA address register"]
pub mod otg_diepdma5;
#[doc = "OTG_DTXFSTS5 register accessor: an alias for `Reg<OTG_DTXFSTS5_SPEC>`"]
pub type OTG_DTXFSTS5 = crate::Reg<otg_dtxfsts5::OTG_DTXFSTS5_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts5;
#[doc = "OTG_DIEPCTL6 register accessor: an alias for `Reg<OTG_DIEPCTL6_SPEC>`"]
pub type OTG_DIEPCTL6 = crate::Reg<otg_diepctl6::OTG_DIEPCTL6_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl6;
#[doc = "OTG_DIEPINT6 register accessor: an alias for `Reg<OTG_DIEPINT6_SPEC>`"]
pub type OTG_DIEPINT6 = crate::Reg<otg_diepint6::OTG_DIEPINT6_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint6;
#[doc = "OTG_DIEPTSIZ6 register accessor: an alias for `Reg<OTG_DIEPTSIZ6_SPEC>`"]
pub type OTG_DIEPTSIZ6 = crate::Reg<otg_dieptsiz6::OTG_DIEPTSIZ6_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz6;
#[doc = "OTG_DIEPDMA6 register accessor: an alias for `Reg<OTG_DIEPDMA6_SPEC>`"]
pub type OTG_DIEPDMA6 = crate::Reg<otg_diepdma6::OTG_DIEPDMA6_SPEC>;
#[doc = "OTG device IN endpoint 6 DMA address register"]
pub mod otg_diepdma6;
#[doc = "OTG_DTXFSTS6 register accessor: an alias for `Reg<OTG_DTXFSTS6_SPEC>`"]
pub type OTG_DTXFSTS6 = crate::Reg<otg_dtxfsts6::OTG_DTXFSTS6_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts6;
#[doc = "OTG_DIEPCTL7 register accessor: an alias for `Reg<OTG_DIEPCTL7_SPEC>`"]
pub type OTG_DIEPCTL7 = crate::Reg<otg_diepctl7::OTG_DIEPCTL7_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl7;
#[doc = "OTG_DIEPINT7 register accessor: an alias for `Reg<OTG_DIEPINT7_SPEC>`"]
pub type OTG_DIEPINT7 = crate::Reg<otg_diepint7::OTG_DIEPINT7_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint7;
#[doc = "OTG_DIEPTSIZ7 register accessor: an alias for `Reg<OTG_DIEPTSIZ7_SPEC>`"]
pub type OTG_DIEPTSIZ7 = crate::Reg<otg_dieptsiz7::OTG_DIEPTSIZ7_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz7;
#[doc = "OTG_DIEPDMA7 register accessor: an alias for `Reg<OTG_DIEPDMA7_SPEC>`"]
pub type OTG_DIEPDMA7 = crate::Reg<otg_diepdma7::OTG_DIEPDMA7_SPEC>;
#[doc = "OTG device IN endpoint 7 DMA address register"]
pub mod otg_diepdma7;
#[doc = "OTG_DTXFSTS7 register accessor: an alias for `Reg<OTG_DTXFSTS7_SPEC>`"]
pub type OTG_DTXFSTS7 = crate::Reg<otg_dtxfsts7::OTG_DTXFSTS7_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts7;
#[doc = "OTG_DIEPCTL8 register accessor: an alias for `Reg<OTG_DIEPCTL8_SPEC>`"]
pub type OTG_DIEPCTL8 = crate::Reg<otg_diepctl8::OTG_DIEPCTL8_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_diepctl8;
#[doc = "OTG_DIEPINT8 register accessor: an alias for `Reg<OTG_DIEPINT8_SPEC>`"]
pub type OTG_DIEPINT8 = crate::Reg<otg_diepint8::OTG_DIEPINT8_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_diepint8;
#[doc = "OTG_DIEPTSIZ8 register accessor: an alias for `Reg<OTG_DIEPTSIZ8_SPEC>`"]
pub type OTG_DIEPTSIZ8 = crate::Reg<otg_dieptsiz8::OTG_DIEPTSIZ8_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_dieptsiz8;
#[doc = "OTG_DIEPDMA8 register accessor: an alias for `Reg<OTG_DIEPDMA8_SPEC>`"]
pub type OTG_DIEPDMA8 = crate::Reg<otg_diepdma8::OTG_DIEPDMA8_SPEC>;
#[doc = "OTG device IN endpoint 8 DMA address register"]
pub mod otg_diepdma8;
#[doc = "OTG_DTXFSTS8 register accessor: an alias for `Reg<OTG_DTXFSTS8_SPEC>`"]
pub type OTG_DTXFSTS8 = crate::Reg<otg_dtxfsts8::OTG_DTXFSTS8_SPEC>;
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO."]
pub mod otg_dtxfsts8;
#[doc = "OTG_DOEPCTL0 register accessor: an alias for `Reg<OTG_DOEPCTL0_SPEC>`"]
pub type OTG_DOEPCTL0 = crate::Reg<otg_doepctl0::OTG_DOEPCTL0_SPEC>;
#[doc = "This section describes the OTG_DOEPCTL0 register."]
pub mod otg_doepctl0;
#[doc = "OTG_DOEPINT0 register accessor: an alias for `Reg<OTG_DOEPINT0_SPEC>`"]
pub type OTG_DOEPINT0 = crate::Reg<otg_doepint0::OTG_DOEPINT0_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint0;
#[doc = "OTG_DOEPTSIZ0 register accessor: an alias for `Reg<OTG_DOEPTSIZ0_SPEC>`"]
pub type OTG_DOEPTSIZ0 = crate::Reg<otg_doeptsiz0::OTG_DOEPTSIZ0_SPEC>;
#[doc = "The application must modify this register before enabling endpoint 0."]
pub mod otg_doeptsiz0;
#[doc = "OTG_DOEPDMA0 register accessor: an alias for `Reg<OTG_DOEPDMA0_SPEC>`"]
pub type OTG_DOEPDMA0 = crate::Reg<otg_doepdma0::OTG_DOEPDMA0_SPEC>;
#[doc = "OTG device OUT endpoint 0 DMA address register"]
pub mod otg_doepdma0;
#[doc = "OTG_DOEPCTL1 register accessor: an alias for `Reg<OTG_DOEPCTL1_SPEC>`"]
pub type OTG_DOEPCTL1 = crate::Reg<otg_doepctl1::OTG_DOEPCTL1_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl1;
#[doc = "OTG_DOEPINT1 register accessor: an alias for `Reg<OTG_DOEPINT1_SPEC>`"]
pub type OTG_DOEPINT1 = crate::Reg<otg_doepint1::OTG_DOEPINT1_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint1;
#[doc = "OTG_DOEPTSIZ1 register accessor: an alias for `Reg<OTG_DOEPTSIZ1_SPEC>`"]
pub type OTG_DOEPTSIZ1 = crate::Reg<otg_doeptsiz1::OTG_DOEPTSIZ1_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz1;
#[doc = "OTG_DOEPDMA1 register accessor: an alias for `Reg<OTG_DOEPDMA1_SPEC>`"]
pub type OTG_DOEPDMA1 = crate::Reg<otg_doepdma1::OTG_DOEPDMA1_SPEC>;
#[doc = "OTG device OUT endpoint 1 DMA address register"]
pub mod otg_doepdma1;
#[doc = "OTG_DOEPCTL2 register accessor: an alias for `Reg<OTG_DOEPCTL2_SPEC>`"]
pub type OTG_DOEPCTL2 = crate::Reg<otg_doepctl2::OTG_DOEPCTL2_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl2;
#[doc = "OTG_DOEPINT2 register accessor: an alias for `Reg<OTG_DOEPINT2_SPEC>`"]
pub type OTG_DOEPINT2 = crate::Reg<otg_doepint2::OTG_DOEPINT2_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint2;
#[doc = "OTG_DOEPTSIZ2 register accessor: an alias for `Reg<OTG_DOEPTSIZ2_SPEC>`"]
pub type OTG_DOEPTSIZ2 = crate::Reg<otg_doeptsiz2::OTG_DOEPTSIZ2_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz2;
#[doc = "OTG_DOEPDMA2 register accessor: an alias for `Reg<OTG_DOEPDMA2_SPEC>`"]
pub type OTG_DOEPDMA2 = crate::Reg<otg_doepdma2::OTG_DOEPDMA2_SPEC>;
#[doc = "OTG device OUT endpoint 2 DMA address register"]
pub mod otg_doepdma2;
#[doc = "OTG_DOEPCTL3 register accessor: an alias for `Reg<OTG_DOEPCTL3_SPEC>`"]
pub type OTG_DOEPCTL3 = crate::Reg<otg_doepctl3::OTG_DOEPCTL3_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl3;
#[doc = "OTG_DOEPINT3 register accessor: an alias for `Reg<OTG_DOEPINT3_SPEC>`"]
pub type OTG_DOEPINT3 = crate::Reg<otg_doepint3::OTG_DOEPINT3_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint3;
#[doc = "OTG_DOEPTSIZ3 register accessor: an alias for `Reg<OTG_DOEPTSIZ3_SPEC>`"]
pub type OTG_DOEPTSIZ3 = crate::Reg<otg_doeptsiz3::OTG_DOEPTSIZ3_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz3;
#[doc = "OTG_DOEPDMA3 register accessor: an alias for `Reg<OTG_DOEPDMA3_SPEC>`"]
pub type OTG_DOEPDMA3 = crate::Reg<otg_doepdma3::OTG_DOEPDMA3_SPEC>;
#[doc = "OTG device OUT endpoint 3 DMA address register"]
pub mod otg_doepdma3;
#[doc = "OTG_DOEPCTL4 register accessor: an alias for `Reg<OTG_DOEPCTL4_SPEC>`"]
pub type OTG_DOEPCTL4 = crate::Reg<otg_doepctl4::OTG_DOEPCTL4_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl4;
#[doc = "OTG_DOEPINT4 register accessor: an alias for `Reg<OTG_DOEPINT4_SPEC>`"]
pub type OTG_DOEPINT4 = crate::Reg<otg_doepint4::OTG_DOEPINT4_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint4;
#[doc = "OTG_DOEPTSIZ4 register accessor: an alias for `Reg<OTG_DOEPTSIZ4_SPEC>`"]
pub type OTG_DOEPTSIZ4 = crate::Reg<otg_doeptsiz4::OTG_DOEPTSIZ4_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz4;
#[doc = "OTG_DOEPDMA4 register accessor: an alias for `Reg<OTG_DOEPDMA4_SPEC>`"]
pub type OTG_DOEPDMA4 = crate::Reg<otg_doepdma4::OTG_DOEPDMA4_SPEC>;
#[doc = "OTG device OUT endpoint 4 DMA address register"]
pub mod otg_doepdma4;
#[doc = "OTG_DOEPCTL5 register accessor: an alias for `Reg<OTG_DOEPCTL5_SPEC>`"]
pub type OTG_DOEPCTL5 = crate::Reg<otg_doepctl5::OTG_DOEPCTL5_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl5;
#[doc = "OTG_DOEPINT5 register accessor: an alias for `Reg<OTG_DOEPINT5_SPEC>`"]
pub type OTG_DOEPINT5 = crate::Reg<otg_doepint5::OTG_DOEPINT5_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint5;
#[doc = "OTG_DOEPTSIZ5 register accessor: an alias for `Reg<OTG_DOEPTSIZ5_SPEC>`"]
pub type OTG_DOEPTSIZ5 = crate::Reg<otg_doeptsiz5::OTG_DOEPTSIZ5_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz5;
#[doc = "OTG_DOEPDMA5 register accessor: an alias for `Reg<OTG_DOEPDMA5_SPEC>`"]
pub type OTG_DOEPDMA5 = crate::Reg<otg_doepdma5::OTG_DOEPDMA5_SPEC>;
#[doc = "OTG device OUT endpoint 5 DMA address register"]
pub mod otg_doepdma5;
#[doc = "OTG_DOEPCTL6 register accessor: an alias for `Reg<OTG_DOEPCTL6_SPEC>`"]
pub type OTG_DOEPCTL6 = crate::Reg<otg_doepctl6::OTG_DOEPCTL6_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl6;
#[doc = "OTG_DOEPINT6 register accessor: an alias for `Reg<OTG_DOEPINT6_SPEC>`"]
pub type OTG_DOEPINT6 = crate::Reg<otg_doepint6::OTG_DOEPINT6_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint6;
#[doc = "OTG_DOEPTSIZ6 register accessor: an alias for `Reg<OTG_DOEPTSIZ6_SPEC>`"]
pub type OTG_DOEPTSIZ6 = crate::Reg<otg_doeptsiz6::OTG_DOEPTSIZ6_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz6;
#[doc = "OTG_DOEPDMA6 register accessor: an alias for `Reg<OTG_DOEPDMA6_SPEC>`"]
pub type OTG_DOEPDMA6 = crate::Reg<otg_doepdma6::OTG_DOEPDMA6_SPEC>;
#[doc = "OTG device OUT endpoint 6 DMA address register"]
pub mod otg_doepdma6;
#[doc = "OTG_DOEPCTL7 register accessor: an alias for `Reg<OTG_DOEPCTL7_SPEC>`"]
pub type OTG_DOEPCTL7 = crate::Reg<otg_doepctl7::OTG_DOEPCTL7_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl7;
#[doc = "OTG_DOEPINT7 register accessor: an alias for `Reg<OTG_DOEPINT7_SPEC>`"]
pub type OTG_DOEPINT7 = crate::Reg<otg_doepint7::OTG_DOEPINT7_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint7;
#[doc = "OTG_DOEPTSIZ7 register accessor: an alias for `Reg<OTG_DOEPTSIZ7_SPEC>`"]
pub type OTG_DOEPTSIZ7 = crate::Reg<otg_doeptsiz7::OTG_DOEPTSIZ7_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz7;
#[doc = "OTG_DOEPDMA7 register accessor: an alias for `Reg<OTG_DOEPDMA7_SPEC>`"]
pub type OTG_DOEPDMA7 = crate::Reg<otg_doepdma7::OTG_DOEPDMA7_SPEC>;
#[doc = "OTG device OUT endpoint 7 DMA address register"]
pub mod otg_doepdma7;
#[doc = "OTG_DOEPCTL8 register accessor: an alias for `Reg<OTG_DOEPCTL8_SPEC>`"]
pub type OTG_DOEPCTL8 = crate::Reg<otg_doepctl8::OTG_DOEPCTL8_SPEC>;
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0."]
pub mod otg_doepctl8;
#[doc = "OTG_DOEPINT8 register accessor: an alias for `Reg<OTG_DOEPINT8_SPEC>`"]
pub type OTG_DOEPINT8 = crate::Reg<otg_doepint8::OTG_DOEPINT8_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers."]
pub mod otg_doepint8;
#[doc = "OTG_DOEPTSIZ8 register accessor: an alias for `Reg<OTG_DOEPTSIZ8_SPEC>`"]
pub type OTG_DOEPTSIZ8 = crate::Reg<otg_doeptsiz8::OTG_DOEPTSIZ8_SPEC>;
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit."]
pub mod otg_doeptsiz8;
#[doc = "OTG_DOEPDMA8 register accessor: an alias for `Reg<OTG_DOEPDMA8_SPEC>`"]
pub type OTG_DOEPDMA8 = crate::Reg<otg_doepdma8::OTG_DOEPDMA8_SPEC>;
#[doc = "OTG device OUT endpoint 8 DMA address register"]
pub mod otg_doepdma8;
#[doc = "OTG_PCGCCTL register accessor: an alias for `Reg<OTG_PCGCCTL_SPEC>`"]
pub type OTG_PCGCCTL = crate::Reg<otg_pcgcctl::OTG_PCGCCTL_SPEC>;
#[doc = "This register is available in host and device modes."]
pub mod otg_pcgcctl;
