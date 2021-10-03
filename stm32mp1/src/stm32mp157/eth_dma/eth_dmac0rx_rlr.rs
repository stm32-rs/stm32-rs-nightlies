#[doc = "Register `ETH_DMAC0RxRLR` reader"]
pub struct R(crate::R<ETH_DMAC0RXRLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RXRLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RXRLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RXRLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0RxRLR` writer"]
pub struct W(crate::W<ETH_DMAC0RXRLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RXRLR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RXRLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RXRLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDRL` reader - Receive Descriptor Ring Length"]
pub struct RDRL_R(crate::FieldReader<u16, u16>);
impl RDRL_R {
    pub(crate) fn new(bits: u16) -> Self {
        RDRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDRL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDRL` writer - Receive Descriptor Ring Length"]
pub struct RDRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&self) -> RDRL_R {
        RDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Receive Descriptor Ring Length"]
    #[inline(always)]
    pub fn rdrl(&mut self) -> RDRL_W {
        RDRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx descriptor ring length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_rlr](index.html) module"]
pub struct ETH_DMAC0RXRLR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RXRLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0rx_rlr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0RXRLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_rlr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0RXRLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0RxRLR to value 0x8000"]
impl crate::Resettable for ETH_DMAC0RXRLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
