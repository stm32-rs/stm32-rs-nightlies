///Register `LPUART_CR1` reader
pub type R = crate::R<LPUART_CR1rs>;
///Register `LPUART_CR1` writer
pub type W = crate::W<LPUART_CR1rs>;
///Field `UE` reader - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit.
pub type UE_R = crate::BitReader;
///Field `UE` writer - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit.
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UESM` reader - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.
pub type UESM_R = crate::BitReader;
///Field `UESM` writer - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_R = crate::BitReader;
///Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_R = crate::BitReader;
///Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
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
///Field `TXFNFIE` reader - TXFIFO not full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_R = crate::BitReader;
///Field `TXFNFIE` writer - TXFIFO not full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_R = crate::BitReader;
///Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type PS_R = crate::BitReader;
///Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0).
pub type PCE_R = crate::BitReader;
///Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0).
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE` reader - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type WAKE_R = crate::BitReader;
///Field `WAKE` writer - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0).
pub type M0_R = crate::BitReader;
///Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0).
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MME` reader - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_R = crate::BitReader;
///Field `MME` writer - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_R = crate::BitReader;
///Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDT` reader - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEAT` reader - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0).
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
pub type M1_R = crate::BitReader;
/**Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
= 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\]
= 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\]
= 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software.
pub type FIFOEN_R = crate::BitReader;
///Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software.
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFEIE` reader - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_R = crate::BitReader;
///Field `TXFEIE` writer - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFIE` reader - RXFIFO Full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_R = crate::BitReader;
///Field `RXFFIE` writer - RXFIFO Full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
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
    ///Bit 7 - TXFIFO not full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    /**Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\]
    = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\]
    = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFIFO Full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_CR1")
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
            .field("dedt", &self.dedt())
            .field("deat", &self.deat())
            .field("m1", &self.m1())
            .field("fifoen", &self.fifoen())
            .field("txfeie", &self.txfeie())
            .field("rxffie", &self.rxffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPUART enable When this bit is cleared, the LPUART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the LPUART is kept, but all the status flags, in the LPUART_ISR are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be reset before and the software must wait for the TC bit in the LPUART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit.
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<LPUART_CR1rs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - LPUART enable in low-power mode When this bit is cleared, the LPUART cannot wake up the MCU from low-power mode. When this bit is set, the LPUART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode, and clear it when exiting low-power mode.
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<LPUART_CR1rs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<LPUART_CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit (0 followed by 1) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1. To ensure the required duration, the software can poll the TEACK bit in the LPUART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<LPUART_CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<LPUART_CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxfneie(&mut self) -> RXFNEIE_W<LPUART_CR1rs> {
        RXFNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<LPUART_CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXFIFO not full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&mut self) -> TXFNFIE_W<LPUART_CR1rs> {
        TXFNFIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<LPUART_CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<LPUART_CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<LPUART_CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the LPUART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<LPUART_CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1) description). This bit can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<LPUART_CR1rs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable This bit activates the Mute mode function of the LPUART. When set, the LPUART can switch between the active and Mute modes, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<LPUART_CR1rs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<LPUART_CR1rs> {
        CMIE_W::new(self, 14)
    }
    ///Bits 16:20 - Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal.It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section132.4.14: RS232 Hardware flow control and RS485 Driver Enable. If the LPUART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<LPUART_CR1rs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details, refer Section131.5.21: RS232 Hardware flow control and RS485 Driver Enable. This bitfield can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<LPUART_CR1rs> {
        DEAT_W::new(self, 21)
    }
    /**Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\]
    = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\]
    = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\]
    = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the LPUART is disabled (UE=0). Note: In 7-bit data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.*/
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<LPUART_CR1rs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<LPUART_CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<LPUART_CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFIFO Full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<LPUART_CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
/**LPUART control register 1

You can [`read`](crate::Reg::read) this register and get [`lpuart_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#LPUART1:LPUART_CR1)*/
pub struct LPUART_CR1rs;
impl crate::RegisterSpec for LPUART_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`lpuart_cr1::R`](R) reader structure
impl crate::Readable for LPUART_CR1rs {}
///`write(|w| ..)` method takes [`lpuart_cr1::W`](W) writer structure
impl crate::Writable for LPUART_CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPUART_CR1 to value 0
impl crate::Resettable for LPUART_CR1rs {
    const RESET_VALUE: u32 = 0;
}