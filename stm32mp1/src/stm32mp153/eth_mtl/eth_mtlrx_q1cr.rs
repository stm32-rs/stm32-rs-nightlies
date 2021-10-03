#[doc = "Register `ETH_MTLRxQ1CR` reader"]
pub struct R(crate::R<ETH_MTLRXQ1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLRXQ1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLRXQ1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLRXQ1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLRxQ1CR` writer"]
pub struct W(crate::W<ETH_MTLRXQ1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLRXQ1CR_SPEC>;
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
impl From<crate::W<ETH_MTLRXQ1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLRXQ1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXQ_WEGT` reader - RXQ_WEGT"]
pub struct RXQ_WEGT_R(crate::FieldReader<u8, u8>);
impl RXQ_WEGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQ_WEGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ_WEGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQ_FRM_ARBIT` reader - RXQ_FRM_ARBIT"]
pub struct RXQ_FRM_ARBIT_R(crate::FieldReader<bool, bool>);
impl RXQ_FRM_ARBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXQ_FRM_ARBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQ_FRM_ARBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - RXQ_WEGT"]
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - RXQ_FRM_ARBIT"]
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtlrx_q1cr](index.html) module"]
pub struct ETH_MTLRXQ1CR_SPEC;
impl crate::RegisterSpec for ETH_MTLRXQ1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtlrx_q1cr::R](R) reader structure"]
impl crate::Readable for ETH_MTLRXQ1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtlrx_q1cr::W](W) writer structure"]
impl crate::Writable for ETH_MTLRXQ1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLRxQ1CR to value 0"]
impl crate::Resettable for ETH_MTLRXQ1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
