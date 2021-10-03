#[doc = "Register `SPDIFRX_IFCR` reader"]
pub struct R(crate::R<SPDIFRX_IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_IFCR` writer"]
pub struct W(crate::W<SPDIFRX_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_IFCR_SPEC>;
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
impl From<crate::W<SPDIFRX_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERRCF` writer - PERRCF"]
pub struct PERRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRCF_W<'a> {
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
#[doc = "Field `OVRCF` writer - OVRCF"]
pub struct OVRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRCF_W<'a> {
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
#[doc = "Field `SBDCF` writer - SBDCF"]
pub struct SBDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SBDCF_W<'a> {
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
#[doc = "Field `SYNCDCF` writer - SYNCDCF"]
pub struct SYNCDCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCDCF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - PERRCF"]
    #[inline(always)]
    pub fn perrcf(&mut self) -> PERRCF_W {
        PERRCF_W { w: self }
    }
    #[doc = "Bit 3 - OVRCF"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W {
        OVRCF_W { w: self }
    }
    #[doc = "Bit 4 - SBDCF"]
    #[inline(always)]
    pub fn sbdcf(&mut self) -> SBDCF_W {
        SBDCF_W { w: self }
    }
    #[doc = "Bit 5 - SYNCDCF"]
    #[inline(always)]
    pub fn syncdcf(&mut self) -> SYNCDCF_W {
        SYNCDCF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_ifcr](index.html) module"]
pub struct SPDIFRX_IFCR_SPEC;
impl crate::RegisterSpec for SPDIFRX_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_ifcr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_ifcr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPDIFRX_IFCR to value 0"]
impl crate::Resettable for SPDIFRX_IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
