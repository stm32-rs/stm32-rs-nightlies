///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
/**EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    Enabled = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::Disabled,
            true => EIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE::Disabled
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE::Enabled
    }
}
///Field `EIE` writer - EIE: Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NF=1or UDR = 1 in the USART_ISR register). -0: Interrupt is inhibited -1: An interrupt is generated when FE=1 or ORE=1 or NF=1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Disabled)
    }
    ///An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Enabled)
    }
}
/**HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    ///0: Half duplex mode is not selected
    NotSelected = 0,
    ///1: Half duplex mode is selected
    Selected = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::NotSelected,
            true => HDSEL::Selected,
        }
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL::NotSelected
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL::Selected
    }
}
///Field `HDSEL` writer - HDSEL: Half-duplex selection Selection of Single-wire Half-duplex mode -0: Half duplex mode is not selected -1: Half duplex mode is selected This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::NotSelected)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::Selected)
    }
}
/**DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    ///0: DMA mode is disabled for reception
    Disabled = 0,
    ///1: DMA mode is enabled for reception
    Enabled = 1,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            false => DMAR::Disabled,
            true => DMAR::Enabled,
        }
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::Disabled
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::Enabled
    }
}
///Field `DMAR` writer - DMAR: DMA enable receiver This bit is set/reset by software -1: DMA mode is enabled for reception -0: DMA mode is disabled for reception
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Disabled)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Enabled)
    }
}
/**DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    ///0: DMA mode is disabled for transmission
    Disabled = 0,
    ///1: DMA mode is enabled for transmission
    Enabled = 1,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            false => DMAT::Disabled,
            true => DMAT::Enabled,
        }
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT::Disabled
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT::Enabled
    }
}
///Field `DMAT` writer - DMAT: DMA enable transmitter This bit is set/reset by software -1: DMA mode is enabled for transmission -0: DMA mode is disabled for transmission
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Disabled)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Enabled)
    }
}
/**RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE {
    ///0: RTS hardware flow control disabled
    Disabled = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer
    Enabled = 1,
}
impl From<RTSE> for bool {
    #[inline(always)]
    fn from(variant: RTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSE` reader - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
pub type RTSE_R = crate::BitReader<RTSE>;
impl RTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTSE {
        match self.bits {
            false => RTSE::Disabled,
            true => RTSE::Enabled,
        }
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE::Disabled
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE::Enabled
    }
}
///Field `RTSE` writer - RTSE: RTS enable -0: RTS hardware flow control disabled -1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The nRTS output is asserted (pulled to 0) when data can be received. This bit can only be written when the USART is disabled (UE=0).
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Disabled)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Enabled)
    }
}
/**CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE {
    ///0: CTS hardware flow control disabled
    Disabled = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is asserted
    Enabled = 1,
}
impl From<CTSE> for bool {
    #[inline(always)]
    fn from(variant: CTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
pub type CTSE_R = crate::BitReader<CTSE>;
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSE {
        match self.bits {
            false => CTSE::Disabled,
            true => CTSE::Enabled,
        }
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE::Disabled
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE::Enabled
    }
}
///Field `CTSE` writer - CTSE: CTS enable -0: CTS hardware flow control disabled -1: CTS mode enabled, data is only transmitted when the nCTS input is asserted (tied to 0). If the nCTS input is deasserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while nCTS is asserted, the transmission is postponed until nCTS is asserted. This bit can only be written when the USART is disabled (UE=0)
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Disabled)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is asserted
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Enabled)
    }
}
/**CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An interrupt is generated whenever CTSIF=1 in the ISR register
    Enabled = 1,
}
impl From<CTSIE> for bool {
    #[inline(always)]
    fn from(variant: CTSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIE` reader - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
pub type CTSIE_R = crate::BitReader<CTSIE>;
impl CTSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE {
        match self.bits {
            false => CTSIE::Disabled,
            true => CTSIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE::Disabled
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE::Enabled
    }
}
///Field `CTSIE` writer - CTSIE: CTS interrupt enable -0: Interrupt is inhibited -1: An interrupt is generated whenever CTSIF=1 in the USART_ISR register
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Disabled)
    }
    ///An interrupt is generated whenever CTSIF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Enabled)
    }
}
/**OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    Enabled = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    Disabled = 1,
}
impl From<OVRDIS> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRDIS` reader - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
pub type OVRDIS_R = crate::BitReader<OVRDIS>;
impl OVRDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRDIS {
        match self.bits {
            false => OVRDIS::Enabled,
            true => OVRDIS::Disabled,
        }
    }
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS::Enabled
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS::Disabled
    }
}
///Field `OVRDIS` writer - OVRDIS: Overrun Disable This bit is used to disable the receive overrun detection. -0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data. -1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO will be bypassed and data will be written directly in USARTx_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0).
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG, OVRDIS>;
impl<'a, REG> OVRDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Enabled)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Disabled)
    }
}
/**DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE {
    ///0: DMA is not disabled in case of reception error
    NotDisabled = 0,
    ///1: DMA is disabled following a reception error
    Disabled = 1,
}
impl From<DDRE> for bool {
    #[inline(always)]
    fn from(variant: DDRE) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRE` reader - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
pub type DDRE_R = crate::BitReader<DDRE>;
impl DDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDRE {
        match self.bits {
            false => DDRE::NotDisabled,
            true => DDRE::Disabled,
        }
    }
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE::NotDisabled
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE::Disabled
    }
}
///Field `DDRE` writer - DDRE: DMA Disable on Reception Error -0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data will be transferred. (used for Smartcard mode) -1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE(RXFNE is case FIFO mode is enabled) before clearing the error flag. This bit can only be written when the USART is disabled (UE=0).
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG, DDRE>;
impl<'a, REG> DDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA is not disabled in case of reception error
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::NotDisabled)
    }
    ///DMA is disabled following a reception error
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::Disabled)
    }
}
/**DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM {
    ///0: DE function is disabled
    Disabled = 0,
    ///1: The DE signal is output on the RTS pin
    Enabled = 1,
}
impl From<DEM> for bool {
    #[inline(always)]
    fn from(variant: DEM) -> Self {
        variant as u8 != 0
    }
}
///Field `DEM` reader - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
pub type DEM_R = crate::BitReader<DEM>;
impl DEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEM {
        match self.bits {
            false => DEM::Disabled,
            true => DEM::Enabled,
        }
    }
    ///DE function is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM::Disabled
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM::Enabled
    }
}
///Field `DEM` writer - DEM: Driver enable mode This bit allows the user to activate the external transceiver control, through the DE signal. -0: DE function is disabled. -1: DE function is enabled. The DE signal is output on the RTS pin. This bit can only be written when the USART is disabled (UE=0).
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG, DEM>;
impl<'a, REG> DEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE function is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Disabled)
    }
    ///The DE signal is output on the RTS pin
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Enabled)
    }
}
/**DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP {
    ///0: DE signal is active high
    High = 0,
    ///1: DE signal is active low
    Low = 1,
}
impl From<DEP> for bool {
    #[inline(always)]
    fn from(variant: DEP) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
pub type DEP_R = crate::BitReader<DEP>;
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEP {
        match self.bits {
            false => DEP::High,
            true => DEP::Low,
        }
    }
    ///DE signal is active high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP::High
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP::Low
    }
}
///Field `DEP` writer - DEP: Driver enable polarity selection -0: DE signal is active high. -1: DE signal is active low. This bit can only be written when the USART is disabled (UE=0).
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE signal is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::High)
    }
    ///DE signal is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::Low)
    }
}
///Field `WUS` reader - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
pub type WUS_R = crate::FieldReader;
///Field `WUS` writer - WUS\[1:0\]: Wakeup from Stop mode interrupt flag selection This bit-field specify the event which activates the WUF (Wakeup from Stop mode flag). -00: WUF active on address match (as defined by ADD\[7:0\] and ADDM7) -01:Reserved. -10: WUF active on Start bit detection -11: WUF active on RXNE. This bit field can only be written when the LPUART is disabled (UE=0).
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WUFIE` reader - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
pub type WUFIE_R = crate::BitReader;
///Field `WUFIE` writer - WUFIE: Wakeup from Stop mode interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An LPUART interrupt is generated whenever WUF=1 in the LPUART_ISR register
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
/**TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    Enabled = 1,
}
impl From<TXFTIE> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFTIE` reader - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
pub type TXFTIE_R = crate::BitReader<TXFTIE>;
impl TXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFTIE {
        match self.bits {
            false => TXFTIE::Disabled,
            true => TXFTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFTIE::Disabled
    }
    ///USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFTIE::Enabled
    }
}
///Field `TXFTIE` writer - TXFTIE: TXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when TXFIFO reaches the threshold programmed in TXFTCFG.
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFTIE>;
impl<'a, REG> TXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::Disabled)
    }
    ///USART interrupt generated when Transmit FIFO reaches the threshold programmed in TXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::Enabled)
    }
}
/**RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFTCFG {
    ///0: RXFIFO reaches 1/8 of its depth
    Depth1_8 = 0,
    ///1: RXFIFO reaches 1/4 of its depth
    Depth1_4 = 1,
    ///2: RXFIFO reaches 1/2 of its depth
    Depth1_2 = 2,
    ///3: RXFIFO reaches 3/4 of its depth
    Depth3_4 = 3,
    ///4: RXFIFO reaches 7/8 of its depth
    Depth7_8 = 4,
    ///5: RXFIFO becomes full
    Full = 5,
}
impl From<RXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for RXFTCFG {}
///Field `RXFTCFG` reader - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
pub type RXFTCFG_R = crate::FieldReader<RXFTCFG>;
impl RXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXFTCFG> {
        match self.bits {
            0 => Some(RXFTCFG::Depth1_8),
            1 => Some(RXFTCFG::Depth1_4),
            2 => Some(RXFTCFG::Depth1_2),
            3 => Some(RXFTCFG::Depth3_4),
            4 => Some(RXFTCFG::Depth7_8),
            5 => Some(RXFTCFG::Full),
            _ => None,
        }
    }
    ///RXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == RXFTCFG::Depth1_8
    }
    ///RXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == RXFTCFG::Depth1_4
    }
    ///RXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == RXFTCFG::Depth1_2
    }
    ///RXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == RXFTCFG::Depth3_4
    }
    ///RXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == RXFTCFG::Depth7_8
    }
    ///RXFIFO becomes full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFTCFG::Full
    }
}
///Field `RXFTCFG` writer - RXFTCFG: Receive FIFO threshold configuration -000:Receive FIFO reaches 1/8 of its depth. -001:Receive FIFO reaches 1/4 of its depth. -010:Receive FIFO reaches 1/2 of its depth. -011:Receive FIFO reaches 3/4 of its depth. -100:Receive FIFO reaches 7/8 of its depth. -101:Receive FIFO becomes full. Remaining combinations: Reserved.
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RXFTCFG>;
impl<'a, REG> RXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_8)
    }
    ///RXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_4)
    }
    ///RXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth1_2)
    }
    ///RXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth3_4)
    }
    ///RXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Depth7_8)
    }
    ///RXFIFO becomes full
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::Full)
    }
}
/**RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    Enabled = 1,
}
impl From<RXFTIE> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFTIE` reader - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
pub type RXFTIE_R = crate::BitReader<RXFTIE>;
impl RXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFTIE {
        match self.bits {
            false => RXFTIE::Disabled,
            true => RXFTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFTIE::Disabled
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFTIE::Enabled
    }
}
///Field `RXFTIE` writer - RXFTIE: RXFIFO threshold interrupt enable This bit is set and cleared by software. -0: Interrupt is inhibited -1: An USART interrupt is generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFTIE>;
impl<'a, REG> RXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::Disabled)
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::Enabled)
    }
}
/**TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFTCFG {
    ///0: TXFIFO reaches 1/8 of its depth
    Depth1_8 = 0,
    ///1: TXFIFO reaches 1/4 of its depth
    Depth1_4 = 1,
    ///2: TXFIFO reaches 1/2 of its depth
    Depth1_2 = 2,
    ///3: TXFIFO reaches 3/4 of its depth
    Depth3_4 = 3,
    ///4: TXFIFO reaches 7/8 of its depth
    Depth7_8 = 4,
    ///5: TXFIFO becomes empty
    Empty = 5,
}
impl From<TXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for TXFTCFG {}
///Field `TXFTCFG` reader - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
pub type TXFTCFG_R = crate::FieldReader<TXFTCFG>;
impl TXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFTCFG> {
        match self.bits {
            0 => Some(TXFTCFG::Depth1_8),
            1 => Some(TXFTCFG::Depth1_4),
            2 => Some(TXFTCFG::Depth1_2),
            3 => Some(TXFTCFG::Depth3_4),
            4 => Some(TXFTCFG::Depth7_8),
            5 => Some(TXFTCFG::Empty),
            _ => None,
        }
    }
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_depth_1_8(&self) -> bool {
        *self == TXFTCFG::Depth1_8
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_depth_1_4(&self) -> bool {
        *self == TXFTCFG::Depth1_4
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_depth_1_2(&self) -> bool {
        *self == TXFTCFG::Depth1_2
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_depth_3_4(&self) -> bool {
        *self == TXFTCFG::Depth3_4
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_depth_7_8(&self) -> bool {
        *self == TXFTCFG::Depth7_8
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFTCFG::Empty
    }
}
///Field `TXFTCFG` writer - TXFTCFG: TXFIFO threshold configuration -000:TXFIFO reaches 1/8 of its depth. -001:TXFIFO reaches 1/4 of its depth. -010:TXFIFO reaches 1/2 of its depth. -011:TXFIFO reaches 3/4 of its depth. -100:TXFIFO reaches 7/8 of its depth. -101:TXFIFO becomes empty. Remaining combinations: Reserved.
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TXFTCFG>;
impl<'a, REG> TXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn depth_1_8(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_8)
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn depth_1_4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_4)
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn depth_1_2(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth1_2)
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn depth_3_4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth3_4)
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn depth_7_8(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Depth7_8)
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::Empty)
    }
}
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#LPUART:CR3)*/
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
