#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "ADC enable command\n\nValue on reset: 0"]
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
#[doc = "Field `ADEN` reader - ADC enable command"]
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
#[doc = "ADC enable command\n\nValue on reset: 0"]
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
#[doc = "Field `ADEN` writer - ADC enable command"]
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
#[doc = "ADC disable command\n\nValue on reset: 0"]
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
#[doc = "Field `ADDIS` reader - ADC disable command"]
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
#[doc = "ADC disable command\n\nValue on reset: 0"]
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
#[doc = "Field `ADDIS` writer - ADC disable command"]
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
#[doc = "ADC start conversion command\n\nValue on reset: 0"]
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
#[doc = "Field `ADSTART` reader - ADC start conversion command"]
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
#[doc = "ADC start conversion command\n\nValue on reset: 0"]
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
#[doc = "Field `ADSTART` writer - ADC start conversion command"]
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
#[doc = "ADC stop conversion command\n\nValue on reset: 0"]
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
#[doc = "Field `ADSTP` reader - ADC stop conversion command"]
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
#[doc = "ADC stop conversion command\n\nValue on reset: 0"]
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
#[doc = "Field `ADSTP` writer - ADC stop conversion command"]
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
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALR {
    #[doc = "0: ADC calibration either not yet performed or completed"]
    NotCalibrating = 0,
    #[doc = "1: ADC calibration in progress"]
    Calibrating = 1,
}
impl From<ADCALR> for bool {
    #[inline(always)]
    fn from(variant: ADCALR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub type ADCAL_R = crate::BitReader<ADCALR>;
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADCALR {
        match self.bits {
            false => ADCALR::NotCalibrating,
            true => ADCALR::Calibrating,
        }
    }
    #[doc = "ADC calibration either not yet performed or completed"]
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        *self == ADCALR::NotCalibrating
    }
    #[doc = "ADC calibration in progress"]
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        *self == ADCALR::Calibrating
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALW {
    #[doc = "1: Start the ADC calibration sequence"]
    StartCalibration = 1,
}
impl From<ADCALW> for bool {
    #[inline(always)]
    fn from(variant: ADCALW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub type ADCAL_W<'a, REG> = crate::BitWriter<'a, REG, ADCALW>;
impl<'a, REG> ADCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start the ADC calibration sequence"]
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(ADCALW::StartCalibration)
    }
}
impl R {
    #[doc = "Bit 0 - ADC enable command"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable command"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<CRrs> {
        ADEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<CRrs> {
        ADDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<CRrs> {
        ADSTART_W::new(self, 2)
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<CRrs> {
        ADSTP_W::new(self, 4)
    }
    #[doc = "Bit 31 - ADC calibration"]
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
