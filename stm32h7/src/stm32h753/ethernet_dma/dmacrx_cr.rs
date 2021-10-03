#[doc = "Register `DMACRxCR` reader"]
pub struct R(crate::R<DMACRXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACRXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACRXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACRXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACRxCR` writer"]
pub struct W(crate::W<DMACRXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACRXCR_SPEC>;
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
impl From<crate::W<DMACRXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACRXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Start or Stop Receive Command"]
pub struct SR_R(crate::FieldReader<bool, bool>);
impl SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Start or Stop Receive Command"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub struct RBSZ_R(crate::FieldReader<u16, u16>);
impl RBSZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        RBSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBSZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub struct RBSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 1)) | ((value as u32 & 0x3fff) << 1);
        self.w
    }
}
#[doc = "Field `RXPBL` reader - RXPBL"]
pub struct RXPBL_R(crate::FieldReader<u8, u8>);
impl RXPBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXPBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPBL` writer - RXPBL"]
pub struct RXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub struct RPF_R(crate::FieldReader<bool, bool>);
impl RPF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub struct RPF_W<'a> {
    w: &'a mut W,
}
impl<'a> RPF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W {
        RBSZ_W { w: self }
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W {
        RXPBL_W { w: self }
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W {
        RPF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacrx_cr](index.html) module"]
pub struct DMACRXCR_SPEC;
impl crate::RegisterSpec for DMACRXCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacrx_cr::R](R) reader structure"]
impl crate::Readable for DMACRXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacrx_cr::W](W) writer structure"]
impl crate::Writable for DMACRXCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACRxCR to value 0"]
impl crate::Resettable for DMACRXCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
