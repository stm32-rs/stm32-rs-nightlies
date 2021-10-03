#[doc = "Register `RTC_ICSR` reader"]
pub struct R(crate::R<RTC_ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ICSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ICSR` writer"]
pub struct W(crate::W<RTC_ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ICSR_SPEC>;
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
impl From<crate::W<RTC_ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ICSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRAWF` reader - ALRAWF"]
pub struct ALRAWF_R(crate::FieldReader<bool, bool>);
impl ALRAWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRAWF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRAWF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRBWF` reader - ALRBWF"]
pub struct ALRBWF_R(crate::FieldReader<bool, bool>);
impl ALRBWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRBWF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRBWF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUTWF` reader - WUTWF"]
pub struct WUTWF_R(crate::FieldReader<bool, bool>);
impl WUTWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUTWF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUTWF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHPF` reader - SHPF"]
pub struct SHPF_R(crate::FieldReader<bool, bool>);
impl SHPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITS` reader - INITS"]
pub struct INITS_R(crate::FieldReader<bool, bool>);
impl INITS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSF` reader - RSF"]
pub struct RSF_R(crate::FieldReader<bool, bool>);
impl RSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSF` writer - RSF"]
pub struct RSF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSF_W<'a> {
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
#[doc = "Field `INITF` reader - INITF"]
pub struct INITF_R(crate::FieldReader<bool, bool>);
impl INITF_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` reader - INIT"]
pub struct INIT_R(crate::FieldReader<bool, bool>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - INIT"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RECALPF` reader - RECALPF"]
pub struct RECALPF_R(crate::FieldReader<bool, bool>);
impl RECALPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECALPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECALPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ALRAWF"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALRBWF"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WUTWF"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SHPF"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - INITS"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INITF"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RECALPF"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RSF"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W {
        RSF_W { w: self }
    }
    #[doc = "Bit 7 - INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_icsr](index.html) module"]
pub struct RTC_ICSR_SPEC;
impl crate::RegisterSpec for RTC_ICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_icsr::R](R) reader structure"]
impl crate::Readable for RTC_ICSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_icsr::W](W) writer structure"]
impl crate::Writable for RTC_ICSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ICSR to value 0x07"]
impl crate::Resettable for RTC_ICSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
