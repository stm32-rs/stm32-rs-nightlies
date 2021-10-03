#[doc = "Register `WRPCR` reader"]
pub struct R(crate::R<WRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPCR` writer"]
pub struct W(crate::W<WRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPCR_SPEC>;
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
impl From<crate::W<WRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLEN` reader - PLLEN"]
pub struct PLLEN_R(crate::FieldReader<bool, bool>);
impl PLLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLEN` writer - PLLEN"]
pub struct PLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLEN_W<'a> {
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
#[doc = "Field `NDIV` reader - NDIV"]
pub struct NDIV_R(crate::FieldReader<u8, u8>);
impl NDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDIV` writer - NDIV"]
pub struct NDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 2)) | ((value as u32 & 0x7f) << 2);
        self.w
    }
}
#[doc = "Field `IDF` reader - IDF"]
pub struct IDF_R(crate::FieldReader<u8, u8>);
impl IDF_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDF` writer - IDF"]
pub struct IDF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `ODF` reader - ODF"]
pub struct ODF_R(crate::FieldReader<u8, u8>);
impl ODF_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODF` writer - ODF"]
pub struct ODF_W<'a> {
    w: &'a mut W,
}
impl<'a> ODF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `REGEN` reader - REGEN"]
pub struct REGEN_R(crate::FieldReader<bool, bool>);
impl REGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGEN` writer - REGEN"]
pub struct REGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `BGREN` reader - BGREN"]
pub struct BGREN_R(crate::FieldReader<bool, bool>);
impl BGREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGREN` writer - BGREN"]
pub struct BGREN_W<'a> {
    w: &'a mut W,
}
impl<'a> BGREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    pub fn bgren(&self) -> BGREN_R {
        BGREN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W { w: self }
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W {
        NDIV_W { w: self }
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&mut self) -> IDF_W {
        IDF_W { w: self }
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W {
        ODF_W { w: self }
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&mut self) -> REGEN_W {
        REGEN_W { w: self }
    }
    #[doc = "Bit 28 - BGREN"]
    #[inline(always)]
    pub fn bgren(&mut self) -> BGREN_W {
        BGREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper regulator and PLL control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrpcr](index.html) module"]
pub struct WRPCR_SPEC;
impl crate::RegisterSpec for WRPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrpcr::R](R) reader structure"]
impl crate::Readable for WRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrpcr::W](W) writer structure"]
impl crate::Writable for WRPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRPCR to value 0"]
impl crate::Resettable for WRPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
