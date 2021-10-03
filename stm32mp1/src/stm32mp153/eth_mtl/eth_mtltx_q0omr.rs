#[doc = "Register `ETH_MTLTxQ0OMR` reader"]
pub struct R(crate::R<ETH_MTLTXQ0OMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTXQ0OMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTXQ0OMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTXQ0OMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MTLTxQ0OMR` writer"]
pub struct W(crate::W<ETH_MTLTXQ0OMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTXQ0OMR_SPEC>;
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
impl From<crate::W<ETH_MTLTXQ0OMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTXQ0OMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTQ` reader - FTQ"]
pub struct FTQ_R(crate::FieldReader<bool, bool>);
impl FTQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTQ` writer - FTQ"]
pub struct FTQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FTQ_W<'a> {
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
#[doc = "Field `TSF` reader - TSF"]
pub struct TSF_R(crate::FieldReader<bool, bool>);
impl TSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSF` writer - TSF"]
pub struct TSF_W<'a> {
    w: &'a mut W,
}
impl<'a> TSF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TXQEN` reader - TXQEN"]
pub struct TXQEN_R(crate::FieldReader<u8, u8>);
impl TXQEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQEN` writer - TXQEN"]
pub struct TXQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXQEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TTC` reader - TTC"]
pub struct TTC_R(crate::FieldReader<u8, u8>);
impl TTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTC` writer - TTC"]
pub struct TTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `TQS` reader - TQS"]
pub struct TQS_R(crate::FieldReader<u16, u16>);
impl TQS_R {
    pub(crate) fn new(bits: u16) -> Self {
        TQS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQS` writer - TQS"]
pub struct TQS_W<'a> {
    w: &'a mut W,
}
impl<'a> TQS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FTQ"]
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - TXQEN"]
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:24 - TQS"]
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FTQ"]
    #[inline(always)]
    pub fn ftq(&mut self) -> FTQ_W {
        FTQ_W { w: self }
    }
    #[doc = "Bit 1 - TSF"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W {
        TSF_W { w: self }
    }
    #[doc = "Bits 2:3 - TXQEN"]
    #[inline(always)]
    pub fn txqen(&mut self) -> TXQEN_W {
        TXQEN_W { w: self }
    }
    #[doc = "Bits 4:5 - TTC"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W {
        TTC_W { w: self }
    }
    #[doc = "Bits 16:24 - TQS"]
    #[inline(always)]
    pub fn tqs(&mut self) -> TQS_W {
        TQS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue 0 operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_mtltx_q0omr](index.html) module"]
pub struct ETH_MTLTXQ0OMR_SPEC;
impl crate::RegisterSpec for ETH_MTLTXQ0OMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_mtltx_q0omr::R](R) reader structure"]
impl crate::Readable for ETH_MTLTXQ0OMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_mtltx_q0omr::W](W) writer structure"]
impl crate::Writable for ETH_MTLTXQ0OMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MTLTxQ0OMR to value 0"]
impl crate::Resettable for ETH_MTLTXQ0OMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
