#[doc = "Register `DMACTxCR` reader"]
pub struct R(crate::R<DMACTXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACTXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACTXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACTXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACTxCR` writer"]
pub struct W(crate::W<DMACTXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACTXCR_SPEC>;
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
impl From<crate::W<DMACTXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACTXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - Start or Stop Transmission Command"]
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
#[doc = "Field `ST` writer - Start or Stop Transmission Command"]
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
#[doc = "Field `OSF` reader - Operate on Second Packet"]
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
#[doc = "Field `OSF` writer - Operate on Second Packet"]
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
#[doc = "Field `TSE` reader - TCP Segmentation Enabled"]
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
#[doc = "Field `TSE` writer - TCP Segmentation Enabled"]
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
#[doc = "Field `TXPBL` reader - Transmit Programmable Burst Length"]
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
#[doc = "Field `TXPBL` writer - Transmit Programmable Burst Length"]
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
impl R {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&self) -> TXPBL_R {
        TXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Transmission Command"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bit 4 - Operate on Second Packet"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W {
        OSF_W { w: self }
    }
    #[doc = "Bit 12 - TCP Segmentation Enabled"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W {
        TSE_W { w: self }
    }
    #[doc = "Bits 16:21 - Transmit Programmable Burst Length"]
    #[inline(always)]
    pub fn txpbl(&mut self) -> TXPBL_W {
        TXPBL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel transmit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactx_cr](index.html) module"]
pub struct DMACTXCR_SPEC;
impl crate::RegisterSpec for DMACTXCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmactx_cr::R](R) reader structure"]
impl crate::Readable for DMACTXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmactx_cr::W](W) writer structure"]
impl crate::Writable for DMACTXCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACTxCR to value 0"]
impl crate::Resettable for DMACTXCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
