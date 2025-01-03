///Register `CR3` reader
pub type R = crate::R<CR3rs>;
///Register `CR3` writer
pub type W = crate::W<CR3rs>;
///Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1 or UDR = 1 in the USART_ISR register).
pub type EIE_R = crate::BitReader;
///Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1 or UDR = 1 in the USART_ISR register).
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type IREN_R = crate::BitReader;
///Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type IRLP_R = crate::BitReader;
///Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_R = crate::BitReader;
///Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type SCEN_R = crate::BitReader;
///Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAR` reader - DMA enable receiver This bit is set/reset by software
pub type DMAR_R = crate::BitReader;
///Field `DMAR` writer - DMA enable receiver This bit is set/reset by software
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software
pub type DMAT_R = crate::BitReader;
///Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type RTSE_R = crate::BitReader;
///Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type CTSE_R = crate::BitReader;
///Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type CTSIE_R = crate::BitReader;
///Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).
pub type ONEBIT_R = crate::BitReader;
///Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data
pub type OVRDIS_R = crate::BitReader;
///Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
pub type DDRE_R = crate::BitReader;
///Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .
pub type DEM_R = crate::BitReader;
///Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type SCARCNT_R = crate::FieldReader;
///Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUS {
    ///0: WUF active on address match
    Address = 0,
    ///2: WuF active on Start bit detection
    Start = 2,
    ///3: WUF active on RXNE
    Rxne = 3,
}
impl From<WUS> for u8 {
    #[inline(always)]
    fn from(variant: WUS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUS {
    type Ux = u8;
}
impl crate::IsEnum for WUS {}
///Field `WUS` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
pub type WUS_R = crate::FieldReader<WUS>;
impl WUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUS> {
        match self.bits {
            0 => Some(WUS::Address),
            2 => Some(WUS::Start),
            3 => Some(WUS::Rxne),
            _ => None,
        }
    }
    ///WUF active on address match
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS::Address
    }
    ///WuF active on Start bit detection
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS::Start
    }
    ///WUF active on RXNE
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS::Rxne
    }
}
///Field `WUS` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUS>;
impl<'a, REG> WUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///WUF active on address match
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Address)
    }
    ///WuF active on Start bit detection
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Start)
    }
    ///WUF active on RXNE
    #[inline(always)]
    pub fn rxne(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Rxne)
    }
}
/**Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE {
    ///0: Interrupt is inhibited
    Disabled = 0,
    ///1: An USART interrupt is generated whenever WUF=1 in the ISR register
    Enabled = 1,
}
impl From<WUFIE> for bool {
    #[inline(always)]
    fn from(variant: WUFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
pub type WUFIE_R = crate::BitReader<WUFIE>;
impl WUFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUFIE {
        match self.bits {
            false => WUFIE::Disabled,
            true => WUFIE::Enabled,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE::Disabled
    }
    ///An USART interrupt is generated whenever WUF=1 in the ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE::Enabled
    }
}
///Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG, WUFIE>;
impl<'a, REG> WUFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::Disabled)
    }
    ///An USART interrupt is generated whenever WUF=1 in the ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::Enabled)
    }
}
/**TXFIFO threshold interrupt enable This bit is set and cleared by software.

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
///Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software.
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
///Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software.
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
/**Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGTIE {
    ///0: Interrupt inhibited
    Disabled = 0,
    ///1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    Enabled = 1,
}
impl From<TCBGTIE> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type TCBGTIE_R = crate::BitReader<TCBGTIE>;
impl TCBGTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGTIE {
        match self.bits {
            false => TCBGTIE::Disabled,
            true => TCBGTIE::Enabled,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCBGTIE::Disabled
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCBGTIE::Enabled
    }
}
///Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG, TCBGTIE>;
impl<'a, REG> TCBGTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::Disabled)
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::Enabled)
    }
}
/**Receive FIFO threshold configuration Remaining combinations: Reserved

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
///Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved
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
///Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved
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
/**RXFIFO threshold interrupt enable This bit is set and cleared by software.

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
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software.
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
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software.
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
/**TXFIFO threshold configuration Remaining combinations: Reserved

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
///Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved
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
///Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved
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
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1 or UDR = 1 in the USART_ISR register).
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("eie", &self.eie())
            .field("iren", &self.iren())
            .field("irlp", &self.irlp())
            .field("hdsel", &self.hdsel())
            .field("nack", &self.nack())
            .field("scen", &self.scen())
            .field("dmar", &self.dmar())
            .field("dmat", &self.dmat())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .field("ctsie", &self.ctsie())
            .field("onebit", &self.onebit())
            .field("ovrdis", &self.ovrdis())
            .field("ddre", &self.ddre())
            .field("dem", &self.dem())
            .field("dep", &self.dep())
            .field("scarcnt", &self.scarcnt())
            .field("wus", &self.wus())
            .field("wufie", &self.wufie())
            .field("txftie", &self.txftie())
            .field("tcbgtie", &self.tcbgtie())
            .field("rxftcfg", &self.rxftcfg())
            .field("rxftie", &self.rxftie())
            .field("txftcfg", &self.txftcfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE=1 or ORE=1 or NE=1 or UDR = 1 in the USART_ISR register).
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE=0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CR3rs> {
        NACK_W::new(self, 4)
    }
    ///Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE=0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<CR3rs> {
        SCEN_W::new(self, 5)
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE=0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE=0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
    ///Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE=0). Note: This control bit enables checking the communication flow w/o reading the data
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. .
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE=0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE=0). When the USART is enabled (UE=1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W<CR3rs> {
        SCARCNT_W::new(self, 17)
    }
    ///Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UE=0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<CR3rs> {
        WUS_W::new(self, 20)
    }
    ///Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page835.
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<CR3rs> {
        WUFIE_W::new(self, 22)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to .
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<CR3rs> {
        TCBGTIE_W::new(self, 24)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**Control register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#USART1:CR3)*/
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
///`read()` method returns [`cr3::R`](R) reader structure
impl crate::Readable for CR3rs {}
///`write(|w| ..)` method takes [`cr3::W`](W) writer structure
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
