#[doc = "Register `MPCBB2_VCTR5` reader"]
pub struct R(crate::R<MPCBB2_VCTR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR5` writer"]
pub struct W(crate::W<MPCBB2_VCTR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR5_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B160` reader - B160"]
pub struct B160_R(crate::FieldReader<bool, bool>);
impl B160_R {
    pub(crate) fn new(bits: bool) -> Self {
        B160_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B160_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B160` writer - B160"]
pub struct B160_W<'a> {
    w: &'a mut W,
}
impl<'a> B160_W<'a> {
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
#[doc = "Field `B161` reader - B161"]
pub struct B161_R(crate::FieldReader<bool, bool>);
impl B161_R {
    pub(crate) fn new(bits: bool) -> Self {
        B161_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B161_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B161` writer - B161"]
pub struct B161_W<'a> {
    w: &'a mut W,
}
impl<'a> B161_W<'a> {
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
#[doc = "Field `B162` reader - B162"]
pub struct B162_R(crate::FieldReader<bool, bool>);
impl B162_R {
    pub(crate) fn new(bits: bool) -> Self {
        B162_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B162_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B162` writer - B162"]
pub struct B162_W<'a> {
    w: &'a mut W,
}
impl<'a> B162_W<'a> {
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
#[doc = "Field `B163` reader - B163"]
pub struct B163_R(crate::FieldReader<bool, bool>);
impl B163_R {
    pub(crate) fn new(bits: bool) -> Self {
        B163_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B163_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B163` writer - B163"]
pub struct B163_W<'a> {
    w: &'a mut W,
}
impl<'a> B163_W<'a> {
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
#[doc = "Field `B164` reader - B164"]
pub struct B164_R(crate::FieldReader<bool, bool>);
impl B164_R {
    pub(crate) fn new(bits: bool) -> Self {
        B164_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B164_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B164` writer - B164"]
pub struct B164_W<'a> {
    w: &'a mut W,
}
impl<'a> B164_W<'a> {
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
#[doc = "Field `B165` reader - B165"]
pub struct B165_R(crate::FieldReader<bool, bool>);
impl B165_R {
    pub(crate) fn new(bits: bool) -> Self {
        B165_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B165_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B165` writer - B165"]
pub struct B165_W<'a> {
    w: &'a mut W,
}
impl<'a> B165_W<'a> {
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
#[doc = "Field `B166` reader - B166"]
pub struct B166_R(crate::FieldReader<bool, bool>);
impl B166_R {
    pub(crate) fn new(bits: bool) -> Self {
        B166_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B166_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B166` writer - B166"]
pub struct B166_W<'a> {
    w: &'a mut W,
}
impl<'a> B166_W<'a> {
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
#[doc = "Field `B167` reader - B167"]
pub struct B167_R(crate::FieldReader<bool, bool>);
impl B167_R {
    pub(crate) fn new(bits: bool) -> Self {
        B167_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B167_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B167` writer - B167"]
pub struct B167_W<'a> {
    w: &'a mut W,
}
impl<'a> B167_W<'a> {
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
#[doc = "Field `B168` reader - B168"]
pub struct B168_R(crate::FieldReader<bool, bool>);
impl B168_R {
    pub(crate) fn new(bits: bool) -> Self {
        B168_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B168_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B168` writer - B168"]
pub struct B168_W<'a> {
    w: &'a mut W,
}
impl<'a> B168_W<'a> {
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
#[doc = "Field `B169` reader - B169"]
pub struct B169_R(crate::FieldReader<bool, bool>);
impl B169_R {
    pub(crate) fn new(bits: bool) -> Self {
        B169_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B169_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B169` writer - B169"]
pub struct B169_W<'a> {
    w: &'a mut W,
}
impl<'a> B169_W<'a> {
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
#[doc = "Field `B170` reader - B170"]
pub struct B170_R(crate::FieldReader<bool, bool>);
impl B170_R {
    pub(crate) fn new(bits: bool) -> Self {
        B170_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B170_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B170` writer - B170"]
pub struct B170_W<'a> {
    w: &'a mut W,
}
impl<'a> B170_W<'a> {
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
#[doc = "Field `B171` reader - B171"]
pub struct B171_R(crate::FieldReader<bool, bool>);
impl B171_R {
    pub(crate) fn new(bits: bool) -> Self {
        B171_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B171_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B171` writer - B171"]
pub struct B171_W<'a> {
    w: &'a mut W,
}
impl<'a> B171_W<'a> {
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
#[doc = "Field `B172` reader - B172"]
pub struct B172_R(crate::FieldReader<bool, bool>);
impl B172_R {
    pub(crate) fn new(bits: bool) -> Self {
        B172_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B172_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B172` writer - B172"]
pub struct B172_W<'a> {
    w: &'a mut W,
}
impl<'a> B172_W<'a> {
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
#[doc = "Field `B173` reader - B173"]
pub struct B173_R(crate::FieldReader<bool, bool>);
impl B173_R {
    pub(crate) fn new(bits: bool) -> Self {
        B173_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B173_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B173` writer - B173"]
pub struct B173_W<'a> {
    w: &'a mut W,
}
impl<'a> B173_W<'a> {
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
#[doc = "Field `B174` reader - B174"]
pub struct B174_R(crate::FieldReader<bool, bool>);
impl B174_R {
    pub(crate) fn new(bits: bool) -> Self {
        B174_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B174_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B174` writer - B174"]
pub struct B174_W<'a> {
    w: &'a mut W,
}
impl<'a> B174_W<'a> {
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
#[doc = "Field `B175` reader - B175"]
pub struct B175_R(crate::FieldReader<bool, bool>);
impl B175_R {
    pub(crate) fn new(bits: bool) -> Self {
        B175_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B175_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B175` writer - B175"]
pub struct B175_W<'a> {
    w: &'a mut W,
}
impl<'a> B175_W<'a> {
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
#[doc = "Field `B176` reader - B176"]
pub struct B176_R(crate::FieldReader<bool, bool>);
impl B176_R {
    pub(crate) fn new(bits: bool) -> Self {
        B176_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B176_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B176` writer - B176"]
pub struct B176_W<'a> {
    w: &'a mut W,
}
impl<'a> B176_W<'a> {
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
#[doc = "Field `B177` reader - B177"]
pub struct B177_R(crate::FieldReader<bool, bool>);
impl B177_R {
    pub(crate) fn new(bits: bool) -> Self {
        B177_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B177_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B177` writer - B177"]
pub struct B177_W<'a> {
    w: &'a mut W,
}
impl<'a> B177_W<'a> {
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
#[doc = "Field `B178` reader - B178"]
pub struct B178_R(crate::FieldReader<bool, bool>);
impl B178_R {
    pub(crate) fn new(bits: bool) -> Self {
        B178_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B178_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B178` writer - B178"]
pub struct B178_W<'a> {
    w: &'a mut W,
}
impl<'a> B178_W<'a> {
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
#[doc = "Field `B179` reader - B179"]
pub struct B179_R(crate::FieldReader<bool, bool>);
impl B179_R {
    pub(crate) fn new(bits: bool) -> Self {
        B179_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B179_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B179` writer - B179"]
pub struct B179_W<'a> {
    w: &'a mut W,
}
impl<'a> B179_W<'a> {
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
#[doc = "Field `B180` reader - B180"]
pub struct B180_R(crate::FieldReader<bool, bool>);
impl B180_R {
    pub(crate) fn new(bits: bool) -> Self {
        B180_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B180_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B180` writer - B180"]
pub struct B180_W<'a> {
    w: &'a mut W,
}
impl<'a> B180_W<'a> {
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
#[doc = "Field `B181` reader - B181"]
pub struct B181_R(crate::FieldReader<bool, bool>);
impl B181_R {
    pub(crate) fn new(bits: bool) -> Self {
        B181_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B181_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B181` writer - B181"]
pub struct B181_W<'a> {
    w: &'a mut W,
}
impl<'a> B181_W<'a> {
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
#[doc = "Field `B182` reader - B182"]
pub struct B182_R(crate::FieldReader<bool, bool>);
impl B182_R {
    pub(crate) fn new(bits: bool) -> Self {
        B182_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B182_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B182` writer - B182"]
pub struct B182_W<'a> {
    w: &'a mut W,
}
impl<'a> B182_W<'a> {
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
#[doc = "Field `B183` reader - B183"]
pub struct B183_R(crate::FieldReader<bool, bool>);
impl B183_R {
    pub(crate) fn new(bits: bool) -> Self {
        B183_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B183_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B183` writer - B183"]
pub struct B183_W<'a> {
    w: &'a mut W,
}
impl<'a> B183_W<'a> {
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
#[doc = "Field `B184` reader - B184"]
pub struct B184_R(crate::FieldReader<bool, bool>);
impl B184_R {
    pub(crate) fn new(bits: bool) -> Self {
        B184_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B184_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B184` writer - B184"]
pub struct B184_W<'a> {
    w: &'a mut W,
}
impl<'a> B184_W<'a> {
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
#[doc = "Field `B185` reader - B185"]
pub struct B185_R(crate::FieldReader<bool, bool>);
impl B185_R {
    pub(crate) fn new(bits: bool) -> Self {
        B185_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B185_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B185` writer - B185"]
pub struct B185_W<'a> {
    w: &'a mut W,
}
impl<'a> B185_W<'a> {
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
#[doc = "Field `B186` reader - B186"]
pub struct B186_R(crate::FieldReader<bool, bool>);
impl B186_R {
    pub(crate) fn new(bits: bool) -> Self {
        B186_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B186_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B186` writer - B186"]
pub struct B186_W<'a> {
    w: &'a mut W,
}
impl<'a> B186_W<'a> {
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
#[doc = "Field `B187` reader - B187"]
pub struct B187_R(crate::FieldReader<bool, bool>);
impl B187_R {
    pub(crate) fn new(bits: bool) -> Self {
        B187_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B187_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B187` writer - B187"]
pub struct B187_W<'a> {
    w: &'a mut W,
}
impl<'a> B187_W<'a> {
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
#[doc = "Field `B188` reader - B188"]
pub struct B188_R(crate::FieldReader<bool, bool>);
impl B188_R {
    pub(crate) fn new(bits: bool) -> Self {
        B188_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B188_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B188` writer - B188"]
pub struct B188_W<'a> {
    w: &'a mut W,
}
impl<'a> B188_W<'a> {
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
#[doc = "Field `B189` reader - B189"]
pub struct B189_R(crate::FieldReader<bool, bool>);
impl B189_R {
    pub(crate) fn new(bits: bool) -> Self {
        B189_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B189_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B189` writer - B189"]
pub struct B189_W<'a> {
    w: &'a mut W,
}
impl<'a> B189_W<'a> {
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
#[doc = "Field `B190` reader - B190"]
pub struct B190_R(crate::FieldReader<bool, bool>);
impl B190_R {
    pub(crate) fn new(bits: bool) -> Self {
        B190_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B190_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B190` writer - B190"]
pub struct B190_W<'a> {
    w: &'a mut W,
}
impl<'a> B190_W<'a> {
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
#[doc = "Field `B191` reader - B191"]
pub struct B191_R(crate::FieldReader<bool, bool>);
impl B191_R {
    pub(crate) fn new(bits: bool) -> Self {
        B191_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B191_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B191` writer - B191"]
pub struct B191_W<'a> {
    w: &'a mut W,
}
impl<'a> B191_W<'a> {
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
    #[doc = "Bit 0 - B160"]
    #[inline(always)]
    pub fn b160(&self) -> B160_R {
        B160_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B161"]
    #[inline(always)]
    pub fn b161(&self) -> B161_R {
        B161_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B162"]
    #[inline(always)]
    pub fn b162(&self) -> B162_R {
        B162_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B163"]
    #[inline(always)]
    pub fn b163(&self) -> B163_R {
        B163_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B164"]
    #[inline(always)]
    pub fn b164(&self) -> B164_R {
        B164_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B165"]
    #[inline(always)]
    pub fn b165(&self) -> B165_R {
        B165_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B166"]
    #[inline(always)]
    pub fn b166(&self) -> B166_R {
        B166_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B167"]
    #[inline(always)]
    pub fn b167(&self) -> B167_R {
        B167_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B168"]
    #[inline(always)]
    pub fn b168(&self) -> B168_R {
        B168_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B169"]
    #[inline(always)]
    pub fn b169(&self) -> B169_R {
        B169_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B170"]
    #[inline(always)]
    pub fn b170(&self) -> B170_R {
        B170_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B171"]
    #[inline(always)]
    pub fn b171(&self) -> B171_R {
        B171_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B172"]
    #[inline(always)]
    pub fn b172(&self) -> B172_R {
        B172_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B173"]
    #[inline(always)]
    pub fn b173(&self) -> B173_R {
        B173_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B174"]
    #[inline(always)]
    pub fn b174(&self) -> B174_R {
        B174_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B175"]
    #[inline(always)]
    pub fn b175(&self) -> B175_R {
        B175_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B176"]
    #[inline(always)]
    pub fn b176(&self) -> B176_R {
        B176_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B177"]
    #[inline(always)]
    pub fn b177(&self) -> B177_R {
        B177_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B178"]
    #[inline(always)]
    pub fn b178(&self) -> B178_R {
        B178_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B179"]
    #[inline(always)]
    pub fn b179(&self) -> B179_R {
        B179_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B180"]
    #[inline(always)]
    pub fn b180(&self) -> B180_R {
        B180_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B181"]
    #[inline(always)]
    pub fn b181(&self) -> B181_R {
        B181_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B182"]
    #[inline(always)]
    pub fn b182(&self) -> B182_R {
        B182_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B183"]
    #[inline(always)]
    pub fn b183(&self) -> B183_R {
        B183_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B184"]
    #[inline(always)]
    pub fn b184(&self) -> B184_R {
        B184_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B185"]
    #[inline(always)]
    pub fn b185(&self) -> B185_R {
        B185_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B186"]
    #[inline(always)]
    pub fn b186(&self) -> B186_R {
        B186_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B187"]
    #[inline(always)]
    pub fn b187(&self) -> B187_R {
        B187_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B188"]
    #[inline(always)]
    pub fn b188(&self) -> B188_R {
        B188_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B189"]
    #[inline(always)]
    pub fn b189(&self) -> B189_R {
        B189_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B190"]
    #[inline(always)]
    pub fn b190(&self) -> B190_R {
        B190_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B191"]
    #[inline(always)]
    pub fn b191(&self) -> B191_R {
        B191_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B160"]
    #[inline(always)]
    pub fn b160(&mut self) -> B160_W {
        B160_W { w: self }
    }
    #[doc = "Bit 1 - B161"]
    #[inline(always)]
    pub fn b161(&mut self) -> B161_W {
        B161_W { w: self }
    }
    #[doc = "Bit 2 - B162"]
    #[inline(always)]
    pub fn b162(&mut self) -> B162_W {
        B162_W { w: self }
    }
    #[doc = "Bit 3 - B163"]
    #[inline(always)]
    pub fn b163(&mut self) -> B163_W {
        B163_W { w: self }
    }
    #[doc = "Bit 4 - B164"]
    #[inline(always)]
    pub fn b164(&mut self) -> B164_W {
        B164_W { w: self }
    }
    #[doc = "Bit 5 - B165"]
    #[inline(always)]
    pub fn b165(&mut self) -> B165_W {
        B165_W { w: self }
    }
    #[doc = "Bit 6 - B166"]
    #[inline(always)]
    pub fn b166(&mut self) -> B166_W {
        B166_W { w: self }
    }
    #[doc = "Bit 7 - B167"]
    #[inline(always)]
    pub fn b167(&mut self) -> B167_W {
        B167_W { w: self }
    }
    #[doc = "Bit 8 - B168"]
    #[inline(always)]
    pub fn b168(&mut self) -> B168_W {
        B168_W { w: self }
    }
    #[doc = "Bit 9 - B169"]
    #[inline(always)]
    pub fn b169(&mut self) -> B169_W {
        B169_W { w: self }
    }
    #[doc = "Bit 10 - B170"]
    #[inline(always)]
    pub fn b170(&mut self) -> B170_W {
        B170_W { w: self }
    }
    #[doc = "Bit 11 - B171"]
    #[inline(always)]
    pub fn b171(&mut self) -> B171_W {
        B171_W { w: self }
    }
    #[doc = "Bit 12 - B172"]
    #[inline(always)]
    pub fn b172(&mut self) -> B172_W {
        B172_W { w: self }
    }
    #[doc = "Bit 13 - B173"]
    #[inline(always)]
    pub fn b173(&mut self) -> B173_W {
        B173_W { w: self }
    }
    #[doc = "Bit 14 - B174"]
    #[inline(always)]
    pub fn b174(&mut self) -> B174_W {
        B174_W { w: self }
    }
    #[doc = "Bit 15 - B175"]
    #[inline(always)]
    pub fn b175(&mut self) -> B175_W {
        B175_W { w: self }
    }
    #[doc = "Bit 16 - B176"]
    #[inline(always)]
    pub fn b176(&mut self) -> B176_W {
        B176_W { w: self }
    }
    #[doc = "Bit 17 - B177"]
    #[inline(always)]
    pub fn b177(&mut self) -> B177_W {
        B177_W { w: self }
    }
    #[doc = "Bit 18 - B178"]
    #[inline(always)]
    pub fn b178(&mut self) -> B178_W {
        B178_W { w: self }
    }
    #[doc = "Bit 19 - B179"]
    #[inline(always)]
    pub fn b179(&mut self) -> B179_W {
        B179_W { w: self }
    }
    #[doc = "Bit 20 - B180"]
    #[inline(always)]
    pub fn b180(&mut self) -> B180_W {
        B180_W { w: self }
    }
    #[doc = "Bit 21 - B181"]
    #[inline(always)]
    pub fn b181(&mut self) -> B181_W {
        B181_W { w: self }
    }
    #[doc = "Bit 22 - B182"]
    #[inline(always)]
    pub fn b182(&mut self) -> B182_W {
        B182_W { w: self }
    }
    #[doc = "Bit 23 - B183"]
    #[inline(always)]
    pub fn b183(&mut self) -> B183_W {
        B183_W { w: self }
    }
    #[doc = "Bit 24 - B184"]
    #[inline(always)]
    pub fn b184(&mut self) -> B184_W {
        B184_W { w: self }
    }
    #[doc = "Bit 25 - B185"]
    #[inline(always)]
    pub fn b185(&mut self) -> B185_W {
        B185_W { w: self }
    }
    #[doc = "Bit 26 - B186"]
    #[inline(always)]
    pub fn b186(&mut self) -> B186_W {
        B186_W { w: self }
    }
    #[doc = "Bit 27 - B187"]
    #[inline(always)]
    pub fn b187(&mut self) -> B187_W {
        B187_W { w: self }
    }
    #[doc = "Bit 28 - B188"]
    #[inline(always)]
    pub fn b188(&mut self) -> B188_W {
        B188_W { w: self }
    }
    #[doc = "Bit 29 - B189"]
    #[inline(always)]
    pub fn b189(&mut self) -> B189_W {
        B189_W { w: self }
    }
    #[doc = "Bit 30 - B190"]
    #[inline(always)]
    pub fn b190(&mut self) -> B190_W {
        B190_W { w: self }
    }
    #[doc = "Bit 31 - B191"]
    #[inline(always)]
    pub fn b191(&mut self) -> B191_W {
        B191_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr5](index.html) module"]
pub struct MPCBB2_VCTR5_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr5::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr5::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR5 to value 0"]
impl crate::Resettable for MPCBB2_VCTR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
