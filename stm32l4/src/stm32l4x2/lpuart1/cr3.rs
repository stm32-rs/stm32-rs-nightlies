#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    Enabled = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::Disabled,
            true => EIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE::Disabled
    }
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE::Enabled
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Disabled)
    }
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Enabled)
    }
}
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    #[doc = "0: Half duplex mode is not selected"]
    NotSelected = 0,
    #[doc = "1: Half duplex mode is selected"]
    Selected = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::NotSelected,
            true => HDSEL::Selected,
        }
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL::NotSelected
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL::Selected
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::NotSelected)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::Selected)
    }
}
#[doc = "DMA enable receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    #[doc = "0: DMA mode is disabled for reception"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    Enabled = 1,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            false => DMAR::Disabled,
            true => DMAR::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::Disabled
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::Enabled
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Disabled)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Enabled)
    }
}
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    #[doc = "0: DMA mode is disabled for transmission"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    Enabled = 1,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            false => DMAT::Disabled,
            true => DMAT::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT::Disabled
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT::Enabled
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Disabled)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Enabled)
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE {
    #[doc = "0: RTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer"]
    Enabled = 1,
}
impl From<RTSE> for bool {
    #[inline(always)]
    fn from(variant: RTSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<RTSE>;
impl RTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTSE {
        match self.bits {
            false => RTSE::Disabled,
            true => RTSE::Enabled,
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE::Disabled
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE::Enabled
    }
}
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Disabled)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Enabled)
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE {
    #[doc = "0: CTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    Enabled = 1,
}
impl From<CTSE> for bool {
    #[inline(always)]
    fn from(variant: CTSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<CTSE>;
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSE {
        match self.bits {
            false => CTSE::Disabled,
            true => CTSE::Enabled,
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE::Disabled
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE::Enabled
    }
}
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Disabled)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Enabled)
    }
}
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF=1 in the ISR register"]
    Enabled = 1,
}
impl From<CTSIE> for bool {
    #[inline(always)]
    fn from(variant: CTSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<CTSIE>;
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE {
        match self.bits {
            false => CTSIE::Disabled,
            true => CTSIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE::Disabled
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE::Enabled
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Disabled)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Enabled)
    }
}
#[doc = "Overrun Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    Enabled = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    Disabled = 1,
}
impl From<OVRDIS> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OVRDIS_R = crate::BitReader<OVRDIS>;
impl OVRDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRDIS {
        match self.bits {
            false => OVRDIS::Enabled,
            true => OVRDIS::Disabled,
        }
    }
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS::Enabled
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS::Disabled
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG, OVRDIS>;
impl<'a, REG> OVRDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Enabled)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::Disabled)
    }
}
#[doc = "DMA Disable on Reception Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE {
    #[doc = "0: DMA is not disabled in case of reception error"]
    NotDisabled = 0,
    #[doc = "1: DMA is disabled following a reception error"]
    Disabled = 1,
}
impl From<DDRE> for bool {
    #[inline(always)]
    fn from(variant: DDRE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DDRE_R = crate::BitReader<DDRE>;
impl DDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DDRE {
        match self.bits {
            false => DDRE::NotDisabled,
            true => DDRE::Disabled,
        }
    }
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE::NotDisabled
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE::Disabled
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG, DDRE>;
impl<'a, REG> DDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::NotDisabled)
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::Disabled)
    }
}
#[doc = "Driver enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM {
    #[doc = "0: DE function is disabled"]
    Disabled = 0,
    #[doc = "1: The DE signal is output on the RTS pin"]
    Enabled = 1,
}
impl From<DEM> for bool {
    #[inline(always)]
    fn from(variant: DEM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader<DEM>;
impl DEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEM {
        match self.bits {
            false => DEM::Disabled,
            true => DEM::Enabled,
        }
    }
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM::Disabled
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM::Enabled
    }
}
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG, DEM>;
impl<'a, REG> DEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Disabled)
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::Enabled)
    }
}
#[doc = "Driver enable polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP {
    #[doc = "0: DE signal is active high"]
    High = 0,
    #[doc = "1: DE signal is active low"]
    Low = 1,
}
impl From<DEP> for bool {
    #[inline(always)]
    fn from(variant: DEP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DEP_R = crate::BitReader<DEP>;
impl DEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEP {
        match self.bits {
            false => DEP::High,
            true => DEP::Low,
        }
    }
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP::High
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP::Low
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::High)
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::Low)
    }
}
#[doc = "Wakeup from Stop mode interrupt flag selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUS {
    #[doc = "0: WUF active on address match"]
    Address = 0,
    #[doc = "2: WuF active on Start bit detection"]
    Start = 2,
    #[doc = "3: WUF active on RXNE"]
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
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_R = crate::FieldReader<WUS>;
impl WUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUS> {
        match self.bits {
            0 => Some(WUS::Address),
            2 => Some(WUS::Start),
            3 => Some(WUS::Rxne),
            _ => None,
        }
    }
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS::Address
    }
    #[doc = "WuF active on Start bit detection"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS::Start
    }
    #[doc = "WUF active on RXNE"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS::Rxne
    }
}
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUS>;
impl<'a, REG> WUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Address)
    }
    #[doc = "WuF active on Start bit detection"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Start)
    }
    #[doc = "WUF active on RXNE"]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::Rxne)
    }
}
#[doc = "Wakeup from Stop mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An USART interrupt is generated whenever WUF=1 in the ISR register"]
    Enabled = 1,
}
impl From<WUFIE> for bool {
    #[inline(always)]
    fn from(variant: WUFIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_R = crate::BitReader<WUFIE>;
impl WUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUFIE {
        match self.bits {
            false => WUFIE::Disabled,
            true => WUFIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE::Disabled
    }
    #[doc = "An USART interrupt is generated whenever WUF=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE::Enabled
    }
}
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG, WUFIE>;
impl<'a, REG> WUFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::Disabled)
    }
    #[doc = "An USART interrupt is generated whenever WUF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OVRDIS_W<CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<CR3rs> {
        DDRE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DEM_W<CR3rs> {
        DEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<CR3rs> {
        DEP_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WUS_W<CR3rs> {
        WUS_W::new(self, 20)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WUFIE_W<CR3rs> {
        WUFIE_W::new(self, 22)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
