#[doc = "Register `ETH_MACTSEACR` reader"]
pub struct R(crate::R<ETH_MACTSEACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACTSEACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACTSEACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACTSEACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACTSEACR` writer"]
pub struct W(crate::W<ETH_MACTSEACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACTSEACR_SPEC>;
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
impl From<crate::W<ETH_MACTSEACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACTSEACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTEAC` reader - OSTEAC"]
pub struct OSTEAC_R(crate::FieldReader<u32, u32>);
impl OSTEAC_R {
    pub(crate) fn new(bits: u32) -> Self {
        OSTEAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTEAC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTEAC` writer - OSTEAC"]
pub struct OSTEAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTEAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&self) -> OSTEAC_R {
        OSTEAC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&mut self) -> OSTEAC_W {
        OSTEAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MAC Timestamp Egress Asymmetry Correction register contains the Egress Asymmetry Correction value to be used while updating the correction field in PDelay_Req PTP messages.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mactseacr](index.html) module"]
pub struct ETH_MACTSEACR_SPEC;
impl crate::RegisterSpec for ETH_MACTSEACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mactseacr::R](R) reader structure"]
impl crate::Readable for ETH_MACTSEACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mactseacr::W](W) writer structure"]
impl crate::Writable for ETH_MACTSEACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACTSEACR to value 0"]
impl crate::Resettable for ETH_MACTSEACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
