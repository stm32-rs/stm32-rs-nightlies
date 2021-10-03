#[doc = "Register `EECR1` reader"]
pub struct R(crate::R<EECR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR1` writer"]
pub struct W(crate::W<EECR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR1_SPEC>;
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
impl From<crate::W<EECR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE5FAST` reader - External Event 5 Fast mode"]
pub struct EE5FAST_R(crate::FieldReader<bool, bool>);
impl EE5FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE5FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5FAST` writer - External Event 5 Fast mode"]
pub struct EE5FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `EE5SNS` reader - External Event 5 Sensitivity"]
pub struct EE5SNS_R(crate::FieldReader<u8, u8>);
impl EE5SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE5SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5SNS` writer - External Event 5 Sensitivity"]
pub struct EE5SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `EE5POL` reader - External Event 5 Polarity"]
pub struct EE5POL_R(crate::FieldReader<bool, bool>);
impl EE5POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE5POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5POL` writer - External Event 5 Polarity"]
pub struct EE5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5POL_W<'a> {
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
#[doc = "Field `EE5SRC` reader - External Event 5 Source"]
pub struct EE5SRC_R(crate::FieldReader<u8, u8>);
impl EE5SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE5SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5SRC` writer - External Event 5 Source"]
pub struct EE5SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `EE4FAST` reader - External Event 4 Fast mode"]
pub struct EE4FAST_R(crate::FieldReader<bool, bool>);
impl EE4FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE4FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4FAST` writer - External Event 4 Fast mode"]
pub struct EE4FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4FAST_W<'a> {
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
#[doc = "Field `EE4SNS` reader - External Event 4 Sensitivity"]
pub struct EE4SNS_R(crate::FieldReader<u8, u8>);
impl EE4SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE4SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4SNS` writer - External Event 4 Sensitivity"]
pub struct EE4SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `EE4POL` reader - External Event 4 Polarity"]
pub struct EE4POL_R(crate::FieldReader<bool, bool>);
impl EE4POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE4POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4POL` writer - External Event 4 Polarity"]
pub struct EE4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4POL_W<'a> {
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
#[doc = "Field `EE4SRC` reader - External Event 4 Source"]
pub struct EE4SRC_R(crate::FieldReader<u8, u8>);
impl EE4SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE4SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4SRC` writer - External Event 4 Source"]
pub struct EE4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `EE3FAST` reader - External Event 3 Fast mode"]
pub struct EE3FAST_R(crate::FieldReader<bool, bool>);
impl EE3FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE3FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3FAST` writer - External Event 3 Fast mode"]
pub struct EE3FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3FAST_W<'a> {
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
#[doc = "Field `EE3SNS` reader - External Event 3 Sensitivity"]
pub struct EE3SNS_R(crate::FieldReader<u8, u8>);
impl EE3SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE3SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3SNS` writer - External Event 3 Sensitivity"]
pub struct EE3SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `EE3POL` reader - External Event 3 Polarity"]
pub struct EE3POL_R(crate::FieldReader<bool, bool>);
impl EE3POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE3POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3POL` writer - External Event 3 Polarity"]
pub struct EE3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3POL_W<'a> {
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
#[doc = "Field `EE3SRC` reader - External Event 3 Source"]
pub struct EE3SRC_R(crate::FieldReader<u8, u8>);
impl EE3SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE3SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3SRC` writer - External Event 3 Source"]
pub struct EE3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `EE2FAST` reader - External Event 2 Fast mode"]
pub struct EE2FAST_R(crate::FieldReader<bool, bool>);
impl EE2FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE2FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2FAST` writer - External Event 2 Fast mode"]
pub struct EE2FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EE2SNS` reader - External Event 2 Sensitivity"]
pub struct EE2SNS_R(crate::FieldReader<u8, u8>);
impl EE2SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE2SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2SNS` writer - External Event 2 Sensitivity"]
pub struct EE2SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `EE2POL` reader - External Event 2 Polarity"]
pub struct EE2POL_R(crate::FieldReader<bool, bool>);
impl EE2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE2POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2POL` writer - External Event 2 Polarity"]
pub struct EE2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2POL_W<'a> {
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
#[doc = "Field `EE2SRC` reader - External Event 2 Source"]
pub struct EE2SRC_R(crate::FieldReader<u8, u8>);
impl EE2SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE2SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2SRC` writer - External Event 2 Source"]
pub struct EE2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `EE1FAST` reader - External Event 1 Fast mode"]
pub struct EE1FAST_R(crate::FieldReader<bool, bool>);
impl EE1FAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE1FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1FAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1FAST` writer - External Event 1 Fast mode"]
pub struct EE1FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EE1SNS` reader - External Event 1 Sensitivity"]
pub struct EE1SNS_R(crate::FieldReader<u8, u8>);
impl EE1SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE1SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1SNS` writer - External Event 1 Sensitivity"]
pub struct EE1SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `EE1POL` reader - External Event 1 Polarity"]
pub struct EE1POL_R(crate::FieldReader<bool, bool>);
impl EE1POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE1POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1POL` writer - External Event 1 Polarity"]
pub struct EE1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1POL_W<'a> {
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
#[doc = "Field `EE1SRC` reader - External Event 1 Source"]
pub struct EE1SRC_R(crate::FieldReader<u8, u8>);
impl EE1SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE1SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1SRC` writer - External Event 1 Source"]
pub struct EE1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&self) -> EE5FAST_R {
        EE5FAST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&self) -> EE5SNS_R {
        EE5SNS_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&self) -> EE5POL_R {
        EE5POL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&self) -> EE5SRC_R {
        EE5SRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&self) -> EE4FAST_R {
        EE4FAST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&self) -> EE4SNS_R {
        EE4SNS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&self) -> EE4POL_R {
        EE4POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&self) -> EE4SRC_R {
        EE4SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&self) -> EE3FAST_R {
        EE3FAST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&self) -> EE3SNS_R {
        EE3SNS_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&self) -> EE3POL_R {
        EE3POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&self) -> EE3SRC_R {
        EE3SRC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&self) -> EE2FAST_R {
        EE2FAST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&self) -> EE2SNS_R {
        EE2SNS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&self) -> EE2POL_R {
        EE2POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&self) -> EE2SRC_R {
        EE2SRC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&self) -> EE1FAST_R {
        EE1FAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&self) -> EE1SNS_R {
        EE1SNS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&self) -> EE1POL_R {
        EE1POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&self) -> EE1SRC_R {
        EE1SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&mut self) -> EE5FAST_W {
        EE5FAST_W { w: self }
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&mut self) -> EE5SNS_W {
        EE5SNS_W { w: self }
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&mut self) -> EE5POL_W {
        EE5POL_W { w: self }
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&mut self) -> EE5SRC_W {
        EE5SRC_W { w: self }
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&mut self) -> EE4FAST_W {
        EE4FAST_W { w: self }
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&mut self) -> EE4SNS_W {
        EE4SNS_W { w: self }
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&mut self) -> EE4POL_W {
        EE4POL_W { w: self }
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&mut self) -> EE4SRC_W {
        EE4SRC_W { w: self }
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&mut self) -> EE3FAST_W {
        EE3FAST_W { w: self }
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&mut self) -> EE3SNS_W {
        EE3SNS_W { w: self }
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&mut self) -> EE3POL_W {
        EE3POL_W { w: self }
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&mut self) -> EE3SRC_W {
        EE3SRC_W { w: self }
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&mut self) -> EE2FAST_W {
        EE2FAST_W { w: self }
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&mut self) -> EE2SNS_W {
        EE2SNS_W { w: self }
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&mut self) -> EE2POL_W {
        EE2POL_W { w: self }
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&mut self) -> EE2SRC_W {
        EE2SRC_W { w: self }
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&mut self) -> EE1FAST_W {
        EE1FAST_W { w: self }
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&mut self) -> EE1SNS_W {
        EE1SNS_W { w: self }
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&mut self) -> EE1POL_W {
        EE1POL_W { w: self }
    }
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&mut self) -> EE1SRC_W {
        EE1SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr1](index.html) module"]
pub struct EECR1_SPEC;
impl crate::RegisterSpec for EECR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr1::R](R) reader structure"]
impl crate::Readable for EECR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr1::W](W) writer structure"]
impl crate::Writable for EECR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR1 to value 0"]
impl crate::Resettable for EECR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
