#[doc = "Register `ETH_DMAC0RxIWTR` reader"]
pub struct R(crate::R<ETH_DMAC0RXIWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RXIWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RXIWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RXIWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0RxIWTR` writer"]
pub struct W(crate::W<ETH_DMAC0RXIWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RXIWTR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RXIWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RXIWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count"]
pub struct RWT_R(crate::FieldReader<u8, u8>);
impl RWT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RWT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count"]
pub struct RWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_iwtr](index.html) module"]
pub struct ETH_DMAC0RXIWTR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RXIWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0rx_iwtr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0RXIWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_iwtr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0RXIWTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0RxIWTR to value 0"]
impl crate::Resettable for ETH_DMAC0RXIWTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
