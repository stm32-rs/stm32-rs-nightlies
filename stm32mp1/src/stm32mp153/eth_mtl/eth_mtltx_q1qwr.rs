#[doc = "Register `ETH_MTLTxQ1QWR` reader"]
pub struct R(crate::R<ETH_MTLTXQ1QWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ1QWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ1QWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ1QWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ1QWR` writer"]
pub struct W(crate::W<ETH_MTLTXQ1QWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQ1QWR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQ1QWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQ1QWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISCQW` reader - ISCQW"]
pub struct ISCQW_R(crate::FieldReader<u32, u32>);
impl ISCQW_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISCQW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISCQW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISCQW` writer - ISCQW"]
pub struct ISCQW_W<'a> {
    w: &'a mut W,
}
impl<'a> ISCQW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x001f_ffff) | (value as u32 & 0x001f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:20 - ISCQW"]
    #[inline(always)]
    pub fn iscqw(&self) -> ISCQW_R {
        ISCQW_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - ISCQW"]
    #[inline(always)]
    pub fn iscqw(&mut self) -> ISCQW_W {
        ISCQW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides the average traffic transmitted on queue 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q1qwr](index.html) module"]
pub struct ETH_MTLTXQ1QWR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ1QWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q1qwr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ1QWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q1qwr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQ1QWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLTxQ1QWR to value 0"]
impl crate::Resettable for ETH_MTLTXQ1QWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
