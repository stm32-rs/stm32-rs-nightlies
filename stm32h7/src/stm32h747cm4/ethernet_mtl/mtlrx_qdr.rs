#[doc = "Register `MTLRxQDR` reader"]
pub struct R(crate::R<MTLRXQDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRXQDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRXQDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRXQDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLRxQDR` writer"]
pub struct W(crate::W<MTLRXQDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLRXQDR_SPEC>;
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
impl From<crate::W<MTLRXQDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLRXQDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue"]
pub struct PRXQ_R(crate::FieldReader<u16, u16>);
impl PRXQ_R {
    pub(crate) fn new(bits: u16) -> Self {
        PRXQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRXQ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRXQ` writer - Number of Packets in Receive Queue"]
pub struct PRXQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRXQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status"]
pub struct RXQSTS_R(crate::FieldReader<u8, u8>);
impl RXQSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXQSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXQSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXQSTS` writer - MTL Rx Queue Fill-Level Status"]
pub struct RXQSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQSTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State"]
pub struct RRCSTS_R(crate::FieldReader<u8, u8>);
impl RRCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRCSTS` writer - MTL Rx Queue Read Controller State"]
pub struct RRCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRCSTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status"]
pub struct RWCSTS_R(crate::FieldReader<bool, bool>);
impl RWCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWCSTS` writer - MTL Rx Queue Write Controller Active Status"]
pub struct RWCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCSTS_W<'a> {
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
impl R {
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&mut self) -> PRXQ_W {
        PRXQ_W { w: self }
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&mut self) -> RXQSTS_W {
        RXQSTS_W { w: self }
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&mut self) -> RRCSTS_W {
        RRCSTS_W { w: self }
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&mut self) -> RWCSTS_W {
        RWCSTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qdr](index.html) module"]
pub struct MTLRXQDR_SPEC;
impl crate::RegisterSpec for MTLRXQDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qdr::R](R) reader structure"]
impl crate::Readable for MTLRXQDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlrx_qdr::W](W) writer structure"]
impl crate::Writable for MTLRXQDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRXQDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
