#[doc = "Register `CONFR1` reader"]
pub struct R(crate::R<CONFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFR1` writer"]
pub struct W(crate::W<CONFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR1_SPEC>;
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
impl From<crate::W<CONFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NF` reader - Number of color components This field defines the number of color components minus 1."]
pub struct NF_R(crate::FieldReader<u8, u8>);
impl NF_R {
    pub(crate) fn new(bits: u8) -> Self {
        NF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NF` writer - Number of color components This field defines the number of color components minus 1."]
pub struct NF_W<'a> {
    w: &'a mut W,
}
impl<'a> NF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DE` reader - Decoding Enable This bit selects the coding or decoding process"]
pub struct DE_R(crate::FieldReader<bool, bool>);
impl DE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DE` writer - Decoding Enable This bit selects the coding or decoding process"]
pub struct DE_W<'a> {
    w: &'a mut W,
}
impl<'a> DE_W<'a> {
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
#[doc = "Field `COLORSPACE` reader - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub struct COLORSPACE_R(crate::FieldReader<u8, u8>);
impl COLORSPACE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COLORSPACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLORSPACE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLORSPACE` writer - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
pub struct COLORSPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> COLORSPACE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `NS` reader - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub struct NS_R(crate::FieldReader<u8, u8>);
impl NS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NS` writer - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
pub struct NS_W<'a> {
    w: &'a mut W,
}
impl<'a> NS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `HDR` reader - Header Processing This bit enable the header processing (generation/parsing)."]
pub struct HDR_R(crate::FieldReader<bool, bool>);
impl HDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDR` writer - Header Processing This bit enable the header processing (generation/parsing)."]
pub struct HDR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDR_W<'a> {
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
#[doc = "Field `YSIZE` reader - Y Size This field defines the number of lines in source image."]
pub struct YSIZE_R(crate::FieldReader<u16, u16>);
impl YSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        YSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YSIZE` writer - Y Size This field defines the number of lines in source image."]
pub struct YSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> YSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    pub fn colorspace(&self) -> COLORSPACE_R {
        COLORSPACE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    pub fn ysize(&self) -> YSIZE_R {
        YSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of color components This field defines the number of color components minus 1."]
    #[inline(always)]
    pub fn nf(&mut self) -> NF_W {
        NF_W { w: self }
    }
    #[doc = "Bit 3 - Decoding Enable This bit selects the coding or decoding process"]
    #[inline(always)]
    pub fn de(&mut self) -> DE_W {
        DE_W { w: self }
    }
    #[doc = "Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream."]
    #[inline(always)]
    pub fn colorspace(&mut self) -> COLORSPACE_W {
        COLORSPACE_W { w: self }
    }
    #[doc = "Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment."]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W {
        NS_W { w: self }
    }
    #[doc = "Bit 8 - Header Processing This bit enable the header processing (generation/parsing)."]
    #[inline(always)]
    pub fn hdr(&mut self) -> HDR_W {
        HDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Y Size This field defines the number of lines in source image."]
    #[inline(always)]
    pub fn ysize(&mut self) -> YSIZE_W {
        YSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [confr1](index.html) module"]
pub struct CONFR1_SPEC;
impl crate::RegisterSpec for CONFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [confr1::R](R) reader structure"]
impl crate::Readable for CONFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [confr1::W](W) writer structure"]
impl crate::Writable for CONFR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFR1 to value 0"]
impl crate::Resettable for CONFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
