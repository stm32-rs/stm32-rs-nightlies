#[doc = "Register `CRRCR` reader"]
pub struct R(crate::R<CRRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRRCR` writer"]
pub struct W(crate::W<CRRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRRCR_SPEC>;
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
impl From<crate::W<CRRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSI48CAL` reader - 48 MHz HSI clock calibration"]
pub struct HSI48CAL_R(crate::FieldReader<u8, u8>);
impl HSI48CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI48CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48RDY` reader - 48MHz HSI clock ready flag"]
pub struct HSI48RDY_R(crate::FieldReader<bool, bool>);
impl HSI48RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48ON` reader - 48MHz HSI clock enable bit"]
pub struct HSI48ON_R(crate::FieldReader<bool, bool>);
impl HSI48ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48ON` writer - 48MHz HSI clock enable bit"]
pub struct HSI48ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48ON_W<'a> {
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
#[doc = "Field `HSI48DIV6EN` reader - 48 MHz HSI clock divided by 6 output enable"]
pub struct HSI48DIV6EN_R(crate::FieldReader<bool, bool>);
impl HSI48DIV6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI48DIV6EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI48DIV6EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI48DIV6EN` writer - 48 MHz HSI clock divided by 6 output enable"]
pub struct HSI48DIV6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI48DIV6EN_W<'a> {
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
impl R {
    #[doc = "Bits 8:15 - 48 MHz HSI clock calibration"]
    #[inline(always)]
    pub fn hsi48cal(&self) -> HSI48CAL_R {
        HSI48CAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 1 - 48MHz HSI clock ready flag"]
    #[inline(always)]
    pub fn hsi48rdy(&self) -> HSI48RDY_R {
        HSI48RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    pub fn hsi48on(&self) -> HSI48ON_R {
        HSI48ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    pub fn hsi48div6en(&self) -> HSI48DIV6EN_R {
        HSI48DIV6EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 48MHz HSI clock enable bit"]
    #[inline(always)]
    pub fn hsi48on(&mut self) -> HSI48ON_W {
        HSI48ON_W { w: self }
    }
    #[doc = "Bit 2 - 48 MHz HSI clock divided by 6 output enable"]
    #[inline(always)]
    pub fn hsi48div6en(&mut self) -> HSI48DIV6EN_W {
        HSI48DIV6EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock recovery RC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crrcr](index.html) module"]
pub struct CRRCR_SPEC;
impl crate::RegisterSpec for CRRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crrcr::R](R) reader structure"]
impl crate::Readable for CRRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crrcr::W](W) writer structure"]
impl crate::Writable for CRRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CRRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
