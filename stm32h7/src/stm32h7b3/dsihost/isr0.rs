#[doc = "Register `ISR0` reader"]
pub struct R(crate::R<ISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR0` writer"]
pub struct W(crate::W<ISR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR0_SPEC>;
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
impl From<crate::W<ISR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE4` reader - PHY error 4"]
pub struct PE4_R(crate::FieldReader<bool, bool>);
impl PE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE4` writer - PHY error 4"]
pub struct PE4_W<'a> {
    w: &'a mut W,
}
impl<'a> PE4_W<'a> {
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
#[doc = "Field `PE3` reader - PHY error 3"]
pub struct PE3_R(crate::FieldReader<bool, bool>);
impl PE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE3` writer - PHY error 3"]
pub struct PE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PE3_W<'a> {
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
#[doc = "Field `PE2` reader - PHY error 2"]
pub struct PE2_R(crate::FieldReader<bool, bool>);
impl PE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE2` writer - PHY error 2"]
pub struct PE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PE2_W<'a> {
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
#[doc = "Field `PE1` reader - PHY error 1"]
pub struct PE1_R(crate::FieldReader<bool, bool>);
impl PE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE1` writer - PHY error 1"]
pub struct PE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PE1_W<'a> {
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
#[doc = "Field `PE0` reader - PHY error 0"]
pub struct PE0_R(crate::FieldReader<bool, bool>);
impl PE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE0` writer - PHY error 0"]
pub struct PE0_W<'a> {
    w: &'a mut W,
}
impl<'a> PE0_W<'a> {
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
#[doc = "Field `AE15` reader - Acknowledge error 15"]
pub struct AE15_R(crate::FieldReader<bool, bool>);
impl AE15_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE15` writer - Acknowledge error 15"]
pub struct AE15_W<'a> {
    w: &'a mut W,
}
impl<'a> AE15_W<'a> {
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
#[doc = "Field `AE14` reader - Acknowledge error 14"]
pub struct AE14_R(crate::FieldReader<bool, bool>);
impl AE14_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE14` writer - Acknowledge error 14"]
pub struct AE14_W<'a> {
    w: &'a mut W,
}
impl<'a> AE14_W<'a> {
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
#[doc = "Field `AE13` reader - Acknowledge error 13"]
pub struct AE13_R(crate::FieldReader<bool, bool>);
impl AE13_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE13` writer - Acknowledge error 13"]
pub struct AE13_W<'a> {
    w: &'a mut W,
}
impl<'a> AE13_W<'a> {
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
#[doc = "Field `AE12` reader - Acknowledge error 12"]
pub struct AE12_R(crate::FieldReader<bool, bool>);
impl AE12_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE12` writer - Acknowledge error 12"]
pub struct AE12_W<'a> {
    w: &'a mut W,
}
impl<'a> AE12_W<'a> {
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
#[doc = "Field `AE11` reader - Acknowledge error 11"]
pub struct AE11_R(crate::FieldReader<bool, bool>);
impl AE11_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE11` writer - Acknowledge error 11"]
pub struct AE11_W<'a> {
    w: &'a mut W,
}
impl<'a> AE11_W<'a> {
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
#[doc = "Field `AE10` reader - Acknowledge error 10"]
pub struct AE10_R(crate::FieldReader<bool, bool>);
impl AE10_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE10` writer - Acknowledge error 10"]
pub struct AE10_W<'a> {
    w: &'a mut W,
}
impl<'a> AE10_W<'a> {
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
#[doc = "Field `AE9` reader - Acknowledge error 9"]
pub struct AE9_R(crate::FieldReader<bool, bool>);
impl AE9_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE9` writer - Acknowledge error 9"]
pub struct AE9_W<'a> {
    w: &'a mut W,
}
impl<'a> AE9_W<'a> {
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
#[doc = "Field `AE8` reader - Acknowledge error 8"]
pub struct AE8_R(crate::FieldReader<bool, bool>);
impl AE8_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE8` writer - Acknowledge error 8"]
pub struct AE8_W<'a> {
    w: &'a mut W,
}
impl<'a> AE8_W<'a> {
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
#[doc = "Field `AE7` reader - Acknowledge error 7"]
pub struct AE7_R(crate::FieldReader<bool, bool>);
impl AE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE7` writer - Acknowledge error 7"]
pub struct AE7_W<'a> {
    w: &'a mut W,
}
impl<'a> AE7_W<'a> {
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
#[doc = "Field `AE6` reader - Acknowledge error 6"]
pub struct AE6_R(crate::FieldReader<bool, bool>);
impl AE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE6` writer - Acknowledge error 6"]
pub struct AE6_W<'a> {
    w: &'a mut W,
}
impl<'a> AE6_W<'a> {
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
#[doc = "Field `AE5` reader - Acknowledge error 5"]
pub struct AE5_R(crate::FieldReader<bool, bool>);
impl AE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE5` writer - Acknowledge error 5"]
pub struct AE5_W<'a> {
    w: &'a mut W,
}
impl<'a> AE5_W<'a> {
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
#[doc = "Field `AE4` reader - Acknowledge error 4"]
pub struct AE4_R(crate::FieldReader<bool, bool>);
impl AE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE4` writer - Acknowledge error 4"]
pub struct AE4_W<'a> {
    w: &'a mut W,
}
impl<'a> AE4_W<'a> {
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
#[doc = "Field `AE3` reader - Acknowledge error 3"]
pub struct AE3_R(crate::FieldReader<bool, bool>);
impl AE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE3` writer - Acknowledge error 3"]
pub struct AE3_W<'a> {
    w: &'a mut W,
}
impl<'a> AE3_W<'a> {
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
#[doc = "Field `AE2` reader - Acknowledge error 2"]
pub struct AE2_R(crate::FieldReader<bool, bool>);
impl AE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE2` writer - Acknowledge error 2"]
pub struct AE2_W<'a> {
    w: &'a mut W,
}
impl<'a> AE2_W<'a> {
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
#[doc = "Field `AE1` reader - Acknowledge error 1"]
pub struct AE1_R(crate::FieldReader<bool, bool>);
impl AE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE1` writer - Acknowledge error 1"]
pub struct AE1_W<'a> {
    w: &'a mut W,
}
impl<'a> AE1_W<'a> {
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
#[doc = "Field `AE0` reader - Acknowledge error 0"]
pub struct AE0_R(crate::FieldReader<bool, bool>);
impl AE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE0` writer - Acknowledge error 0"]
pub struct AE0_W<'a> {
    w: &'a mut W,
}
impl<'a> AE0_W<'a> {
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
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    pub fn pe4(&self) -> PE4_R {
        PE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    pub fn pe1(&self) -> PE1_R {
        PE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    pub fn ae15(&self) -> AE15_R {
        AE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    pub fn ae14(&self) -> AE14_R {
        AE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    pub fn ae13(&self) -> AE13_R {
        AE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    pub fn ae12(&self) -> AE12_R {
        AE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    pub fn ae11(&self) -> AE11_R {
        AE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    pub fn ae10(&self) -> AE10_R {
        AE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    pub fn ae9(&self) -> AE9_R {
        AE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    pub fn ae8(&self) -> AE8_R {
        AE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    pub fn ae7(&self) -> AE7_R {
        AE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    pub fn ae6(&self) -> AE6_R {
        AE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    pub fn ae5(&self) -> AE5_R {
        AE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    pub fn ae4(&self) -> AE4_R {
        AE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    pub fn ae3(&self) -> AE3_R {
        AE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    pub fn ae2(&self) -> AE2_R {
        AE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    pub fn ae1(&self) -> AE1_R {
        AE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    pub fn ae0(&self) -> AE0_R {
        AE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - PHY error 4"]
    #[inline(always)]
    pub fn pe4(&mut self) -> PE4_W {
        PE4_W { w: self }
    }
    #[doc = "Bit 19 - PHY error 3"]
    #[inline(always)]
    pub fn pe3(&mut self) -> PE3_W {
        PE3_W { w: self }
    }
    #[doc = "Bit 18 - PHY error 2"]
    #[inline(always)]
    pub fn pe2(&mut self) -> PE2_W {
        PE2_W { w: self }
    }
    #[doc = "Bit 17 - PHY error 1"]
    #[inline(always)]
    pub fn pe1(&mut self) -> PE1_W {
        PE1_W { w: self }
    }
    #[doc = "Bit 16 - PHY error 0"]
    #[inline(always)]
    pub fn pe0(&mut self) -> PE0_W {
        PE0_W { w: self }
    }
    #[doc = "Bit 15 - Acknowledge error 15"]
    #[inline(always)]
    pub fn ae15(&mut self) -> AE15_W {
        AE15_W { w: self }
    }
    #[doc = "Bit 14 - Acknowledge error 14"]
    #[inline(always)]
    pub fn ae14(&mut self) -> AE14_W {
        AE14_W { w: self }
    }
    #[doc = "Bit 13 - Acknowledge error 13"]
    #[inline(always)]
    pub fn ae13(&mut self) -> AE13_W {
        AE13_W { w: self }
    }
    #[doc = "Bit 12 - Acknowledge error 12"]
    #[inline(always)]
    pub fn ae12(&mut self) -> AE12_W {
        AE12_W { w: self }
    }
    #[doc = "Bit 11 - Acknowledge error 11"]
    #[inline(always)]
    pub fn ae11(&mut self) -> AE11_W {
        AE11_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge error 10"]
    #[inline(always)]
    pub fn ae10(&mut self) -> AE10_W {
        AE10_W { w: self }
    }
    #[doc = "Bit 9 - Acknowledge error 9"]
    #[inline(always)]
    pub fn ae9(&mut self) -> AE9_W {
        AE9_W { w: self }
    }
    #[doc = "Bit 8 - Acknowledge error 8"]
    #[inline(always)]
    pub fn ae8(&mut self) -> AE8_W {
        AE8_W { w: self }
    }
    #[doc = "Bit 7 - Acknowledge error 7"]
    #[inline(always)]
    pub fn ae7(&mut self) -> AE7_W {
        AE7_W { w: self }
    }
    #[doc = "Bit 6 - Acknowledge error 6"]
    #[inline(always)]
    pub fn ae6(&mut self) -> AE6_W {
        AE6_W { w: self }
    }
    #[doc = "Bit 5 - Acknowledge error 5"]
    #[inline(always)]
    pub fn ae5(&mut self) -> AE5_W {
        AE5_W { w: self }
    }
    #[doc = "Bit 4 - Acknowledge error 4"]
    #[inline(always)]
    pub fn ae4(&mut self) -> AE4_W {
        AE4_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge error 3"]
    #[inline(always)]
    pub fn ae3(&mut self) -> AE3_W {
        AE3_W { w: self }
    }
    #[doc = "Bit 2 - Acknowledge error 2"]
    #[inline(always)]
    pub fn ae2(&mut self) -> AE2_W {
        AE2_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge error 1"]
    #[inline(always)]
    pub fn ae1(&mut self) -> AE1_W {
        AE1_W { w: self }
    }
    #[doc = "Bit 0 - Acknowledge error 0"]
    #[inline(always)]
    pub fn ae0(&mut self) -> AE0_W {
        AE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host interrupt and status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr0](index.html) module"]
pub struct ISR0_SPEC;
impl crate::RegisterSpec for ISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr0::R](R) reader structure"]
impl crate::Readable for ISR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr0::W](W) writer structure"]
impl crate::Writable for ISR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR0 to value 0"]
impl crate::Resettable for ISR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
