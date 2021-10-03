#[doc = "Register `TTGTP` reader"]
pub struct R(crate::R<TTGTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTGTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTGTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTGTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTGTP` writer"]
pub struct W(crate::W<TTGTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTGTP_SPEC>;
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
impl From<crate::W<TTGTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTGTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NCL` reader - Time Preset"]
pub struct NCL_R(crate::FieldReader<u16, u16>);
impl NCL_R {
    pub(crate) fn new(bits: u16) -> Self {
        NCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCL` writer - Time Preset"]
pub struct NCL_W<'a> {
    w: &'a mut W,
}
impl<'a> NCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CTP` reader - Cycle Time Target Phase"]
pub struct CTP_R(crate::FieldReader<u16, u16>);
impl CTP_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTP` writer - Cycle Time Target Phase"]
pub struct CTP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&mut self) -> NCL_W {
        NCL_W { w: self }
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CTP_W {
        CTP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Global Time Preset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgtp](index.html) module"]
pub struct TTGTP_SPEC;
impl crate::RegisterSpec for TTGTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttgtp::R](R) reader structure"]
impl crate::Readable for TTGTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttgtp::W](W) writer structure"]
impl crate::Writable for TTGTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTGTP to value 0"]
impl crate::Resettable for TTGTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
