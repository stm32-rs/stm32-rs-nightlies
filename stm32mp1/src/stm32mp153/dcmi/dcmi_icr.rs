#[doc = "Register `DCMI_ICR` writer"]
pub struct W(crate::W<DCMI_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_ICR_SPEC>;
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
impl From<crate::W<DCMI_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_ISC` writer - FRAME_ISC"]
pub struct FRAME_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ISC_W<'a> {
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
#[doc = "Field `OVR_ISC` writer - OVR_ISC"]
pub struct OVR_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_ISC_W<'a> {
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
#[doc = "Field `ERR_ISC` writer - ERR_ISC"]
pub struct ERR_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_ISC_W<'a> {
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
#[doc = "Field `VSYNC_ISC` writer - VSYNC_ISC"]
pub struct VSYNC_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_ISC_W<'a> {
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
#[doc = "Field `LINE_ISC` writer - LINE_ISC"]
pub struct LINE_ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_ISC_W<'a> {
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
    #[doc = "Bit 0 - FRAME_ISC"]
    #[inline(always)]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W {
        FRAME_ISC_W { w: self }
    }
    #[doc = "Bit 1 - OVR_ISC"]
    #[inline(always)]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W {
        OVR_ISC_W { w: self }
    }
    #[doc = "Bit 2 - ERR_ISC"]
    #[inline(always)]
    pub fn err_isc(&mut self) -> ERR_ISC_W {
        ERR_ISC_W { w: self }
    }
    #[doc = "Bit 3 - VSYNC_ISC"]
    #[inline(always)]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W {
        VSYNC_ISC_W { w: self }
    }
    #[doc = "Bit 4 - LINE_ISC"]
    #[inline(always)]
    pub fn line_isc(&mut self) -> LINE_ISC_W {
        LINE_ISC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DCMI_ICR register is write-only.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_icr](index.html) module"]
pub struct DCMI_ICR_SPEC;
impl crate::RegisterSpec for DCMI_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcmi_icr::W](W) writer structure"]
impl crate::Writable for DCMI_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCMI_ICR to value 0"]
impl crate::Resettable for DCMI_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
