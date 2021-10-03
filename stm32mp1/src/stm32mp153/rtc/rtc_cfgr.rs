#[doc = "Register `RTC_CFGR` reader"]
pub struct R(crate::R<RTC_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CFGR` writer"]
pub struct W(crate::W<RTC_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CFGR_SPEC>;
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
impl From<crate::W<RTC_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT2_RMP` reader - OUT2_RMP"]
pub struct OUT2_RMP_R(crate::FieldReader<bool, bool>);
impl OUT2_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT2_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT2_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT2_RMP` writer - OUT2_RMP"]
pub struct OUT2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT2_RMP_W<'a> {
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
#[doc = "Field `LSCOEN` reader - LSCOEN"]
pub struct LSCOEN_R(crate::FieldReader<u8, u8>);
impl LSCOEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSCOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSCOEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSCOEN` writer - LSCOEN"]
pub struct LSCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSCOEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    pub fn out2_rmp(&self) -> OUT2_RMP_R {
        OUT2_RMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OUT2_RMP"]
    #[inline(always)]
    pub fn out2_rmp(&mut self) -> OUT2_RMP_W {
        OUT2_RMP_W { w: self }
    }
    #[doc = "Bits 1:2 - LSCOEN"]
    #[inline(always)]
    pub fn lscoen(&mut self) -> LSCOEN_W {
        LSCOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cfgr](index.html) module"]
pub struct RTC_CFGR_SPEC;
impl crate::RegisterSpec for RTC_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cfgr::R](R) reader structure"]
impl crate::Readable for RTC_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cfgr::W](W) writer structure"]
impl crate::Writable for RTC_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CFGR to value 0"]
impl crate::Resettable for RTC_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
