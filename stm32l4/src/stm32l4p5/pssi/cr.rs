#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Parallel data clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL {
    #[doc = "0: Falling edge active for inputs or rising edge active for outputs"]
    FallingEdge = 0,
    #[doc = "1: Rising edge active for inputs or falling edge active for outputs"]
    RisingEdge = 1,
}
impl From<CKPOL> for bool {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKPOL` reader - Parallel data clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKPOL {
        match self.bits {
            false => CKPOL::FallingEdge,
            true => CKPOL::RisingEdge,
        }
    }
    #[doc = "Falling edge active for inputs or rising edge active for outputs"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL::FallingEdge
    }
    #[doc = "Rising edge active for inputs or falling edge active for outputs"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL::RisingEdge
    }
}
#[doc = "Field `CKPOL` writer - Parallel data clock polarity"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling edge active for inputs or rising edge active for outputs"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::FallingEdge)
    }
    #[doc = "Rising edge active for inputs or falling edge active for outputs"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::RisingEdge)
    }
}
#[doc = "Data enable (PSSI_DE) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL {
    #[doc = "0: PSSI_DE active low (0 indicates that data is valid)"]
    ActiveLow = 0,
    #[doc = "1: PSSI_DE active high (1 indicates that data is valid)"]
    ActiveHigh = 1,
}
impl From<DEPOL> for bool {
    #[inline(always)]
    fn from(variant: DEPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEPOL` reader - Data enable (PSSI_DE) polarity"]
pub type DEPOL_R = crate::BitReader<DEPOL>;
impl DEPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEPOL {
        match self.bits {
            false => DEPOL::ActiveLow,
            true => DEPOL::ActiveHigh,
        }
    }
    #[doc = "PSSI_DE active low (0 indicates that data is valid)"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL::ActiveLow
    }
    #[doc = "PSSI_DE active high (1 indicates that data is valid)"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL::ActiveHigh
    }
}
#[doc = "Field `DEPOL` writer - Data enable (PSSI_DE) polarity"]
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG, DEPOL>;
impl<'a, REG> DEPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PSSI_DE active low (0 indicates that data is valid)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveLow)
    }
    #[doc = "PSSI_DE active high (1 indicates that data is valid)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveHigh)
    }
}
#[doc = "Ready (PSSI_RDY) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDYPOL {
    #[doc = "0: PSSI_RDY active low (0 indicates that the receiver is ready to receive)"]
    ActiveLow = 0,
    #[doc = "1: PSSI_RDY active high (1 indicates that the receiver is ready to receive)"]
    ActiveHigh = 1,
}
impl From<RDYPOL> for bool {
    #[inline(always)]
    fn from(variant: RDYPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDYPOL` reader - Ready (PSSI_RDY) polarity"]
pub type RDYPOL_R = crate::BitReader<RDYPOL>;
impl RDYPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDYPOL {
        match self.bits {
            false => RDYPOL::ActiveLow,
            true => RDYPOL::ActiveHigh,
        }
    }
    #[doc = "PSSI_RDY active low (0 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == RDYPOL::ActiveLow
    }
    #[doc = "PSSI_RDY active high (1 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == RDYPOL::ActiveHigh
    }
}
#[doc = "Field `RDYPOL` writer - Ready (PSSI_RDY) polarity"]
pub type RDYPOL_W<'a, REG> = crate::BitWriter<'a, REG, RDYPOL>;
impl<'a, REG> RDYPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PSSI_RDY active low (0 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(RDYPOL::ActiveLow)
    }
    #[doc = "PSSI_RDY active high (1 indicates that the receiver is ready to receive)"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(RDYPOL::ActiveHigh)
    }
}
#[doc = "Extended data mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDM {
    #[doc = "0: Interface captures 8-bit data on every parallel data clock"]
    BitWidth8 = 0,
    #[doc = "3: The interface captures 16-bit data on every parallel data clock"]
    BitWidth16 = 3,
}
impl From<EDM> for u8 {
    #[inline(always)]
    fn from(variant: EDM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EDM {
    type Ux = u8;
}
#[doc = "Field `EDM` reader - Extended data mode"]
pub type EDM_R = crate::FieldReader<EDM>;
impl EDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EDM> {
        match self.bits {
            0 => Some(EDM::BitWidth8),
            3 => Some(EDM::BitWidth16),
            _ => None,
        }
    }
    #[doc = "Interface captures 8-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn is_bit_width8(&self) -> bool {
        *self == EDM::BitWidth8
    }
    #[doc = "The interface captures 16-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn is_bit_width16(&self) -> bool {
        *self == EDM::BitWidth16
    }
}
#[doc = "Field `EDM` writer - Extended data mode"]
pub type EDM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EDM>;
impl<'a, REG> EDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interface captures 8-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn bit_width8(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth8)
    }
    #[doc = "The interface captures 16-bit data on every parallel data clock"]
    #[inline(always)]
    pub fn bit_width16(self) -> &'a mut crate::W<REG> {
        self.variant(EDM::BitWidth16)
    }
}
#[doc = "PSSI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    #[doc = "0: PSSI disabled"]
    Disabled = 0,
    #[doc = "1: PSSI enabled"]
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - PSSI enable"]
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    #[doc = "PSSI disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    #[doc = "PSSI enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
#[doc = "Field `ENABLE` writer - PSSI enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PSSI disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    #[doc = "PSSI enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
#[doc = "Data enable and ready configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DERDYCFG {
    #[doc = "0: PSSI_DE and PSSI_RDY both disabled"]
    Disabled = 0,
    #[doc = "1: Only PSSI_RDY enabled"]
    Rdy = 1,
    #[doc = "2: Only PSSI_DE enabled"]
    De = 2,
    #[doc = "3: Both PSSI_RDY and PSSI_DE alternate functions enabled"]
    RdyDeAlt = 3,
    #[doc = "4: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin"]
    RdyDe = 4,
    #[doc = "5: Only PSSI_RDY function enabled, but mapped to PSSI_DE pin"]
    RdyRemapped = 5,
    #[doc = "6: Only PSSI_DE function enabled, but mapped to PSSI_RDY pin"]
    DeRemapped = 6,
    #[doc = "7: Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin"]
    RdyDeBidi = 7,
}
impl From<DERDYCFG> for u8 {
    #[inline(always)]
    fn from(variant: DERDYCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DERDYCFG {
    type Ux = u8;
}
#[doc = "Field `DERDYCFG` reader - Data enable and ready configuration"]
pub type DERDYCFG_R = crate::FieldReader<DERDYCFG>;
impl DERDYCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DERDYCFG {
        match self.bits {
            0 => DERDYCFG::Disabled,
            1 => DERDYCFG::Rdy,
            2 => DERDYCFG::De,
            3 => DERDYCFG::RdyDeAlt,
            4 => DERDYCFG::RdyDe,
            5 => DERDYCFG::RdyRemapped,
            6 => DERDYCFG::DeRemapped,
            7 => DERDYCFG::RdyDeBidi,
            _ => unreachable!(),
        }
    }
    #[doc = "PSSI_DE and PSSI_RDY both disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DERDYCFG::Disabled
    }
    #[doc = "Only PSSI_RDY enabled"]
    #[inline(always)]
    pub fn is_rdy(&self) -> bool {
        *self == DERDYCFG::Rdy
    }
    #[doc = "Only PSSI_DE enabled"]
    #[inline(always)]
    pub fn is_de(&self) -> bool {
        *self == DERDYCFG::De
    }
    #[doc = "Both PSSI_RDY and PSSI_DE alternate functions enabled"]
    #[inline(always)]
    pub fn is_rdy_de_alt(&self) -> bool {
        *self == DERDYCFG::RdyDeAlt
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin"]
    #[inline(always)]
    pub fn is_rdy_de(&self) -> bool {
        *self == DERDYCFG::RdyDe
    }
    #[doc = "Only PSSI_RDY function enabled, but mapped to PSSI_DE pin"]
    #[inline(always)]
    pub fn is_rdy_remapped(&self) -> bool {
        *self == DERDYCFG::RdyRemapped
    }
    #[doc = "Only PSSI_DE function enabled, but mapped to PSSI_RDY pin"]
    #[inline(always)]
    pub fn is_de_remapped(&self) -> bool {
        *self == DERDYCFG::DeRemapped
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin"]
    #[inline(always)]
    pub fn is_rdy_de_bidi(&self) -> bool {
        *self == DERDYCFG::RdyDeBidi
    }
}
#[doc = "Field `DERDYCFG` writer - Data enable and ready configuration"]
pub type DERDYCFG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, DERDYCFG>;
impl<'a, REG> DERDYCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PSSI_DE and PSSI_RDY both disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::Disabled)
    }
    #[doc = "Only PSSI_RDY enabled"]
    #[inline(always)]
    pub fn rdy(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::Rdy)
    }
    #[doc = "Only PSSI_DE enabled"]
    #[inline(always)]
    pub fn de(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::De)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE alternate functions enabled"]
    #[inline(always)]
    pub fn rdy_de_alt(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDeAlt)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_RDY pin"]
    #[inline(always)]
    pub fn rdy_de(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDe)
    }
    #[doc = "Only PSSI_RDY function enabled, but mapped to PSSI_DE pin"]
    #[inline(always)]
    pub fn rdy_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyRemapped)
    }
    #[doc = "Only PSSI_DE function enabled, but mapped to PSSI_RDY pin"]
    #[inline(always)]
    pub fn de_remapped(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::DeRemapped)
    }
    #[doc = "Both PSSI_RDY and PSSI_DE features enabled - bidirectional on PSSI_DE pin"]
    #[inline(always)]
    pub fn rdy_de_bidi(self) -> &'a mut crate::W<REG> {
        self.variant(DERDYCFG::RdyDeBidi)
    }
}
#[doc = "DMA enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN {
    #[doc = "0: DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled."]
    Disabled = 0,
    #[doc = "1: DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR"]
    Enabled = 1,
}
impl From<DMAEN> for bool {
    #[inline(always)]
    fn from(variant: DMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<DMAEN>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN {
        match self.bits {
            false => DMAEN::Disabled,
            true => DMAEN::Enabled,
        }
    }
    #[doc = "DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN::Disabled
    }
    #[doc = "DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfers are disabled. The user application can directly access the PSSI_DR register when DMA transfers are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Disabled)
    }
    #[doc = "DMA transfers are enabled (default configuration). A DMA channel in the general-purpose DMA controller must be configured to perform transfers from/to PSSI_DR"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN::Enabled)
    }
}
#[doc = "Data direction selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTEN {
    #[doc = "0: Data is input synchronously with PSSI_PDCK"]
    ReceiveMode = 0,
    #[doc = "1: Data is output synchronously with PSSI_PDCK"]
    TransmitMode = 1,
}
impl From<OUTEN> for bool {
    #[inline(always)]
    fn from(variant: OUTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTEN` reader - Data direction selection bit"]
pub type OUTEN_R = crate::BitReader<OUTEN>;
impl OUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTEN {
        match self.bits {
            false => OUTEN::ReceiveMode,
            true => OUTEN::TransmitMode,
        }
    }
    #[doc = "Data is input synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn is_receive_mode(&self) -> bool {
        *self == OUTEN::ReceiveMode
    }
    #[doc = "Data is output synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn is_transmit_mode(&self) -> bool {
        *self == OUTEN::TransmitMode
    }
}
#[doc = "Field `OUTEN` writer - Data direction selection bit"]
pub type OUTEN_W<'a, REG> = crate::BitWriter<'a, REG, OUTEN>;
impl<'a, REG> OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is input synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn receive_mode(self) -> &'a mut crate::W<REG> {
        self.variant(OUTEN::ReceiveMode)
    }
    #[doc = "Data is output synchronously with PSSI_PDCK"]
    #[inline(always)]
    pub fn transmit_mode(self) -> &'a mut crate::W<REG> {
        self.variant(OUTEN::TransmitMode)
    }
}
impl R {
    #[doc = "Bit 5 - Parallel data clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity"]
    #[inline(always)]
    pub fn rdypol(&self) -> RDYPOL_R {
        RDYPOL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - PSSI enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration"]
    #[inline(always)]
    pub fn derdycfg(&self) -> DERDYCFG_R {
        DERDYCFG_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Parallel data clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<CRrs> {
        CKPOL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Data enable (PSSI_DE) polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<CRrs> {
        DEPOL_W::new(self, 6)
    }
    #[doc = "Bit 8 - Ready (PSSI_RDY) polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rdypol(&mut self) -> RDYPOL_W<CRrs> {
        RDYPOL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Extended data mode"]
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<CRrs> {
        EDM_W::new(self, 10)
    }
    #[doc = "Bit 14 - PSSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 14)
    }
    #[doc = "Bits 18:20 - Data enable and ready configuration"]
    #[inline(always)]
    #[must_use]
    pub fn derdycfg(&mut self) -> DERDYCFG_W<CRrs> {
        DERDYCFG_W::new(self, 18)
    }
    #[doc = "Bit 30 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CRrs> {
        DMAEN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Data direction selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<CRrs> {
        OUTEN_W::new(self, 31)
    }
}
#[doc = "PSSI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x4000_0000"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x4000_0000;
}
