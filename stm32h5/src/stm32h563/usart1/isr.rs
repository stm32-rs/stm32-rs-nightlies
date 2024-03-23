#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
pub type PE_R = crate::BitReader;
#[doc = "Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
pub type FE_R = crate::BitReader;
#[doc = "Field `NE` reader - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page 2317). This error is associated with the character in the USART_RDR."]
pub type NE_R = crate::BitReader;
#[doc = "Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
pub type ORE_R = crate::BitReader;
#[doc = "Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `RXFNE` reader - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register."]
pub type RXFNE_R = crate::BitReader;
#[doc = "Field `TC` reader - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXE/TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by writing to the USART_TDR register."]
pub type TC_R = crate::BitReader;
#[doc = "Field `TXFNF` reader - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE is set at the same time). This bit is used during single buffer transmission."]
pub type TXFNF_R = crate::BitReader;
#[doc = "Field `LBDF` reader - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
pub type LBDF_R = crate::BitReader;
#[doc = "Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTSIF_R = crate::BitReader;
#[doc = "Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
pub type CTS_R = crate::BitReader;
#[doc = "Field `RTOF` reader - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
pub type RTOF_R = crate::BitReader;
#[doc = "Field `EOBF` reader - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
pub type EOBF_R = crate::BitReader;
#[doc = "Field `UDR` reader - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
pub type UDR_R = crate::BitReader;
#[doc = "Field `ABRE` reader - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type ABRE_R = crate::BitReader;
#[doc = "Field `ABRF` reader - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
pub type ABRF_R = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
pub type CMF_R = crate::BitReader;
#[doc = "Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
pub type SBKF_R = crate::BitReader;
#[doc = "Field `RWU` reader - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type RWU_R = crate::BitReader;
#[doc = "Field `WUF` reader - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type WUF_R = crate::BitReader;
#[doc = "Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
pub type TEACK_R = crate::BitReader;
#[doc = "Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
pub type REACK_R = crate::BitReader;
#[doc = "Field `TXFE` reader - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register."]
pub type TXFE_R = crate::BitReader;
#[doc = "Field `RXFF` reader - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register."]
pub type RXFF_R = crate::BitReader;
#[doc = "Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is ‘1’. Refer to on page 2297.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT {
    #[doc = "0: Transmission not completed"]
    NotCompleted = 0,
    #[doc = "1: Transmission has completed"]
    Completed = 1,
}
impl From<TCBGT> for bool {
    #[inline(always)]
    fn from(variant: TCBGT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCBGT` reader - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is ‘1’. Refer to on page 2297."]
pub type TCBGT_R = crate::BitReader<TCBGT>;
impl TCBGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT {
        match self.bits {
            false => TCBGT::NotCompleted,
            true => TCBGT::Completed,
        }
    }
    #[doc = "Transmission not completed"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCBGT::NotCompleted
    }
    #[doc = "Transmission has completed"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCBGT::Completed
    }
}
#[doc = "Field `RXFT` reader - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to ‘101’, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data."]
pub type RXFT_R = crate::BitReader;
#[doc = "Field `TXFT` reader - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register."]
pub type TXFT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Tolerance of the USART receiver to clock deviation on page 2317). This error is associated with the character in the USART_RDR."]
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register."]
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXE/TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by writing to the USART_TDR register."]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE is set at the same time). This bit is used during single buffer transmission."]
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value."]
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not)."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\\[7:0\\]
is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register."]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission."]
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode flag This bit is set by hardware, when a wakeup event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE=1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period."]
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is ‘1’. Refer to on page 2297."]
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to ‘101’, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data."]
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register."]
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "USART interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0xc0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0xc0;
}
