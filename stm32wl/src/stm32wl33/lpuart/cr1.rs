///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `UE` reader - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled
pub type UE_R = crate::BitReader;
///Field `UE` writer - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UESM` reader - UESM: LPUART enable in Stop mode When this bit is cleared, the LPUART is not able to wake up the MCU from Stop mode. When this bit is set, the LPUART is able to wake up the MCU from Stop mode, provided that the LPUART clock selection is LSE in the RCC. This bit is set and cleared by software. -0: LPUART not able to wake up the MCU from Stop mode. -1: LPUART able to wake up the MCU from Stop mode. When this function is active, the clock source for the LPUART must be LSE (see RCC chapter)
pub type UESM_R = crate::BitReader;
///Field `UESM` writer - UESM: LPUART enable in Stop mode When this bit is cleared, the LPUART is not able to wake up the MCU from Stop mode. When this bit is set, the LPUART is able to wake up the MCU from Stop mode, provided that the LPUART clock selection is LSE in the RCC. This bit is set and cleared by software. -0: LPUART not able to wake up the MCU from Stop mode. -1: LPUART able to wake up the MCU from Stop mode. When this function is active, the clock source for the LPUART must be LSE (see RCC chapter)
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RE` reader - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit
pub type RE_R = crate::BitReader;
///Field `RE` writer - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TE` reader - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled
pub type TE_R = crate::BitReader;
///Field `TE` writer - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLEIE` reader - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register
pub type IDLEIE_R = crate::BitReader;
///Field `IDLEIE` writer - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE_RXFNEIE` reader - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register
pub type RXNEIE_RXFNEIE_R = crate::BitReader;
///Field `RXNEIE_RXFNEIE` writer - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register
pub type RXNEIE_RXFNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEIE_TXFNFIE` reader - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register
pub type TXEIE_TXFNFIE_R = crate::BitReader;
///Field `TXEIE_TXFNFIE` writer - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register
pub type TXEIE_TXFNFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEIE` reader - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register
pub type PEIE_R = crate::BitReader;
///Field `PEIE` writer - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PS` reader - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0).
pub type PS_R = crate::BitReader;
///Field `PS` writer - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0).
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PCE` reader - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0).
pub type PCE_R = crate::BitReader;
///Field `PCE` writer - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0).
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAKE` reader - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0).
pub type WAKE_R = crate::BitReader;
///Field `WAKE` writer - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0).
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `M_0` reader - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0).
pub type M_0_R = crate::BitReader;
///Field `M_0` writer - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0).
pub type M_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MME` reader - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode
pub type MME_R = crate::BitReader;
///Field `MME` writer - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMIE` reader - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register.
pub type CMIE_R = crate::BitReader;
///Field `CMIE` writer - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register.
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEDT` reader - DEDT\[4:0\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0).
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - DEDT\[4:0\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0).
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEAT` reader - DEAT\[4:0\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0).
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - DEAT\[4:0\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0).
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `M_1` reader - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\[1:0\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s
pub type M_1_R = crate::BitReader;
///Field `M_1` writer - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\[1:0\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s
pub type M_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FIFOEN` reader - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled.
pub type FIFOEN_R = crate::BitReader;
///Field `FIFOEN` writer - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled.
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFEIE` reader - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register
pub type TXFEIE_R = crate::BitReader;
///Field `TXFEIE` writer - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFFIE` reader - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register
pub type RXFFIE_R = crate::BitReader;
///Field `RXFFIE` writer - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UESM: LPUART enable in Stop mode When this bit is cleared, the LPUART is not able to wake up the MCU from Stop mode. When this bit is set, the LPUART is able to wake up the MCU from Stop mode, provided that the LPUART clock selection is LSE in the RCC. This bit is set and cleared by software. -0: LPUART not able to wake up the MCU from Stop mode. -1: LPUART able to wake up the MCU from Stop mode. When this function is active, the clock source for the LPUART must be LSE (see RCC chapter)
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register
    #[inline(always)]
    pub fn rxneie_rxfneie(&self) -> RXNEIE_RXFNEIE_R {
        RXNEIE_RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register
    #[inline(always)]
    pub fn txeie_txfnfie(&self) -> TXEIE_TXFNFIE_R {
        TXEIE_TXFNFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn m_0(&self) -> M_0_R {
        M_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:20 - DEDT\[4:0\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - DEAT\[4:0\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 28 - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\[1:0\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s
    #[inline(always)]
    pub fn m_1(&self) -> M_1_R {
        M_1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled.
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("ue", &self.ue())
            .field("uesm", &self.uesm())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("idleie", &self.idleie())
            .field("rxneie_rxfneie", &self.rxneie_rxfneie())
            .field("tcie", &self.tcie())
            .field("txeie_txfnfie", &self.txeie_txfnfie())
            .field("peie", &self.peie())
            .field("ps", &self.ps())
            .field("pce", &self.pce())
            .field("wake", &self.wake())
            .field("m_0", &self.m_0())
            .field("mme", &self.mme())
            .field("cmie", &self.cmie())
            .field("dedt", &self.dedt())
            .field("deat", &self.deat())
            .field("m_1", &self.m_1())
            .field("fifoen", &self.fifoen())
            .field("txfeie", &self.txfeie())
            .field("rxffie", &self.rxffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - UE: USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and current operations are discarded. The configuration of the USART is kept, but all the status flags, in the USART_ISR are reset. This bit is set and cleared by software. -0: USART prescaler and outputs disabled, low power mode -1: USART enabled
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, CR1rs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - UESM: LPUART enable in Stop mode When this bit is cleared, the LPUART is not able to wake up the MCU from Stop mode. When this bit is set, the LPUART is able to wake up the MCU from Stop mode, provided that the LPUART clock selection is LSE in the RCC. This bit is set and cleared by software. -0: LPUART not able to wake up the MCU from Stop mode. -1: LPUART able to wake up the MCU from Stop mode. When this function is active, the clock source for the LPUART must be LSE (see RCC chapter)
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<'_, CR1rs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - RE: Receiver enable This bit enables the receiver. It is set and cleared by software. -0: Receiver is disabled -1: Receiver is enabled and begins searching for a start bit
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - TE: Transmitter enable This bit enables the transmitter. It is set and cleared by software. -0: Transmitter is disabled -1: Transmitter is enabled
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLEIE: IDLE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever IDLE=1 in the USART_ISR register
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<'_, CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXNEIE/RXFNEIE: Receive data register not empty/RXFIFO not empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated whenever ORE=1 or RXNE/RXFNE=1 in the USART_ISR register
    #[inline(always)]
    pub fn rxneie_rxfneie(&mut self) -> RXNEIE_RXFNEIE_W<'_, CR1rs> {
        RXNEIE_RXFNEIE_W::new(self, 5)
    }
    ///Bit 6 - TCIE: Transmission complete interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TC=1 in the USART_ISR register
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXEIE/TXFNFIE: Transmit data regsiter empty/TXFIFO not full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever TXE/TXFNF =1 in the USART_ISR register
    #[inline(always)]
    pub fn txeie_txfnfie(&mut self) -> TXEIE_TXFNFIE_W<'_, CR1rs> {
        TXEIE_TXFNFIE_W::new(self, 7)
    }
    ///Bit 8 - PEIE: PE interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated whenever PE=1 in the USART_ISR register
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - PS: Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity will be selected after the current byte. -0: Even parity -1: Odd parity This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - PCE: Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M=1; 8th bit if M=0) and parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). -0: Parity control disabled -1: Parity control enabled This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<'_, CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - WAKE: Receiver wakeup method This bit determines the USART wakeup method from Mute mode. It is set or cleared by software. -0: Idle line -1: Address mark This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - M0: Word length This bit, with bit 28 (M1) determine the word length. It is set or cleared by software. See Bit -28 (M1)description. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn m_0(&mut self) -> M_0_W<'_, CR1rs> {
        M_0_W::new(self, 12)
    }
    ///Bit 13 - MME: Mute mode enable This bit activates the mute mode function of the USART. When set, the USART can switch between the active and mute modes, as defined by the WAKE bit. It is set and cleared by software. -0: Receiver in active mode permanently -1: Receiver can switch between mute mode and active mode
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<'_, CR1rs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - CMIE: Character match interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: A USART interrupt is generated when the CMF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<'_, CR1rs> {
        CMIE_W::new(self, 14)
    }
    ///Bits 16:20 - DEDT\[4:0\]: Driver Enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<'_, CR1rs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - DEAT\[4:0\]: Driver Enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<'_, CR1rs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 28 - Word length This bit, with bit 12 (M0) determine the word length. It is set or cleared by software. M\[1:0\] = 00: 1 Start bit, 8 Data bits, n Stop bit M\[1:0\] = 01: 1 Start bit, 9 Data bits, n Stop bit M\[1:0\] = 10: 1 Start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE=0).s
    #[inline(always)]
    pub fn m_1(&mut self) -> M_1_W<'_, CR1rs> {
        M_1_W::new(self, 28)
    }
    ///Bit 29 - FIFOEN :FIFO mode enable This bit is set and cleared by software. -0: FIFO mode is disabled. -1: FIFO mode is enabled.
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<'_, CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFEIE :TXFIFO empty interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFE=1 in the USART_ISR register
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<'_, CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFFIE :RXFIFO Full interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when RXFF=1 in the USART_ISR register
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<'_, CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
/**CR1 register

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPUART:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
