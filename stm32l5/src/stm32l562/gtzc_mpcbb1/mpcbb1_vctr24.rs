#[doc = "Register `MPCBB1_VCTR24` reader"]
pub struct R(crate::R<MPCBB1_VCTR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR24` writer"]
pub struct W(crate::W<MPCBB1_VCTR24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR24_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B768` reader - B768"]
pub struct B768_R(crate::FieldReader<bool, bool>);
impl B768_R {
    pub(crate) fn new(bits: bool) -> Self {
        B768_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B768_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B768` writer - B768"]
pub struct B768_W<'a> {
    w: &'a mut W,
}
impl<'a> B768_W<'a> {
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
#[doc = "Field `B769` reader - B769"]
pub struct B769_R(crate::FieldReader<bool, bool>);
impl B769_R {
    pub(crate) fn new(bits: bool) -> Self {
        B769_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B769_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B769` writer - B769"]
pub struct B769_W<'a> {
    w: &'a mut W,
}
impl<'a> B769_W<'a> {
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
#[doc = "Field `B770` reader - B770"]
pub struct B770_R(crate::FieldReader<bool, bool>);
impl B770_R {
    pub(crate) fn new(bits: bool) -> Self {
        B770_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B770_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B770` writer - B770"]
pub struct B770_W<'a> {
    w: &'a mut W,
}
impl<'a> B770_W<'a> {
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
#[doc = "Field `B771` reader - B771"]
pub struct B771_R(crate::FieldReader<bool, bool>);
impl B771_R {
    pub(crate) fn new(bits: bool) -> Self {
        B771_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B771_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B771` writer - B771"]
pub struct B771_W<'a> {
    w: &'a mut W,
}
impl<'a> B771_W<'a> {
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
#[doc = "Field `B772` reader - B772"]
pub struct B772_R(crate::FieldReader<bool, bool>);
impl B772_R {
    pub(crate) fn new(bits: bool) -> Self {
        B772_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B772_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B772` writer - B772"]
pub struct B772_W<'a> {
    w: &'a mut W,
}
impl<'a> B772_W<'a> {
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
#[doc = "Field `B773` reader - B773"]
pub struct B773_R(crate::FieldReader<bool, bool>);
impl B773_R {
    pub(crate) fn new(bits: bool) -> Self {
        B773_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B773_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B773` writer - B773"]
pub struct B773_W<'a> {
    w: &'a mut W,
}
impl<'a> B773_W<'a> {
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
#[doc = "Field `B774` reader - B774"]
pub struct B774_R(crate::FieldReader<bool, bool>);
impl B774_R {
    pub(crate) fn new(bits: bool) -> Self {
        B774_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B774_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B774` writer - B774"]
pub struct B774_W<'a> {
    w: &'a mut W,
}
impl<'a> B774_W<'a> {
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
#[doc = "Field `B775` reader - B775"]
pub struct B775_R(crate::FieldReader<bool, bool>);
impl B775_R {
    pub(crate) fn new(bits: bool) -> Self {
        B775_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B775_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B775` writer - B775"]
pub struct B775_W<'a> {
    w: &'a mut W,
}
impl<'a> B775_W<'a> {
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
#[doc = "Field `B776` reader - B776"]
pub struct B776_R(crate::FieldReader<bool, bool>);
impl B776_R {
    pub(crate) fn new(bits: bool) -> Self {
        B776_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B776_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B776` writer - B776"]
pub struct B776_W<'a> {
    w: &'a mut W,
}
impl<'a> B776_W<'a> {
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
#[doc = "Field `B777` reader - B777"]
pub struct B777_R(crate::FieldReader<bool, bool>);
impl B777_R {
    pub(crate) fn new(bits: bool) -> Self {
        B777_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B777_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B777` writer - B777"]
pub struct B777_W<'a> {
    w: &'a mut W,
}
impl<'a> B777_W<'a> {
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
#[doc = "Field `B778` reader - B778"]
pub struct B778_R(crate::FieldReader<bool, bool>);
impl B778_R {
    pub(crate) fn new(bits: bool) -> Self {
        B778_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B778_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B778` writer - B778"]
pub struct B778_W<'a> {
    w: &'a mut W,
}
impl<'a> B778_W<'a> {
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
#[doc = "Field `B779` reader - B779"]
pub struct B779_R(crate::FieldReader<bool, bool>);
impl B779_R {
    pub(crate) fn new(bits: bool) -> Self {
        B779_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B779_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B779` writer - B779"]
pub struct B779_W<'a> {
    w: &'a mut W,
}
impl<'a> B779_W<'a> {
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
#[doc = "Field `B780` reader - B780"]
pub struct B780_R(crate::FieldReader<bool, bool>);
impl B780_R {
    pub(crate) fn new(bits: bool) -> Self {
        B780_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B780_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B780` writer - B780"]
pub struct B780_W<'a> {
    w: &'a mut W,
}
impl<'a> B780_W<'a> {
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
#[doc = "Field `B781` reader - B781"]
pub struct B781_R(crate::FieldReader<bool, bool>);
impl B781_R {
    pub(crate) fn new(bits: bool) -> Self {
        B781_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B781_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B781` writer - B781"]
pub struct B781_W<'a> {
    w: &'a mut W,
}
impl<'a> B781_W<'a> {
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
#[doc = "Field `B782` reader - B782"]
pub struct B782_R(crate::FieldReader<bool, bool>);
impl B782_R {
    pub(crate) fn new(bits: bool) -> Self {
        B782_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B782_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B782` writer - B782"]
pub struct B782_W<'a> {
    w: &'a mut W,
}
impl<'a> B782_W<'a> {
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
#[doc = "Field `B783` reader - B783"]
pub struct B783_R(crate::FieldReader<bool, bool>);
impl B783_R {
    pub(crate) fn new(bits: bool) -> Self {
        B783_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B783_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B783` writer - B783"]
pub struct B783_W<'a> {
    w: &'a mut W,
}
impl<'a> B783_W<'a> {
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
#[doc = "Field `B784` reader - B784"]
pub struct B784_R(crate::FieldReader<bool, bool>);
impl B784_R {
    pub(crate) fn new(bits: bool) -> Self {
        B784_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B784_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B784` writer - B784"]
pub struct B784_W<'a> {
    w: &'a mut W,
}
impl<'a> B784_W<'a> {
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
#[doc = "Field `B785` reader - B785"]
pub struct B785_R(crate::FieldReader<bool, bool>);
impl B785_R {
    pub(crate) fn new(bits: bool) -> Self {
        B785_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B785_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B785` writer - B785"]
pub struct B785_W<'a> {
    w: &'a mut W,
}
impl<'a> B785_W<'a> {
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
#[doc = "Field `B786` reader - B786"]
pub struct B786_R(crate::FieldReader<bool, bool>);
impl B786_R {
    pub(crate) fn new(bits: bool) -> Self {
        B786_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B786_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B786` writer - B786"]
pub struct B786_W<'a> {
    w: &'a mut W,
}
impl<'a> B786_W<'a> {
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
#[doc = "Field `B787` reader - B787"]
pub struct B787_R(crate::FieldReader<bool, bool>);
impl B787_R {
    pub(crate) fn new(bits: bool) -> Self {
        B787_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B787_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B787` writer - B787"]
pub struct B787_W<'a> {
    w: &'a mut W,
}
impl<'a> B787_W<'a> {
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
#[doc = "Field `B788` reader - B788"]
pub struct B788_R(crate::FieldReader<bool, bool>);
impl B788_R {
    pub(crate) fn new(bits: bool) -> Self {
        B788_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B788_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B788` writer - B788"]
pub struct B788_W<'a> {
    w: &'a mut W,
}
impl<'a> B788_W<'a> {
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
#[doc = "Field `B789` reader - B789"]
pub struct B789_R(crate::FieldReader<bool, bool>);
impl B789_R {
    pub(crate) fn new(bits: bool) -> Self {
        B789_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B789_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B789` writer - B789"]
pub struct B789_W<'a> {
    w: &'a mut W,
}
impl<'a> B789_W<'a> {
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
#[doc = "Field `B790` reader - B790"]
pub struct B790_R(crate::FieldReader<bool, bool>);
impl B790_R {
    pub(crate) fn new(bits: bool) -> Self {
        B790_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B790_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B790` writer - B790"]
pub struct B790_W<'a> {
    w: &'a mut W,
}
impl<'a> B790_W<'a> {
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
#[doc = "Field `B791` reader - B791"]
pub struct B791_R(crate::FieldReader<bool, bool>);
impl B791_R {
    pub(crate) fn new(bits: bool) -> Self {
        B791_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B791_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B791` writer - B791"]
pub struct B791_W<'a> {
    w: &'a mut W,
}
impl<'a> B791_W<'a> {
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
#[doc = "Field `B792` reader - B792"]
pub struct B792_R(crate::FieldReader<bool, bool>);
impl B792_R {
    pub(crate) fn new(bits: bool) -> Self {
        B792_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B792_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B792` writer - B792"]
pub struct B792_W<'a> {
    w: &'a mut W,
}
impl<'a> B792_W<'a> {
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
#[doc = "Field `B793` reader - B793"]
pub struct B793_R(crate::FieldReader<bool, bool>);
impl B793_R {
    pub(crate) fn new(bits: bool) -> Self {
        B793_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B793_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B793` writer - B793"]
pub struct B793_W<'a> {
    w: &'a mut W,
}
impl<'a> B793_W<'a> {
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
#[doc = "Field `B794` reader - B794"]
pub struct B794_R(crate::FieldReader<bool, bool>);
impl B794_R {
    pub(crate) fn new(bits: bool) -> Self {
        B794_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B794_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B794` writer - B794"]
pub struct B794_W<'a> {
    w: &'a mut W,
}
impl<'a> B794_W<'a> {
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
#[doc = "Field `B795` reader - B795"]
pub struct B795_R(crate::FieldReader<bool, bool>);
impl B795_R {
    pub(crate) fn new(bits: bool) -> Self {
        B795_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B795_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B795` writer - B795"]
pub struct B795_W<'a> {
    w: &'a mut W,
}
impl<'a> B795_W<'a> {
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
#[doc = "Field `B796` reader - B796"]
pub struct B796_R(crate::FieldReader<bool, bool>);
impl B796_R {
    pub(crate) fn new(bits: bool) -> Self {
        B796_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B796_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B796` writer - B796"]
pub struct B796_W<'a> {
    w: &'a mut W,
}
impl<'a> B796_W<'a> {
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
#[doc = "Field `B797` reader - B797"]
pub struct B797_R(crate::FieldReader<bool, bool>);
impl B797_R {
    pub(crate) fn new(bits: bool) -> Self {
        B797_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B797_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B797` writer - B797"]
pub struct B797_W<'a> {
    w: &'a mut W,
}
impl<'a> B797_W<'a> {
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
#[doc = "Field `B798` reader - B798"]
pub struct B798_R(crate::FieldReader<bool, bool>);
impl B798_R {
    pub(crate) fn new(bits: bool) -> Self {
        B798_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B798_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B798` writer - B798"]
pub struct B798_W<'a> {
    w: &'a mut W,
}
impl<'a> B798_W<'a> {
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
#[doc = "Field `B799` reader - B799"]
pub struct B799_R(crate::FieldReader<bool, bool>);
impl B799_R {
    pub(crate) fn new(bits: bool) -> Self {
        B799_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B799_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B799` writer - B799"]
pub struct B799_W<'a> {
    w: &'a mut W,
}
impl<'a> B799_W<'a> {
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
impl R {
    #[doc = "Bit 0 - B768"]
    #[inline(always)]
    pub fn b768(&self) -> B768_R {
        B768_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B769"]
    #[inline(always)]
    pub fn b769(&self) -> B769_R {
        B769_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B770"]
    #[inline(always)]
    pub fn b770(&self) -> B770_R {
        B770_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B771"]
    #[inline(always)]
    pub fn b771(&self) -> B771_R {
        B771_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B772"]
    #[inline(always)]
    pub fn b772(&self) -> B772_R {
        B772_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B773"]
    #[inline(always)]
    pub fn b773(&self) -> B773_R {
        B773_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B774"]
    #[inline(always)]
    pub fn b774(&self) -> B774_R {
        B774_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B775"]
    #[inline(always)]
    pub fn b775(&self) -> B775_R {
        B775_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B776"]
    #[inline(always)]
    pub fn b776(&self) -> B776_R {
        B776_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B777"]
    #[inline(always)]
    pub fn b777(&self) -> B777_R {
        B777_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B778"]
    #[inline(always)]
    pub fn b778(&self) -> B778_R {
        B778_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B779"]
    #[inline(always)]
    pub fn b779(&self) -> B779_R {
        B779_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B780"]
    #[inline(always)]
    pub fn b780(&self) -> B780_R {
        B780_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B781"]
    #[inline(always)]
    pub fn b781(&self) -> B781_R {
        B781_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B782"]
    #[inline(always)]
    pub fn b782(&self) -> B782_R {
        B782_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B783"]
    #[inline(always)]
    pub fn b783(&self) -> B783_R {
        B783_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B784"]
    #[inline(always)]
    pub fn b784(&self) -> B784_R {
        B784_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B785"]
    #[inline(always)]
    pub fn b785(&self) -> B785_R {
        B785_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B786"]
    #[inline(always)]
    pub fn b786(&self) -> B786_R {
        B786_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B787"]
    #[inline(always)]
    pub fn b787(&self) -> B787_R {
        B787_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B788"]
    #[inline(always)]
    pub fn b788(&self) -> B788_R {
        B788_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B789"]
    #[inline(always)]
    pub fn b789(&self) -> B789_R {
        B789_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B790"]
    #[inline(always)]
    pub fn b790(&self) -> B790_R {
        B790_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B791"]
    #[inline(always)]
    pub fn b791(&self) -> B791_R {
        B791_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B792"]
    #[inline(always)]
    pub fn b792(&self) -> B792_R {
        B792_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B793"]
    #[inline(always)]
    pub fn b793(&self) -> B793_R {
        B793_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B794"]
    #[inline(always)]
    pub fn b794(&self) -> B794_R {
        B794_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B795"]
    #[inline(always)]
    pub fn b795(&self) -> B795_R {
        B795_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B796"]
    #[inline(always)]
    pub fn b796(&self) -> B796_R {
        B796_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B797"]
    #[inline(always)]
    pub fn b797(&self) -> B797_R {
        B797_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B798"]
    #[inline(always)]
    pub fn b798(&self) -> B798_R {
        B798_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B799"]
    #[inline(always)]
    pub fn b799(&self) -> B799_R {
        B799_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B768"]
    #[inline(always)]
    pub fn b768(&mut self) -> B768_W {
        B768_W { w: self }
    }
    #[doc = "Bit 1 - B769"]
    #[inline(always)]
    pub fn b769(&mut self) -> B769_W {
        B769_W { w: self }
    }
    #[doc = "Bit 2 - B770"]
    #[inline(always)]
    pub fn b770(&mut self) -> B770_W {
        B770_W { w: self }
    }
    #[doc = "Bit 3 - B771"]
    #[inline(always)]
    pub fn b771(&mut self) -> B771_W {
        B771_W { w: self }
    }
    #[doc = "Bit 4 - B772"]
    #[inline(always)]
    pub fn b772(&mut self) -> B772_W {
        B772_W { w: self }
    }
    #[doc = "Bit 5 - B773"]
    #[inline(always)]
    pub fn b773(&mut self) -> B773_W {
        B773_W { w: self }
    }
    #[doc = "Bit 6 - B774"]
    #[inline(always)]
    pub fn b774(&mut self) -> B774_W {
        B774_W { w: self }
    }
    #[doc = "Bit 7 - B775"]
    #[inline(always)]
    pub fn b775(&mut self) -> B775_W {
        B775_W { w: self }
    }
    #[doc = "Bit 8 - B776"]
    #[inline(always)]
    pub fn b776(&mut self) -> B776_W {
        B776_W { w: self }
    }
    #[doc = "Bit 9 - B777"]
    #[inline(always)]
    pub fn b777(&mut self) -> B777_W {
        B777_W { w: self }
    }
    #[doc = "Bit 10 - B778"]
    #[inline(always)]
    pub fn b778(&mut self) -> B778_W {
        B778_W { w: self }
    }
    #[doc = "Bit 11 - B779"]
    #[inline(always)]
    pub fn b779(&mut self) -> B779_W {
        B779_W { w: self }
    }
    #[doc = "Bit 12 - B780"]
    #[inline(always)]
    pub fn b780(&mut self) -> B780_W {
        B780_W { w: self }
    }
    #[doc = "Bit 13 - B781"]
    #[inline(always)]
    pub fn b781(&mut self) -> B781_W {
        B781_W { w: self }
    }
    #[doc = "Bit 14 - B782"]
    #[inline(always)]
    pub fn b782(&mut self) -> B782_W {
        B782_W { w: self }
    }
    #[doc = "Bit 15 - B783"]
    #[inline(always)]
    pub fn b783(&mut self) -> B783_W {
        B783_W { w: self }
    }
    #[doc = "Bit 16 - B784"]
    #[inline(always)]
    pub fn b784(&mut self) -> B784_W {
        B784_W { w: self }
    }
    #[doc = "Bit 17 - B785"]
    #[inline(always)]
    pub fn b785(&mut self) -> B785_W {
        B785_W { w: self }
    }
    #[doc = "Bit 18 - B786"]
    #[inline(always)]
    pub fn b786(&mut self) -> B786_W {
        B786_W { w: self }
    }
    #[doc = "Bit 19 - B787"]
    #[inline(always)]
    pub fn b787(&mut self) -> B787_W {
        B787_W { w: self }
    }
    #[doc = "Bit 20 - B788"]
    #[inline(always)]
    pub fn b788(&mut self) -> B788_W {
        B788_W { w: self }
    }
    #[doc = "Bit 21 - B789"]
    #[inline(always)]
    pub fn b789(&mut self) -> B789_W {
        B789_W { w: self }
    }
    #[doc = "Bit 22 - B790"]
    #[inline(always)]
    pub fn b790(&mut self) -> B790_W {
        B790_W { w: self }
    }
    #[doc = "Bit 23 - B791"]
    #[inline(always)]
    pub fn b791(&mut self) -> B791_W {
        B791_W { w: self }
    }
    #[doc = "Bit 24 - B792"]
    #[inline(always)]
    pub fn b792(&mut self) -> B792_W {
        B792_W { w: self }
    }
    #[doc = "Bit 25 - B793"]
    #[inline(always)]
    pub fn b793(&mut self) -> B793_W {
        B793_W { w: self }
    }
    #[doc = "Bit 26 - B794"]
    #[inline(always)]
    pub fn b794(&mut self) -> B794_W {
        B794_W { w: self }
    }
    #[doc = "Bit 27 - B795"]
    #[inline(always)]
    pub fn b795(&mut self) -> B795_W {
        B795_W { w: self }
    }
    #[doc = "Bit 28 - B796"]
    #[inline(always)]
    pub fn b796(&mut self) -> B796_W {
        B796_W { w: self }
    }
    #[doc = "Bit 29 - B797"]
    #[inline(always)]
    pub fn b797(&mut self) -> B797_W {
        B797_W { w: self }
    }
    #[doc = "Bit 30 - B798"]
    #[inline(always)]
    pub fn b798(&mut self) -> B798_W {
        B798_W { w: self }
    }
    #[doc = "Bit 31 - B799"]
    #[inline(always)]
    pub fn b799(&mut self) -> B799_W {
        B799_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr24](index.html) module"]
pub struct MPCBB1_VCTR24_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr24::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr24::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR24_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR24 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR24_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
