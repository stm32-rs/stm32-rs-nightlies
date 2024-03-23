#[doc = "Register `ISR_enabled` reader"]
pub type R = crate::R<ISR_ENABLEDrs>;
#[doc = "Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the LPUART_ICR register. An interrupt is generated if PEIE = 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
pub type PE_R = crate::BitReader;
#[doc = "Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIEÂ =Â 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
pub type FE_R = crate::BitReader;
#[doc = "Field `NE` reader - Start bit noise detection flag This bit is set by hardware when noise is detected on the start bit of a received frame. It is cleared by software, writing 1 to the NECF bit in the LPUART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. This error is associated with the character in the LPUART_RDR."]
pub type NE_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the LPUART_ICR register. An interrupt is generated if RXFNEIEÂ =Â 1 or EIE = 1 in the LPUART_CR1 register. Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the LPUART_CR3 register."]
pub type ORE_R = crate::BitReader;
#[doc = "Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle line is detected. An interrupt is generated if IDLEIEÂ =Â 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the LPUART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MMEÂ =Â 1), IDLE is set if the LPUART is not mute (RWUÂ =Â 0), whatever the Mute mode selected by the WAKE bit. If RWUÂ =Â 1, IDLE is not set."]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXFNE` reader - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register. An interrupt is generated if RXFNEIEÂ =Â 1 in the LPUART_CR1 register."]
pub type RXFNE_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete This bit is set by hardware if the transmission of a frame containing data is complete and if TXFF is set. An interrupt is generated if TCIEÂ =Â 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the TCCF in the LPUART_ICR register or by a write to the LPUART_TDR register. An interrupt is generated if TCIEÂ =Â 1 in the LPUART_CR1 register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately."]
pub type TC_R = crate::BitReader;
#[doc = "Field `TXFNF` reader - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the LPUART_TDR. The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). An interrupt is generated if the TXFNFIE bit Â =Â 1 in the LPUART_CR1 register. Note: This bit is used during single buffer transmission."]
pub type TXFNF_R = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the LPUART_ICR register. An interrupt is generated if CTSIEÂ =Â 1 in the LPUART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTSIF_R = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTS_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the LPUART_ICR register. An interrupt is generated if CMIEÂ =Â 1in the LPUART_CR1 register."]
pub type CMF_R = crate::BitReader;
#[doc = "Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
pub type SBKF_R = crate::BitReader;
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode This bit indicates if the LPUART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the LPUART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the LPUART_RQR register. Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value."]
pub type RWU_R = crate::BitReader;
#[doc = "Field `WUF` reader - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the LPUART_ICR register. An interrupt is generated if WUFIEÂ =Â 1 in the LPUART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value"]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the LPUART. It can be used when an idle frame request is generated by writing TEÂ =Â 0, followed by TEÂ =Â 1 in the LPUART_CR1 register, in order to respect the TEÂ =Â 0 minimum period."]
pub type TEACK_R = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the LPUART. It can be used to verify that the LPUART is ready for reception before entering low-power mode. Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value."]
pub type REACK_R = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the LPUART_RQR register. An interrupt is generated if the TXFEIE bit Â =Â 1 (bit 30) in the LPUART_CR1 register."]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `RXFF` reader - RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOÂ sizeÂ +Â 1 (RXFIFO full + 1 data in the LPUART_RDR register. An interrupt is generated if the RXFFIE bit Â =Â 1 in the LPUART_CR1 register."]
pub type RXFF_R = crate::BitReader;
#[doc = "Field `RXFT` reader - RXFIFO threshold flag This bit is set by hardware when the RXFIFO reaches the threshold programmed in RXFTCFG in LPUART_CR3 register i.e. the Receive FIFO contains RXFTCFG data. An interrupt is generated if the RXFTIE bit Â =Â 1 (bit 27) in the LPUART_CR3 register."]
pub type RXFT_R = crate::BitReader;
#[doc = "Field `TXFT` reader - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG in LPUART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit Â =Â 1 (bit 31) in the LPUART_CR3 register."]
pub type TXFT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the LPUART_ICR register. An interrupt is generated if PEIE = 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the LPUART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIEÂ =Â 1 in the LPUART_CR1 register. Note: This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start bit noise detection flag This bit is set by hardware when noise is detected on the start bit of a received frame. It is cleared by software, writing 1 to the NECF bit in the LPUART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. This error is associated with the character in the LPUART_RDR."]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the LPUART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the LPUART_ICR register. An interrupt is generated if RXFNEIEÂ =Â 1 or EIE = 1 in the LPUART_CR1 register. Note: When this bit is set, the LPUART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected This bit is set by hardware when an Idle line is detected. An interrupt is generated if IDLEIEÂ =Â 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the LPUART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MMEÂ =Â 1), IDLE is set if the LPUART is not mute (RWUÂ =Â 0), whatever the Mute mode selected by the WAKE bit. If RWUÂ =Â 1, IDLE is not set."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the LPUART_RDR register. Every read of the LPUART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the LPUART_RQR register. An interrupt is generated if RXFNEIEÂ =Â 1 in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete This bit is set by hardware if the transmission of a frame containing data is complete and if TXFF is set. An interrupt is generated if TCIEÂ =Â 1 in the LPUART_CR1 register. It is cleared by software, writing 1 to the TCCF in the LPUART_ICR register or by a write to the LPUART_TDR register. An interrupt is generated if TCIEÂ =Â 1 in the LPUART_CR1 register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the LPUART_TDR. Every write in the LPUART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the LPUART_TDR. The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE are set at the same time). An interrupt is generated if the TXFNFIE bit Â =Â 1 in the LPUART_CR1 register. Note: This bit is used during single buffer transmission."]
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the LPUART_ICR register. An interrupt is generated if CTSIEÂ =Â 1 in the LPUART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the LPUART_ICR register. An interrupt is generated if CMIEÂ =Â 1in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the LPUART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode This bit indicates if the LPUART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the LPUART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the LPUART_RQR register. Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the LPUART_ICR register. An interrupt is generated if WUFIEÂ =Â 1 in the LPUART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value"]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the LPUART. It can be used when an idle frame request is generated by writing TEÂ =Â 0, followed by TEÂ =Â 1 in the LPUART_CR1 register, in order to respect the TEÂ =Â 0 minimum period."]
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the LPUART. It can be used to verify that the LPUART is ready for reception before entering low-power mode. Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO empty This bit is set by hardware when TXFIFO is empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the LPUART_RQR register. An interrupt is generated if the TXFEIE bit Â =Â 1 (bit 30) in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO full This bit is set by hardware when the number of received data corresponds to RXFIFOÂ sizeÂ +Â 1 (RXFIFO full + 1 data in the LPUART_RDR register. An interrupt is generated if the RXFFIE bit Â =Â 1 in the LPUART_CR1 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFIFO threshold flag This bit is set by hardware when the RXFIFO reaches the threshold programmed in RXFTCFG in LPUART_CR3 register i.e. the Receive FIFO contains RXFTCFG data. An interrupt is generated if the RXFTIE bit Â =Â 1 (bit 27) in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG in LPUART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit Â =Â 1 (bit 31) in the LPUART_CR3 register."]
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "LPUART interrupt and status register \\[alternate\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr_enabled::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_ENABLEDrs;
impl crate::RegisterSpec for ISR_ENABLEDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr_enabled::R`](R) reader structure"]
impl crate::Readable for ISR_ENABLEDrs {}
#[doc = "`reset()` method sets ISR_enabled to value 0x0080_00c0"]
impl crate::Resettable for ISR_ENABLEDrs {
    const RESET_VALUE: u32 = 0x0080_00c0;
}
