#[doc = "Register `ETH_DMAC0TxCR` reader"]
pub struct R(crate::R<ETH_DMAC0TXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0TXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0TXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0TXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0TxCR` writer"]
pub struct W(crate::W<ETH_DMAC0TXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0TXCR_SPEC>;
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
impl From<crate::W<ETH_DMAC0TXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0TXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - ST"]
pub struct ST_R(crate::FieldReader<bool, bool>);
impl ST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ST` writer - ST"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
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
#[doc = "Field `TCW` reader - TCW"]
pub struct TCW_R(crate::FieldReader<u8, u8>);
impl TCW_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCW` writer - TCW"]
pub struct TCW_W<'a> {
    w: &'a mut W,
}
impl<'a> TCW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `OSF` reader - OSF"]
pub struct OSF_R(crate::FieldReader<bool, bool>);
impl OSF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSF` writer - OSF"]
pub struct OSF_W<'a> {
    w: &'a mut W,
}
impl<'a> OSF_W<'a> {
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
#[doc = "Field `TSE` reader - TSE"]
pub struct TSE_R(crate::FieldReader<bool, bool>);
impl TSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSE` writer - TSE"]
pub struct TSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TXPBL` reader - TXPBL"]
pub struct TXPBL_R(crate::FieldReader<u8, u8>);
impl TXPBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXPBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPBL` writer - TXPBL"]
pub struct TXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `TQOS` reader - TQOS"]
pub struct TQOS_R(crate::FieldReader<u8, u8>);
impl TQOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TQOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TQOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TQOS` writer - TQOS"]
pub struct TQOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TQOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    pub fn tcw(&self) -> TCW_R {
        TCW_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    pub fn tqos(&self) -> TQOS_R {
        TQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ST"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bits 1:3 - TCW"]
    #[inline(always)]
    pub fn tcw(&mut self) -> TCW_W {
        TCW_W { w: self }
    }
    #[doc = "Bit 4 - OSF"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    #[doc = "Bit 12 - TSE"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bits 16:21 - TXPBL"]
    #[inline(always)]
    pub fn txpbl(&mut self) -> TXPBL_W {
        TXPBL_W { w: self }
    }
    #[doc = "Bits 24:27 - TQOS"]
    #[inline(always)]
    pub fn tqos(&mut self) -> TQOS_W {
        TQOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 0 transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0tx_cr](index.html) module"]
pub struct ETH_DMAC0TXCR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0TXCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0tx_cr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0TXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0tx_cr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0TXCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0TxCR to value 0"]
impl crate::Resettable for ETH_DMAC0TXCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
