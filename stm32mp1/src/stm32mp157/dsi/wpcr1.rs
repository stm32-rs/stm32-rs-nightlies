#[doc = "Register `WPCR1` reader"]
pub struct R(crate::R<WPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR1` writer"]
pub struct W(crate::W<WPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR1_SPEC>;
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
impl From<crate::W<WPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKEWCL` reader - SKEWCL"]
pub struct SKEWCL_R(crate::FieldReader<u8, u8>);
impl SKEWCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SKEWCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKEWCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEWCL` writer - SKEWCL"]
pub struct SKEWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SKEWDL` reader - SKEWDL"]
pub struct SKEWDL_R(crate::FieldReader<u8, u8>);
impl SKEWDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SKEWDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKEWDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEWDL` writer - SKEWDL"]
pub struct SKEWDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEWDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `LPTXSRCL` reader - LPTXSRCL"]
pub struct LPTXSRCL_R(crate::FieldReader<u8, u8>);
impl LPTXSRCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTXSRCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTXSRCL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTXSRCL` writer - LPTXSRCL"]
pub struct LPTXSRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTXSRCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `LPTXSRDL` reader - LPTXSRDL"]
pub struct LPTXSRDL_R(crate::FieldReader<u8, u8>);
impl LPTXSRDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTXSRDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTXSRDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTXSRDL` writer - LPTXSRDL"]
pub struct LPTXSRDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTXSRDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SDDCCL` reader - SDDCCL"]
pub struct SDDCCL_R(crate::FieldReader<bool, bool>);
impl SDDCCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDDCCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDDCCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDDCCL` writer - SDDCCL"]
pub struct SDDCCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDCCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SDDCDL` reader - SDDCDL"]
pub struct SDDCDL_R(crate::FieldReader<bool, bool>);
impl SDDCDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDDCDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDDCDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDDCDL` writer - SDDCDL"]
pub struct SDDCDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDDCDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `HSTXSRUCL` reader - HSTXSRUCL"]
pub struct HSTXSRUCL_R(crate::FieldReader<bool, bool>);
impl HSTXSRUCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTXSRUCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRUCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRUCL` writer - HSTXSRUCL"]
pub struct HSTXSRUCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRUCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `HSTXSRDCL` reader - HSTXSRDCL"]
pub struct HSTXSRDCL_R(crate::FieldReader<bool, bool>);
impl HSTXSRDCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTXSRDCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRDCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRDCL` writer - HSTXSRDCL"]
pub struct HSTXSRDCL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRDCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `HSTXSRUDL` reader - HSTXSRUDL"]
pub struct HSTXSRUDL_R(crate::FieldReader<bool, bool>);
impl HSTXSRUDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTXSRUDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRUDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRUDL` writer - HSTXSRUDL"]
pub struct HSTXSRUDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRUDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `HSTXSRDDL` reader - HSTXSRDDL"]
pub struct HSTXSRDDL_R(crate::FieldReader<bool, bool>);
impl HSTXSRDDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSTXSRDDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTXSRDDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTXSRDDL` writer - HSTXSRDDL"]
pub struct HSTXSRDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTXSRDDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&self) -> SKEWCL_R {
        SKEWCL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&self) -> SKEWDL_R {
        SKEWDL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&self) -> LPTXSRCL_R {
        LPTXSRCL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&self) -> LPTXSRDL_R {
        LPTXSRDL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&self) -> SDDCCL_R {
        SDDCCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&self) -> SDDCDL_R {
        SDDCDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&self) -> HSTXSRUCL_R {
        HSTXSRUCL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&self) -> HSTXSRDCL_R {
        HSTXSRDCL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&self) -> HSTXSRUDL_R {
        HSTXSRUDL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&self) -> HSTXSRDDL_R {
        HSTXSRDDL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&mut self) -> SKEWCL_W {
        SKEWCL_W { w: self }
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&mut self) -> SKEWDL_W {
        SKEWDL_W { w: self }
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&mut self) -> LPTXSRCL_W {
        LPTXSRCL_W { w: self }
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&mut self) -> LPTXSRDL_W {
        LPTXSRDL_W { w: self }
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&mut self) -> SDDCCL_W {
        SDDCCL_W { w: self }
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&mut self) -> SDDCDL_W {
        SDDCDL_W { w: self }
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&mut self) -> HSTXSRUCL_W {
        HSTXSRUCL_W { w: self }
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&mut self) -> HSTXSRDCL_W {
        HSTXSRDCL_W { w: self }
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&mut self) -> HSTXSRUDL_W {
        HSTXSRUDL_W { w: self }
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&mut self) -> HSTXSRDDL_W {
        HSTXSRDDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr1](index.html) module"]
pub struct WPCR1_SPEC;
impl crate::RegisterSpec for WPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr1::R](R) reader structure"]
impl crate::Readable for WPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr1::W](W) writer structure"]
impl crate::Writable for WPCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
