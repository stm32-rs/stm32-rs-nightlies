#[doc = "Register `DMAMR` reader"]
pub struct R(crate::R<DMAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMR` writer"]
pub struct W(crate::W<DMAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMR_SPEC>;
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
impl From<crate::W<DMAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTM` reader - Interrupt Mode"]
pub struct INTM_R(crate::FieldReader<u8, u8>);
impl INTM_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTM` writer - Interrupt Mode"]
pub struct INTM_W<'a> {
    w: &'a mut W,
}
impl<'a> INTM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PR` reader - Priority ratio"]
pub struct PR_R(crate::FieldReader<u8, u8>);
impl PR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` writer - Priority ratio"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `TXPR` reader - Transmit priority"]
pub struct TXPR_R(crate::FieldReader<bool, bool>);
impl TXPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPR` writer - Transmit priority"]
pub struct TXPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `DA` reader - DMA Tx or Rx Arbitration Scheme"]
pub struct DA_R(crate::FieldReader<bool, bool>);
impl DA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - DMA Tx or Rx Arbitration Scheme"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
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
#[doc = "Field `SWR` reader - Software Reset"]
pub struct SWR_R(crate::FieldReader<bool, bool>);
impl SWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWR` writer - Software Reset"]
pub struct SWR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWR_W<'a> {
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
    #[doc = "Bits 16:17 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Interrupt Mode"]
    #[inline(always)]
    pub fn intm(&mut self) -> INTM_W {
        INTM_W { w: self }
    }
    #[doc = "Bits 12:14 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Bit 11 - Transmit priority"]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W {
        TXPR_W { w: self }
    }
    #[doc = "Bit 1 - DMA Tx or Rx Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W {
        SWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamr](index.html) module"]
pub struct DMAMR_SPEC;
impl crate::RegisterSpec for DMAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamr::R](R) reader structure"]
impl crate::Readable for DMAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamr::W](W) writer structure"]
impl crate::Writable for DMAMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAMR to value 0"]
impl crate::Resettable for DMAMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
