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
#[doc = "Field `FLT6SRC_1` reader - FLT6SRC"]
pub struct FLT6SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT6SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6SRC_1` writer - FLT6SRC"]
pub struct FLT6SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `FLT5SRC_1` reader - FLT5SRC_1"]
pub struct FLT5SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT5SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT5SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT5SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT5SRC_1` writer - FLT5SRC_1"]
pub struct FLT5SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT5SRC_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `FLT4SRC_1` reader - FLT4SRC_1"]
pub struct FLT4SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT4SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4SRC_1` writer - FLT4SRC_1"]
pub struct FLT4SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4SRC_1_W<'a> {
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
#[doc = "Field `FLT3SRC_1` reader - FLT3SRC_1"]
pub struct FLT3SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT3SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3SRC_1` writer - FLT3SRC_1"]
pub struct FLT3SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3SRC_1_W<'a> {
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
#[doc = "Field `FLT2SRC_1` reader - FLT2SRC_1"]
pub struct FLT2SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT2SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2SRC_1` writer - FLT2SRC_1"]
pub struct FLT2SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2SRC_1_W<'a> {
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
#[doc = "Field `FLT1SRC_1` reader - FLT1SRC_1"]
pub struct FLT1SRC_1_R(crate::FieldReader<bool, bool>);
impl FLT1SRC_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1SRC_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1SRC_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1SRC_1` writer - FLT1SRC_1"]
pub struct FLT1SRC_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1SRC_1_W<'a> {
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
#[doc = "Field `FLT6LCK` reader - FLT6LCK"]
pub struct FLT6LCK_R(crate::FieldReader<bool, bool>);
impl FLT6LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6LCK` writer - FLT6LCK"]
pub struct FLT6LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6LCK_W<'a> {
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
#[doc = "Field `FLT6F` reader - FLT6F"]
pub struct FLT6F_R(crate::FieldReader<u8, u8>);
impl FLT6F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6F` writer - FLT6F"]
pub struct FLT6F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `FLT6SRC_0` reader - FLT6F"]
pub struct FLT6SRC_0_R(crate::FieldReader<bool, bool>);
impl FLT6SRC_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6SRC_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6SRC_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6SRC_0` writer - FLT6F"]
pub struct FLT6SRC_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6SRC_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FLT6P` reader - FLT6P"]
pub struct FLT6P_R(crate::FieldReader<bool, bool>);
impl FLT6P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6P` writer - FLT6P"]
pub struct FLT6P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6P_W<'a> {
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
#[doc = "Field `FLT6E` reader - FLT6E"]
pub struct FLT6E_R(crate::FieldReader<bool, bool>);
impl FLT6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT6E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT6E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT6E` writer - FLT6E"]
pub struct FLT6E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT6E_W<'a> {
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
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&self) -> FLT6SRC_1_R {
        FLT6SRC_1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&self) -> FLT5SRC_1_R {
        FLT5SRC_1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&self) -> FLT4SRC_1_R {
        FLT4SRC_1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&self) -> FLT3SRC_1_R {
        FLT3SRC_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&self) -> FLT2SRC_1_R {
        FLT2SRC_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&self) -> FLT1SRC_1_R {
        FLT1SRC_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&self) -> FLT6LCK_R {
        FLT6LCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&self) -> FLT6F_R {
        FLT6F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&self) -> FLT6SRC_0_R {
        FLT6SRC_0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&self) -> FLT6P_R {
        FLT6P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&self) -> FLT6E_R {
        FLT6E_R::new(((self.bits >> 8) & 0x01) != 0)
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
    #[doc = "Bit 21 - FLT6SRC"]
    #[inline(always)]
    pub fn flt6src_1(&mut self) -> FLT6SRC_1_W {
        FLT6SRC_1_W { w: self }
    }
    #[doc = "Bit 20 - FLT5SRC_1"]
    #[inline(always)]
    pub fn flt5src_1(&mut self) -> FLT5SRC_1_W {
        FLT5SRC_1_W { w: self }
    }
    #[doc = "Bit 19 - FLT4SRC_1"]
    #[inline(always)]
    pub fn flt4src_1(&mut self) -> FLT4SRC_1_W {
        FLT4SRC_1_W { w: self }
    }
    #[doc = "Bit 18 - FLT3SRC_1"]
    #[inline(always)]
    pub fn flt3src_1(&mut self) -> FLT3SRC_1_W {
        FLT3SRC_1_W { w: self }
    }
    #[doc = "Bit 17 - FLT2SRC_1"]
    #[inline(always)]
    pub fn flt2src_1(&mut self) -> FLT2SRC_1_W {
        FLT2SRC_1_W { w: self }
    }
    #[doc = "Bit 16 - FLT1SRC_1"]
    #[inline(always)]
    pub fn flt1src_1(&mut self) -> FLT1SRC_1_W {
        FLT1SRC_1_W { w: self }
    }
    #[doc = "Bit 15 - FLT6LCK"]
    #[inline(always)]
    pub fn flt6lck(&mut self) -> FLT6LCK_W {
        FLT6LCK_W { w: self }
    }
    #[doc = "Bits 11:14 - FLT6F"]
    #[inline(always)]
    pub fn flt6f(&mut self) -> FLT6F_W {
        FLT6F_W { w: self }
    }
    #[doc = "Bit 10 - FLT6F"]
    #[inline(always)]
    pub fn flt6src_0(&mut self) -> FLT6SRC_0_W {
        FLT6SRC_0_W { w: self }
    }
    #[doc = "Bit 9 - FLT6P"]
    #[inline(always)]
    pub fn flt6p(&mut self) -> FLT6P_W {
        FLT6P_W { w: self }
    }
    #[doc = "Bit 8 - FLT6E"]
    #[inline(always)]
    pub fn flt6e(&mut self) -> FLT6E_W {
        FLT6E_W { w: self }
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
