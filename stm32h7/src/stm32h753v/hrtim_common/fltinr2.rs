#[doc = "Register `FLTINR2` reader"]
pub struct R(crate::R<FLTINR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR2` writer"]
pub struct W(crate::W<FLTINR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR2_SPEC>;
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
impl From<crate::W<FLTINR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLTSD` reader - FLTSD"]
pub struct FLTSD_R(crate::FieldReader<u8, u8>);
impl FLTSD_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLTSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTSD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTSD` writer - FLTSD"]
pub struct FLTSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTSD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub struct FLT5LCK_R(crate::FieldReader<bool, bool>);
impl FLT5LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub struct FLT5LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5LCK_W<'a> {
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
#[doc = "Field `FLT5F` reader - FLT5F"]
pub struct FLT5F_R(crate::FieldReader<u8, u8>);
impl FLT5F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5F` writer - FLT5F"]
pub struct FLT5F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub struct FLT5SRC_R(crate::FieldReader<bool, bool>);
impl FLT5SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub struct FLT5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5SRC_W<'a> {
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
#[doc = "Field `FLT5P` reader - FLT5P"]
pub struct FLT5P_R(crate::FieldReader<bool, bool>);
impl FLT5P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5P` writer - FLT5P"]
pub struct FLT5P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5P_W<'a> {
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
#[doc = "Field `FLT5E` reader - FLT5E"]
pub struct FLT5E_R(crate::FieldReader<bool, bool>);
impl FLT5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5E` writer - FLT5E"]
pub struct FLT5E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5E_W<'a> {
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
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FLTSD_R {
        FLTSD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> FLT5LCK_R {
        FLT5LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> FLT5F_R {
        FLT5F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> FLT5SRC_R {
        FLT5SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> FLT5P_R {
        FLT5P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> FLT5E_R {
        FLT5E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&mut self) -> FLTSD_W {
        FLTSD_W { w: self }
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&mut self) -> FLT5LCK_W {
        FLT5LCK_W { w: self }
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&mut self) -> FLT5F_W {
        FLT5F_W { w: self }
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&mut self) -> FLT5SRC_W {
        FLT5SRC_W { w: self }
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&mut self) -> FLT5P_W {
        FLT5P_W { w: self }
    }
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&mut self) -> FLT5E_W {
        FLT5E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr2](index.html) module"]
pub struct FLTINR2_SPEC;
impl crate::RegisterSpec for FLTINR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr2::R](R) reader structure"]
impl crate::Readable for FLTINR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr2::W](W) writer structure"]
impl crate::Writable for FLTINR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for FLTINR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
