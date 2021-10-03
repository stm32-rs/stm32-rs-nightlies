#[doc = "Register `ETH_DMAA4DACR` reader"]
pub struct R(crate::R<ETH_DMAA4DACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4DACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4DACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4DACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAA4DACR` writer"]
pub struct W(crate::W<ETH_DMAA4DACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4DACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4DACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4DACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDWC` reader - TDWC"]
pub struct TDWC_R(crate::FieldReader<u8, u8>);
impl TDWC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDWC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDWC` writer - TDWC"]
pub struct TDWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TDWD` reader - TDWD"]
pub struct TDWD_R(crate::FieldReader<u8, u8>);
impl TDWD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDWD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDWD` writer - TDWD"]
pub struct TDWD_W<'a> {
    w: &'a mut W,
}
impl<'a> TDWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RDRC` reader - RDRC"]
pub struct RDRC_R(crate::FieldReader<u8, u8>);
impl RDRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDRC` writer - RDRC"]
pub struct RDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RDP` reader - RDP"]
pub struct RDP_R(crate::FieldReader<u8, u8>);
impl RDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDP` writer - RDP"]
pub struct RDP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `WRP` reader - WRP"]
pub struct WRP_R(crate::FieldReader<u8, u8>);
impl WRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRP` writer - WRP"]
pub struct WRP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&self) -> TDWC_R {
        TDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&self) -> TDWD_R {
        TDWD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&self) -> RDRC_R {
        RDRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&mut self) -> TDWC_W {
        TDWC_W { w: self }
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&mut self) -> TDWD_W {
        TDWD_W { w: self }
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&mut self) -> RDRC_W {
        RDRC_W { w: self }
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W {
        RDP_W { w: self }
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&mut self) -> WRP_W {
        WRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI4 descriptor ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4dacr](index.html) module"]
pub struct ETH_DMAA4DACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4DACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaa4dacr::R](R) reader structure"]
impl crate::Readable for ETH_DMAA4DACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4dacr::W](W) writer structure"]
impl crate::Writable for ETH_DMAA4DACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAA4DACR to value 0"]
impl crate::Resettable for ETH_DMAA4DACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
