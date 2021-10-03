#[doc = "Register `QUADSPI_FCR` writer"]
pub struct W(crate::W<QUADSPI_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_FCR_SPEC>;
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
impl From<crate::W<QUADSPI_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEF` writer - CTEF"]
pub struct CTEF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEF_W<'a> {
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
#[doc = "Field `CTCF` writer - CTCF"]
pub struct CTCF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCF_W<'a> {
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
#[doc = "Field `CSMF` writer - CSMF"]
pub struct CSMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMF_W<'a> {
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
#[doc = "Field `CTOF` writer - CTOF"]
pub struct CTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTOF_W<'a> {
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
impl W {
    #[doc = "Bit 0 - CTEF"]
    #[inline(always)]
    pub fn ctef(&mut self) -> CTEF_W {
        CTEF_W { w: self }
    }
    #[doc = "Bit 1 - CTCF"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CTCF_W {
        CTCF_W { w: self }
    }
    #[doc = "Bit 3 - CSMF"]
    #[inline(always)]
    pub fn csmf(&mut self) -> CSMF_W {
        CSMF_W { w: self }
    }
    #[doc = "Bit 4 - CTOF"]
    #[inline(always)]
    pub fn ctof(&mut self) -> CTOF_W {
        CTOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_fcr](index.html) module"]
pub struct QUADSPI_FCR_SPEC;
impl crate::RegisterSpec for QUADSPI_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [quadspi_fcr::W](W) writer structure"]
impl crate::Writable for QUADSPI_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUADSPI_FCR to value 0"]
impl crate::Resettable for QUADSPI_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
