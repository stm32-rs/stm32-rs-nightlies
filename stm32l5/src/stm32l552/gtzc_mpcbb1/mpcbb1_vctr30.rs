#[doc = "Register `MPCBB1_VCTR30` reader"]
pub struct R(crate::R<MPCBB1_VCTR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR30` writer"]
pub struct W(crate::W<MPCBB1_VCTR30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR30_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B960` reader - B960"]
pub struct B960_R(crate::FieldReader<bool, bool>);
impl B960_R {
    pub(crate) fn new(bits: bool) -> Self {
        B960_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B960_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B960` writer - B960"]
pub struct B960_W<'a> {
    w: &'a mut W,
}
impl<'a> B960_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B961` reader - B961"]
pub struct B961_R(crate::FieldReader<bool, bool>);
impl B961_R {
    pub(crate) fn new(bits: bool) -> Self {
        B961_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B961_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B961` writer - B961"]
pub struct B961_W<'a> {
    w: &'a mut W,
}
impl<'a> B961_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B962` reader - B962"]
pub struct B962_R(crate::FieldReader<bool, bool>);
impl B962_R {
    pub(crate) fn new(bits: bool) -> Self {
        B962_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B962_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B962` writer - B962"]
pub struct B962_W<'a> {
    w: &'a mut W,
}
impl<'a> B962_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B963` reader - B963"]
pub struct B963_R(crate::FieldReader<bool, bool>);
impl B963_R {
    pub(crate) fn new(bits: bool) -> Self {
        B963_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B963_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B963` writer - B963"]
pub struct B963_W<'a> {
    w: &'a mut W,
}
impl<'a> B963_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B964` reader - B964"]
pub struct B964_R(crate::FieldReader<bool, bool>);
impl B964_R {
    pub(crate) fn new(bits: bool) -> Self {
        B964_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B964_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B964` writer - B964"]
pub struct B964_W<'a> {
    w: &'a mut W,
}
impl<'a> B964_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B965` reader - B965"]
pub struct B965_R(crate::FieldReader<bool, bool>);
impl B965_R {
    pub(crate) fn new(bits: bool) -> Self {
        B965_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B965_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B965` writer - B965"]
pub struct B965_W<'a> {
    w: &'a mut W,
}
impl<'a> B965_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B966` reader - B966"]
pub struct B966_R(crate::FieldReader<bool, bool>);
impl B966_R {
    pub(crate) fn new(bits: bool) -> Self {
        B966_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B966_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B966` writer - B966"]
pub struct B966_W<'a> {
    w: &'a mut W,
}
impl<'a> B966_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B967` reader - B967"]
pub struct B967_R(crate::FieldReader<bool, bool>);
impl B967_R {
    pub(crate) fn new(bits: bool) -> Self {
        B967_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B967_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B967` writer - B967"]
pub struct B967_W<'a> {
    w: &'a mut W,
}
impl<'a> B967_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B968` reader - B968"]
pub struct B968_R(crate::FieldReader<bool, bool>);
impl B968_R {
    pub(crate) fn new(bits: bool) -> Self {
        B968_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B968_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B968` writer - B968"]
pub struct B968_W<'a> {
    w: &'a mut W,
}
impl<'a> B968_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B969` reader - B969"]
pub struct B969_R(crate::FieldReader<bool, bool>);
impl B969_R {
    pub(crate) fn new(bits: bool) -> Self {
        B969_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B969_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B969` writer - B969"]
pub struct B969_W<'a> {
    w: &'a mut W,
}
impl<'a> B969_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B970` reader - B970"]
pub struct B970_R(crate::FieldReader<bool, bool>);
impl B970_R {
    pub(crate) fn new(bits: bool) -> Self {
        B970_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B970_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B970` writer - B970"]
pub struct B970_W<'a> {
    w: &'a mut W,
}
impl<'a> B970_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B971` reader - B971"]
pub struct B971_R(crate::FieldReader<bool, bool>);
impl B971_R {
    pub(crate) fn new(bits: bool) -> Self {
        B971_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B971_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B971` writer - B971"]
pub struct B971_W<'a> {
    w: &'a mut W,
}
impl<'a> B971_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B972` reader - B972"]
pub struct B972_R(crate::FieldReader<bool, bool>);
impl B972_R {
    pub(crate) fn new(bits: bool) -> Self {
        B972_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B972_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B972` writer - B972"]
pub struct B972_W<'a> {
    w: &'a mut W,
}
impl<'a> B972_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B973` reader - B973"]
pub struct B973_R(crate::FieldReader<bool, bool>);
impl B973_R {
    pub(crate) fn new(bits: bool) -> Self {
        B973_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B973_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B973` writer - B973"]
pub struct B973_W<'a> {
    w: &'a mut W,
}
impl<'a> B973_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B974` reader - B974"]
pub struct B974_R(crate::FieldReader<bool, bool>);
impl B974_R {
    pub(crate) fn new(bits: bool) -> Self {
        B974_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B974_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B974` writer - B974"]
pub struct B974_W<'a> {
    w: &'a mut W,
}
impl<'a> B974_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B975` reader - B975"]
pub struct B975_R(crate::FieldReader<bool, bool>);
impl B975_R {
    pub(crate) fn new(bits: bool) -> Self {
        B975_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B975_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B975` writer - B975"]
pub struct B975_W<'a> {
    w: &'a mut W,
}
impl<'a> B975_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B976` reader - B976"]
pub struct B976_R(crate::FieldReader<bool, bool>);
impl B976_R {
    pub(crate) fn new(bits: bool) -> Self {
        B976_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B976_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B976` writer - B976"]
pub struct B976_W<'a> {
    w: &'a mut W,
}
impl<'a> B976_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B977` reader - B977"]
pub struct B977_R(crate::FieldReader<bool, bool>);
impl B977_R {
    pub(crate) fn new(bits: bool) -> Self {
        B977_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B977_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B977` writer - B977"]
pub struct B977_W<'a> {
    w: &'a mut W,
}
impl<'a> B977_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B978` reader - B978"]
pub struct B978_R(crate::FieldReader<bool, bool>);
impl B978_R {
    pub(crate) fn new(bits: bool) -> Self {
        B978_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B978_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B978` writer - B978"]
pub struct B978_W<'a> {
    w: &'a mut W,
}
impl<'a> B978_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B979` reader - B979"]
pub struct B979_R(crate::FieldReader<bool, bool>);
impl B979_R {
    pub(crate) fn new(bits: bool) -> Self {
        B979_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B979_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B979` writer - B979"]
pub struct B979_W<'a> {
    w: &'a mut W,
}
impl<'a> B979_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B980` reader - B980"]
pub struct B980_R(crate::FieldReader<bool, bool>);
impl B980_R {
    pub(crate) fn new(bits: bool) -> Self {
        B980_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B980_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B980` writer - B980"]
pub struct B980_W<'a> {
    w: &'a mut W,
}
impl<'a> B980_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B981` reader - B981"]
pub struct B981_R(crate::FieldReader<bool, bool>);
impl B981_R {
    pub(crate) fn new(bits: bool) -> Self {
        B981_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B981_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B981` writer - B981"]
pub struct B981_W<'a> {
    w: &'a mut W,
}
impl<'a> B981_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B982` reader - B982"]
pub struct B982_R(crate::FieldReader<bool, bool>);
impl B982_R {
    pub(crate) fn new(bits: bool) -> Self {
        B982_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B982_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B982` writer - B982"]
pub struct B982_W<'a> {
    w: &'a mut W,
}
impl<'a> B982_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B983` reader - B983"]
pub struct B983_R(crate::FieldReader<bool, bool>);
impl B983_R {
    pub(crate) fn new(bits: bool) -> Self {
        B983_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B983_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B983` writer - B983"]
pub struct B983_W<'a> {
    w: &'a mut W,
}
impl<'a> B983_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B984` reader - B984"]
pub struct B984_R(crate::FieldReader<bool, bool>);
impl B984_R {
    pub(crate) fn new(bits: bool) -> Self {
        B984_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B984_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B984` writer - B984"]
pub struct B984_W<'a> {
    w: &'a mut W,
}
impl<'a> B984_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B985` reader - B985"]
pub struct B985_R(crate::FieldReader<bool, bool>);
impl B985_R {
    pub(crate) fn new(bits: bool) -> Self {
        B985_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B985_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B985` writer - B985"]
pub struct B985_W<'a> {
    w: &'a mut W,
}
impl<'a> B985_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B986` reader - B986"]
pub struct B986_R(crate::FieldReader<bool, bool>);
impl B986_R {
    pub(crate) fn new(bits: bool) -> Self {
        B986_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B986_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B986` writer - B986"]
pub struct B986_W<'a> {
    w: &'a mut W,
}
impl<'a> B986_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B987` reader - B987"]
pub struct B987_R(crate::FieldReader<bool, bool>);
impl B987_R {
    pub(crate) fn new(bits: bool) -> Self {
        B987_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B987_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B987` writer - B987"]
pub struct B987_W<'a> {
    w: &'a mut W,
}
impl<'a> B987_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B988` reader - B988"]
pub struct B988_R(crate::FieldReader<bool, bool>);
impl B988_R {
    pub(crate) fn new(bits: bool) -> Self {
        B988_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B988_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B988` writer - B988"]
pub struct B988_W<'a> {
    w: &'a mut W,
}
impl<'a> B988_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B989` reader - B989"]
pub struct B989_R(crate::FieldReader<bool, bool>);
impl B989_R {
    pub(crate) fn new(bits: bool) -> Self {
        B989_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B989_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B989` writer - B989"]
pub struct B989_W<'a> {
    w: &'a mut W,
}
impl<'a> B989_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B990` reader - B990"]
pub struct B990_R(crate::FieldReader<bool, bool>);
impl B990_R {
    pub(crate) fn new(bits: bool) -> Self {
        B990_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B990_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B990` writer - B990"]
pub struct B990_W<'a> {
    w: &'a mut W,
}
impl<'a> B990_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B991` reader - B991"]
pub struct B991_R(crate::FieldReader<bool, bool>);
impl B991_R {
    pub(crate) fn new(bits: bool) -> Self {
        B991_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B991_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B991` writer - B991"]
pub struct B991_W<'a> {
    w: &'a mut W,
}
impl<'a> B991_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B960"]
    #[inline(always)]
    pub fn b960(&self) -> B960_R {
        B960_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B961"]
    #[inline(always)]
    pub fn b961(&self) -> B961_R {
        B961_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B962"]
    #[inline(always)]
    pub fn b962(&self) -> B962_R {
        B962_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B963"]
    #[inline(always)]
    pub fn b963(&self) -> B963_R {
        B963_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B964"]
    #[inline(always)]
    pub fn b964(&self) -> B964_R {
        B964_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B965"]
    #[inline(always)]
    pub fn b965(&self) -> B965_R {
        B965_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B966"]
    #[inline(always)]
    pub fn b966(&self) -> B966_R {
        B966_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B967"]
    #[inline(always)]
    pub fn b967(&self) -> B967_R {
        B967_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B968"]
    #[inline(always)]
    pub fn b968(&self) -> B968_R {
        B968_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B969"]
    #[inline(always)]
    pub fn b969(&self) -> B969_R {
        B969_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B970"]
    #[inline(always)]
    pub fn b970(&self) -> B970_R {
        B970_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B971"]
    #[inline(always)]
    pub fn b971(&self) -> B971_R {
        B971_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B972"]
    #[inline(always)]
    pub fn b972(&self) -> B972_R {
        B972_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B973"]
    #[inline(always)]
    pub fn b973(&self) -> B973_R {
        B973_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B974"]
    #[inline(always)]
    pub fn b974(&self) -> B974_R {
        B974_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B975"]
    #[inline(always)]
    pub fn b975(&self) -> B975_R {
        B975_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B976"]
    #[inline(always)]
    pub fn b976(&self) -> B976_R {
        B976_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B977"]
    #[inline(always)]
    pub fn b977(&self) -> B977_R {
        B977_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B978"]
    #[inline(always)]
    pub fn b978(&self) -> B978_R {
        B978_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B979"]
    #[inline(always)]
    pub fn b979(&self) -> B979_R {
        B979_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B980"]
    #[inline(always)]
    pub fn b980(&self) -> B980_R {
        B980_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B981"]
    #[inline(always)]
    pub fn b981(&self) -> B981_R {
        B981_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B982"]
    #[inline(always)]
    pub fn b982(&self) -> B982_R {
        B982_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B983"]
    #[inline(always)]
    pub fn b983(&self) -> B983_R {
        B983_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B984"]
    #[inline(always)]
    pub fn b984(&self) -> B984_R {
        B984_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B985"]
    #[inline(always)]
    pub fn b985(&self) -> B985_R {
        B985_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B986"]
    #[inline(always)]
    pub fn b986(&self) -> B986_R {
        B986_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B987"]
    #[inline(always)]
    pub fn b987(&self) -> B987_R {
        B987_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B988"]
    #[inline(always)]
    pub fn b988(&self) -> B988_R {
        B988_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B989"]
    #[inline(always)]
    pub fn b989(&self) -> B989_R {
        B989_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B990"]
    #[inline(always)]
    pub fn b990(&self) -> B990_R {
        B990_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B991"]
    #[inline(always)]
    pub fn b991(&self) -> B991_R {
        B991_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B960"]
    #[inline(always)]
    pub fn b960(&mut self) -> B960_W {
        B960_W { w: self }
    }
    #[doc = "Bit 1 - B961"]
    #[inline(always)]
    pub fn b961(&mut self) -> B961_W {
        B961_W { w: self }
    }
    #[doc = "Bit 2 - B962"]
    #[inline(always)]
    pub fn b962(&mut self) -> B962_W {
        B962_W { w: self }
    }
    #[doc = "Bit 3 - B963"]
    #[inline(always)]
    pub fn b963(&mut self) -> B963_W {
        B963_W { w: self }
    }
    #[doc = "Bit 4 - B964"]
    #[inline(always)]
    pub fn b964(&mut self) -> B964_W {
        B964_W { w: self }
    }
    #[doc = "Bit 5 - B965"]
    #[inline(always)]
    pub fn b965(&mut self) -> B965_W {
        B965_W { w: self }
    }
    #[doc = "Bit 6 - B966"]
    #[inline(always)]
    pub fn b966(&mut self) -> B966_W {
        B966_W { w: self }
    }
    #[doc = "Bit 7 - B967"]
    #[inline(always)]
    pub fn b967(&mut self) -> B967_W {
        B967_W { w: self }
    }
    #[doc = "Bit 8 - B968"]
    #[inline(always)]
    pub fn b968(&mut self) -> B968_W {
        B968_W { w: self }
    }
    #[doc = "Bit 9 - B969"]
    #[inline(always)]
    pub fn b969(&mut self) -> B969_W {
        B969_W { w: self }
    }
    #[doc = "Bit 10 - B970"]
    #[inline(always)]
    pub fn b970(&mut self) -> B970_W {
        B970_W { w: self }
    }
    #[doc = "Bit 11 - B971"]
    #[inline(always)]
    pub fn b971(&mut self) -> B971_W {
        B971_W { w: self }
    }
    #[doc = "Bit 12 - B972"]
    #[inline(always)]
    pub fn b972(&mut self) -> B972_W {
        B972_W { w: self }
    }
    #[doc = "Bit 13 - B973"]
    #[inline(always)]
    pub fn b973(&mut self) -> B973_W {
        B973_W { w: self }
    }
    #[doc = "Bit 14 - B974"]
    #[inline(always)]
    pub fn b974(&mut self) -> B974_W {
        B974_W { w: self }
    }
    #[doc = "Bit 15 - B975"]
    #[inline(always)]
    pub fn b975(&mut self) -> B975_W {
        B975_W { w: self }
    }
    #[doc = "Bit 16 - B976"]
    #[inline(always)]
    pub fn b976(&mut self) -> B976_W {
        B976_W { w: self }
    }
    #[doc = "Bit 17 - B977"]
    #[inline(always)]
    pub fn b977(&mut self) -> B977_W {
        B977_W { w: self }
    }
    #[doc = "Bit 18 - B978"]
    #[inline(always)]
    pub fn b978(&mut self) -> B978_W {
        B978_W { w: self }
    }
    #[doc = "Bit 19 - B979"]
    #[inline(always)]
    pub fn b979(&mut self) -> B979_W {
        B979_W { w: self }
    }
    #[doc = "Bit 20 - B980"]
    #[inline(always)]
    pub fn b980(&mut self) -> B980_W {
        B980_W { w: self }
    }
    #[doc = "Bit 21 - B981"]
    #[inline(always)]
    pub fn b981(&mut self) -> B981_W {
        B981_W { w: self }
    }
    #[doc = "Bit 22 - B982"]
    #[inline(always)]
    pub fn b982(&mut self) -> B982_W {
        B982_W { w: self }
    }
    #[doc = "Bit 23 - B983"]
    #[inline(always)]
    pub fn b983(&mut self) -> B983_W {
        B983_W { w: self }
    }
    #[doc = "Bit 24 - B984"]
    #[inline(always)]
    pub fn b984(&mut self) -> B984_W {
        B984_W { w: self }
    }
    #[doc = "Bit 25 - B985"]
    #[inline(always)]
    pub fn b985(&mut self) -> B985_W {
        B985_W { w: self }
    }
    #[doc = "Bit 26 - B986"]
    #[inline(always)]
    pub fn b986(&mut self) -> B986_W {
        B986_W { w: self }
    }
    #[doc = "Bit 27 - B987"]
    #[inline(always)]
    pub fn b987(&mut self) -> B987_W {
        B987_W { w: self }
    }
    #[doc = "Bit 28 - B988"]
    #[inline(always)]
    pub fn b988(&mut self) -> B988_W {
        B988_W { w: self }
    }
    #[doc = "Bit 29 - B989"]
    #[inline(always)]
    pub fn b989(&mut self) -> B989_W {
        B989_W { w: self }
    }
    #[doc = "Bit 30 - B990"]
    #[inline(always)]
    pub fn b990(&mut self) -> B990_W {
        B990_W { w: self }
    }
    #[doc = "Bit 31 - B991"]
    #[inline(always)]
    pub fn b991(&mut self) -> B991_W {
        B991_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr30](index.html) module"]
pub struct MPCBB1_VCTR30_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr30::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr30::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR30_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR30 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
