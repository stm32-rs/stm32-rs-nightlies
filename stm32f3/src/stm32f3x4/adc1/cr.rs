#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "ADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENR {
    #[doc = "0: ADC disabled"]
    Disabled = 0,
    #[doc = "1: ADC enabled"]
    Enabled = 1,
}
impl From<ADENR> for bool {
    #[inline(always)]
    fn from(variant: ADENR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader<ADENR>;
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADENR {
        match self.bits {
            false => ADENR::Disabled,
            true => ADENR::Enabled,
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADENR::Disabled
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADENR::Enabled
    }
}
#[doc = "ADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW {
    #[doc = "1: Enable the ADC"]
    Enabled = 1,
}
impl From<ADENW> for bool {
    #[inline(always)]
    fn from(variant: ADENW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` writer - ADEN"]
pub type ADEN_W<'a, REG> = crate::BitWriter<'a, REG, ADENW>;
impl<'a, REG> ADEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADENW::Enabled)
    }
}
#[doc = "ADDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISR {
    #[doc = "0: No disable command active"]
    NotDisabling = 0,
    #[doc = "1: ADC disabling"]
    Disabling = 1,
}
impl From<ADDISR> for bool {
    #[inline(always)]
    fn from(variant: ADDISR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader<ADDISR>;
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDISR {
        match self.bits {
            false => ADDISR::NotDisabling,
            true => ADDISR::Disabling,
        }
    }
    #[doc = "No disable command active"]
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        *self == ADDISR::NotDisabling
    }
    #[doc = "ADC disabling"]
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        *self == ADDISR::Disabling
    }
}
#[doc = "ADDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW {
    #[doc = "1: Disable the ADC"]
    Disable = 1,
}
impl From<ADDISW> for bool {
    #[inline(always)]
    fn from(variant: ADDISW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type ADDIS_W<'a, REG> = crate::BitWriter<'a, REG, ADDISW>;
impl<'a, REG> ADDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the ADC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ADDISW::Disable)
    }
}
#[doc = "ADSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTR {
    #[doc = "0: No conversion ongoing"]
    NotActive = 0,
    #[doc = "1: ADC operating and may be converting"]
    Active = 1,
}
impl From<ADSTARTR> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader<ADSTARTR>;
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTARTR {
        match self.bits {
            false => ADSTARTR::NotActive,
            true => ADSTARTR::Active,
        }
    }
    #[doc = "No conversion ongoing"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == ADSTARTR::NotActive
    }
    #[doc = "ADC operating and may be converting"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ADSTARTR::Active
    }
}
#[doc = "ADSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTARTW {
    #[doc = "1: Start the ADC conversion (may be delayed for hardware triggers)"]
    StartConversion = 1,
}
impl From<ADSTARTW> for bool {
    #[inline(always)]
    fn from(variant: ADSTARTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type ADSTART_W<'a, REG> = crate::BitWriter<'a, REG, ADSTARTW>;
impl<'a, REG> ADSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start the ADC conversion (may be delayed for hardware triggers)"]
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTARTW::StartConversion)
    }
}
#[doc = "Field `JADSTART` reader - JADSTART"]
pub use ADSTART_R as JADSTART_R;
#[doc = "Field `JADSTART` writer - JADSTART"]
pub use ADSTART_W as JADSTART_W;
#[doc = "ADSTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPR {
    #[doc = "0: No stop command active"]
    NotStopping = 0,
    #[doc = "1: ADC stopping conversion"]
    Stopping = 1,
}
impl From<ADSTPR> for bool {
    #[inline(always)]
    fn from(variant: ADSTPR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader<ADSTPR>;
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADSTPR {
        match self.bits {
            false => ADSTPR::NotStopping,
            true => ADSTPR::Stopping,
        }
    }
    #[doc = "No stop command active"]
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        *self == ADSTPR::NotStopping
    }
    #[doc = "ADC stopping conversion"]
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        *self == ADSTPR::Stopping
    }
}
#[doc = "ADSTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTPW {
    #[doc = "1: Stop the active conversion"]
    StopConversion = 1,
}
impl From<ADSTPW> for bool {
    #[inline(always)]
    fn from(variant: ADSTPW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG, ADSTPW>;
impl<'a, REG> ADSTP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop the active conversion"]
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut crate::W<REG> {
        self.variant(ADSTPW::StopConversion)
    }
}
#[doc = "Field `JADSTP` reader - JADSTP"]
pub use ADSTP_R as JADSTP_R;
#[doc = "Field `JADSTP` writer - JADSTP"]
pub use ADSTP_W as JADSTP_W;
#[doc = "ADVREGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADVREGEN {
    #[doc = "0: Intermediate state required when moving the ADC voltage regulator between states"]
    Intermediate = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    Enabled = 1,
    #[doc = "2: ADC voltage regulator disabled"]
    Disabled = 2,
}
impl From<ADVREGEN> for u8 {
    #[inline(always)]
    fn from(variant: ADVREGEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADVREGEN {
    type Ux = u8;
}
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::FieldReader<ADVREGEN>;
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADVREGEN> {
        match self.bits {
            0 => Some(ADVREGEN::Intermediate),
            1 => Some(ADVREGEN::Enabled),
            2 => Some(ADVREGEN::Disabled),
            _ => None,
        }
    }
    #[doc = "Intermediate state required when moving the ADC voltage regulator between states"]
    #[inline(always)]
    pub fn is_intermediate(&self) -> bool {
        *self == ADVREGEN::Intermediate
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN::Enabled
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN::Disabled
    }
}
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADVREGEN>;
impl<'a, REG> ADVREGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Intermediate state required when moving the ADC voltage regulator between states"]
    #[inline(always)]
    pub fn intermediate(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Intermediate)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Enabled)
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADVREGEN::Disabled)
    }
}
#[doc = "ADCALDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF {
    #[doc = "0: Calibration for single-ended mode"]
    SingleEnded = 0,
    #[doc = "1: Calibration for differential mode"]
    Differential = 1,
}
impl From<ADCALDIF> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub type ADCALDIF_R = crate::BitReader<ADCALDIF>;
impl ADCALDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCALDIF {
        match self.bits {
            false => ADCALDIF::SingleEnded,
            true => ADCALDIF::Differential,
        }
    }
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF::SingleEnded
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF::Differential
    }
}
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub type ADCALDIF_W<'a, REG> = crate::BitWriter<'a, REG, ADCALDIF>;
impl<'a, REG> ADCALDIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::SingleEnded)
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALDIF::Differential)
    }
}
#[doc = "ADCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL {
    #[doc = "0: Calibration complete"]
    Complete = 0,
    #[doc = "1: Start the calibration of the ADC"]
    Calibration = 1,
}
impl From<ADCAL> for bool {
    #[inline(always)]
    fn from(variant: ADCAL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader<ADCAL>;
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCAL {
        match self.bits {
            false => ADCAL::Complete,
            true => ADCAL::Calibration,
        }
    }
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ADCAL::Complete
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == ADCAL::Calibration
    }
}
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCAL>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::Complete)
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCAL::Calibration)
    }
}
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<CRrs> {
        JADSTART_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<CRrs> {
        JADSTP_W::new(self, 5)
    }
    #[doc = "Bits 28:29 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<CRrs> {
        ADVREGEN_W::new(self, 28)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<CRrs> {
        ADCALDIF_W::new(self, 30)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CRrs> {
        ADCAL_W::new(self, 31)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
