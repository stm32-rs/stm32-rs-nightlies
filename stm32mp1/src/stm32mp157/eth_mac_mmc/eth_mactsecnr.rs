#[doc = "Register `ETH_MACTSECNR` reader"]
pub struct R(crate::R<ETH_MACTSECNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSECNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSECNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSECNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSECNR` writer"]
pub struct W(crate::W<ETH_MACTSECNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSECNR_SPEC>;
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
impl From<crate::W<ETH_MACTSECNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSECNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEC` reader - TSEC"]
pub struct TSEC_R(crate::FieldReader<u32, u32>);
impl TSEC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEC` writer - TSEC"]
pub struct TSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&mut self) -> TSEC_W {
        TSEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactsecnr](index.html) module"]
pub struct ETH_MACTSECNR_SPEC;
impl crate::RegisterSpec for ETH_MACTSECNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactsecnr::R](R) reader structure"]
impl crate::Readable for ETH_MACTSECNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactsecnr::W](W) writer structure"]
impl crate::Writable for ETH_MACTSECNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACTSECNR to value 0"]
impl crate::Resettable for ETH_MACTSECNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
