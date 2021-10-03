#[doc = "Register `MPCBB1_VCTR20` reader"]
pub struct R(crate::R<MPCBB1_VCTR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR20` writer"]
pub struct W(crate::W<MPCBB1_VCTR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR20_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B640` reader - B640"]
pub struct B640_R(crate::FieldReader<bool, bool>);
impl B640_R {
    pub(crate) fn new(bits: bool) -> Self {
        B640_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B640_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B640` writer - B640"]
pub struct B640_W<'a> {
    w: &'a mut W,
}
impl<'a> B640_W<'a> {
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
#[doc = "Field `B641` reader - B641"]
pub struct B641_R(crate::FieldReader<bool, bool>);
impl B641_R {
    pub(crate) fn new(bits: bool) -> Self {
        B641_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B641_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B641` writer - B641"]
pub struct B641_W<'a> {
    w: &'a mut W,
}
impl<'a> B641_W<'a> {
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
#[doc = "Field `B642` reader - B642"]
pub struct B642_R(crate::FieldReader<bool, bool>);
impl B642_R {
    pub(crate) fn new(bits: bool) -> Self {
        B642_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B642_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B642` writer - B642"]
pub struct B642_W<'a> {
    w: &'a mut W,
}
impl<'a> B642_W<'a> {
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
#[doc = "Field `B643` reader - B643"]
pub struct B643_R(crate::FieldReader<bool, bool>);
impl B643_R {
    pub(crate) fn new(bits: bool) -> Self {
        B643_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B643_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B643` writer - B643"]
pub struct B643_W<'a> {
    w: &'a mut W,
}
impl<'a> B643_W<'a> {
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
#[doc = "Field `B644` reader - B644"]
pub struct B644_R(crate::FieldReader<bool, bool>);
impl B644_R {
    pub(crate) fn new(bits: bool) -> Self {
        B644_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B644_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B644` writer - B644"]
pub struct B644_W<'a> {
    w: &'a mut W,
}
impl<'a> B644_W<'a> {
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
#[doc = "Field `B645` reader - B645"]
pub struct B645_R(crate::FieldReader<bool, bool>);
impl B645_R {
    pub(crate) fn new(bits: bool) -> Self {
        B645_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B645_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B645` writer - B645"]
pub struct B645_W<'a> {
    w: &'a mut W,
}
impl<'a> B645_W<'a> {
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
#[doc = "Field `B646` reader - B646"]
pub struct B646_R(crate::FieldReader<bool, bool>);
impl B646_R {
    pub(crate) fn new(bits: bool) -> Self {
        B646_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B646_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B646` writer - B646"]
pub struct B646_W<'a> {
    w: &'a mut W,
}
impl<'a> B646_W<'a> {
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
#[doc = "Field `B647` reader - B647"]
pub struct B647_R(crate::FieldReader<bool, bool>);
impl B647_R {
    pub(crate) fn new(bits: bool) -> Self {
        B647_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B647_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B647` writer - B647"]
pub struct B647_W<'a> {
    w: &'a mut W,
}
impl<'a> B647_W<'a> {
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
#[doc = "Field `B648` reader - B648"]
pub struct B648_R(crate::FieldReader<bool, bool>);
impl B648_R {
    pub(crate) fn new(bits: bool) -> Self {
        B648_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B648_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B648` writer - B648"]
pub struct B648_W<'a> {
    w: &'a mut W,
}
impl<'a> B648_W<'a> {
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
#[doc = "Field `B649` reader - B649"]
pub struct B649_R(crate::FieldReader<bool, bool>);
impl B649_R {
    pub(crate) fn new(bits: bool) -> Self {
        B649_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B649_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B649` writer - B649"]
pub struct B649_W<'a> {
    w: &'a mut W,
}
impl<'a> B649_W<'a> {
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
#[doc = "Field `B650` reader - B650"]
pub struct B650_R(crate::FieldReader<bool, bool>);
impl B650_R {
    pub(crate) fn new(bits: bool) -> Self {
        B650_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B650_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B650` writer - B650"]
pub struct B650_W<'a> {
    w: &'a mut W,
}
impl<'a> B650_W<'a> {
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
#[doc = "Field `B651` reader - B651"]
pub struct B651_R(crate::FieldReader<bool, bool>);
impl B651_R {
    pub(crate) fn new(bits: bool) -> Self {
        B651_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B651_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B651` writer - B651"]
pub struct B651_W<'a> {
    w: &'a mut W,
}
impl<'a> B651_W<'a> {
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
#[doc = "Field `B652` reader - B652"]
pub struct B652_R(crate::FieldReader<bool, bool>);
impl B652_R {
    pub(crate) fn new(bits: bool) -> Self {
        B652_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B652_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B652` writer - B652"]
pub struct B652_W<'a> {
    w: &'a mut W,
}
impl<'a> B652_W<'a> {
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
#[doc = "Field `B653` reader - B653"]
pub struct B653_R(crate::FieldReader<bool, bool>);
impl B653_R {
    pub(crate) fn new(bits: bool) -> Self {
        B653_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B653_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B653` writer - B653"]
pub struct B653_W<'a> {
    w: &'a mut W,
}
impl<'a> B653_W<'a> {
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
#[doc = "Field `B654` reader - B654"]
pub struct B654_R(crate::FieldReader<bool, bool>);
impl B654_R {
    pub(crate) fn new(bits: bool) -> Self {
        B654_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B654_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B654` writer - B654"]
pub struct B654_W<'a> {
    w: &'a mut W,
}
impl<'a> B654_W<'a> {
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
#[doc = "Field `B655` reader - B655"]
pub struct B655_R(crate::FieldReader<bool, bool>);
impl B655_R {
    pub(crate) fn new(bits: bool) -> Self {
        B655_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B655_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B655` writer - B655"]
pub struct B655_W<'a> {
    w: &'a mut W,
}
impl<'a> B655_W<'a> {
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
#[doc = "Field `B656` reader - B656"]
pub struct B656_R(crate::FieldReader<bool, bool>);
impl B656_R {
    pub(crate) fn new(bits: bool) -> Self {
        B656_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B656_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B656` writer - B656"]
pub struct B656_W<'a> {
    w: &'a mut W,
}
impl<'a> B656_W<'a> {
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
#[doc = "Field `B657` reader - B657"]
pub struct B657_R(crate::FieldReader<bool, bool>);
impl B657_R {
    pub(crate) fn new(bits: bool) -> Self {
        B657_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B657_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B657` writer - B657"]
pub struct B657_W<'a> {
    w: &'a mut W,
}
impl<'a> B657_W<'a> {
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
#[doc = "Field `B658` reader - B658"]
pub struct B658_R(crate::FieldReader<bool, bool>);
impl B658_R {
    pub(crate) fn new(bits: bool) -> Self {
        B658_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B658_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B658` writer - B658"]
pub struct B658_W<'a> {
    w: &'a mut W,
}
impl<'a> B658_W<'a> {
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
#[doc = "Field `B659` reader - B659"]
pub struct B659_R(crate::FieldReader<bool, bool>);
impl B659_R {
    pub(crate) fn new(bits: bool) -> Self {
        B659_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B659_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B659` writer - B659"]
pub struct B659_W<'a> {
    w: &'a mut W,
}
impl<'a> B659_W<'a> {
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
#[doc = "Field `B660` reader - B660"]
pub struct B660_R(crate::FieldReader<bool, bool>);
impl B660_R {
    pub(crate) fn new(bits: bool) -> Self {
        B660_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B660_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B660` writer - B660"]
pub struct B660_W<'a> {
    w: &'a mut W,
}
impl<'a> B660_W<'a> {
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
#[doc = "Field `B661` reader - B661"]
pub struct B661_R(crate::FieldReader<bool, bool>);
impl B661_R {
    pub(crate) fn new(bits: bool) -> Self {
        B661_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B661_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B661` writer - B661"]
pub struct B661_W<'a> {
    w: &'a mut W,
}
impl<'a> B661_W<'a> {
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
#[doc = "Field `B662` reader - B662"]
pub struct B662_R(crate::FieldReader<bool, bool>);
impl B662_R {
    pub(crate) fn new(bits: bool) -> Self {
        B662_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B662_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B662` writer - B662"]
pub struct B662_W<'a> {
    w: &'a mut W,
}
impl<'a> B662_W<'a> {
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
#[doc = "Field `B663` reader - B663"]
pub struct B663_R(crate::FieldReader<bool, bool>);
impl B663_R {
    pub(crate) fn new(bits: bool) -> Self {
        B663_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B663_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B663` writer - B663"]
pub struct B663_W<'a> {
    w: &'a mut W,
}
impl<'a> B663_W<'a> {
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
#[doc = "Field `B664` reader - B664"]
pub struct B664_R(crate::FieldReader<bool, bool>);
impl B664_R {
    pub(crate) fn new(bits: bool) -> Self {
        B664_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B664_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B664` writer - B664"]
pub struct B664_W<'a> {
    w: &'a mut W,
}
impl<'a> B664_W<'a> {
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
#[doc = "Field `B665` reader - B665"]
pub struct B665_R(crate::FieldReader<bool, bool>);
impl B665_R {
    pub(crate) fn new(bits: bool) -> Self {
        B665_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B665_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B665` writer - B665"]
pub struct B665_W<'a> {
    w: &'a mut W,
}
impl<'a> B665_W<'a> {
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
#[doc = "Field `B666` reader - B666"]
pub struct B666_R(crate::FieldReader<bool, bool>);
impl B666_R {
    pub(crate) fn new(bits: bool) -> Self {
        B666_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B666_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B666` writer - B666"]
pub struct B666_W<'a> {
    w: &'a mut W,
}
impl<'a> B666_W<'a> {
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
#[doc = "Field `B667` reader - B667"]
pub struct B667_R(crate::FieldReader<bool, bool>);
impl B667_R {
    pub(crate) fn new(bits: bool) -> Self {
        B667_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B667_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B667` writer - B667"]
pub struct B667_W<'a> {
    w: &'a mut W,
}
impl<'a> B667_W<'a> {
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
#[doc = "Field `B668` reader - B668"]
pub struct B668_R(crate::FieldReader<bool, bool>);
impl B668_R {
    pub(crate) fn new(bits: bool) -> Self {
        B668_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B668_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B668` writer - B668"]
pub struct B668_W<'a> {
    w: &'a mut W,
}
impl<'a> B668_W<'a> {
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
#[doc = "Field `B669` reader - B669"]
pub struct B669_R(crate::FieldReader<bool, bool>);
impl B669_R {
    pub(crate) fn new(bits: bool) -> Self {
        B669_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B669_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B669` writer - B669"]
pub struct B669_W<'a> {
    w: &'a mut W,
}
impl<'a> B669_W<'a> {
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
#[doc = "Field `B670` reader - B670"]
pub struct B670_R(crate::FieldReader<bool, bool>);
impl B670_R {
    pub(crate) fn new(bits: bool) -> Self {
        B670_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B670_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B670` writer - B670"]
pub struct B670_W<'a> {
    w: &'a mut W,
}
impl<'a> B670_W<'a> {
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
#[doc = "Field `B671` reader - B671"]
pub struct B671_R(crate::FieldReader<bool, bool>);
impl B671_R {
    pub(crate) fn new(bits: bool) -> Self {
        B671_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B671_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B671` writer - B671"]
pub struct B671_W<'a> {
    w: &'a mut W,
}
impl<'a> B671_W<'a> {
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
    #[doc = "Bit 0 - B640"]
    #[inline(always)]
    pub fn b640(&self) -> B640_R {
        B640_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B641"]
    #[inline(always)]
    pub fn b641(&self) -> B641_R {
        B641_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B642"]
    #[inline(always)]
    pub fn b642(&self) -> B642_R {
        B642_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B643"]
    #[inline(always)]
    pub fn b643(&self) -> B643_R {
        B643_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B644"]
    #[inline(always)]
    pub fn b644(&self) -> B644_R {
        B644_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B645"]
    #[inline(always)]
    pub fn b645(&self) -> B645_R {
        B645_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B646"]
    #[inline(always)]
    pub fn b646(&self) -> B646_R {
        B646_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B647"]
    #[inline(always)]
    pub fn b647(&self) -> B647_R {
        B647_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B648"]
    #[inline(always)]
    pub fn b648(&self) -> B648_R {
        B648_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B649"]
    #[inline(always)]
    pub fn b649(&self) -> B649_R {
        B649_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B650"]
    #[inline(always)]
    pub fn b650(&self) -> B650_R {
        B650_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B651"]
    #[inline(always)]
    pub fn b651(&self) -> B651_R {
        B651_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B652"]
    #[inline(always)]
    pub fn b652(&self) -> B652_R {
        B652_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B653"]
    #[inline(always)]
    pub fn b653(&self) -> B653_R {
        B653_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B654"]
    #[inline(always)]
    pub fn b654(&self) -> B654_R {
        B654_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B655"]
    #[inline(always)]
    pub fn b655(&self) -> B655_R {
        B655_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B656"]
    #[inline(always)]
    pub fn b656(&self) -> B656_R {
        B656_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B657"]
    #[inline(always)]
    pub fn b657(&self) -> B657_R {
        B657_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B658"]
    #[inline(always)]
    pub fn b658(&self) -> B658_R {
        B658_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B659"]
    #[inline(always)]
    pub fn b659(&self) -> B659_R {
        B659_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B660"]
    #[inline(always)]
    pub fn b660(&self) -> B660_R {
        B660_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B661"]
    #[inline(always)]
    pub fn b661(&self) -> B661_R {
        B661_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B662"]
    #[inline(always)]
    pub fn b662(&self) -> B662_R {
        B662_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B663"]
    #[inline(always)]
    pub fn b663(&self) -> B663_R {
        B663_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B664"]
    #[inline(always)]
    pub fn b664(&self) -> B664_R {
        B664_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B665"]
    #[inline(always)]
    pub fn b665(&self) -> B665_R {
        B665_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B666"]
    #[inline(always)]
    pub fn b666(&self) -> B666_R {
        B666_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B667"]
    #[inline(always)]
    pub fn b667(&self) -> B667_R {
        B667_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B668"]
    #[inline(always)]
    pub fn b668(&self) -> B668_R {
        B668_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B669"]
    #[inline(always)]
    pub fn b669(&self) -> B669_R {
        B669_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B670"]
    #[inline(always)]
    pub fn b670(&self) -> B670_R {
        B670_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B671"]
    #[inline(always)]
    pub fn b671(&self) -> B671_R {
        B671_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B640"]
    #[inline(always)]
    pub fn b640(&mut self) -> B640_W {
        B640_W { w: self }
    }
    #[doc = "Bit 1 - B641"]
    #[inline(always)]
    pub fn b641(&mut self) -> B641_W {
        B641_W { w: self }
    }
    #[doc = "Bit 2 - B642"]
    #[inline(always)]
    pub fn b642(&mut self) -> B642_W {
        B642_W { w: self }
    }
    #[doc = "Bit 3 - B643"]
    #[inline(always)]
    pub fn b643(&mut self) -> B643_W {
        B643_W { w: self }
    }
    #[doc = "Bit 4 - B644"]
    #[inline(always)]
    pub fn b644(&mut self) -> B644_W {
        B644_W { w: self }
    }
    #[doc = "Bit 5 - B645"]
    #[inline(always)]
    pub fn b645(&mut self) -> B645_W {
        B645_W { w: self }
    }
    #[doc = "Bit 6 - B646"]
    #[inline(always)]
    pub fn b646(&mut self) -> B646_W {
        B646_W { w: self }
    }
    #[doc = "Bit 7 - B647"]
    #[inline(always)]
    pub fn b647(&mut self) -> B647_W {
        B647_W { w: self }
    }
    #[doc = "Bit 8 - B648"]
    #[inline(always)]
    pub fn b648(&mut self) -> B648_W {
        B648_W { w: self }
    }
    #[doc = "Bit 9 - B649"]
    #[inline(always)]
    pub fn b649(&mut self) -> B649_W {
        B649_W { w: self }
    }
    #[doc = "Bit 10 - B650"]
    #[inline(always)]
    pub fn b650(&mut self) -> B650_W {
        B650_W { w: self }
    }
    #[doc = "Bit 11 - B651"]
    #[inline(always)]
    pub fn b651(&mut self) -> B651_W {
        B651_W { w: self }
    }
    #[doc = "Bit 12 - B652"]
    #[inline(always)]
    pub fn b652(&mut self) -> B652_W {
        B652_W { w: self }
    }
    #[doc = "Bit 13 - B653"]
    #[inline(always)]
    pub fn b653(&mut self) -> B653_W {
        B653_W { w: self }
    }
    #[doc = "Bit 14 - B654"]
    #[inline(always)]
    pub fn b654(&mut self) -> B654_W {
        B654_W { w: self }
    }
    #[doc = "Bit 15 - B655"]
    #[inline(always)]
    pub fn b655(&mut self) -> B655_W {
        B655_W { w: self }
    }
    #[doc = "Bit 16 - B656"]
    #[inline(always)]
    pub fn b656(&mut self) -> B656_W {
        B656_W { w: self }
    }
    #[doc = "Bit 17 - B657"]
    #[inline(always)]
    pub fn b657(&mut self) -> B657_W {
        B657_W { w: self }
    }
    #[doc = "Bit 18 - B658"]
    #[inline(always)]
    pub fn b658(&mut self) -> B658_W {
        B658_W { w: self }
    }
    #[doc = "Bit 19 - B659"]
    #[inline(always)]
    pub fn b659(&mut self) -> B659_W {
        B659_W { w: self }
    }
    #[doc = "Bit 20 - B660"]
    #[inline(always)]
    pub fn b660(&mut self) -> B660_W {
        B660_W { w: self }
    }
    #[doc = "Bit 21 - B661"]
    #[inline(always)]
    pub fn b661(&mut self) -> B661_W {
        B661_W { w: self }
    }
    #[doc = "Bit 22 - B662"]
    #[inline(always)]
    pub fn b662(&mut self) -> B662_W {
        B662_W { w: self }
    }
    #[doc = "Bit 23 - B663"]
    #[inline(always)]
    pub fn b663(&mut self) -> B663_W {
        B663_W { w: self }
    }
    #[doc = "Bit 24 - B664"]
    #[inline(always)]
    pub fn b664(&mut self) -> B664_W {
        B664_W { w: self }
    }
    #[doc = "Bit 25 - B665"]
    #[inline(always)]
    pub fn b665(&mut self) -> B665_W {
        B665_W { w: self }
    }
    #[doc = "Bit 26 - B666"]
    #[inline(always)]
    pub fn b666(&mut self) -> B666_W {
        B666_W { w: self }
    }
    #[doc = "Bit 27 - B667"]
    #[inline(always)]
    pub fn b667(&mut self) -> B667_W {
        B667_W { w: self }
    }
    #[doc = "Bit 28 - B668"]
    #[inline(always)]
    pub fn b668(&mut self) -> B668_W {
        B668_W { w: self }
    }
    #[doc = "Bit 29 - B669"]
    #[inline(always)]
    pub fn b669(&mut self) -> B669_W {
        B669_W { w: self }
    }
    #[doc = "Bit 30 - B670"]
    #[inline(always)]
    pub fn b670(&mut self) -> B670_W {
        B670_W { w: self }
    }
    #[doc = "Bit 31 - B671"]
    #[inline(always)]
    pub fn b671(&mut self) -> B671_W {
        B671_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr20](index.html) module"]
pub struct MPCBB1_VCTR20_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr20::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr20::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR20 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
