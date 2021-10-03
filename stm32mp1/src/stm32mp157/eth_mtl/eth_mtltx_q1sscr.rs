#[doc = "Register `ETH_MTLTxQ1SSCR` reader"]
pub struct R(crate::R<ETH_MTLTXQ1SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ1SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ1SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ1SSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1SSCR` writer"]
pub struct W(crate::W<ETH_MTLTXQ1SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQ1SSCR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQ1SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQ1SSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSC` reader - SSC"]
pub struct SSC_R(crate::FieldReader<u16, u16>);
impl SSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        SSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSC` writer - SSC"]
pub struct SSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    pub fn ssc(&self) -> SSC_R {
        SSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - SSC"]
    #[inline(always)]
    pub fn ssc(&mut self) -> SSC_W {
        SSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1sscr](index.html) module"]
pub struct ETH_MTLTXQ1SSCR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ1SSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1sscr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1SSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1sscr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1SSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1SSCR to value 0"]
impl crate::Resettable for ETH_MTLTXQ1SSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
