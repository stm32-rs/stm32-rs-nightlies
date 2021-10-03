#[doc = "Register `MPCBB2_VCTR7` reader"]
pub struct R(crate::R<MPCBB2_VCTR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR7` writer"]
pub struct W(crate::W<MPCBB2_VCTR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR7_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B224` reader - B224"]
pub struct B224_R(crate::FieldReader<bool, bool>);
impl B224_R {
    pub(crate) fn new(bits: bool) -> Self {
        B224_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B224_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B224` writer - B224"]
pub struct B224_W<'a> {
    w: &'a mut W,
}
impl<'a> B224_W<'a> {
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
#[doc = "Field `B225` reader - B225"]
pub struct B225_R(crate::FieldReader<bool, bool>);
impl B225_R {
    pub(crate) fn new(bits: bool) -> Self {
        B225_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B225_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B225` writer - B225"]
pub struct B225_W<'a> {
    w: &'a mut W,
}
impl<'a> B225_W<'a> {
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
#[doc = "Field `B226` reader - B226"]
pub struct B226_R(crate::FieldReader<bool, bool>);
impl B226_R {
    pub(crate) fn new(bits: bool) -> Self {
        B226_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B226_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B226` writer - B226"]
pub struct B226_W<'a> {
    w: &'a mut W,
}
impl<'a> B226_W<'a> {
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
#[doc = "Field `B227` reader - B227"]
pub struct B227_R(crate::FieldReader<bool, bool>);
impl B227_R {
    pub(crate) fn new(bits: bool) -> Self {
        B227_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B227_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B227` writer - B227"]
pub struct B227_W<'a> {
    w: &'a mut W,
}
impl<'a> B227_W<'a> {
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
#[doc = "Field `B228` reader - B228"]
pub struct B228_R(crate::FieldReader<bool, bool>);
impl B228_R {
    pub(crate) fn new(bits: bool) -> Self {
        B228_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B228_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B228` writer - B228"]
pub struct B228_W<'a> {
    w: &'a mut W,
}
impl<'a> B228_W<'a> {
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
#[doc = "Field `B229` reader - B229"]
pub struct B229_R(crate::FieldReader<bool, bool>);
impl B229_R {
    pub(crate) fn new(bits: bool) -> Self {
        B229_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B229_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B229` writer - B229"]
pub struct B229_W<'a> {
    w: &'a mut W,
}
impl<'a> B229_W<'a> {
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
#[doc = "Field `B230` reader - B230"]
pub struct B230_R(crate::FieldReader<bool, bool>);
impl B230_R {
    pub(crate) fn new(bits: bool) -> Self {
        B230_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B230_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B230` writer - B230"]
pub struct B230_W<'a> {
    w: &'a mut W,
}
impl<'a> B230_W<'a> {
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
#[doc = "Field `B231` reader - B231"]
pub struct B231_R(crate::FieldReader<bool, bool>);
impl B231_R {
    pub(crate) fn new(bits: bool) -> Self {
        B231_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B231_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B231` writer - B231"]
pub struct B231_W<'a> {
    w: &'a mut W,
}
impl<'a> B231_W<'a> {
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
#[doc = "Field `B232` reader - B232"]
pub struct B232_R(crate::FieldReader<bool, bool>);
impl B232_R {
    pub(crate) fn new(bits: bool) -> Self {
        B232_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B232_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B232` writer - B232"]
pub struct B232_W<'a> {
    w: &'a mut W,
}
impl<'a> B232_W<'a> {
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
#[doc = "Field `B233` reader - B233"]
pub struct B233_R(crate::FieldReader<bool, bool>);
impl B233_R {
    pub(crate) fn new(bits: bool) -> Self {
        B233_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B233_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B233` writer - B233"]
pub struct B233_W<'a> {
    w: &'a mut W,
}
impl<'a> B233_W<'a> {
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
#[doc = "Field `B234` reader - B234"]
pub struct B234_R(crate::FieldReader<bool, bool>);
impl B234_R {
    pub(crate) fn new(bits: bool) -> Self {
        B234_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B234_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B234` writer - B234"]
pub struct B234_W<'a> {
    w: &'a mut W,
}
impl<'a> B234_W<'a> {
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
#[doc = "Field `B235` reader - B235"]
pub struct B235_R(crate::FieldReader<bool, bool>);
impl B235_R {
    pub(crate) fn new(bits: bool) -> Self {
        B235_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B235_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B235` writer - B235"]
pub struct B235_W<'a> {
    w: &'a mut W,
}
impl<'a> B235_W<'a> {
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
#[doc = "Field `B236` reader - B236"]
pub struct B236_R(crate::FieldReader<bool, bool>);
impl B236_R {
    pub(crate) fn new(bits: bool) -> Self {
        B236_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B236_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B236` writer - B236"]
pub struct B236_W<'a> {
    w: &'a mut W,
}
impl<'a> B236_W<'a> {
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
#[doc = "Field `B237` reader - B237"]
pub struct B237_R(crate::FieldReader<bool, bool>);
impl B237_R {
    pub(crate) fn new(bits: bool) -> Self {
        B237_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B237_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B237` writer - B237"]
pub struct B237_W<'a> {
    w: &'a mut W,
}
impl<'a> B237_W<'a> {
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
#[doc = "Field `B238` reader - B238"]
pub struct B238_R(crate::FieldReader<bool, bool>);
impl B238_R {
    pub(crate) fn new(bits: bool) -> Self {
        B238_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B238_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B238` writer - B238"]
pub struct B238_W<'a> {
    w: &'a mut W,
}
impl<'a> B238_W<'a> {
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
#[doc = "Field `B239` reader - B239"]
pub struct B239_R(crate::FieldReader<bool, bool>);
impl B239_R {
    pub(crate) fn new(bits: bool) -> Self {
        B239_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B239_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B239` writer - B239"]
pub struct B239_W<'a> {
    w: &'a mut W,
}
impl<'a> B239_W<'a> {
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
#[doc = "Field `B240` reader - B240"]
pub struct B240_R(crate::FieldReader<bool, bool>);
impl B240_R {
    pub(crate) fn new(bits: bool) -> Self {
        B240_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B240_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B240` writer - B240"]
pub struct B240_W<'a> {
    w: &'a mut W,
}
impl<'a> B240_W<'a> {
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
#[doc = "Field `B241` reader - B241"]
pub struct B241_R(crate::FieldReader<bool, bool>);
impl B241_R {
    pub(crate) fn new(bits: bool) -> Self {
        B241_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B241_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B241` writer - B241"]
pub struct B241_W<'a> {
    w: &'a mut W,
}
impl<'a> B241_W<'a> {
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
#[doc = "Field `B242` reader - B242"]
pub struct B242_R(crate::FieldReader<bool, bool>);
impl B242_R {
    pub(crate) fn new(bits: bool) -> Self {
        B242_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B242_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B242` writer - B242"]
pub struct B242_W<'a> {
    w: &'a mut W,
}
impl<'a> B242_W<'a> {
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
#[doc = "Field `B243` reader - B243"]
pub struct B243_R(crate::FieldReader<bool, bool>);
impl B243_R {
    pub(crate) fn new(bits: bool) -> Self {
        B243_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B243_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B243` writer - B243"]
pub struct B243_W<'a> {
    w: &'a mut W,
}
impl<'a> B243_W<'a> {
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
#[doc = "Field `B244` reader - B244"]
pub struct B244_R(crate::FieldReader<bool, bool>);
impl B244_R {
    pub(crate) fn new(bits: bool) -> Self {
        B244_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B244_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B244` writer - B244"]
pub struct B244_W<'a> {
    w: &'a mut W,
}
impl<'a> B244_W<'a> {
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
#[doc = "Field `B245` reader - B245"]
pub struct B245_R(crate::FieldReader<bool, bool>);
impl B245_R {
    pub(crate) fn new(bits: bool) -> Self {
        B245_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B245_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B245` writer - B245"]
pub struct B245_W<'a> {
    w: &'a mut W,
}
impl<'a> B245_W<'a> {
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
#[doc = "Field `B246` reader - B246"]
pub struct B246_R(crate::FieldReader<bool, bool>);
impl B246_R {
    pub(crate) fn new(bits: bool) -> Self {
        B246_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B246_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B246` writer - B246"]
pub struct B246_W<'a> {
    w: &'a mut W,
}
impl<'a> B246_W<'a> {
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
#[doc = "Field `B247` reader - B247"]
pub struct B247_R(crate::FieldReader<bool, bool>);
impl B247_R {
    pub(crate) fn new(bits: bool) -> Self {
        B247_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B247_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B247` writer - B247"]
pub struct B247_W<'a> {
    w: &'a mut W,
}
impl<'a> B247_W<'a> {
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
#[doc = "Field `B248` reader - B248"]
pub struct B248_R(crate::FieldReader<bool, bool>);
impl B248_R {
    pub(crate) fn new(bits: bool) -> Self {
        B248_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B248_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B248` writer - B248"]
pub struct B248_W<'a> {
    w: &'a mut W,
}
impl<'a> B248_W<'a> {
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
#[doc = "Field `B249` reader - B249"]
pub struct B249_R(crate::FieldReader<bool, bool>);
impl B249_R {
    pub(crate) fn new(bits: bool) -> Self {
        B249_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B249_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B249` writer - B249"]
pub struct B249_W<'a> {
    w: &'a mut W,
}
impl<'a> B249_W<'a> {
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
#[doc = "Field `B250` reader - B250"]
pub struct B250_R(crate::FieldReader<bool, bool>);
impl B250_R {
    pub(crate) fn new(bits: bool) -> Self {
        B250_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B250_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B250` writer - B250"]
pub struct B250_W<'a> {
    w: &'a mut W,
}
impl<'a> B250_W<'a> {
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
#[doc = "Field `B251` reader - B251"]
pub struct B251_R(crate::FieldReader<bool, bool>);
impl B251_R {
    pub(crate) fn new(bits: bool) -> Self {
        B251_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B251_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B251` writer - B251"]
pub struct B251_W<'a> {
    w: &'a mut W,
}
impl<'a> B251_W<'a> {
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
#[doc = "Field `B252` reader - B252"]
pub struct B252_R(crate::FieldReader<bool, bool>);
impl B252_R {
    pub(crate) fn new(bits: bool) -> Self {
        B252_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B252_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B252` writer - B252"]
pub struct B252_W<'a> {
    w: &'a mut W,
}
impl<'a> B252_W<'a> {
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
#[doc = "Field `B253` reader - B253"]
pub struct B253_R(crate::FieldReader<bool, bool>);
impl B253_R {
    pub(crate) fn new(bits: bool) -> Self {
        B253_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B253_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B253` writer - B253"]
pub struct B253_W<'a> {
    w: &'a mut W,
}
impl<'a> B253_W<'a> {
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
#[doc = "Field `B254` reader - B254"]
pub struct B254_R(crate::FieldReader<bool, bool>);
impl B254_R {
    pub(crate) fn new(bits: bool) -> Self {
        B254_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B254_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B254` writer - B254"]
pub struct B254_W<'a> {
    w: &'a mut W,
}
impl<'a> B254_W<'a> {
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
#[doc = "Field `B255` reader - B255"]
pub struct B255_R(crate::FieldReader<bool, bool>);
impl B255_R {
    pub(crate) fn new(bits: bool) -> Self {
        B255_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B255_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B255` writer - B255"]
pub struct B255_W<'a> {
    w: &'a mut W,
}
impl<'a> B255_W<'a> {
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
    #[doc = "Bit 0 - B224"]
    #[inline(always)]
    pub fn b224(&self) -> B224_R {
        B224_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B225"]
    #[inline(always)]
    pub fn b225(&self) -> B225_R {
        B225_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B226"]
    #[inline(always)]
    pub fn b226(&self) -> B226_R {
        B226_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B227"]
    #[inline(always)]
    pub fn b227(&self) -> B227_R {
        B227_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B228"]
    #[inline(always)]
    pub fn b228(&self) -> B228_R {
        B228_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B229"]
    #[inline(always)]
    pub fn b229(&self) -> B229_R {
        B229_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B230"]
    #[inline(always)]
    pub fn b230(&self) -> B230_R {
        B230_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B231"]
    #[inline(always)]
    pub fn b231(&self) -> B231_R {
        B231_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B232"]
    #[inline(always)]
    pub fn b232(&self) -> B232_R {
        B232_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B233"]
    #[inline(always)]
    pub fn b233(&self) -> B233_R {
        B233_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B234"]
    #[inline(always)]
    pub fn b234(&self) -> B234_R {
        B234_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B235"]
    #[inline(always)]
    pub fn b235(&self) -> B235_R {
        B235_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B236"]
    #[inline(always)]
    pub fn b236(&self) -> B236_R {
        B236_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B237"]
    #[inline(always)]
    pub fn b237(&self) -> B237_R {
        B237_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B238"]
    #[inline(always)]
    pub fn b238(&self) -> B238_R {
        B238_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B239"]
    #[inline(always)]
    pub fn b239(&self) -> B239_R {
        B239_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B240"]
    #[inline(always)]
    pub fn b240(&self) -> B240_R {
        B240_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B241"]
    #[inline(always)]
    pub fn b241(&self) -> B241_R {
        B241_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B242"]
    #[inline(always)]
    pub fn b242(&self) -> B242_R {
        B242_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B243"]
    #[inline(always)]
    pub fn b243(&self) -> B243_R {
        B243_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B244"]
    #[inline(always)]
    pub fn b244(&self) -> B244_R {
        B244_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B245"]
    #[inline(always)]
    pub fn b245(&self) -> B245_R {
        B245_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B246"]
    #[inline(always)]
    pub fn b246(&self) -> B246_R {
        B246_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B247"]
    #[inline(always)]
    pub fn b247(&self) -> B247_R {
        B247_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B248"]
    #[inline(always)]
    pub fn b248(&self) -> B248_R {
        B248_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B249"]
    #[inline(always)]
    pub fn b249(&self) -> B249_R {
        B249_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B250"]
    #[inline(always)]
    pub fn b250(&self) -> B250_R {
        B250_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B251"]
    #[inline(always)]
    pub fn b251(&self) -> B251_R {
        B251_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B252"]
    #[inline(always)]
    pub fn b252(&self) -> B252_R {
        B252_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B253"]
    #[inline(always)]
    pub fn b253(&self) -> B253_R {
        B253_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B254"]
    #[inline(always)]
    pub fn b254(&self) -> B254_R {
        B254_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B255"]
    #[inline(always)]
    pub fn b255(&self) -> B255_R {
        B255_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B224"]
    #[inline(always)]
    pub fn b224(&mut self) -> B224_W {
        B224_W { w: self }
    }
    #[doc = "Bit 1 - B225"]
    #[inline(always)]
    pub fn b225(&mut self) -> B225_W {
        B225_W { w: self }
    }
    #[doc = "Bit 2 - B226"]
    #[inline(always)]
    pub fn b226(&mut self) -> B226_W {
        B226_W { w: self }
    }
    #[doc = "Bit 3 - B227"]
    #[inline(always)]
    pub fn b227(&mut self) -> B227_W {
        B227_W { w: self }
    }
    #[doc = "Bit 4 - B228"]
    #[inline(always)]
    pub fn b228(&mut self) -> B228_W {
        B228_W { w: self }
    }
    #[doc = "Bit 5 - B229"]
    #[inline(always)]
    pub fn b229(&mut self) -> B229_W {
        B229_W { w: self }
    }
    #[doc = "Bit 6 - B230"]
    #[inline(always)]
    pub fn b230(&mut self) -> B230_W {
        B230_W { w: self }
    }
    #[doc = "Bit 7 - B231"]
    #[inline(always)]
    pub fn b231(&mut self) -> B231_W {
        B231_W { w: self }
    }
    #[doc = "Bit 8 - B232"]
    #[inline(always)]
    pub fn b232(&mut self) -> B232_W {
        B232_W { w: self }
    }
    #[doc = "Bit 9 - B233"]
    #[inline(always)]
    pub fn b233(&mut self) -> B233_W {
        B233_W { w: self }
    }
    #[doc = "Bit 10 - B234"]
    #[inline(always)]
    pub fn b234(&mut self) -> B234_W {
        B234_W { w: self }
    }
    #[doc = "Bit 11 - B235"]
    #[inline(always)]
    pub fn b235(&mut self) -> B235_W {
        B235_W { w: self }
    }
    #[doc = "Bit 12 - B236"]
    #[inline(always)]
    pub fn b236(&mut self) -> B236_W {
        B236_W { w: self }
    }
    #[doc = "Bit 13 - B237"]
    #[inline(always)]
    pub fn b237(&mut self) -> B237_W {
        B237_W { w: self }
    }
    #[doc = "Bit 14 - B238"]
    #[inline(always)]
    pub fn b238(&mut self) -> B238_W {
        B238_W { w: self }
    }
    #[doc = "Bit 15 - B239"]
    #[inline(always)]
    pub fn b239(&mut self) -> B239_W {
        B239_W { w: self }
    }
    #[doc = "Bit 16 - B240"]
    #[inline(always)]
    pub fn b240(&mut self) -> B240_W {
        B240_W { w: self }
    }
    #[doc = "Bit 17 - B241"]
    #[inline(always)]
    pub fn b241(&mut self) -> B241_W {
        B241_W { w: self }
    }
    #[doc = "Bit 18 - B242"]
    #[inline(always)]
    pub fn b242(&mut self) -> B242_W {
        B242_W { w: self }
    }
    #[doc = "Bit 19 - B243"]
    #[inline(always)]
    pub fn b243(&mut self) -> B243_W {
        B243_W { w: self }
    }
    #[doc = "Bit 20 - B244"]
    #[inline(always)]
    pub fn b244(&mut self) -> B244_W {
        B244_W { w: self }
    }
    #[doc = "Bit 21 - B245"]
    #[inline(always)]
    pub fn b245(&mut self) -> B245_W {
        B245_W { w: self }
    }
    #[doc = "Bit 22 - B246"]
    #[inline(always)]
    pub fn b246(&mut self) -> B246_W {
        B246_W { w: self }
    }
    #[doc = "Bit 23 - B247"]
    #[inline(always)]
    pub fn b247(&mut self) -> B247_W {
        B247_W { w: self }
    }
    #[doc = "Bit 24 - B248"]
    #[inline(always)]
    pub fn b248(&mut self) -> B248_W {
        B248_W { w: self }
    }
    #[doc = "Bit 25 - B249"]
    #[inline(always)]
    pub fn b249(&mut self) -> B249_W {
        B249_W { w: self }
    }
    #[doc = "Bit 26 - B250"]
    #[inline(always)]
    pub fn b250(&mut self) -> B250_W {
        B250_W { w: self }
    }
    #[doc = "Bit 27 - B251"]
    #[inline(always)]
    pub fn b251(&mut self) -> B251_W {
        B251_W { w: self }
    }
    #[doc = "Bit 28 - B252"]
    #[inline(always)]
    pub fn b252(&mut self) -> B252_W {
        B252_W { w: self }
    }
    #[doc = "Bit 29 - B253"]
    #[inline(always)]
    pub fn b253(&mut self) -> B253_W {
        B253_W { w: self }
    }
    #[doc = "Bit 30 - B254"]
    #[inline(always)]
    pub fn b254(&mut self) -> B254_W {
        B254_W { w: self }
    }
    #[doc = "Bit 31 - B255"]
    #[inline(always)]
    pub fn b255(&mut self) -> B255_W {
        B255_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr7](index.html) module"]
pub struct MPCBB2_VCTR7_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr7::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr7::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR7 to value 0"]
impl crate::Resettable for MPCBB2_VCTR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
