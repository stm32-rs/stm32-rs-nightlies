#[doc = "Register `MPCBB2_VCTR19` reader"]
pub struct R(crate::R<MPCBB2_VCTR19_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR19_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR19_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR19_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR19` writer"]
pub struct W(crate::W<MPCBB2_VCTR19_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR19_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR19_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR19_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B608` reader - B608"]
pub struct B608_R(crate::FieldReader<bool, bool>);
impl B608_R {
    pub(crate) fn new(bits: bool) -> Self {
        B608_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B608_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B608` writer - B608"]
pub struct B608_W<'a> {
    w: &'a mut W,
}
impl<'a> B608_W<'a> {
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
#[doc = "Field `B609` reader - B609"]
pub struct B609_R(crate::FieldReader<bool, bool>);
impl B609_R {
    pub(crate) fn new(bits: bool) -> Self {
        B609_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B609_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B609` writer - B609"]
pub struct B609_W<'a> {
    w: &'a mut W,
}
impl<'a> B609_W<'a> {
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
#[doc = "Field `B610` reader - B610"]
pub struct B610_R(crate::FieldReader<bool, bool>);
impl B610_R {
    pub(crate) fn new(bits: bool) -> Self {
        B610_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B610_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B610` writer - B610"]
pub struct B610_W<'a> {
    w: &'a mut W,
}
impl<'a> B610_W<'a> {
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
#[doc = "Field `B611` reader - B611"]
pub struct B611_R(crate::FieldReader<bool, bool>);
impl B611_R {
    pub(crate) fn new(bits: bool) -> Self {
        B611_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B611_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B611` writer - B611"]
pub struct B611_W<'a> {
    w: &'a mut W,
}
impl<'a> B611_W<'a> {
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
#[doc = "Field `B612` reader - B612"]
pub struct B612_R(crate::FieldReader<bool, bool>);
impl B612_R {
    pub(crate) fn new(bits: bool) -> Self {
        B612_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B612_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B612` writer - B612"]
pub struct B612_W<'a> {
    w: &'a mut W,
}
impl<'a> B612_W<'a> {
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
#[doc = "Field `B613` reader - B613"]
pub struct B613_R(crate::FieldReader<bool, bool>);
impl B613_R {
    pub(crate) fn new(bits: bool) -> Self {
        B613_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B613_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B613` writer - B613"]
pub struct B613_W<'a> {
    w: &'a mut W,
}
impl<'a> B613_W<'a> {
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
#[doc = "Field `B614` reader - B614"]
pub struct B614_R(crate::FieldReader<bool, bool>);
impl B614_R {
    pub(crate) fn new(bits: bool) -> Self {
        B614_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B614_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B614` writer - B614"]
pub struct B614_W<'a> {
    w: &'a mut W,
}
impl<'a> B614_W<'a> {
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
#[doc = "Field `B615` reader - B615"]
pub struct B615_R(crate::FieldReader<bool, bool>);
impl B615_R {
    pub(crate) fn new(bits: bool) -> Self {
        B615_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B615_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B615` writer - B615"]
pub struct B615_W<'a> {
    w: &'a mut W,
}
impl<'a> B615_W<'a> {
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
#[doc = "Field `B616` reader - B616"]
pub struct B616_R(crate::FieldReader<bool, bool>);
impl B616_R {
    pub(crate) fn new(bits: bool) -> Self {
        B616_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B616_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B616` writer - B616"]
pub struct B616_W<'a> {
    w: &'a mut W,
}
impl<'a> B616_W<'a> {
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
#[doc = "Field `B617` reader - B617"]
pub struct B617_R(crate::FieldReader<bool, bool>);
impl B617_R {
    pub(crate) fn new(bits: bool) -> Self {
        B617_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B617_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B617` writer - B617"]
pub struct B617_W<'a> {
    w: &'a mut W,
}
impl<'a> B617_W<'a> {
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
#[doc = "Field `B618` reader - B618"]
pub struct B618_R(crate::FieldReader<bool, bool>);
impl B618_R {
    pub(crate) fn new(bits: bool) -> Self {
        B618_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B618_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B618` writer - B618"]
pub struct B618_W<'a> {
    w: &'a mut W,
}
impl<'a> B618_W<'a> {
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
#[doc = "Field `B619` reader - B619"]
pub struct B619_R(crate::FieldReader<bool, bool>);
impl B619_R {
    pub(crate) fn new(bits: bool) -> Self {
        B619_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B619_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B619` writer - B619"]
pub struct B619_W<'a> {
    w: &'a mut W,
}
impl<'a> B619_W<'a> {
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
#[doc = "Field `B620` reader - B620"]
pub struct B620_R(crate::FieldReader<bool, bool>);
impl B620_R {
    pub(crate) fn new(bits: bool) -> Self {
        B620_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B620_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B620` writer - B620"]
pub struct B620_W<'a> {
    w: &'a mut W,
}
impl<'a> B620_W<'a> {
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
#[doc = "Field `B621` reader - B621"]
pub struct B621_R(crate::FieldReader<bool, bool>);
impl B621_R {
    pub(crate) fn new(bits: bool) -> Self {
        B621_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B621_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B621` writer - B621"]
pub struct B621_W<'a> {
    w: &'a mut W,
}
impl<'a> B621_W<'a> {
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
#[doc = "Field `B622` reader - B622"]
pub struct B622_R(crate::FieldReader<bool, bool>);
impl B622_R {
    pub(crate) fn new(bits: bool) -> Self {
        B622_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B622_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B622` writer - B622"]
pub struct B622_W<'a> {
    w: &'a mut W,
}
impl<'a> B622_W<'a> {
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
#[doc = "Field `B623` reader - B623"]
pub struct B623_R(crate::FieldReader<bool, bool>);
impl B623_R {
    pub(crate) fn new(bits: bool) -> Self {
        B623_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B623_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B623` writer - B623"]
pub struct B623_W<'a> {
    w: &'a mut W,
}
impl<'a> B623_W<'a> {
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
#[doc = "Field `B624` reader - B624"]
pub struct B624_R(crate::FieldReader<bool, bool>);
impl B624_R {
    pub(crate) fn new(bits: bool) -> Self {
        B624_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B624_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B624` writer - B624"]
pub struct B624_W<'a> {
    w: &'a mut W,
}
impl<'a> B624_W<'a> {
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
#[doc = "Field `B625` reader - B625"]
pub struct B625_R(crate::FieldReader<bool, bool>);
impl B625_R {
    pub(crate) fn new(bits: bool) -> Self {
        B625_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B625_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B625` writer - B625"]
pub struct B625_W<'a> {
    w: &'a mut W,
}
impl<'a> B625_W<'a> {
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
#[doc = "Field `B626` reader - B626"]
pub struct B626_R(crate::FieldReader<bool, bool>);
impl B626_R {
    pub(crate) fn new(bits: bool) -> Self {
        B626_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B626_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B626` writer - B626"]
pub struct B626_W<'a> {
    w: &'a mut W,
}
impl<'a> B626_W<'a> {
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
#[doc = "Field `B627` reader - B627"]
pub struct B627_R(crate::FieldReader<bool, bool>);
impl B627_R {
    pub(crate) fn new(bits: bool) -> Self {
        B627_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B627_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B627` writer - B627"]
pub struct B627_W<'a> {
    w: &'a mut W,
}
impl<'a> B627_W<'a> {
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
#[doc = "Field `B628` reader - B628"]
pub struct B628_R(crate::FieldReader<bool, bool>);
impl B628_R {
    pub(crate) fn new(bits: bool) -> Self {
        B628_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B628_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B628` writer - B628"]
pub struct B628_W<'a> {
    w: &'a mut W,
}
impl<'a> B628_W<'a> {
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
#[doc = "Field `B629` reader - B629"]
pub struct B629_R(crate::FieldReader<bool, bool>);
impl B629_R {
    pub(crate) fn new(bits: bool) -> Self {
        B629_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B629_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B629` writer - B629"]
pub struct B629_W<'a> {
    w: &'a mut W,
}
impl<'a> B629_W<'a> {
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
#[doc = "Field `B630` reader - B630"]
pub struct B630_R(crate::FieldReader<bool, bool>);
impl B630_R {
    pub(crate) fn new(bits: bool) -> Self {
        B630_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B630_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B630` writer - B630"]
pub struct B630_W<'a> {
    w: &'a mut W,
}
impl<'a> B630_W<'a> {
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
#[doc = "Field `B631` reader - B631"]
pub struct B631_R(crate::FieldReader<bool, bool>);
impl B631_R {
    pub(crate) fn new(bits: bool) -> Self {
        B631_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B631_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B631` writer - B631"]
pub struct B631_W<'a> {
    w: &'a mut W,
}
impl<'a> B631_W<'a> {
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
#[doc = "Field `B632` reader - B632"]
pub struct B632_R(crate::FieldReader<bool, bool>);
impl B632_R {
    pub(crate) fn new(bits: bool) -> Self {
        B632_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B632_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B632` writer - B632"]
pub struct B632_W<'a> {
    w: &'a mut W,
}
impl<'a> B632_W<'a> {
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
#[doc = "Field `B633` reader - B633"]
pub struct B633_R(crate::FieldReader<bool, bool>);
impl B633_R {
    pub(crate) fn new(bits: bool) -> Self {
        B633_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B633_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B633` writer - B633"]
pub struct B633_W<'a> {
    w: &'a mut W,
}
impl<'a> B633_W<'a> {
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
#[doc = "Field `B634` reader - B634"]
pub struct B634_R(crate::FieldReader<bool, bool>);
impl B634_R {
    pub(crate) fn new(bits: bool) -> Self {
        B634_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B634_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B634` writer - B634"]
pub struct B634_W<'a> {
    w: &'a mut W,
}
impl<'a> B634_W<'a> {
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
#[doc = "Field `B635` reader - B635"]
pub struct B635_R(crate::FieldReader<bool, bool>);
impl B635_R {
    pub(crate) fn new(bits: bool) -> Self {
        B635_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B635_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B635` writer - B635"]
pub struct B635_W<'a> {
    w: &'a mut W,
}
impl<'a> B635_W<'a> {
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
#[doc = "Field `B636` reader - B636"]
pub struct B636_R(crate::FieldReader<bool, bool>);
impl B636_R {
    pub(crate) fn new(bits: bool) -> Self {
        B636_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B636_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B636` writer - B636"]
pub struct B636_W<'a> {
    w: &'a mut W,
}
impl<'a> B636_W<'a> {
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
#[doc = "Field `B637` reader - B637"]
pub struct B637_R(crate::FieldReader<bool, bool>);
impl B637_R {
    pub(crate) fn new(bits: bool) -> Self {
        B637_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B637_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B637` writer - B637"]
pub struct B637_W<'a> {
    w: &'a mut W,
}
impl<'a> B637_W<'a> {
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
#[doc = "Field `B638` reader - B638"]
pub struct B638_R(crate::FieldReader<bool, bool>);
impl B638_R {
    pub(crate) fn new(bits: bool) -> Self {
        B638_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B638_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B638` writer - B638"]
pub struct B638_W<'a> {
    w: &'a mut W,
}
impl<'a> B638_W<'a> {
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
#[doc = "Field `B639` reader - B639"]
pub struct B639_R(crate::FieldReader<bool, bool>);
impl B639_R {
    pub(crate) fn new(bits: bool) -> Self {
        B639_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B639_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B639` writer - B639"]
pub struct B639_W<'a> {
    w: &'a mut W,
}
impl<'a> B639_W<'a> {
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
    #[doc = "Bit 0 - B608"]
    #[inline(always)]
    pub fn b608(&self) -> B608_R {
        B608_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B609"]
    #[inline(always)]
    pub fn b609(&self) -> B609_R {
        B609_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B610"]
    #[inline(always)]
    pub fn b610(&self) -> B610_R {
        B610_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B611"]
    #[inline(always)]
    pub fn b611(&self) -> B611_R {
        B611_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B612"]
    #[inline(always)]
    pub fn b612(&self) -> B612_R {
        B612_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B613"]
    #[inline(always)]
    pub fn b613(&self) -> B613_R {
        B613_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B614"]
    #[inline(always)]
    pub fn b614(&self) -> B614_R {
        B614_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B615"]
    #[inline(always)]
    pub fn b615(&self) -> B615_R {
        B615_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B616"]
    #[inline(always)]
    pub fn b616(&self) -> B616_R {
        B616_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B617"]
    #[inline(always)]
    pub fn b617(&self) -> B617_R {
        B617_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B618"]
    #[inline(always)]
    pub fn b618(&self) -> B618_R {
        B618_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B619"]
    #[inline(always)]
    pub fn b619(&self) -> B619_R {
        B619_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B620"]
    #[inline(always)]
    pub fn b620(&self) -> B620_R {
        B620_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B621"]
    #[inline(always)]
    pub fn b621(&self) -> B621_R {
        B621_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B622"]
    #[inline(always)]
    pub fn b622(&self) -> B622_R {
        B622_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B623"]
    #[inline(always)]
    pub fn b623(&self) -> B623_R {
        B623_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B624"]
    #[inline(always)]
    pub fn b624(&self) -> B624_R {
        B624_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B625"]
    #[inline(always)]
    pub fn b625(&self) -> B625_R {
        B625_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B626"]
    #[inline(always)]
    pub fn b626(&self) -> B626_R {
        B626_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B627"]
    #[inline(always)]
    pub fn b627(&self) -> B627_R {
        B627_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B628"]
    #[inline(always)]
    pub fn b628(&self) -> B628_R {
        B628_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B629"]
    #[inline(always)]
    pub fn b629(&self) -> B629_R {
        B629_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B630"]
    #[inline(always)]
    pub fn b630(&self) -> B630_R {
        B630_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B631"]
    #[inline(always)]
    pub fn b631(&self) -> B631_R {
        B631_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B632"]
    #[inline(always)]
    pub fn b632(&self) -> B632_R {
        B632_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B633"]
    #[inline(always)]
    pub fn b633(&self) -> B633_R {
        B633_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B634"]
    #[inline(always)]
    pub fn b634(&self) -> B634_R {
        B634_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B635"]
    #[inline(always)]
    pub fn b635(&self) -> B635_R {
        B635_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B636"]
    #[inline(always)]
    pub fn b636(&self) -> B636_R {
        B636_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B637"]
    #[inline(always)]
    pub fn b637(&self) -> B637_R {
        B637_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B638"]
    #[inline(always)]
    pub fn b638(&self) -> B638_R {
        B638_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B639"]
    #[inline(always)]
    pub fn b639(&self) -> B639_R {
        B639_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B608"]
    #[inline(always)]
    pub fn b608(&mut self) -> B608_W {
        B608_W { w: self }
    }
    #[doc = "Bit 1 - B609"]
    #[inline(always)]
    pub fn b609(&mut self) -> B609_W {
        B609_W { w: self }
    }
    #[doc = "Bit 2 - B610"]
    #[inline(always)]
    pub fn b610(&mut self) -> B610_W {
        B610_W { w: self }
    }
    #[doc = "Bit 3 - B611"]
    #[inline(always)]
    pub fn b611(&mut self) -> B611_W {
        B611_W { w: self }
    }
    #[doc = "Bit 4 - B612"]
    #[inline(always)]
    pub fn b612(&mut self) -> B612_W {
        B612_W { w: self }
    }
    #[doc = "Bit 5 - B613"]
    #[inline(always)]
    pub fn b613(&mut self) -> B613_W {
        B613_W { w: self }
    }
    #[doc = "Bit 6 - B614"]
    #[inline(always)]
    pub fn b614(&mut self) -> B614_W {
        B614_W { w: self }
    }
    #[doc = "Bit 7 - B615"]
    #[inline(always)]
    pub fn b615(&mut self) -> B615_W {
        B615_W { w: self }
    }
    #[doc = "Bit 8 - B616"]
    #[inline(always)]
    pub fn b616(&mut self) -> B616_W {
        B616_W { w: self }
    }
    #[doc = "Bit 9 - B617"]
    #[inline(always)]
    pub fn b617(&mut self) -> B617_W {
        B617_W { w: self }
    }
    #[doc = "Bit 10 - B618"]
    #[inline(always)]
    pub fn b618(&mut self) -> B618_W {
        B618_W { w: self }
    }
    #[doc = "Bit 11 - B619"]
    #[inline(always)]
    pub fn b619(&mut self) -> B619_W {
        B619_W { w: self }
    }
    #[doc = "Bit 12 - B620"]
    #[inline(always)]
    pub fn b620(&mut self) -> B620_W {
        B620_W { w: self }
    }
    #[doc = "Bit 13 - B621"]
    #[inline(always)]
    pub fn b621(&mut self) -> B621_W {
        B621_W { w: self }
    }
    #[doc = "Bit 14 - B622"]
    #[inline(always)]
    pub fn b622(&mut self) -> B622_W {
        B622_W { w: self }
    }
    #[doc = "Bit 15 - B623"]
    #[inline(always)]
    pub fn b623(&mut self) -> B623_W {
        B623_W { w: self }
    }
    #[doc = "Bit 16 - B624"]
    #[inline(always)]
    pub fn b624(&mut self) -> B624_W {
        B624_W { w: self }
    }
    #[doc = "Bit 17 - B625"]
    #[inline(always)]
    pub fn b625(&mut self) -> B625_W {
        B625_W { w: self }
    }
    #[doc = "Bit 18 - B626"]
    #[inline(always)]
    pub fn b626(&mut self) -> B626_W {
        B626_W { w: self }
    }
    #[doc = "Bit 19 - B627"]
    #[inline(always)]
    pub fn b627(&mut self) -> B627_W {
        B627_W { w: self }
    }
    #[doc = "Bit 20 - B628"]
    #[inline(always)]
    pub fn b628(&mut self) -> B628_W {
        B628_W { w: self }
    }
    #[doc = "Bit 21 - B629"]
    #[inline(always)]
    pub fn b629(&mut self) -> B629_W {
        B629_W { w: self }
    }
    #[doc = "Bit 22 - B630"]
    #[inline(always)]
    pub fn b630(&mut self) -> B630_W {
        B630_W { w: self }
    }
    #[doc = "Bit 23 - B631"]
    #[inline(always)]
    pub fn b631(&mut self) -> B631_W {
        B631_W { w: self }
    }
    #[doc = "Bit 24 - B632"]
    #[inline(always)]
    pub fn b632(&mut self) -> B632_W {
        B632_W { w: self }
    }
    #[doc = "Bit 25 - B633"]
    #[inline(always)]
    pub fn b633(&mut self) -> B633_W {
        B633_W { w: self }
    }
    #[doc = "Bit 26 - B634"]
    #[inline(always)]
    pub fn b634(&mut self) -> B634_W {
        B634_W { w: self }
    }
    #[doc = "Bit 27 - B635"]
    #[inline(always)]
    pub fn b635(&mut self) -> B635_W {
        B635_W { w: self }
    }
    #[doc = "Bit 28 - B636"]
    #[inline(always)]
    pub fn b636(&mut self) -> B636_W {
        B636_W { w: self }
    }
    #[doc = "Bit 29 - B637"]
    #[inline(always)]
    pub fn b637(&mut self) -> B637_W {
        B637_W { w: self }
    }
    #[doc = "Bit 30 - B638"]
    #[inline(always)]
    pub fn b638(&mut self) -> B638_W {
        B638_W { w: self }
    }
    #[doc = "Bit 31 - B639"]
    #[inline(always)]
    pub fn b639(&mut self) -> B639_W {
        B639_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr19](index.html) module"]
pub struct MPCBB2_VCTR19_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr19::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR19_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr19::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR19_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR19 to value 0"]
impl crate::Resettable for MPCBB2_VCTR19_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
