#[doc = "Register `ETH_DMAA4RxACR` reader"]
pub struct R(crate::R<ETH_DMAA4RXACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4RXACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4RXACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4RXACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAA4RxACR` writer"]
pub struct W(crate::W<ETH_DMAA4RXACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4RXACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4RXACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4RXACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWC` reader - RDWC"]
pub struct RDWC_R(crate::FieldReader<u8, u8>);
impl RDWC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDWC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDWC` writer - RDWC"]
pub struct RDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RPC` reader - RPC"]
pub struct RPC_R(crate::FieldReader<u8, u8>);
impl RPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPC` writer - RPC"]
pub struct RPC_W<'a> {
    w: &'a mut W,
}
impl<'a> RPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RHC` reader - RHC"]
pub struct RHC_R(crate::FieldReader<u8, u8>);
impl RHC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RHC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RHC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHC` writer - RHC"]
pub struct RHC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RDC` reader - RDC"]
pub struct RDC_R(crate::FieldReader<u8, u8>);
impl RDC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDC` writer - RDC"]
pub struct RDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&self) -> RDWC_R {
        RDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&self) -> RPC_R {
        RPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&self) -> RDC_R {
        RDC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDWC"]
    #[inline(always)]
    pub fn rdwc(&mut self) -> RDWC_W {
        RDWC_W { w: self }
    }
    #[doc = "Bits 8:11 - RPC"]
    #[inline(always)]
    pub fn rpc(&mut self) -> RPC_W {
        RPC_W { w: self }
    }
    #[doc = "Bits 16:19 - RHC"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W {
        RHC_W { w: self }
    }
    #[doc = "Bits 24:25 - RDC"]
    #[inline(always)]
    pub fn rdc(&mut self) -> RDC_W {
        RDC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI4 receive channel ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4rx_acr](index.html) module"]
pub struct ETH_DMAA4RXACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4RXACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaa4rx_acr::R](R) reader structure"]
impl crate::Readable for ETH_DMAA4RXACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4rx_acr::W](W) writer structure"]
impl crate::Writable for ETH_DMAA4RXACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAA4RxACR to value 0"]
impl crate::Resettable for ETH_DMAA4RXACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
