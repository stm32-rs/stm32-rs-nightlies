///Register `CR1_enabled` reader
pub type R = crate::R<CR1_ENABLEDrs>;
///Register `CR1_enabled` writer
pub type W = crate::W<CR1_ENABLEDrs>;
///Field `UE` reader - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_R = crate::BitReader;
///Field `UE` writer - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UESM` reader - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type UESM_R = crate::BitReader;
///Field `UESM` writer - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_R = crate::BitReader;
///Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_R = crate::BitReader;
///Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_R = crate::BitReader;
///Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFNEIE` reader - RXFIFO not empty interrupt enable This bit is set and cleared by software.
pub type RXFNEIE_R = crate::BitReader;
///Field `RXFNEIE` writer - RXFIFO not empty interrupt enable This bit is set and cleared by software.
pub type RXFNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNFIE` reader - TXFIFO not-full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_R = crate::BitReader;
///Field `TXFNFIE` writer - TXFIFO not-full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_R = crate::BitReader;
///Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
pub type PS_R = crate::BitReader;
///Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
pub type PCE_R = crate::BitReader;
///Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE` reader - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
pub type WAKE_R = crate::BitReader;
///Field `WAKE` writer - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
pub type M0_R = crate::BitReader;
///Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MME` reader - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_R = crate::BitReader;
///Field `MME` writer - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_R = crate::BitReader;
///Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVER8` reader - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_R = crate::BitReader;
///Field `OVER8` writer - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDT` reader - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEAT` reader - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `RTOIE` reader - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type RTOIE_R = crate::BitReader;
///Field `RTOIE` writer - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOBIE` reader - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type EOBIE_R = crate::BitReader;
///Field `EOBIE` writer - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
= 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
= 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
= 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
pub type M1_R = crate::BitReader;
/**Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
= 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
= 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
= 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_R = crate::BitReader;
///Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFEIE` reader - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_R = crate::BitReader;
///Field `TXFEIE` writer - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFIE` reader - RXFIFO full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_R = crate::BitReader;
///Field `RXFFIE` writer - RXFIFO full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not-full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    /**Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
    = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
    = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFIFO full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1_enabled")
            .field("ue", &self.ue())
            .field("uesm", &self.uesm())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("idleie", &self.idleie())
            .field("rxfneie", &self.rxfneie())
            .field("tcie", &self.tcie())
            .field("txfnfie", &self.txfnfie())
            .field("peie", &self.peie())
            .field("ps", &self.ps())
            .field("pce", &self.pce())
            .field("wake", &self.wake())
            .field("m0", &self.m0())
            .field("mme", &self.mme())
            .field("cmie", &self.cmie())
            .field("over8", &self.over8())
            .field("dedt", &self.dedt())
            .field("deat", &self.deat())
            .field("rtoie", &self.rtoie())
            .field("eobie", &self.eobie())
            .field("m1", &self.m1())
            .field("fifoen", &self.fifoen())
            .field("txfeie", &self.txfeie())
            .field("rxffie", &self.rxffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<CR1_ENABLEDrs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<CR1_ENABLEDrs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<CR1_ENABLEDrs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<CR1_ENABLEDrs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1_ENABLEDrs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxfneie(&mut self) -> RXFNEIE_W<CR1_ENABLEDrs> {
        RXFNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<CR1_ENABLEDrs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXFIFO not-full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&mut self) -> TXFNFIE_W<CR1_ENABLEDrs> {
        TXFNFIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<CR1_ENABLEDrs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<CR1_ENABLEDrs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<CR1_ENABLEDrs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<CR1_ENABLEDrs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<CR1_ENABLEDrs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<CR1_ENABLEDrs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<CR1_ENABLEDrs> {
        CMIE_W::new(self, 14)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<CR1_ENABLEDrs> {
        OVER8_W::new(self, 15)
    }
    ///Bits 16:20 - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<CR1_ENABLEDrs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<CR1_ENABLEDrs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<CR1_ENABLEDrs> {
        RTOIE_W::new(self, 26)
    }
    ///Bit 27 - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn eobie(&mut self) -> EOBIE_W<CR1_ENABLEDrs> {
        EOBIE_W::new(self, 27)
    }
    /**Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\]
    = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\]
    = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<CR1_ENABLEDrs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<CR1_ENABLEDrs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<CR1_ENABLEDrs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFIFO full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<CR1_ENABLEDrs> {
        RXFFIE_W::new(self, 31)
    }
}
/**USART control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1_enabled::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1_enabled::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#USART1:CR1_enabled)*/
pub struct CR1_ENABLEDrs;
impl crate::RegisterSpec for CR1_ENABLEDrs {
    type Ux = u32;
}
///`read()` method returns [`cr1_enabled::R`](R) reader structure
impl crate::Readable for CR1_ENABLEDrs {}
///`write(|w| ..)` method takes [`cr1_enabled::W`](W) writer structure
impl crate::Writable for CR1_ENABLEDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR1_enabled to value 0
impl crate::Resettable for CR1_ENABLEDrs {
    const RESET_VALUE: u32 = 0;
}
