#[doc = "Register `RTOR` reader"]
pub struct R(crate::R<RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTOR` writer"]
pub struct W(crate::W<RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTOR_SPEC>;
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
impl From<crate::W<RTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEN` reader - Block Length"]
pub struct BLEN_R(crate::FieldReader<u8, u8>);
impl BLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEN` writer - Block Length"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `RTO` reader - Receiver timeout value"]
pub struct RTO_R(crate::FieldReader<u32, u32>);
impl RTO_R {
    pub(crate) fn new(bits: u32) -> Self {
        RTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO` writer - Receiver timeout value"]
pub struct RTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W {
        RTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](index.html) module"]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtor::R](R) reader structure"]
impl crate::Readable for RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtor::W](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
