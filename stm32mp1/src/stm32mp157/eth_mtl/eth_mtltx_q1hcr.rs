#[doc = "Register `ETH_MTLTxQ1HCR` reader"]
pub struct R(crate::R<ETH_MTLTXQ1HCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ1HCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ1HCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ1HCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1HCR` writer"]
pub struct W(crate::W<ETH_MTLTXQ1HCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQ1HCR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQ1HCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQ1HCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HC` reader - HC"]
pub struct HC_R(crate::FieldReader<u32, u32>);
impl HC_R {
    pub(crate) fn new(bits: u32) -> Self {
        HC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HC` writer - HC"]
pub struct HC_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | (value as u32 & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - HC"]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - HC"]
    #[inline(always)]
    pub fn hc(&mut self) -> HC_W {
        HC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1hcr](index.html) module"]
pub struct ETH_MTLTXQ1HCR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ1HCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1hcr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1HCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1hcr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1HCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1HCR to value 0"]
impl crate::Resettable for ETH_MTLTXQ1HCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
