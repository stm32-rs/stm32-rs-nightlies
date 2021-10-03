#[doc = "Register `QUADSPI_DCR` reader"]
pub struct R(crate::R<QUADSPI_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_DCR` writer"]
pub struct W(crate::W<QUADSPI_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_DCR_SPEC>;
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
impl From<crate::W<QUADSPI_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKMODE` reader - CKMODE"]
pub struct CKMODE_R(crate::FieldReader<bool, bool>);
impl CKMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - CKMODE"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
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
#[doc = "Field `CSHT` reader - CSHT"]
pub struct CSHT_R(crate::FieldReader<u8, u8>);
impl CSHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSHT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSHT` writer - CSHT"]
pub struct CSHT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `FSIZE` reader - FSIZE"]
pub struct FSIZE_R(crate::FieldReader<u8, u8>);
impl FSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSIZE` writer - FSIZE"]
pub struct FSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    pub fn fsize(&self) -> FSIZE_R {
        FSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Bits 8:10 - CSHT"]
    #[inline(always)]
    pub fn csht(&mut self) -> CSHT_W {
        CSHT_W { w: self }
    }
    #[doc = "Bits 16:20 - FSIZE"]
    #[inline(always)]
    pub fn fsize(&mut self) -> FSIZE_W {
        FSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI device configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_dcr](index.html) module"]
pub struct QUADSPI_DCR_SPEC;
impl crate::RegisterSpec for QUADSPI_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_dcr::R](R) reader structure"]
impl crate::Readable for QUADSPI_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_dcr::W](W) writer structure"]
impl crate::Writable for QUADSPI_DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUADSPI_DCR to value 0"]
impl crate::Resettable for QUADSPI_DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
