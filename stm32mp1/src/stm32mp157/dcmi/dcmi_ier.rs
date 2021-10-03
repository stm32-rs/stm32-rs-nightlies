#[doc = "Register `DCMI_IER` reader"]
pub struct R(crate::R<DCMI_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCMI_IER` writer"]
pub struct W(crate::W<DCMI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCMI_IER_SPEC>;
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
impl From<crate::W<DCMI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCMI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_IE` reader - FRAME_IE"]
pub struct FRAME_IE_R(crate::FieldReader<bool, bool>);
impl FRAME_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_IE` writer - FRAME_IE"]
pub struct FRAME_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_IE_W<'a> {
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
#[doc = "Field `OVR_IE` reader - OVR_IE"]
pub struct OVR_IE_R(crate::FieldReader<bool, bool>);
impl OVR_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_IE` writer - OVR_IE"]
pub struct OVR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_IE_W<'a> {
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
#[doc = "Field `ERR_IE` reader - ERR_IE"]
pub struct ERR_IE_R(crate::FieldReader<bool, bool>);
impl ERR_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_IE` writer - ERR_IE"]
pub struct ERR_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_IE_W<'a> {
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
#[doc = "Field `VSYNC_IE` reader - VSYNC_IE"]
pub struct VSYNC_IE_R(crate::FieldReader<bool, bool>);
impl VSYNC_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC_IE` writer - VSYNC_IE"]
pub struct VSYNC_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_IE_W<'a> {
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
#[doc = "Field `LINE_IE` reader - LINE_IE"]
pub struct LINE_IE_R(crate::FieldReader<bool, bool>);
impl LINE_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINE_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINE_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINE_IE` writer - LINE_IE"]
pub struct LINE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LINE_IE"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    pub fn frame_ie(&mut self) -> FRAME_IE_W {
        FRAME_IE_W { w: self }
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    pub fn ovr_ie(&mut self) -> OVR_IE_W {
        OVR_IE_W { w: self }
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&mut self) -> ERR_IE_W {
        ERR_IE_W { w: self }
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W {
        VSYNC_IE_W { w: self }
    }
    #[doc = "Bit 4 - LINE_IE"]
    #[inline(always)]
    pub fn line_ie(&mut self) -> LINE_IE_W {
        LINE_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ier](index.html) module"]
pub struct DCMI_IER_SPEC;
impl crate::RegisterSpec for DCMI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_ier::R](R) reader structure"]
impl crate::Readable for DCMI_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcmi_ier::W](W) writer structure"]
impl crate::Writable for DCMI_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCMI_IER to value 0"]
impl crate::Resettable for DCMI_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
