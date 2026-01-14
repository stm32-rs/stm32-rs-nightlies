///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDSEL` reader - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_R = crate::BitReader;
///Field `HDSEL` writer - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAR` reader - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
pub type DMAR_R = crate::BitReader;
///Field `DMAR` writer - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAT` reader - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
pub type DMAT_R = crate::BitReader;
///Field `DMAT` writer - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTSE` reader - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
pub type RTSE_R = crate::BitReader;
///Field `RTSE` writer - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSE` reader - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
pub type CTSE_R = crate::BitReader;
///Field `CTSE` writer - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIE` reader - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
pub type CTSIE_R = crate::BitReader;
///Field `CTSIE` writer - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRDIS` reader - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
pub type OVRDIS_R = crate::BitReader;
///Field `OVRDIS` writer - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRE` reader - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
pub type DDRE_R = crate::BitReader;
///Field `DDRE` writer - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM` reader - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
pub type DEM_R = crate::BitReader;
///Field `DEM` writer - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEP` reader - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUS` reader - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
pub type WUS_R = crate::FieldReader;
///Field `WUS` writer - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUFIE` reader - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
pub type WUFIE_R = crate::BitReader;
///Field `WUFIE` writer - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTIE` reader - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
pub type TXFTIE_R = crate::BitReader;
///Field `TXFTIE` writer - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFTCFG` reader - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
pub type RXFTCFG_R = crate::FieldReader;
///Field `RXFTCFG` writer - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXFTIE` reader - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
pub type RXFTIE_R = crate::BitReader;
///Field `RXFTIE` writer - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFTCFG` reader - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
pub type TXFTCFG_R = crate::FieldReader;
///Field `TXFTCFG` writer - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 20:21 - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:27 - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("eie", &self.eie())
            .field("hdsel", &self.hdsel())
            .field("dmar", &self.dmar())
            .field("dmat", &self.dmat())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .field("ctsie", &self.ctsie())
            .field("ovrdis", &self.ovrdis())
            .field("ddre", &self.ddre())
            .field("dem", &self.dem())
            .field("dep", &self.dep())
            .field("wus", &self.wus())
            .field("wufie", &self.wufie())
            .field("txftie", &self.txftie())
            .field("rxftcfg", &self.rxftcfg())
            .field("rxftie", &self.rxftie())
            .field("txftcfg", &self.txftcfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 3 - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<'_, CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 6 - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<'_, CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<'_, CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<'_, CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<'_, CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 12 - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<'_, CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<'_, CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<'_, CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 20:21 - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<'_, CR3rs> {
        WUS_W::new(self, 20)
    }
    ///Bit 22 - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<'_, CR3rs> {
        WUFIE_W::new(self, 22)
    }
    ///Bit 23 - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<'_, CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bits 25:27 - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<'_, CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<'_, CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<'_, CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**CR3 register

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPUART:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {}
