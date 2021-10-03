#[doc = "Register `JPEG_CONFR7` reader"]
pub struct R(crate::R<JPEG_CONFR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JPEG_CONFR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JPEG_CONFR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JPEG_CONFR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JPEG_CONFR7` writer"]
pub struct W(crate::W<JPEG_CONFR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JPEG_CONFR7_SPEC>;
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
impl From<crate::W<JPEG_CONFR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JPEG_CONFR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HD` reader - Huffman DC"]
pub struct HD_R(crate::FieldReader<bool, bool>);
impl HD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HD` writer - Huffman DC"]
pub struct HD_W<'a> {
    w: &'a mut W,
}
impl<'a> HD_W<'a> {
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
#[doc = "Field `HA` reader - Huffman AC"]
pub struct HA_R(crate::FieldReader<bool, bool>);
impl HA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HA` writer - Huffman AC"]
pub struct HA_W<'a> {
    w: &'a mut W,
}
impl<'a> HA_W<'a> {
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
#[doc = "Field `QT` reader - Quantization Table"]
pub struct QT_R(crate::FieldReader<u8, u8>);
impl QT_R {
    pub(crate) fn new(bits: u8) -> Self {
        QT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QT` writer - Quantization Table"]
pub struct QT_W<'a> {
    w: &'a mut W,
}
impl<'a> QT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `NB` reader - Number of Block"]
pub struct NB_R(crate::FieldReader<u8, u8>);
impl NB_R {
    pub(crate) fn new(bits: u8) -> Self {
        NB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NB` writer - Number of Block"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `VSF` reader - Vertical Sampling Factor"]
pub struct VSF_R(crate::FieldReader<u8, u8>);
impl VSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        VSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSF` writer - Vertical Sampling Factor"]
pub struct VSF_W<'a> {
    w: &'a mut W,
}
impl<'a> VSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `HSF` reader - Horizontal Sampling Factor"]
pub struct HSF_R(crate::FieldReader<u8, u8>);
impl HSF_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSF` writer - Horizontal Sampling Factor"]
pub struct HSF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Huffman DC"]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Huffman AC"]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Quantization Table"]
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Number of Block"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor"]
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor"]
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman DC"]
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W {
        HD_W { w: self }
    }
    #[doc = "Bit 1 - Huffman AC"]
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W {
        HA_W { w: self }
    }
    #[doc = "Bits 2:3 - Quantization Table"]
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W {
        QT_W { w: self }
    }
    #[doc = "Bits 4:7 - Number of Block"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor"]
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W {
        VSF_W { w: self }
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor"]
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W {
        HSF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG codec configuration register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_confr7](index.html) module"]
pub struct JPEG_CONFR7_SPEC;
impl crate::RegisterSpec for JPEG_CONFR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jpeg_confr7::R](R) reader structure"]
impl crate::Readable for JPEG_CONFR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jpeg_confr7::W](W) writer structure"]
impl crate::Writable for JPEG_CONFR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets JPEG_CONFR7 to value 0"]
impl crate::Resettable for JPEG_CONFR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
