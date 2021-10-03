#[doc = "Register `DLLCR` reader"]
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLLCR` writer"]
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub struct CALRTE_R(crate::FieldReader<u8, u8>);
impl CALRTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALRTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALRTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub struct CALRTE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub struct CALEN_R(crate::FieldReader<bool, bool>);
impl CALEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub struct CALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALEN_W<'a> {
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
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub struct CAL_R(crate::FieldReader<bool, bool>);
impl CAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
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
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&mut self) -> CALRTE_W {
        CALRTE_W { w: self }
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CALEN_W {
        CALEN_W { w: self }
    }
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](index.html) module"]
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllcr::R](R) reader structure"]
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dllcr::W](W) writer structure"]
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DLLCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
