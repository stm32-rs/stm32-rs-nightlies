#[doc = "Register `FLTINR1` reader"]
pub struct R(crate::R<FLTINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTINR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTINR1` writer"]
pub struct W(crate::W<FLTINR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTINR1_SPEC>;
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
impl From<crate::W<FLTINR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTINR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT4LCK` reader - FLT4LCK"]
pub struct FLT4LCK_R(crate::FieldReader<bool, bool>);
impl FLT4LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4LCK` writer - FLT4LCK"]
pub struct FLT4LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4LCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `FLT4F` reader - FLT4F"]
pub struct FLT4F_R(crate::FieldReader<u8, u8>);
impl FLT4F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4F` writer - FLT4F"]
pub struct FLT4F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | ((value as u32 & 0x0f) << 27);
        self.w
    }
}
#[doc = "Field `FLT4SRC` reader - FLT4SRC"]
pub struct FLT4SRC_R(crate::FieldReader<bool, bool>);
impl FLT4SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4SRC` writer - FLT4SRC"]
pub struct FLT4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `FLT4P` reader - FLT4P"]
pub struct FLT4P_R(crate::FieldReader<bool, bool>);
impl FLT4P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4P` writer - FLT4P"]
pub struct FLT4P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `FLT4E` reader - FLT4E"]
pub struct FLT4E_R(crate::FieldReader<bool, bool>);
impl FLT4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT4E` writer - FLT4E"]
pub struct FLT4E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT4E_W<'a> {
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
#[doc = "Field `FLT3LCK` reader - FLT3LCK"]
pub struct FLT3LCK_R(crate::FieldReader<bool, bool>);
impl FLT3LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3LCK` writer - FLT3LCK"]
pub struct FLT3LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3LCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FLT3F` reader - FLT3F"]
pub struct FLT3F_R(crate::FieldReader<u8, u8>);
impl FLT3F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3F` writer - FLT3F"]
pub struct FLT3F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `FLT3SRC` reader - FLT3SRC"]
pub struct FLT3SRC_R(crate::FieldReader<bool, bool>);
impl FLT3SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3SRC` writer - FLT3SRC"]
pub struct FLT3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3SRC_W<'a> {
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
#[doc = "Field `FLT3P` reader - FLT3P"]
pub struct FLT3P_R(crate::FieldReader<bool, bool>);
impl FLT3P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3P` writer - FLT3P"]
pub struct FLT3P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3P_W<'a> {
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
#[doc = "Field `FLT3E` reader - FLT3E"]
pub struct FLT3E_R(crate::FieldReader<bool, bool>);
impl FLT3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT3E` writer - FLT3E"]
pub struct FLT3E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT3E_W<'a> {
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
#[doc = "Field `FLT2LCK` reader - FLT2LCK"]
pub struct FLT2LCK_R(crate::FieldReader<bool, bool>);
impl FLT2LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2LCK` writer - FLT2LCK"]
pub struct FLT2LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2LCK_W<'a> {
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
#[doc = "Field `FLT2F` reader - FLT2F"]
pub struct FLT2F_R(crate::FieldReader<u8, u8>);
impl FLT2F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2F` writer - FLT2F"]
pub struct FLT2F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | ((value as u32 & 0x0f) << 11);
        self.w
    }
}
#[doc = "Field `FLT2SRC` reader - FLT2SRC"]
pub struct FLT2SRC_R(crate::FieldReader<bool, bool>);
impl FLT2SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2SRC` writer - FLT2SRC"]
pub struct FLT2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2SRC_W<'a> {
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
#[doc = "Field `FLT2P` reader - FLT2P"]
pub struct FLT2P_R(crate::FieldReader<bool, bool>);
impl FLT2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2P` writer - FLT2P"]
pub struct FLT2P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2P_W<'a> {
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
#[doc = "Field `FLT2E` reader - FLT2E"]
pub struct FLT2E_R(crate::FieldReader<bool, bool>);
impl FLT2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT2E` writer - FLT2E"]
pub struct FLT2E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT2E_W<'a> {
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
#[doc = "Field `FLT1LCK` reader - FLT1LCK"]
pub struct FLT1LCK_R(crate::FieldReader<bool, bool>);
impl FLT1LCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1LCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1LCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1LCK` writer - FLT1LCK"]
pub struct FLT1LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1LCK_W<'a> {
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
#[doc = "Field `FLT1F` reader - FLT1F"]
pub struct FLT1F_R(crate::FieldReader<u8, u8>);
impl FLT1F_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLT1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1F_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1F` writer - FLT1F"]
pub struct FLT1F_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `FLT1SRC` reader - FLT1SRC"]
pub struct FLT1SRC_R(crate::FieldReader<bool, bool>);
impl FLT1SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1SRC` writer - FLT1SRC"]
pub struct FLT1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1SRC_W<'a> {
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
#[doc = "Field `FLT1P` reader - FLT1P"]
pub struct FLT1P_R(crate::FieldReader<bool, bool>);
impl FLT1P_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1P` writer - FLT1P"]
pub struct FLT1P_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1P_W<'a> {
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
#[doc = "Field `FLT1E` reader - FLT1E"]
pub struct FLT1E_R(crate::FieldReader<bool, bool>);
impl FLT1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLT1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT1E` writer - FLT1E"]
pub struct FLT1E_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT1E_W<'a> {
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
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&self) -> FLT4LCK_R {
        FLT4LCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&self) -> FLT4F_R {
        FLT4F_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&self) -> FLT4SRC_R {
        FLT4SRC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&self) -> FLT4P_R {
        FLT4P_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&self) -> FLT4E_R {
        FLT4E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&self) -> FLT3LCK_R {
        FLT3LCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&self) -> FLT3F_R {
        FLT3F_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&self) -> FLT3SRC_R {
        FLT3SRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&self) -> FLT3P_R {
        FLT3P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&self) -> FLT3E_R {
        FLT3E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&self) -> FLT2LCK_R {
        FLT2LCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&self) -> FLT2F_R {
        FLT2F_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&self) -> FLT2SRC_R {
        FLT2SRC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&self) -> FLT2P_R {
        FLT2P_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&self) -> FLT2E_R {
        FLT2E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&self) -> FLT1LCK_R {
        FLT1LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&self) -> FLT1F_R {
        FLT1F_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&self) -> FLT1SRC_R {
        FLT1SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&self) -> FLT1P_R {
        FLT1P_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&self) -> FLT1E_R {
        FLT1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - FLT4LCK"]
    #[inline(always)]
    pub fn flt4lck(&mut self) -> FLT4LCK_W {
        FLT4LCK_W { w: self }
    }
    #[doc = "Bits 27:30 - FLT4F"]
    #[inline(always)]
    pub fn flt4f(&mut self) -> FLT4F_W {
        FLT4F_W { w: self }
    }
    #[doc = "Bit 26 - FLT4SRC"]
    #[inline(always)]
    pub fn flt4src(&mut self) -> FLT4SRC_W {
        FLT4SRC_W { w: self }
    }
    #[doc = "Bit 25 - FLT4P"]
    #[inline(always)]
    pub fn flt4p(&mut self) -> FLT4P_W {
        FLT4P_W { w: self }
    }
    #[doc = "Bit 24 - FLT4E"]
    #[inline(always)]
    pub fn flt4e(&mut self) -> FLT4E_W {
        FLT4E_W { w: self }
    }
    #[doc = "Bit 23 - FLT3LCK"]
    #[inline(always)]
    pub fn flt3lck(&mut self) -> FLT3LCK_W {
        FLT3LCK_W { w: self }
    }
    #[doc = "Bits 19:22 - FLT3F"]
    #[inline(always)]
    pub fn flt3f(&mut self) -> FLT3F_W {
        FLT3F_W { w: self }
    }
    #[doc = "Bit 18 - FLT3SRC"]
    #[inline(always)]
    pub fn flt3src(&mut self) -> FLT3SRC_W {
        FLT3SRC_W { w: self }
    }
    #[doc = "Bit 17 - FLT3P"]
    #[inline(always)]
    pub fn flt3p(&mut self) -> FLT3P_W {
        FLT3P_W { w: self }
    }
    #[doc = "Bit 16 - FLT3E"]
    #[inline(always)]
    pub fn flt3e(&mut self) -> FLT3E_W {
        FLT3E_W { w: self }
    }
    #[doc = "Bit 15 - FLT2LCK"]
    #[inline(always)]
    pub fn flt2lck(&mut self) -> FLT2LCK_W {
        FLT2LCK_W { w: self }
    }
    #[doc = "Bits 11:14 - FLT2F"]
    #[inline(always)]
    pub fn flt2f(&mut self) -> FLT2F_W {
        FLT2F_W { w: self }
    }
    #[doc = "Bit 10 - FLT2SRC"]
    #[inline(always)]
    pub fn flt2src(&mut self) -> FLT2SRC_W {
        FLT2SRC_W { w: self }
    }
    #[doc = "Bit 9 - FLT2P"]
    #[inline(always)]
    pub fn flt2p(&mut self) -> FLT2P_W {
        FLT2P_W { w: self }
    }
    #[doc = "Bit 8 - FLT2E"]
    #[inline(always)]
    pub fn flt2e(&mut self) -> FLT2E_W {
        FLT2E_W { w: self }
    }
    #[doc = "Bit 7 - FLT1LCK"]
    #[inline(always)]
    pub fn flt1lck(&mut self) -> FLT1LCK_W {
        FLT1LCK_W { w: self }
    }
    #[doc = "Bits 3:6 - FLT1F"]
    #[inline(always)]
    pub fn flt1f(&mut self) -> FLT1F_W {
        FLT1F_W { w: self }
    }
    #[doc = "Bit 2 - FLT1SRC"]
    #[inline(always)]
    pub fn flt1src(&mut self) -> FLT1SRC_W {
        FLT1SRC_W { w: self }
    }
    #[doc = "Bit 1 - FLT1P"]
    #[inline(always)]
    pub fn flt1p(&mut self) -> FLT1P_W {
        FLT1P_W { w: self }
    }
    #[doc = "Bit 0 - FLT1E"]
    #[inline(always)]
    pub fn flt1e(&mut self) -> FLT1E_W {
        FLT1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM Fault Input Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr1](index.html) module"]
pub struct FLTINR1_SPEC;
impl crate::RegisterSpec for FLTINR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltinr1::R](R) reader structure"]
impl crate::Readable for FLTINR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltinr1::W](W) writer structure"]
impl crate::Writable for FLTINR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTINR1 to value 0"]
impl crate::Resettable for FLTINR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
