#[doc = "Register `FLTINR4` reader"]
pub struct R(crate::R<FLTINR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR4` writer"]
pub struct W(crate::W<FLTINR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR4_SPEC>;
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
impl From<crate::W<FLTINR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT6RSTM` reader - FLT6RSTM"]
pub struct FLT6RSTM_R(crate::FieldReader<bool, bool>);
impl FLT6RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6RSTM` writer - FLT6RSTM"]
pub struct FLT6RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6RSTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `FLT6CRES` reader - FLT6CRES"]
pub struct FLT6CRES_R(crate::FieldReader<bool, bool>);
impl FLT6CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6CRES` writer - FLT6CRES"]
pub struct FLT6CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6CRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FLT6CNT` reader - FLT6CNT"]
pub struct FLT6CNT_R(crate::FieldReader<u8, u8>);
impl FLT6CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT6CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6CNT` writer - FLT6CNT"]
pub struct FLT6CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u32 & 0x0f) << 10);
        self.w
    }
}
#[doc = "Field `FLT6BLKS` reader - FLT6BLKS"]
pub struct FLT6BLKS_R(crate::FieldReader<bool, bool>);
impl FLT6BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6BLKS` writer - FLT6BLKS"]
pub struct FLT6BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6BLKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FLT6BLKE` reader - FLT6BLKE"]
pub struct FLT6BLKE_R(crate::FieldReader<bool, bool>);
impl FLT6BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6BLKE` writer - FLT6BLKE"]
pub struct FLT6BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6BLKE_W<'a> {
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
#[doc = "Field `FLT5RSTM` reader - FLT5RSTM"]
pub struct FLT5RSTM_R(crate::FieldReader<bool, bool>);
impl FLT5RSTM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5RSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5RSTM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5RSTM` writer - FLT5RSTM"]
pub struct FLT5RSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5RSTM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FLT5CRES` reader - FLT5CRES"]
pub struct FLT5CRES_R(crate::FieldReader<bool, bool>);
impl FLT5CRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5CRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5CRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5CRES` writer - FLT5CRES"]
pub struct FLT5CRES_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5CRES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `FLT5CNT` reader - FLT5CNT"]
pub struct FLT5CNT_R(crate::FieldReader<u8, u8>);
impl FLT5CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT5CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5CNT` writer - FLT5CNT"]
pub struct FLT5CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | ((value as u32 & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `FLT5BLKS` reader - FLT5BLKS"]
pub struct FLT5BLKS_R(crate::FieldReader<bool, bool>);
impl FLT5BLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5BLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5BLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5BLKS` writer - FLT5BLKS"]
pub struct FLT5BLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5BLKS_W<'a> {
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
#[doc = "Field `FLT5BLKE` reader - FLT5BLKE"]
pub struct FLT5BLKE_R(crate::FieldReader<bool, bool>);
impl FLT5BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5BLKE` writer - FLT5BLKE"]
pub struct FLT5BLKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5BLKE_W<'a> {
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
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    pub fn flt6rstm(&self) -> FLT6RSTM_R {
        FLT6RSTM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    pub fn flt6cres(&self) -> FLT6CRES_R {
        FLT6CRES_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    pub fn flt6cnt(&self) -> FLT6CNT_R {
        FLT6CNT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    pub fn flt6blks(&self) -> FLT6BLKS_R {
        FLT6BLKS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    pub fn flt6blke(&self) -> FLT6BLKE_R {
        FLT6BLKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    pub fn flt5rstm(&self) -> FLT5RSTM_R {
        FLT5RSTM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    pub fn flt5cres(&self) -> FLT5CRES_R {
        FLT5CRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    pub fn flt5cnt(&self) -> FLT5CNT_R {
        FLT5CNT_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    pub fn flt5blks(&self) -> FLT5BLKS_R {
        FLT5BLKS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    pub fn flt5blke(&self) -> FLT5BLKE_R {
        FLT5BLKE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - FLT6RSTM"]
    #[inline(always)]
    pub fn flt6rstm(&mut self) -> FLT6RSTM_W {
        FLT6RSTM_W { w: self }
    }
    #[doc = "Bit 14 - FLT6CRES"]
    #[inline(always)]
    pub fn flt6cres(&mut self) -> FLT6CRES_W {
        FLT6CRES_W { w: self }
    }
    #[doc = "Bits 10:13 - FLT6CNT"]
    #[inline(always)]
    pub fn flt6cnt(&mut self) -> FLT6CNT_W {
        FLT6CNT_W { w: self }
    }
    #[doc = "Bit 9 - FLT6BLKS"]
    #[inline(always)]
    pub fn flt6blks(&mut self) -> FLT6BLKS_W {
        FLT6BLKS_W { w: self }
    }
    #[doc = "Bit 8 - FLT6BLKE"]
    #[inline(always)]
    pub fn flt6blke(&mut self) -> FLT6BLKE_W {
        FLT6BLKE_W { w: self }
    }
    #[doc = "Bit 7 - FLT5RSTM"]
    #[inline(always)]
    pub fn flt5rstm(&mut self) -> FLT5RSTM_W {
        FLT5RSTM_W { w: self }
    }
    #[doc = "Bit 6 - FLT5CRES"]
    #[inline(always)]
    pub fn flt5cres(&mut self) -> FLT5CRES_W {
        FLT5CRES_W { w: self }
    }
    #[doc = "Bits 2:5 - FLT5CNT"]
    #[inline(always)]
    pub fn flt5cnt(&mut self) -> FLT5CNT_W {
        FLT5CNT_W { w: self }
    }
    #[doc = "Bit 1 - FLT5BLKS"]
    #[inline(always)]
    pub fn flt5blks(&mut self) -> FLT5BLKS_W {
        FLT5BLKS_W { w: self }
    }
    #[doc = "Bit 0 - FLT5BLKE"]
    #[inline(always)]
    pub fn flt5blke(&mut self) -> FLT5BLKE_W {
        FLT5BLKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr4](index.html) module"]
pub struct FLTINR4_SPEC;
impl crate::RegisterSpec for FLTINR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr4::R](R) reader structure"]
impl crate::Readable for FLTINR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr4::W](W) writer structure"]
impl crate::Writable for FLTINR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTINR4 to value 0"]
impl crate::Resettable for FLTINR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
