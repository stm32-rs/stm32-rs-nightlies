#[doc = "Register `PDMCR` reader"]
pub struct R(crate::R<PDMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMCR` writer"]
pub struct W(crate::W<PDMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMCR_SPEC>;
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
impl From<crate::W<PDMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDMEN` reader - PDM enable"]
pub struct PDMEN_R(crate::FieldReader<bool, bool>);
impl PDMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMEN` writer - PDM enable"]
pub struct PDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMEN_W<'a> {
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
#[doc = "Field `MICNBR` reader - MICNBR"]
pub struct MICNBR_R(crate::FieldReader<u8, u8>);
impl MICNBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        MICNBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICNBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICNBR` writer - MICNBR"]
pub struct MICNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> MICNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CKEN1` reader - Clock enable of bitstream clock number 1"]
pub struct CKEN1_R(crate::FieldReader<bool, bool>);
impl CKEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEN1` writer - Clock enable of bitstream clock number 1"]
pub struct CKEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CKEN2` reader - CKEN2"]
pub struct CKEN2_R(crate::FieldReader<bool, bool>);
impl CKEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEN2` writer - CKEN2"]
pub struct CKEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&mut self) -> PDMEN_W {
        PDMEN_W { w: self }
    }
    #[doc = "Bits 4:5 - MICNBR"]
    #[inline(always)]
    pub fn micnbr(&mut self) -> MICNBR_W {
        MICNBR_W { w: self }
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&mut self) -> CKEN1_W {
        CKEN1_W { w: self }
    }
    #[doc = "Bit 9 - CKEN2"]
    #[inline(always)]
    pub fn cken2(&mut self) -> CKEN2_W {
        CKEN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmcr](index.html) module"]
pub struct PDMCR_SPEC;
impl crate::RegisterSpec for PDMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmcr::R](R) reader structure"]
impl crate::Readable for PDMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmcr::W](W) writer structure"]
impl crate::Writable for PDMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMCR to value 0"]
impl crate::Resettable for PDMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
