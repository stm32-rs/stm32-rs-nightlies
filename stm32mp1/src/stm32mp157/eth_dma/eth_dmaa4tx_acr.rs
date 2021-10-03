#[doc = "Register `ETH_DMAA4TxACR` reader"]
pub struct R(crate::R<ETH_DMAA4TXACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4TXACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4TXACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4TXACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAA4TxACR` writer"]
pub struct W(crate::W<ETH_DMAA4TXACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4TXACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4TXACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4TXACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDRC` reader - TDRC"]
pub struct TDRC_R(crate::FieldReader<u8, u8>);
impl TDRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDRC` writer - TDRC"]
pub struct TDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TEC` reader - TEC"]
pub struct TEC_R(crate::FieldReader<u8, u8>);
impl TEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEC` writer - TEC"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `THC` reader - THC"]
pub struct THC_R(crate::FieldReader<u8, u8>);
impl THC_R {
    pub(crate) fn new(bits: u8) -> Self {
        THC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THC` writer - THC"]
pub struct THC_W<'a> {
    w: &'a mut W,
}
impl<'a> THC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDRC"]
    #[inline(always)]
    pub fn tdrc(&mut self) -> TDRC_W {
        TDRC_W { w: self }
    }
    #[doc = "Bits 8:11 - TEC"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Bits 16:19 - THC"]
    #[inline(always)]
    pub fn thc(&mut self) -> THC_W {
        THC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI4 transmit channel ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4tx_acr](index.html) module"]
pub struct ETH_DMAA4TXACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4TXACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaa4tx_acr::R](R) reader structure"]
impl crate::Readable for ETH_DMAA4TXACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4tx_acr::W](W) writer structure"]
impl crate::Writable for ETH_DMAA4TXACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAA4TxACR to value 0"]
impl crate::Resettable for ETH_DMAA4TXACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
