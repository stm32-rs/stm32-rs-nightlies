#[doc = "Register `MTLTxQDR` reader"]
pub struct R(crate::R<MTLTXQDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTXQDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTXQDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTXQDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLTxQDR` writer"]
pub struct W(crate::W<MTLTXQDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLTXQDR_SPEC>;
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
impl From<crate::W<MTLTXQDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLTXQDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue"]
pub struct STXSTSF_R(crate::FieldReader<u8, u8>);
impl STXSTSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        STXSTSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STXSTSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STXSTSF` writer - Number of Status Words in Tx Status FIFO of Queue"]
pub struct STXSTSF_W<'a> {
    w: &'a mut W,
}
impl<'a> STXSTSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue"]
pub struct PTXQ_R(crate::FieldReader<u8, u8>);
impl PTXQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTXQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTXQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTXQ` writer - Number of Packets in the Transmit Queue"]
pub struct PTXQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status"]
pub struct TXSTSFSTS_R(crate::FieldReader<bool, bool>);
impl TXSTSFSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTSFSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTSFSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTSFSTS` writer - MTL Tx Status FIFO Full Status"]
pub struct TXSTSFSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTSFSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status"]
pub struct TXQSTS_R(crate::FieldReader<bool, bool>);
impl TXQSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXQSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQSTS` writer - MTL Tx Queue Not Empty Status"]
pub struct TXQSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXQSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status"]
pub struct TWCSTS_R(crate::FieldReader<bool, bool>);
impl TWCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWCSTS` writer - MTL Tx Queue Write Controller Status"]
pub struct TWCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TWCSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status"]
pub struct TRCSTS_R(crate::FieldReader<u8, u8>);
impl TRCSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRCSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRCSTS` writer - MTL Tx Queue Read Controller Status"]
pub struct TRCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCSTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause"]
pub struct TXQPAUSED_R(crate::FieldReader<bool, bool>);
impl TXQPAUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXQPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXQPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXQPAUSED` writer - Transmit Queue in Pause"]
pub struct TXQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> TXQPAUSED_W<'a> {
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
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&mut self) -> STXSTSF_W {
        STXSTSF_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&mut self) -> PTXQ_W {
        PTXQ_W { w: self }
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&mut self) -> TXSTSFSTS_W {
        TXSTSFSTS_W { w: self }
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&mut self) -> TXQSTS_W {
        TXQSTS_W { w: self }
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&mut self) -> TWCSTS_W {
        TWCSTS_W { w: self }
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&mut self) -> TRCSTS_W {
        TRCSTS_W { w: self }
    }
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&mut self) -> TXQPAUSED_W {
        TXQPAUSED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qdr](index.html) module"]
pub struct MTLTXQDR_SPEC;
impl crate::RegisterSpec for MTLTXQDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qdr::R](R) reader structure"]
impl crate::Readable for MTLTXQDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtltx_qdr::W](W) writer structure"]
impl crate::Writable for MTLTXQDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLTxQDR to value 0"]
impl crate::Resettable for MTLTXQDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
