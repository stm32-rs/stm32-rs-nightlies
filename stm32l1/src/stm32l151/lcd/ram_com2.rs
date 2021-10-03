#[doc = "Register `RAM_COM2` reader"]
pub struct R(crate::R<RAM_COM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_COM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_COM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_COM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_COM2` writer"]
pub struct W(crate::W<RAM_COM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_COM2_SPEC>;
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
impl From<crate::W<RAM_COM2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_COM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S31` reader - S31"]
pub struct S31_R(crate::FieldReader<bool, bool>);
impl S31_R {
    pub(crate) fn new(bits: bool) -> Self {
        S31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S31` writer - S31"]
pub struct S31_W<'a> {
    w: &'a mut W,
}
impl<'a> S31_W<'a> {
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
#[doc = "Field `S30` reader - S30"]
pub struct S30_R(crate::FieldReader<bool, bool>);
impl S30_R {
    pub(crate) fn new(bits: bool) -> Self {
        S30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S30` writer - S30"]
pub struct S30_W<'a> {
    w: &'a mut W,
}
impl<'a> S30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `S29` reader - S29"]
pub struct S29_R(crate::FieldReader<bool, bool>);
impl S29_R {
    pub(crate) fn new(bits: bool) -> Self {
        S29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S29` writer - S29"]
pub struct S29_W<'a> {
    w: &'a mut W,
}
impl<'a> S29_W<'a> {
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
#[doc = "Field `S28` reader - S28"]
pub struct S28_R(crate::FieldReader<bool, bool>);
impl S28_R {
    pub(crate) fn new(bits: bool) -> Self {
        S28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S28` writer - S28"]
pub struct S28_W<'a> {
    w: &'a mut W,
}
impl<'a> S28_W<'a> {
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
#[doc = "Field `S27` reader - S27"]
pub struct S27_R(crate::FieldReader<bool, bool>);
impl S27_R {
    pub(crate) fn new(bits: bool) -> Self {
        S27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S27` writer - S27"]
pub struct S27_W<'a> {
    w: &'a mut W,
}
impl<'a> S27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `S26` reader - S26"]
pub struct S26_R(crate::FieldReader<bool, bool>);
impl S26_R {
    pub(crate) fn new(bits: bool) -> Self {
        S26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S26` writer - S26"]
pub struct S26_W<'a> {
    w: &'a mut W,
}
impl<'a> S26_W<'a> {
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
#[doc = "Field `S25` reader - S25"]
pub struct S25_R(crate::FieldReader<bool, bool>);
impl S25_R {
    pub(crate) fn new(bits: bool) -> Self {
        S25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S25` writer - S25"]
pub struct S25_W<'a> {
    w: &'a mut W,
}
impl<'a> S25_W<'a> {
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
#[doc = "Field `S24` reader - S24"]
pub struct S24_R(crate::FieldReader<bool, bool>);
impl S24_R {
    pub(crate) fn new(bits: bool) -> Self {
        S24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S24` writer - S24"]
pub struct S24_W<'a> {
    w: &'a mut W,
}
impl<'a> S24_W<'a> {
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
#[doc = "Field `S23` reader - S23"]
pub struct S23_R(crate::FieldReader<bool, bool>);
impl S23_R {
    pub(crate) fn new(bits: bool) -> Self {
        S23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S23` writer - S23"]
pub struct S23_W<'a> {
    w: &'a mut W,
}
impl<'a> S23_W<'a> {
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
#[doc = "Field `S22` reader - S22"]
pub struct S22_R(crate::FieldReader<bool, bool>);
impl S22_R {
    pub(crate) fn new(bits: bool) -> Self {
        S22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S22` writer - S22"]
pub struct S22_W<'a> {
    w: &'a mut W,
}
impl<'a> S22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `S21` reader - S21"]
pub struct S21_R(crate::FieldReader<bool, bool>);
impl S21_R {
    pub(crate) fn new(bits: bool) -> Self {
        S21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S21` writer - S21"]
pub struct S21_W<'a> {
    w: &'a mut W,
}
impl<'a> S21_W<'a> {
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
#[doc = "Field `S20` reader - S20"]
pub struct S20_R(crate::FieldReader<bool, bool>);
impl S20_R {
    pub(crate) fn new(bits: bool) -> Self {
        S20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S20` writer - S20"]
pub struct S20_W<'a> {
    w: &'a mut W,
}
impl<'a> S20_W<'a> {
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
#[doc = "Field `S19` reader - S19"]
pub struct S19_R(crate::FieldReader<bool, bool>);
impl S19_R {
    pub(crate) fn new(bits: bool) -> Self {
        S19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S19` writer - S19"]
pub struct S19_W<'a> {
    w: &'a mut W,
}
impl<'a> S19_W<'a> {
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
#[doc = "Field `S18` reader - S18"]
pub struct S18_R(crate::FieldReader<bool, bool>);
impl S18_R {
    pub(crate) fn new(bits: bool) -> Self {
        S18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S18` writer - S18"]
pub struct S18_W<'a> {
    w: &'a mut W,
}
impl<'a> S18_W<'a> {
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
#[doc = "Field `S17` reader - S17"]
pub struct S17_R(crate::FieldReader<bool, bool>);
impl S17_R {
    pub(crate) fn new(bits: bool) -> Self {
        S17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S17` writer - S17"]
pub struct S17_W<'a> {
    w: &'a mut W,
}
impl<'a> S17_W<'a> {
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
#[doc = "Field `S16` reader - S16"]
pub struct S16_R(crate::FieldReader<bool, bool>);
impl S16_R {
    pub(crate) fn new(bits: bool) -> Self {
        S16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S16` writer - S16"]
pub struct S16_W<'a> {
    w: &'a mut W,
}
impl<'a> S16_W<'a> {
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
#[doc = "Field `S15` reader - S15"]
pub struct S15_R(crate::FieldReader<bool, bool>);
impl S15_R {
    pub(crate) fn new(bits: bool) -> Self {
        S15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S15` writer - S15"]
pub struct S15_W<'a> {
    w: &'a mut W,
}
impl<'a> S15_W<'a> {
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
#[doc = "Field `S14` reader - S14"]
pub struct S14_R(crate::FieldReader<bool, bool>);
impl S14_R {
    pub(crate) fn new(bits: bool) -> Self {
        S14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S14` writer - S14"]
pub struct S14_W<'a> {
    w: &'a mut W,
}
impl<'a> S14_W<'a> {
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
#[doc = "Field `S13` reader - S13"]
pub struct S13_R(crate::FieldReader<bool, bool>);
impl S13_R {
    pub(crate) fn new(bits: bool) -> Self {
        S13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S13` writer - S13"]
pub struct S13_W<'a> {
    w: &'a mut W,
}
impl<'a> S13_W<'a> {
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
#[doc = "Field `S12` reader - S12"]
pub struct S12_R(crate::FieldReader<bool, bool>);
impl S12_R {
    pub(crate) fn new(bits: bool) -> Self {
        S12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S12` writer - S12"]
pub struct S12_W<'a> {
    w: &'a mut W,
}
impl<'a> S12_W<'a> {
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
#[doc = "Field `S11` reader - S11"]
pub struct S11_R(crate::FieldReader<bool, bool>);
impl S11_R {
    pub(crate) fn new(bits: bool) -> Self {
        S11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S11` writer - S11"]
pub struct S11_W<'a> {
    w: &'a mut W,
}
impl<'a> S11_W<'a> {
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
#[doc = "Field `S10` reader - S10"]
pub struct S10_R(crate::FieldReader<bool, bool>);
impl S10_R {
    pub(crate) fn new(bits: bool) -> Self {
        S10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S10` writer - S10"]
pub struct S10_W<'a> {
    w: &'a mut W,
}
impl<'a> S10_W<'a> {
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
#[doc = "Field `S09` reader - S09"]
pub struct S09_R(crate::FieldReader<bool, bool>);
impl S09_R {
    pub(crate) fn new(bits: bool) -> Self {
        S09_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S09_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S09` writer - S09"]
pub struct S09_W<'a> {
    w: &'a mut W,
}
impl<'a> S09_W<'a> {
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
#[doc = "Field `S08` reader - S08"]
pub struct S08_R(crate::FieldReader<bool, bool>);
impl S08_R {
    pub(crate) fn new(bits: bool) -> Self {
        S08_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S08_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S08` writer - S08"]
pub struct S08_W<'a> {
    w: &'a mut W,
}
impl<'a> S08_W<'a> {
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
#[doc = "Field `S07` reader - S07"]
pub struct S07_R(crate::FieldReader<bool, bool>);
impl S07_R {
    pub(crate) fn new(bits: bool) -> Self {
        S07_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S07_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S07` writer - S07"]
pub struct S07_W<'a> {
    w: &'a mut W,
}
impl<'a> S07_W<'a> {
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
#[doc = "Field `S06` reader - S06"]
pub struct S06_R(crate::FieldReader<bool, bool>);
impl S06_R {
    pub(crate) fn new(bits: bool) -> Self {
        S06_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S06_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S06` writer - S06"]
pub struct S06_W<'a> {
    w: &'a mut W,
}
impl<'a> S06_W<'a> {
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
#[doc = "Field `S05` reader - S05"]
pub struct S05_R(crate::FieldReader<bool, bool>);
impl S05_R {
    pub(crate) fn new(bits: bool) -> Self {
        S05_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S05_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S05` writer - S05"]
pub struct S05_W<'a> {
    w: &'a mut W,
}
impl<'a> S05_W<'a> {
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
#[doc = "Field `S04` reader - S04"]
pub struct S04_R(crate::FieldReader<bool, bool>);
impl S04_R {
    pub(crate) fn new(bits: bool) -> Self {
        S04_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S04_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S04` writer - S04"]
pub struct S04_W<'a> {
    w: &'a mut W,
}
impl<'a> S04_W<'a> {
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
#[doc = "Field `S03` reader - S03"]
pub struct S03_R(crate::FieldReader<bool, bool>);
impl S03_R {
    pub(crate) fn new(bits: bool) -> Self {
        S03_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S03_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S03` writer - S03"]
pub struct S03_W<'a> {
    w: &'a mut W,
}
impl<'a> S03_W<'a> {
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
#[doc = "Field `S02` reader - S02"]
pub struct S02_R(crate::FieldReader<bool, bool>);
impl S02_R {
    pub(crate) fn new(bits: bool) -> Self {
        S02_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S02_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S02` writer - S02"]
pub struct S02_W<'a> {
    w: &'a mut W,
}
impl<'a> S02_W<'a> {
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
#[doc = "Field `S01` reader - S01"]
pub struct S01_R(crate::FieldReader<bool, bool>);
impl S01_R {
    pub(crate) fn new(bits: bool) -> Self {
        S01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S01_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S01` writer - S01"]
pub struct S01_W<'a> {
    w: &'a mut W,
}
impl<'a> S01_W<'a> {
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
#[doc = "Field `S00` reader - S00"]
pub struct S00_R(crate::FieldReader<bool, bool>);
impl S00_R {
    pub(crate) fn new(bits: bool) -> Self {
        S00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S00_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S00` writer - S00"]
pub struct S00_W<'a> {
    w: &'a mut W,
}
impl<'a> S00_W<'a> {
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
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    pub fn s31(&self) -> S31_R {
        S31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    pub fn s30(&self) -> S30_R {
        S30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    pub fn s29(&self) -> S29_R {
        S29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    pub fn s28(&self) -> S28_R {
        S28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    pub fn s27(&self) -> S27_R {
        S27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    pub fn s26(&self) -> S26_R {
        S26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    pub fn s25(&self) -> S25_R {
        S25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    pub fn s24(&self) -> S24_R {
        S24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    pub fn s23(&self) -> S23_R {
        S23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    pub fn s22(&self) -> S22_R {
        S22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    pub fn s21(&self) -> S21_R {
        S21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    pub fn s20(&self) -> S20_R {
        S20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    pub fn s19(&self) -> S19_R {
        S19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    pub fn s18(&self) -> S18_R {
        S18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    pub fn s17(&self) -> S17_R {
        S17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    pub fn s16(&self) -> S16_R {
        S16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    pub fn s15(&self) -> S15_R {
        S15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    pub fn s14(&self) -> S14_R {
        S14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&self) -> S13_R {
        S13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&self) -> S12_R {
        S12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&self) -> S11_R {
        S11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&self) -> S10_R {
        S10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    pub fn s09(&self) -> S09_R {
        S09_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    pub fn s08(&self) -> S08_R {
        S08_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    pub fn s07(&self) -> S07_R {
        S07_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    pub fn s06(&self) -> S06_R {
        S06_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    pub fn s05(&self) -> S05_R {
        S05_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    pub fn s04(&self) -> S04_R {
        S04_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    pub fn s03(&self) -> S03_R {
        S03_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    pub fn s02(&self) -> S02_R {
        S02_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    pub fn s01(&self) -> S01_R {
        S01_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    pub fn s00(&self) -> S00_R {
        S00_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - S31"]
    #[inline(always)]
    pub fn s31(&mut self) -> S31_W {
        S31_W { w: self }
    }
    #[doc = "Bit 30 - S30"]
    #[inline(always)]
    pub fn s30(&mut self) -> S30_W {
        S30_W { w: self }
    }
    #[doc = "Bit 29 - S29"]
    #[inline(always)]
    pub fn s29(&mut self) -> S29_W {
        S29_W { w: self }
    }
    #[doc = "Bit 28 - S28"]
    #[inline(always)]
    pub fn s28(&mut self) -> S28_W {
        S28_W { w: self }
    }
    #[doc = "Bit 27 - S27"]
    #[inline(always)]
    pub fn s27(&mut self) -> S27_W {
        S27_W { w: self }
    }
    #[doc = "Bit 26 - S26"]
    #[inline(always)]
    pub fn s26(&mut self) -> S26_W {
        S26_W { w: self }
    }
    #[doc = "Bit 25 - S25"]
    #[inline(always)]
    pub fn s25(&mut self) -> S25_W {
        S25_W { w: self }
    }
    #[doc = "Bit 24 - S24"]
    #[inline(always)]
    pub fn s24(&mut self) -> S24_W {
        S24_W { w: self }
    }
    #[doc = "Bit 23 - S23"]
    #[inline(always)]
    pub fn s23(&mut self) -> S23_W {
        S23_W { w: self }
    }
    #[doc = "Bit 22 - S22"]
    #[inline(always)]
    pub fn s22(&mut self) -> S22_W {
        S22_W { w: self }
    }
    #[doc = "Bit 21 - S21"]
    #[inline(always)]
    pub fn s21(&mut self) -> S21_W {
        S21_W { w: self }
    }
    #[doc = "Bit 20 - S20"]
    #[inline(always)]
    pub fn s20(&mut self) -> S20_W {
        S20_W { w: self }
    }
    #[doc = "Bit 19 - S19"]
    #[inline(always)]
    pub fn s19(&mut self) -> S19_W {
        S19_W { w: self }
    }
    #[doc = "Bit 18 - S18"]
    #[inline(always)]
    pub fn s18(&mut self) -> S18_W {
        S18_W { w: self }
    }
    #[doc = "Bit 17 - S17"]
    #[inline(always)]
    pub fn s17(&mut self) -> S17_W {
        S17_W { w: self }
    }
    #[doc = "Bit 16 - S16"]
    #[inline(always)]
    pub fn s16(&mut self) -> S16_W {
        S16_W { w: self }
    }
    #[doc = "Bit 15 - S15"]
    #[inline(always)]
    pub fn s15(&mut self) -> S15_W {
        S15_W { w: self }
    }
    #[doc = "Bit 14 - S14"]
    #[inline(always)]
    pub fn s14(&mut self) -> S14_W {
        S14_W { w: self }
    }
    #[doc = "Bit 13 - S13"]
    #[inline(always)]
    pub fn s13(&mut self) -> S13_W {
        S13_W { w: self }
    }
    #[doc = "Bit 12 - S12"]
    #[inline(always)]
    pub fn s12(&mut self) -> S12_W {
        S12_W { w: self }
    }
    #[doc = "Bit 11 - S11"]
    #[inline(always)]
    pub fn s11(&mut self) -> S11_W {
        S11_W { w: self }
    }
    #[doc = "Bit 10 - S10"]
    #[inline(always)]
    pub fn s10(&mut self) -> S10_W {
        S10_W { w: self }
    }
    #[doc = "Bit 9 - S09"]
    #[inline(always)]
    pub fn s09(&mut self) -> S09_W {
        S09_W { w: self }
    }
    #[doc = "Bit 8 - S08"]
    #[inline(always)]
    pub fn s08(&mut self) -> S08_W {
        S08_W { w: self }
    }
    #[doc = "Bit 7 - S07"]
    #[inline(always)]
    pub fn s07(&mut self) -> S07_W {
        S07_W { w: self }
    }
    #[doc = "Bit 6 - S06"]
    #[inline(always)]
    pub fn s06(&mut self) -> S06_W {
        S06_W { w: self }
    }
    #[doc = "Bit 5 - S05"]
    #[inline(always)]
    pub fn s05(&mut self) -> S05_W {
        S05_W { w: self }
    }
    #[doc = "Bit 4 - S04"]
    #[inline(always)]
    pub fn s04(&mut self) -> S04_W {
        S04_W { w: self }
    }
    #[doc = "Bit 3 - S03"]
    #[inline(always)]
    pub fn s03(&mut self) -> S03_W {
        S03_W { w: self }
    }
    #[doc = "Bit 2 - S02"]
    #[inline(always)]
    pub fn s02(&mut self) -> S02_W {
        S02_W { w: self }
    }
    #[doc = "Bit 1 - S01"]
    #[inline(always)]
    pub fn s01(&mut self) -> S01_W {
        S01_W { w: self }
    }
    #[doc = "Bit 0 - S00"]
    #[inline(always)]
    pub fn s00(&mut self) -> S00_W {
        S00_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "display memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_com2](index.html) module"]
pub struct RAM_COM2_SPEC;
impl crate::RegisterSpec for RAM_COM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram_com2::R](R) reader structure"]
impl crate::Readable for RAM_COM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_com2::W](W) writer structure"]
impl crate::Writable for RAM_COM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_COM2 to value 0"]
impl crate::Resettable for RAM_COM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
