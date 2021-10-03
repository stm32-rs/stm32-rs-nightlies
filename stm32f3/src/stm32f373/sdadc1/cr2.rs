#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAST` reader - Fast conversion mode selection"]
pub struct FAST_R(crate::FieldReader<bool, bool>);
impl FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAST` writer - Fast conversion mode selection"]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub struct RSWSTART_R(crate::FieldReader<bool, bool>);
impl RSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSWSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSWSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub struct RSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RSWSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub struct RCONT_R(crate::FieldReader<bool, bool>);
impl RCONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub struct RCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RCH` reader - Regular channel selection"]
pub struct RCH_R(crate::FieldReader<u8, u8>);
impl RCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCH` writer - Regular channel selection"]
pub struct RCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub struct JSWSTART_R(crate::FieldReader<bool, bool>);
impl JSWSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSWSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JSWSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub struct JEXTEN_R(crate::FieldReader<u8, u8>);
impl JEXTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEXTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub struct JEXTSEL_R(crate::FieldReader<u8, u8>);
impl JEXTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        JEXTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEXTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `JDS` reader - Delay start of injected conversions."]
pub struct JDS_R(crate::FieldReader<bool, bool>);
impl JDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDS` writer - Delay start of injected conversions."]
pub struct JDS_W<'a> {
    w: &'a mut W,
}
impl<'a> JDS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `JCONT` reader - Continuous mode selection for injected conversions"]
pub struct JCONT_R(crate::FieldReader<bool, bool>);
impl JCONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        JCONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JCONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JCONT` writer - Continuous mode selection for injected conversions"]
pub struct JCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> JCONT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `STARTCALIB` reader - Start calibration"]
pub struct STARTCALIB_R(crate::FieldReader<bool, bool>);
impl STARTCALIB_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTCALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTCALIB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTCALIB` writer - Start calibration"]
pub struct STARTCALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTCALIB_W<'a> {
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
#[doc = "Field `CALIBCNT` reader - Number of calibration sequences to be performed (number of valid configurations)"]
pub struct CALIBCNT_R(crate::FieldReader<u8, u8>);
impl CALIBCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALIBCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIBCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIBCNT` writer - Number of calibration sequences to be performed (number of valid configurations)"]
pub struct CALIBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIBCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `ADON` reader - SDADC enable"]
pub struct ADON_R(crate::FieldReader<bool, bool>);
impl ADON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADON` writer - SDADC enable"]
pub struct ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> ADON_W<'a> {
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
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RSWSTART_R {
        RSWSTART_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    pub fn jds(&self) -> JDS_R {
        JDS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    pub fn jcont(&self) -> JCONT_R {
        JCONT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    pub fn startcalib(&self) -> STARTCALIB_R {
        STARTCALIB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    pub fn calibcnt(&self) -> CALIBCNT_R {
        CALIBCNT_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Fast conversion mode selection"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bit 23 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W {
        RSWSTART_W { w: self }
    }
    #[doc = "Bit 22 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W {
        RCONT_W { w: self }
    }
    #[doc = "Bits 16:19 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W {
        RCH_W { w: self }
    }
    #[doc = "Bit 15 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bit 6 - Delay start of injected conversions."]
    #[inline(always)]
    pub fn jds(&mut self) -> JDS_W {
        JDS_W { w: self }
    }
    #[doc = "Bit 5 - Continuous mode selection for injected conversions"]
    #[inline(always)]
    pub fn jcont(&mut self) -> JCONT_W {
        JCONT_W { w: self }
    }
    #[doc = "Bit 4 - Start calibration"]
    #[inline(always)]
    pub fn startcalib(&mut self) -> STARTCALIB_W {
        STARTCALIB_W { w: self }
    }
    #[doc = "Bits 1:2 - Number of calibration sequences to be performed (number of valid configurations)"]
    #[inline(always)]
    pub fn calibcnt(&mut self) -> CALIBCNT_W {
        CALIBCNT_W { w: self }
    }
    #[doc = "Bit 0 - SDADC enable"]
    #[inline(always)]
    pub fn adon(&mut self) -> ADON_W {
        ADON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
