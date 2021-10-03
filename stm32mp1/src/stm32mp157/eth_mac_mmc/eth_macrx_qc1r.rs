#[doc = "Register `ETH_MACRxQC1R` reader"]
pub struct R(crate::R<ETH_MACRXQC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRXQC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRXQC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRXQC1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxQC1R` writer"]
pub struct W(crate::W<ETH_MACRXQC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRXQC1R_SPEC>;
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
impl From<crate::W<ETH_MACRXQC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRXQC1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVCPQ` reader - AVCPQ"]
pub struct AVCPQ_R(crate::FieldReader<u8, u8>);
impl AVCPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVCPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVCPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVCPQ` writer - AVCPQ"]
pub struct AVCPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVCPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `AVPTPQ` reader - AVPTPQ"]
pub struct AVPTPQ_R(crate::FieldReader<u8, u8>);
impl AVPTPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        AVPTPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVPTPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVPTPQ` writer - AVPTPQ"]
pub struct AVPTPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPTPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `UPQ` reader - UPQ"]
pub struct UPQ_R(crate::FieldReader<u8, u8>);
impl UPQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        UPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPQ` writer - UPQ"]
pub struct UPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> UPQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `MCBCQ` reader - MCBCQ"]
pub struct MCBCQ_R(crate::FieldReader<u8, u8>);
impl MCBCQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCBCQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCBCQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCBCQ` writer - MCBCQ"]
pub struct MCBCQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `MCBCQEN` reader - MCBCQEN"]
pub struct MCBCQEN_R(crate::FieldReader<bool, bool>);
impl MCBCQEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCBCQEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCBCQEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCBCQEN` writer - MCBCQEN"]
pub struct MCBCQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCBCQEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TACPQE` reader - TACPQE"]
pub struct TACPQE_R(crate::FieldReader<bool, bool>);
impl TACPQE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TACPQE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TACPQE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TACPQE` writer - TACPQE"]
pub struct TACPQE_W<'a> {
    w: &'a mut W,
}
impl<'a> TACPQE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    pub fn tacpqe(&self) -> TACPQE_R {
        TACPQE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    pub fn avcpq(&mut self) -> AVCPQ_W {
        AVCPQ_W { w: self }
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    pub fn avptpq(&mut self) -> AVPTPQ_W {
        AVPTPQ_W { w: self }
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    pub fn upq(&mut self) -> UPQ_W {
        UPQ_W { w: self }
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    pub fn mcbcq(&mut self) -> MCBCQ_W {
        MCBCQ_W { w: self }
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W {
        MCBCQEN_W { w: self }
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    pub fn tacpqe(&mut self) -> TACPQE_W {
        TACPQE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_qc1r](index.html) module"]
pub struct ETH_MACRXQC1R_SPEC;
impl crate::RegisterSpec for ETH_MACRXQC1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_qc1r::R](R) reader structure"]
impl crate::Readable for ETH_MACRXQC1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_qc1r::W](W) writer structure"]
impl crate::Writable for ETH_MACRXQC1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACRxQC1R to value 0"]
impl crate::Resettable for ETH_MACRXQC1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
