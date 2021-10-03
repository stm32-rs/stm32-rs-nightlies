#[doc = "Register `MACSSIR` reader"]
pub struct R(crate::R<MACSSIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSSIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSSIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSSIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSSIR` writer"]
pub struct W(crate::W<MACSSIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSSIR_SPEC>;
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
impl From<crate::W<MACSSIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSSIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SNSINC` reader - Sub-nanosecond Increment Value"]
pub struct SNSINC_R(crate::FieldReader<u8, u8>);
impl SNSINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SNSINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SNSINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNSINC` writer - Sub-nanosecond Increment Value"]
pub struct SNSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SNSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SSINC` reader - Sub-second Increment Value"]
pub struct SSINC_R(crate::FieldReader<u8, u8>);
impl SSINC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINC` writer - Sub-second Increment Value"]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Sub-nanosecond Increment Value"]
    #[inline(always)]
    pub fn snsinc(&self) -> SNSINC_R {
        SNSINC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Sub-nanosecond Increment Value"]
    #[inline(always)]
    pub fn snsinc(&mut self) -> SNSINC_W {
        SNSINC_W { w: self }
    }
    #[doc = "Bits 16:23 - Sub-second Increment Value"]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-second increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macssir](index.html) module"]
pub struct MACSSIR_SPEC;
impl crate::RegisterSpec for MACSSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macssir::R](R) reader structure"]
impl crate::Readable for MACSSIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macssir::W](W) writer structure"]
impl crate::Writable for MACSSIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSSIR to value 0"]
impl crate::Resettable for MACSSIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
