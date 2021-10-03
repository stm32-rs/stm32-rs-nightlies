#[doc = "Register `PCONFR` reader"]
pub struct R(crate::R<PCONFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCONFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCONFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCONFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCONFR` writer"]
pub struct W(crate::W<PCONFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCONFR_SPEC>;
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
impl From<crate::W<PCONFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCONFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NL` reader - Number of Lanes"]
pub struct NL_R(crate::FieldReader<u8, u8>);
impl NL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NL` writer - Number of Lanes"]
pub struct NL_W<'a> {
    w: &'a mut W,
}
impl<'a> NL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SW_TIME` reader - Stop Wait Time"]
pub struct SW_TIME_R(crate::FieldReader<u8, u8>);
impl SW_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW_TIME` writer - Stop Wait Time"]
pub struct SW_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of Lanes"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Stop Wait Time"]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Lanes"]
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W {
        NL_W { w: self }
    }
    #[doc = "Bits 8:15 - Stop Wait Time"]
    #[inline(always)]
    pub fn sw_time(&mut self) -> SW_TIME_W {
        SW_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host PHY Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconfr](index.html) module"]
pub struct PCONFR_SPEC;
impl crate::RegisterSpec for PCONFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pconfr::R](R) reader structure"]
impl crate::Readable for PCONFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pconfr::W](W) writer structure"]
impl crate::Writable for PCONFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCONFR to value 0"]
impl crate::Resettable for PCONFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
