#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCRrs>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCRrs>;
#[doc = "LCD-TFT controller enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCEN {
    #[doc = "0: LCD-TFT controller disabled"]
    Disabled = 0,
    #[doc = "1: LCD-TFT controller enabled"]
    Enabled = 1,
}
impl From<LTDCEN> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub type LTDCEN_R = crate::BitReader<LTDCEN>;
impl LTDCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LTDCEN {
        match self.bits {
            false => LTDCEN::Disabled,
            true => LTDCEN::Enabled,
        }
    }
    #[doc = "LCD-TFT controller disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LTDCEN::Disabled
    }
    #[doc = "LCD-TFT controller enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LTDCEN::Enabled
    }
}
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG, LTDCEN>;
impl<'a, REG> LTDCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD-TFT controller disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Disabled)
    }
    #[doc = "LCD-TFT controller enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCEN::Enabled)
    }
}
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub type DBW_R = crate::FieldReader;
#[doc = "Field `DGW` reader - Dither Green Width"]
pub type DGW_R = crate::FieldReader;
#[doc = "Field `DRW` reader - Dither Red Width"]
pub type DRW_R = crate::FieldReader;
#[doc = "Dither Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN {
    #[doc = "0: Dither disabled"]
    Disabled = 0,
    #[doc = "1: Dither enabled"]
    Enabled = 1,
}
impl From<DEN> for bool {
    #[inline(always)]
    fn from(variant: DEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEN` reader - Dither Enable"]
pub type DEN_R = crate::BitReader<DEN>;
impl DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DEN {
        match self.bits {
            false => DEN::Disabled,
            true => DEN::Enabled,
        }
    }
    #[doc = "Dither disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEN::Disabled
    }
    #[doc = "Dither enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEN::Enabled
    }
}
#[doc = "Field `DEN` writer - Dither Enable"]
pub type DEN_W<'a, REG> = crate::BitWriter<'a, REG, DEN>;
impl<'a, REG> DEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dither disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::Disabled)
    }
    #[doc = "Dither enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DEN::Enabled)
    }
}
#[doc = "Pixel Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCPOL {
    #[doc = "0: Pixel clock on rising edge"]
    RisingEdge = 0,
    #[doc = "1: Pixel clock on falling edge"]
    FallingEdge = 1,
}
impl From<PCPOL> for bool {
    #[inline(always)]
    fn from(variant: PCPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub type PCPOL_R = crate::BitReader<PCPOL>;
impl PCPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCPOL {
        match self.bits {
            false => PCPOL::RisingEdge,
            true => PCPOL::FallingEdge,
        }
    }
    #[doc = "Pixel clock on rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == PCPOL::RisingEdge
    }
    #[doc = "Pixel clock on falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == PCPOL::FallingEdge
    }
}
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub type PCPOL_W<'a, REG> = crate::BitWriter<'a, REG, PCPOL>;
impl<'a, REG> PCPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pixel clock on rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCPOL::RisingEdge)
    }
    #[doc = "Pixel clock on falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(PCPOL::FallingEdge)
    }
}
#[doc = "Data Enable Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEPOL {
    #[doc = "0: Data enable polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Data enable polarity is active high"]
    ActiveHigh = 1,
}
impl From<DEPOL> for bool {
    #[inline(always)]
    fn from(variant: DEPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEPOL` reader - Data Enable Polarity"]
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
    #[doc = "Data enable polarity is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == DEPOL::ActiveLow
    }
    #[doc = "Data enable polarity is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == DEPOL::ActiveHigh
    }
}
#[doc = "Field `DEPOL` writer - Data Enable Polarity"]
pub type DEPOL_W<'a, REG> = crate::BitWriter<'a, REG, DEPOL>;
impl<'a, REG> DEPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data enable polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveLow)
    }
    #[doc = "Data enable polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(DEPOL::ActiveHigh)
    }
}
#[doc = "Vertical Synchronization Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSPOL {
    #[doc = "0: Vertical synchronization polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Vertical synchronization polarity is active high"]
    ActiveHigh = 1,
}
impl From<VSPOL> for bool {
    #[inline(always)]
    fn from(variant: VSPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub type VSPOL_R = crate::BitReader<VSPOL>;
impl VSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSPOL {
        match self.bits {
            false => VSPOL::ActiveLow,
            true => VSPOL::ActiveHigh,
        }
    }
    #[doc = "Vertical synchronization polarity is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == VSPOL::ActiveLow
    }
    #[doc = "Vertical synchronization polarity is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == VSPOL::ActiveHigh
    }
}
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub type VSPOL_W<'a, REG> = crate::BitWriter<'a, REG, VSPOL>;
impl<'a, REG> VSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Vertical synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveLow)
    }
    #[doc = "Vertical synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(VSPOL::ActiveHigh)
    }
}
#[doc = "Horizontal Synchronization Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSPOL {
    #[doc = "0: Horizontal synchronization polarity is active low"]
    ActiveLow = 0,
    #[doc = "1: Horizontal synchronization polarity is active high"]
    ActiveHigh = 1,
}
impl From<HSPOL> for bool {
    #[inline(always)]
    fn from(variant: HSPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub type HSPOL_R = crate::BitReader<HSPOL>;
impl HSPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSPOL {
        match self.bits {
            false => HSPOL::ActiveLow,
            true => HSPOL::ActiveHigh,
        }
    }
    #[doc = "Horizontal synchronization polarity is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HSPOL::ActiveLow
    }
    #[doc = "Horizontal synchronization polarity is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HSPOL::ActiveHigh
    }
}
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub type HSPOL_W<'a, REG> = crate::BitWriter<'a, REG, HSPOL>;
impl<'a, REG> HSPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Horizontal synchronization polarity is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveLow)
    }
    #[doc = "Horizontal synchronization polarity is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(HSPOL::ActiveHigh)
    }
}
impl R {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DGW_R {
        DGW_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DRW_R {
        DRW_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DEN_R {
        DEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PCPOL_R {
        PCPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DEPOL_R {
        DEPOL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<GCRrs> {
        LTDCEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    #[must_use]
    pub fn den(&mut self) -> DEN_W<GCRrs> {
        DEN_W::new(self, 16)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pcpol(&mut self) -> PCPOL_W<GCRrs> {
        PCPOL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn depol(&mut self) -> DEPOL_W<GCRrs> {
        DEPOL_W::new(self, 29)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<GCRrs> {
        VSPOL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<GCRrs> {
        HSPOL_W::new(self, 31)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCRrs;
impl crate::RegisterSpec for GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCRrs {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0x2220"]
impl crate::Resettable for GCRrs {
    const RESET_VALUE: u32 = 0x2220;
}
