#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCAL_A {
    #[doc = "0: ADC calibration either not yet performed or completed"]
    NOTCALIBRATING = 0,
    #[doc = "1: ADC calibration in progress"]
    CALIBRATING = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub struct ADCAL_R(crate::FieldReader<bool, ADCAL_A>);
impl ADCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::NOTCALIBRATING,
            true => ADCAL_A::CALIBRATING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCALIBRATING`"]
    #[inline(always)]
    pub fn is_not_calibrating(&self) -> bool {
        **self == ADCAL_A::NOTCALIBRATING
    }
    #[doc = "Checks if the value of the field is `CALIBRATING`"]
    #[inline(always)]
    pub fn is_calibrating(&self) -> bool {
        **self == ADCAL_A::CALIBRATING
    }
}
impl core::ops::Deref for ADCAL_R {
    type Target = crate::FieldReader<bool, ADCAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC calibration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCAL_AW {
    #[doc = "1: Start the ADC calibration sequence"]
    STARTCALIBRATION = 1,
}
impl From<ADCAL_AW> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub struct ADCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start the ADC calibration sequence"]
    #[inline(always)]
    pub fn start_calibration(self) -> &'a mut W {
        self.variant(ADCAL_AW::STARTCALIBRATION)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "ADC stop conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTP_A {
    #[doc = "0: No stop command active"]
    NOTSTOPPING = 0,
    #[doc = "1: ADC stopping conversion"]
    STOPPING = 1,
}
impl From<ADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` reader - ADC stop conversion command"]
pub struct ADSTP_R(crate::FieldReader<bool, ADSTP_A>);
impl ADSTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTP_A {
        match self.bits {
            false => ADSTP_A::NOTSTOPPING,
            true => ADSTP_A::STOPPING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTOPPING`"]
    #[inline(always)]
    pub fn is_not_stopping(&self) -> bool {
        **self == ADSTP_A::NOTSTOPPING
    }
    #[doc = "Checks if the value of the field is `STOPPING`"]
    #[inline(always)]
    pub fn is_stopping(&self) -> bool {
        **self == ADSTP_A::STOPPING
    }
}
impl core::ops::Deref for ADSTP_R {
    type Target = crate::FieldReader<bool, ADSTP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC stop conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTP_AW {
    #[doc = "1: Stop the active conversion"]
    STOPCONVERSION = 1,
}
impl From<ADSTP_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTP` writer - ADC stop conversion command"]
pub struct ADSTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop the active conversion"]
    #[inline(always)]
    pub fn stop_conversion(self) -> &'a mut W {
        self.variant(ADSTP_AW::STOPCONVERSION)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "ADC start conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTART_A {
    #[doc = "0: No conversion ongoing"]
    NOTACTIVE = 0,
    #[doc = "1: ADC operating and may be converting"]
    ACTIVE = 1,
}
impl From<ADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` reader - ADC start conversion command"]
pub struct ADSTART_R(crate::FieldReader<bool, ADSTART_A>);
impl ADSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSTART_A {
        match self.bits {
            false => ADSTART_A::NOTACTIVE,
            true => ADSTART_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTACTIVE`"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        **self == ADSTART_A::NOTACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == ADSTART_A::ACTIVE
    }
}
impl core::ops::Deref for ADSTART_R {
    type Target = crate::FieldReader<bool, ADSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC start conversion command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADSTART_AW {
    #[doc = "1: Start the ADC conversion (may be delayed for hardware triggers)"]
    STARTCONVERSION = 1,
}
impl From<ADSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADSTART` writer - ADC start conversion command"]
pub struct ADSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSTART_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start the ADC conversion (may be delayed for hardware triggers)"]
    #[inline(always)]
    pub fn start_conversion(self) -> &'a mut W {
        self.variant(ADSTART_AW::STARTCONVERSION)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "ADC disable command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIS_A {
    #[doc = "0: No disable command active"]
    NOTDISABLING = 0,
    #[doc = "1: ADC disabling"]
    DISABLING = 1,
}
impl From<ADDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` reader - ADC disable command"]
pub struct ADDIS_R(crate::FieldReader<bool, ADDIS_A>);
impl ADDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDIS_A {
        match self.bits {
            false => ADDIS_A::NOTDISABLING,
            true => ADDIS_A::DISABLING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDISABLING`"]
    #[inline(always)]
    pub fn is_not_disabling(&self) -> bool {
        **self == ADDIS_A::NOTDISABLING
    }
    #[doc = "Checks if the value of the field is `DISABLING`"]
    #[inline(always)]
    pub fn is_disabling(&self) -> bool {
        **self == ADDIS_A::DISABLING
    }
}
impl core::ops::Deref for ADDIS_R {
    type Target = crate::FieldReader<bool, ADDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC disable command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDIS_AW {
    #[doc = "1: Disable the ADC"]
    DISABLE = 1,
}
impl From<ADDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: ADDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDIS` writer - ADC disable command"]
pub struct ADDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the ADC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDIS_AW::DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC enable command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_A {
    #[doc = "0: ADC disabled"]
    DISABLED = 0,
    #[doc = "1: ADC enabled"]
    ENABLED = 1,
}
impl From<ADEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` reader - ADC enable command"]
pub struct ADEN_R(crate::FieldReader<bool, ADEN_A>);
impl ADEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEN_A {
        match self.bits {
            false => ADEN_A::DISABLED,
            true => ADEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADEN_A::ENABLED
    }
}
impl core::ops::Deref for ADEN_R {
    type Target = crate::FieldReader<bool, ADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC enable command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADEN_AW {
    #[doc = "1: Enable the ADC"]
    ENABLED = 1,
}
impl From<ADEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ADEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEN` writer - ADC enable command"]
pub struct ADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADEN_AW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC enable command"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&mut self) -> ADCAL_W {
        ADCAL_W { w: self }
    }
    #[doc = "Bit 4 - ADC stop conversion command"]
    #[inline(always)]
    pub fn adstp(&mut self) -> ADSTP_W {
        ADSTP_W { w: self }
    }
    #[doc = "Bit 2 - ADC start conversion command"]
    #[inline(always)]
    pub fn adstart(&mut self) -> ADSTART_W {
        ADSTART_W { w: self }
    }
    #[doc = "Bit 1 - ADC disable command"]
    #[inline(always)]
    pub fn addis(&mut self) -> ADDIS_W {
        ADDIS_W { w: self }
    }
    #[doc = "Bit 0 - ADC enable command"]
    #[inline(always)]
    pub fn aden(&mut self) -> ADEN_W {
        ADEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
