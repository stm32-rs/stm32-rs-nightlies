#[doc = "Register `ABFSR` reader"]
pub struct R(crate::R<ABFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABFSR` writer"]
pub struct W(crate::W<ABFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABFSR_SPEC>;
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
impl From<crate::W<ABFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITCM` reader - ITCM"]
pub struct ITCM_R(crate::FieldReader<bool, bool>);
impl ITCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITCM` writer - ITCM"]
pub struct ITCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITCM_W<'a> {
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
#[doc = "Field `DTCM` reader - DTCM"]
pub struct DTCM_R(crate::FieldReader<bool, bool>);
impl DTCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCM` writer - DTCM"]
pub struct DTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCM_W<'a> {
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
#[doc = "Field `AHBP` reader - AHBP"]
pub struct AHBP_R(crate::FieldReader<bool, bool>);
impl AHBP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBP` writer - AHBP"]
pub struct AHBP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `AXIM` reader - AXIM"]
pub struct AXIM_R(crate::FieldReader<bool, bool>);
impl AXIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIM` writer - AXIM"]
pub struct AXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIM_W<'a> {
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
#[doc = "Field `EPPB` reader - EPPB"]
pub struct EPPB_R(crate::FieldReader<bool, bool>);
impl EPPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPPB` writer - EPPB"]
pub struct EPPB_W<'a> {
    w: &'a mut W,
}
impl<'a> EPPB_W<'a> {
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
#[doc = "Field `AXIMTYPE` reader - AXIMTYPE"]
pub struct AXIMTYPE_R(crate::FieldReader<u8, u8>);
impl AXIMTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AXIMTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIMTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIMTYPE` writer - AXIMTYPE"]
pub struct AXIMTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ITCM"]
    #[inline(always)]
    pub fn itcm(&mut self) -> ITCM_W {
        ITCM_W { w: self }
    }
    #[doc = "Bit 1 - DTCM"]
    #[inline(always)]
    pub fn dtcm(&mut self) -> DTCM_W {
        DTCM_W { w: self }
    }
    #[doc = "Bit 2 - AHBP"]
    #[inline(always)]
    pub fn ahbp(&mut self) -> AHBP_W {
        AHBP_W { w: self }
    }
    #[doc = "Bit 3 - AXIM"]
    #[inline(always)]
    pub fn axim(&mut self) -> AXIM_W {
        AXIM_W { w: self }
    }
    #[doc = "Bit 4 - EPPB"]
    #[inline(always)]
    pub fn eppb(&mut self) -> EPPB_W {
        EPPB_W { w: self }
    }
    #[doc = "Bits 8:9 - AXIMTYPE"]
    #[inline(always)]
    pub fn aximtype(&mut self) -> AXIMTYPE_W {
        AXIMTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Bus Fault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abfsr](index.html) module"]
pub struct ABFSR_SPEC;
impl crate::RegisterSpec for ABFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abfsr::R](R) reader structure"]
impl crate::Readable for ABFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abfsr::W](W) writer structure"]
impl crate::Writable for ABFSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ABFSR to value 0"]
impl crate::Resettable for ABFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
