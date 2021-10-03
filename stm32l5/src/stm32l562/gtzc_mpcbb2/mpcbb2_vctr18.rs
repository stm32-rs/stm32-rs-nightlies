#[doc = "Register `MPCBB2_VCTR18` reader"]
pub struct R(crate::R<MPCBB2_VCTR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR18` writer"]
pub struct W(crate::W<MPCBB2_VCTR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR18_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B576` reader - B576"]
pub struct B576_R(crate::FieldReader<bool, bool>);
impl B576_R {
    pub(crate) fn new(bits: bool) -> Self {
        B576_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B576_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B576` writer - B576"]
pub struct B576_W<'a> {
    w: &'a mut W,
}
impl<'a> B576_W<'a> {
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
#[doc = "Field `B577` reader - B577"]
pub struct B577_R(crate::FieldReader<bool, bool>);
impl B577_R {
    pub(crate) fn new(bits: bool) -> Self {
        B577_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B577_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B577` writer - B577"]
pub struct B577_W<'a> {
    w: &'a mut W,
}
impl<'a> B577_W<'a> {
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
#[doc = "Field `B578` reader - B578"]
pub struct B578_R(crate::FieldReader<bool, bool>);
impl B578_R {
    pub(crate) fn new(bits: bool) -> Self {
        B578_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B578_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B578` writer - B578"]
pub struct B578_W<'a> {
    w: &'a mut W,
}
impl<'a> B578_W<'a> {
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
#[doc = "Field `B579` reader - B579"]
pub struct B579_R(crate::FieldReader<bool, bool>);
impl B579_R {
    pub(crate) fn new(bits: bool) -> Self {
        B579_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B579_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B579` writer - B579"]
pub struct B579_W<'a> {
    w: &'a mut W,
}
impl<'a> B579_W<'a> {
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
#[doc = "Field `B580` reader - B580"]
pub struct B580_R(crate::FieldReader<bool, bool>);
impl B580_R {
    pub(crate) fn new(bits: bool) -> Self {
        B580_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B580_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B580` writer - B580"]
pub struct B580_W<'a> {
    w: &'a mut W,
}
impl<'a> B580_W<'a> {
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
#[doc = "Field `B581` reader - B581"]
pub struct B581_R(crate::FieldReader<bool, bool>);
impl B581_R {
    pub(crate) fn new(bits: bool) -> Self {
        B581_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B581_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B581` writer - B581"]
pub struct B581_W<'a> {
    w: &'a mut W,
}
impl<'a> B581_W<'a> {
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
#[doc = "Field `B582` reader - B582"]
pub struct B582_R(crate::FieldReader<bool, bool>);
impl B582_R {
    pub(crate) fn new(bits: bool) -> Self {
        B582_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B582_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B582` writer - B582"]
pub struct B582_W<'a> {
    w: &'a mut W,
}
impl<'a> B582_W<'a> {
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
#[doc = "Field `B583` reader - B583"]
pub struct B583_R(crate::FieldReader<bool, bool>);
impl B583_R {
    pub(crate) fn new(bits: bool) -> Self {
        B583_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B583_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B583` writer - B583"]
pub struct B583_W<'a> {
    w: &'a mut W,
}
impl<'a> B583_W<'a> {
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
#[doc = "Field `B584` reader - B584"]
pub struct B584_R(crate::FieldReader<bool, bool>);
impl B584_R {
    pub(crate) fn new(bits: bool) -> Self {
        B584_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B584_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B584` writer - B584"]
pub struct B584_W<'a> {
    w: &'a mut W,
}
impl<'a> B584_W<'a> {
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
#[doc = "Field `B585` reader - B585"]
pub struct B585_R(crate::FieldReader<bool, bool>);
impl B585_R {
    pub(crate) fn new(bits: bool) -> Self {
        B585_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B585_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B585` writer - B585"]
pub struct B585_W<'a> {
    w: &'a mut W,
}
impl<'a> B585_W<'a> {
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
#[doc = "Field `B586` reader - B586"]
pub struct B586_R(crate::FieldReader<bool, bool>);
impl B586_R {
    pub(crate) fn new(bits: bool) -> Self {
        B586_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B586_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B586` writer - B586"]
pub struct B586_W<'a> {
    w: &'a mut W,
}
impl<'a> B586_W<'a> {
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
#[doc = "Field `B587` reader - B587"]
pub struct B587_R(crate::FieldReader<bool, bool>);
impl B587_R {
    pub(crate) fn new(bits: bool) -> Self {
        B587_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B587_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B587` writer - B587"]
pub struct B587_W<'a> {
    w: &'a mut W,
}
impl<'a> B587_W<'a> {
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
#[doc = "Field `B588` reader - B588"]
pub struct B588_R(crate::FieldReader<bool, bool>);
impl B588_R {
    pub(crate) fn new(bits: bool) -> Self {
        B588_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B588_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B588` writer - B588"]
pub struct B588_W<'a> {
    w: &'a mut W,
}
impl<'a> B588_W<'a> {
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
#[doc = "Field `B589` reader - B589"]
pub struct B589_R(crate::FieldReader<bool, bool>);
impl B589_R {
    pub(crate) fn new(bits: bool) -> Self {
        B589_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B589_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B589` writer - B589"]
pub struct B589_W<'a> {
    w: &'a mut W,
}
impl<'a> B589_W<'a> {
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
#[doc = "Field `B590` reader - B590"]
pub struct B590_R(crate::FieldReader<bool, bool>);
impl B590_R {
    pub(crate) fn new(bits: bool) -> Self {
        B590_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B590_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B590` writer - B590"]
pub struct B590_W<'a> {
    w: &'a mut W,
}
impl<'a> B590_W<'a> {
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
#[doc = "Field `B591` reader - B591"]
pub struct B591_R(crate::FieldReader<bool, bool>);
impl B591_R {
    pub(crate) fn new(bits: bool) -> Self {
        B591_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B591_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B591` writer - B591"]
pub struct B591_W<'a> {
    w: &'a mut W,
}
impl<'a> B591_W<'a> {
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
#[doc = "Field `B592` reader - B592"]
pub struct B592_R(crate::FieldReader<bool, bool>);
impl B592_R {
    pub(crate) fn new(bits: bool) -> Self {
        B592_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B592_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B592` writer - B592"]
pub struct B592_W<'a> {
    w: &'a mut W,
}
impl<'a> B592_W<'a> {
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
#[doc = "Field `B593` reader - B593"]
pub struct B593_R(crate::FieldReader<bool, bool>);
impl B593_R {
    pub(crate) fn new(bits: bool) -> Self {
        B593_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B593_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B593` writer - B593"]
pub struct B593_W<'a> {
    w: &'a mut W,
}
impl<'a> B593_W<'a> {
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
#[doc = "Field `B594` reader - B594"]
pub struct B594_R(crate::FieldReader<bool, bool>);
impl B594_R {
    pub(crate) fn new(bits: bool) -> Self {
        B594_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B594_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B594` writer - B594"]
pub struct B594_W<'a> {
    w: &'a mut W,
}
impl<'a> B594_W<'a> {
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
#[doc = "Field `B595` reader - B595"]
pub struct B595_R(crate::FieldReader<bool, bool>);
impl B595_R {
    pub(crate) fn new(bits: bool) -> Self {
        B595_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B595_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B595` writer - B595"]
pub struct B595_W<'a> {
    w: &'a mut W,
}
impl<'a> B595_W<'a> {
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
#[doc = "Field `B596` reader - B596"]
pub struct B596_R(crate::FieldReader<bool, bool>);
impl B596_R {
    pub(crate) fn new(bits: bool) -> Self {
        B596_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B596_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B596` writer - B596"]
pub struct B596_W<'a> {
    w: &'a mut W,
}
impl<'a> B596_W<'a> {
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
#[doc = "Field `B597` reader - B597"]
pub struct B597_R(crate::FieldReader<bool, bool>);
impl B597_R {
    pub(crate) fn new(bits: bool) -> Self {
        B597_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B597_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B597` writer - B597"]
pub struct B597_W<'a> {
    w: &'a mut W,
}
impl<'a> B597_W<'a> {
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
#[doc = "Field `B598` reader - B598"]
pub struct B598_R(crate::FieldReader<bool, bool>);
impl B598_R {
    pub(crate) fn new(bits: bool) -> Self {
        B598_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B598_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B598` writer - B598"]
pub struct B598_W<'a> {
    w: &'a mut W,
}
impl<'a> B598_W<'a> {
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
#[doc = "Field `B599` reader - B599"]
pub struct B599_R(crate::FieldReader<bool, bool>);
impl B599_R {
    pub(crate) fn new(bits: bool) -> Self {
        B599_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B599_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B599` writer - B599"]
pub struct B599_W<'a> {
    w: &'a mut W,
}
impl<'a> B599_W<'a> {
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
#[doc = "Field `B600` reader - B600"]
pub struct B600_R(crate::FieldReader<bool, bool>);
impl B600_R {
    pub(crate) fn new(bits: bool) -> Self {
        B600_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B600_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B600` writer - B600"]
pub struct B600_W<'a> {
    w: &'a mut W,
}
impl<'a> B600_W<'a> {
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
#[doc = "Field `B601` reader - B601"]
pub struct B601_R(crate::FieldReader<bool, bool>);
impl B601_R {
    pub(crate) fn new(bits: bool) -> Self {
        B601_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B601_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B601` writer - B601"]
pub struct B601_W<'a> {
    w: &'a mut W,
}
impl<'a> B601_W<'a> {
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
#[doc = "Field `B602` reader - B602"]
pub struct B602_R(crate::FieldReader<bool, bool>);
impl B602_R {
    pub(crate) fn new(bits: bool) -> Self {
        B602_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B602_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B602` writer - B602"]
pub struct B602_W<'a> {
    w: &'a mut W,
}
impl<'a> B602_W<'a> {
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
#[doc = "Field `B603` reader - B603"]
pub struct B603_R(crate::FieldReader<bool, bool>);
impl B603_R {
    pub(crate) fn new(bits: bool) -> Self {
        B603_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B603_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B603` writer - B603"]
pub struct B603_W<'a> {
    w: &'a mut W,
}
impl<'a> B603_W<'a> {
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
#[doc = "Field `B604` reader - B604"]
pub struct B604_R(crate::FieldReader<bool, bool>);
impl B604_R {
    pub(crate) fn new(bits: bool) -> Self {
        B604_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B604_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B604` writer - B604"]
pub struct B604_W<'a> {
    w: &'a mut W,
}
impl<'a> B604_W<'a> {
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
#[doc = "Field `B605` reader - B605"]
pub struct B605_R(crate::FieldReader<bool, bool>);
impl B605_R {
    pub(crate) fn new(bits: bool) -> Self {
        B605_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B605_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B605` writer - B605"]
pub struct B605_W<'a> {
    w: &'a mut W,
}
impl<'a> B605_W<'a> {
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
#[doc = "Field `B606` reader - B606"]
pub struct B606_R(crate::FieldReader<bool, bool>);
impl B606_R {
    pub(crate) fn new(bits: bool) -> Self {
        B606_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B606_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B606` writer - B606"]
pub struct B606_W<'a> {
    w: &'a mut W,
}
impl<'a> B606_W<'a> {
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
#[doc = "Field `B607` reader - B607"]
pub struct B607_R(crate::FieldReader<bool, bool>);
impl B607_R {
    pub(crate) fn new(bits: bool) -> Self {
        B607_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B607_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B607` writer - B607"]
pub struct B607_W<'a> {
    w: &'a mut W,
}
impl<'a> B607_W<'a> {
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
    #[doc = "Bit 0 - B576"]
    #[inline(always)]
    pub fn b576(&self) -> B576_R {
        B576_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B577"]
    #[inline(always)]
    pub fn b577(&self) -> B577_R {
        B577_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B578"]
    #[inline(always)]
    pub fn b578(&self) -> B578_R {
        B578_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B579"]
    #[inline(always)]
    pub fn b579(&self) -> B579_R {
        B579_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B580"]
    #[inline(always)]
    pub fn b580(&self) -> B580_R {
        B580_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B581"]
    #[inline(always)]
    pub fn b581(&self) -> B581_R {
        B581_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B582"]
    #[inline(always)]
    pub fn b582(&self) -> B582_R {
        B582_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B583"]
    #[inline(always)]
    pub fn b583(&self) -> B583_R {
        B583_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B584"]
    #[inline(always)]
    pub fn b584(&self) -> B584_R {
        B584_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B585"]
    #[inline(always)]
    pub fn b585(&self) -> B585_R {
        B585_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B586"]
    #[inline(always)]
    pub fn b586(&self) -> B586_R {
        B586_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B587"]
    #[inline(always)]
    pub fn b587(&self) -> B587_R {
        B587_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B588"]
    #[inline(always)]
    pub fn b588(&self) -> B588_R {
        B588_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B589"]
    #[inline(always)]
    pub fn b589(&self) -> B589_R {
        B589_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B590"]
    #[inline(always)]
    pub fn b590(&self) -> B590_R {
        B590_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B591"]
    #[inline(always)]
    pub fn b591(&self) -> B591_R {
        B591_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B592"]
    #[inline(always)]
    pub fn b592(&self) -> B592_R {
        B592_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B593"]
    #[inline(always)]
    pub fn b593(&self) -> B593_R {
        B593_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B594"]
    #[inline(always)]
    pub fn b594(&self) -> B594_R {
        B594_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B595"]
    #[inline(always)]
    pub fn b595(&self) -> B595_R {
        B595_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B596"]
    #[inline(always)]
    pub fn b596(&self) -> B596_R {
        B596_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B597"]
    #[inline(always)]
    pub fn b597(&self) -> B597_R {
        B597_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B598"]
    #[inline(always)]
    pub fn b598(&self) -> B598_R {
        B598_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B599"]
    #[inline(always)]
    pub fn b599(&self) -> B599_R {
        B599_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B600"]
    #[inline(always)]
    pub fn b600(&self) -> B600_R {
        B600_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B601"]
    #[inline(always)]
    pub fn b601(&self) -> B601_R {
        B601_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B602"]
    #[inline(always)]
    pub fn b602(&self) -> B602_R {
        B602_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B603"]
    #[inline(always)]
    pub fn b603(&self) -> B603_R {
        B603_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B604"]
    #[inline(always)]
    pub fn b604(&self) -> B604_R {
        B604_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B605"]
    #[inline(always)]
    pub fn b605(&self) -> B605_R {
        B605_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B606"]
    #[inline(always)]
    pub fn b606(&self) -> B606_R {
        B606_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B607"]
    #[inline(always)]
    pub fn b607(&self) -> B607_R {
        B607_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B576"]
    #[inline(always)]
    pub fn b576(&mut self) -> B576_W {
        B576_W { w: self }
    }
    #[doc = "Bit 1 - B577"]
    #[inline(always)]
    pub fn b577(&mut self) -> B577_W {
        B577_W { w: self }
    }
    #[doc = "Bit 2 - B578"]
    #[inline(always)]
    pub fn b578(&mut self) -> B578_W {
        B578_W { w: self }
    }
    #[doc = "Bit 3 - B579"]
    #[inline(always)]
    pub fn b579(&mut self) -> B579_W {
        B579_W { w: self }
    }
    #[doc = "Bit 4 - B580"]
    #[inline(always)]
    pub fn b580(&mut self) -> B580_W {
        B580_W { w: self }
    }
    #[doc = "Bit 5 - B581"]
    #[inline(always)]
    pub fn b581(&mut self) -> B581_W {
        B581_W { w: self }
    }
    #[doc = "Bit 6 - B582"]
    #[inline(always)]
    pub fn b582(&mut self) -> B582_W {
        B582_W { w: self }
    }
    #[doc = "Bit 7 - B583"]
    #[inline(always)]
    pub fn b583(&mut self) -> B583_W {
        B583_W { w: self }
    }
    #[doc = "Bit 8 - B584"]
    #[inline(always)]
    pub fn b584(&mut self) -> B584_W {
        B584_W { w: self }
    }
    #[doc = "Bit 9 - B585"]
    #[inline(always)]
    pub fn b585(&mut self) -> B585_W {
        B585_W { w: self }
    }
    #[doc = "Bit 10 - B586"]
    #[inline(always)]
    pub fn b586(&mut self) -> B586_W {
        B586_W { w: self }
    }
    #[doc = "Bit 11 - B587"]
    #[inline(always)]
    pub fn b587(&mut self) -> B587_W {
        B587_W { w: self }
    }
    #[doc = "Bit 12 - B588"]
    #[inline(always)]
    pub fn b588(&mut self) -> B588_W {
        B588_W { w: self }
    }
    #[doc = "Bit 13 - B589"]
    #[inline(always)]
    pub fn b589(&mut self) -> B589_W {
        B589_W { w: self }
    }
    #[doc = "Bit 14 - B590"]
    #[inline(always)]
    pub fn b590(&mut self) -> B590_W {
        B590_W { w: self }
    }
    #[doc = "Bit 15 - B591"]
    #[inline(always)]
    pub fn b591(&mut self) -> B591_W {
        B591_W { w: self }
    }
    #[doc = "Bit 16 - B592"]
    #[inline(always)]
    pub fn b592(&mut self) -> B592_W {
        B592_W { w: self }
    }
    #[doc = "Bit 17 - B593"]
    #[inline(always)]
    pub fn b593(&mut self) -> B593_W {
        B593_W { w: self }
    }
    #[doc = "Bit 18 - B594"]
    #[inline(always)]
    pub fn b594(&mut self) -> B594_W {
        B594_W { w: self }
    }
    #[doc = "Bit 19 - B595"]
    #[inline(always)]
    pub fn b595(&mut self) -> B595_W {
        B595_W { w: self }
    }
    #[doc = "Bit 20 - B596"]
    #[inline(always)]
    pub fn b596(&mut self) -> B596_W {
        B596_W { w: self }
    }
    #[doc = "Bit 21 - B597"]
    #[inline(always)]
    pub fn b597(&mut self) -> B597_W {
        B597_W { w: self }
    }
    #[doc = "Bit 22 - B598"]
    #[inline(always)]
    pub fn b598(&mut self) -> B598_W {
        B598_W { w: self }
    }
    #[doc = "Bit 23 - B599"]
    #[inline(always)]
    pub fn b599(&mut self) -> B599_W {
        B599_W { w: self }
    }
    #[doc = "Bit 24 - B600"]
    #[inline(always)]
    pub fn b600(&mut self) -> B600_W {
        B600_W { w: self }
    }
    #[doc = "Bit 25 - B601"]
    #[inline(always)]
    pub fn b601(&mut self) -> B601_W {
        B601_W { w: self }
    }
    #[doc = "Bit 26 - B602"]
    #[inline(always)]
    pub fn b602(&mut self) -> B602_W {
        B602_W { w: self }
    }
    #[doc = "Bit 27 - B603"]
    #[inline(always)]
    pub fn b603(&mut self) -> B603_W {
        B603_W { w: self }
    }
    #[doc = "Bit 28 - B604"]
    #[inline(always)]
    pub fn b604(&mut self) -> B604_W {
        B604_W { w: self }
    }
    #[doc = "Bit 29 - B605"]
    #[inline(always)]
    pub fn b605(&mut self) -> B605_W {
        B605_W { w: self }
    }
    #[doc = "Bit 30 - B606"]
    #[inline(always)]
    pub fn b606(&mut self) -> B606_W {
        B606_W { w: self }
    }
    #[doc = "Bit 31 - B607"]
    #[inline(always)]
    pub fn b607(&mut self) -> B607_W {
        B607_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr18](index.html) module"]
pub struct MPCBB2_VCTR18_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr18::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr18::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR18 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
