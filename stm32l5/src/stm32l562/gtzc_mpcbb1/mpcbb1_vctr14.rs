#[doc = "Register `MPCBB1_VCTR14` reader"]
pub struct R(crate::R<MPCBB1_VCTR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR14` writer"]
pub struct W(crate::W<MPCBB1_VCTR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR14_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B448` reader - B448"]
pub struct B448_R(crate::FieldReader<bool, bool>);
impl B448_R {
    pub(crate) fn new(bits: bool) -> Self {
        B448_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B448_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B448` writer - B448"]
pub struct B448_W<'a> {
    w: &'a mut W,
}
impl<'a> B448_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B449` reader - B449"]
pub struct B449_R(crate::FieldReader<bool, bool>);
impl B449_R {
    pub(crate) fn new(bits: bool) -> Self {
        B449_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B449_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B449` writer - B449"]
pub struct B449_W<'a> {
    w: &'a mut W,
}
impl<'a> B449_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B450` reader - B450"]
pub struct B450_R(crate::FieldReader<bool, bool>);
impl B450_R {
    pub(crate) fn new(bits: bool) -> Self {
        B450_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B450_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B450` writer - B450"]
pub struct B450_W<'a> {
    w: &'a mut W,
}
impl<'a> B450_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B451` reader - B451"]
pub struct B451_R(crate::FieldReader<bool, bool>);
impl B451_R {
    pub(crate) fn new(bits: bool) -> Self {
        B451_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B451_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B451` writer - B451"]
pub struct B451_W<'a> {
    w: &'a mut W,
}
impl<'a> B451_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B452` reader - B452"]
pub struct B452_R(crate::FieldReader<bool, bool>);
impl B452_R {
    pub(crate) fn new(bits: bool) -> Self {
        B452_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B452_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B452` writer - B452"]
pub struct B452_W<'a> {
    w: &'a mut W,
}
impl<'a> B452_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B453` reader - B453"]
pub struct B453_R(crate::FieldReader<bool, bool>);
impl B453_R {
    pub(crate) fn new(bits: bool) -> Self {
        B453_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B453_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B453` writer - B453"]
pub struct B453_W<'a> {
    w: &'a mut W,
}
impl<'a> B453_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B454` reader - B454"]
pub struct B454_R(crate::FieldReader<bool, bool>);
impl B454_R {
    pub(crate) fn new(bits: bool) -> Self {
        B454_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B454_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B454` writer - B454"]
pub struct B454_W<'a> {
    w: &'a mut W,
}
impl<'a> B454_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B455` reader - B455"]
pub struct B455_R(crate::FieldReader<bool, bool>);
impl B455_R {
    pub(crate) fn new(bits: bool) -> Self {
        B455_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B455_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B455` writer - B455"]
pub struct B455_W<'a> {
    w: &'a mut W,
}
impl<'a> B455_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B456` reader - B456"]
pub struct B456_R(crate::FieldReader<bool, bool>);
impl B456_R {
    pub(crate) fn new(bits: bool) -> Self {
        B456_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B456_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B456` writer - B456"]
pub struct B456_W<'a> {
    w: &'a mut W,
}
impl<'a> B456_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B457` reader - B457"]
pub struct B457_R(crate::FieldReader<bool, bool>);
impl B457_R {
    pub(crate) fn new(bits: bool) -> Self {
        B457_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B457_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B457` writer - B457"]
pub struct B457_W<'a> {
    w: &'a mut W,
}
impl<'a> B457_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B458` reader - B458"]
pub struct B458_R(crate::FieldReader<bool, bool>);
impl B458_R {
    pub(crate) fn new(bits: bool) -> Self {
        B458_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B458_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B458` writer - B458"]
pub struct B458_W<'a> {
    w: &'a mut W,
}
impl<'a> B458_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B459` reader - B459"]
pub struct B459_R(crate::FieldReader<bool, bool>);
impl B459_R {
    pub(crate) fn new(bits: bool) -> Self {
        B459_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B459_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B459` writer - B459"]
pub struct B459_W<'a> {
    w: &'a mut W,
}
impl<'a> B459_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B460` reader - B460"]
pub struct B460_R(crate::FieldReader<bool, bool>);
impl B460_R {
    pub(crate) fn new(bits: bool) -> Self {
        B460_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B460_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B460` writer - B460"]
pub struct B460_W<'a> {
    w: &'a mut W,
}
impl<'a> B460_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B461` reader - B461"]
pub struct B461_R(crate::FieldReader<bool, bool>);
impl B461_R {
    pub(crate) fn new(bits: bool) -> Self {
        B461_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B461_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B461` writer - B461"]
pub struct B461_W<'a> {
    w: &'a mut W,
}
impl<'a> B461_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B462` reader - B462"]
pub struct B462_R(crate::FieldReader<bool, bool>);
impl B462_R {
    pub(crate) fn new(bits: bool) -> Self {
        B462_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B462_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B462` writer - B462"]
pub struct B462_W<'a> {
    w: &'a mut W,
}
impl<'a> B462_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B463` reader - B463"]
pub struct B463_R(crate::FieldReader<bool, bool>);
impl B463_R {
    pub(crate) fn new(bits: bool) -> Self {
        B463_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B463_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B463` writer - B463"]
pub struct B463_W<'a> {
    w: &'a mut W,
}
impl<'a> B463_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B464` reader - B464"]
pub struct B464_R(crate::FieldReader<bool, bool>);
impl B464_R {
    pub(crate) fn new(bits: bool) -> Self {
        B464_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B464_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B464` writer - B464"]
pub struct B464_W<'a> {
    w: &'a mut W,
}
impl<'a> B464_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B465` reader - B465"]
pub struct B465_R(crate::FieldReader<bool, bool>);
impl B465_R {
    pub(crate) fn new(bits: bool) -> Self {
        B465_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B465_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B465` writer - B465"]
pub struct B465_W<'a> {
    w: &'a mut W,
}
impl<'a> B465_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B466` reader - B466"]
pub struct B466_R(crate::FieldReader<bool, bool>);
impl B466_R {
    pub(crate) fn new(bits: bool) -> Self {
        B466_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B466_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B466` writer - B466"]
pub struct B466_W<'a> {
    w: &'a mut W,
}
impl<'a> B466_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B467` reader - B467"]
pub struct B467_R(crate::FieldReader<bool, bool>);
impl B467_R {
    pub(crate) fn new(bits: bool) -> Self {
        B467_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B467_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B467` writer - B467"]
pub struct B467_W<'a> {
    w: &'a mut W,
}
impl<'a> B467_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B468` reader - B468"]
pub struct B468_R(crate::FieldReader<bool, bool>);
impl B468_R {
    pub(crate) fn new(bits: bool) -> Self {
        B468_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B468_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B468` writer - B468"]
pub struct B468_W<'a> {
    w: &'a mut W,
}
impl<'a> B468_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B469` reader - B469"]
pub struct B469_R(crate::FieldReader<bool, bool>);
impl B469_R {
    pub(crate) fn new(bits: bool) -> Self {
        B469_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B469_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B469` writer - B469"]
pub struct B469_W<'a> {
    w: &'a mut W,
}
impl<'a> B469_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B470` reader - B470"]
pub struct B470_R(crate::FieldReader<bool, bool>);
impl B470_R {
    pub(crate) fn new(bits: bool) -> Self {
        B470_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B470_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B470` writer - B470"]
pub struct B470_W<'a> {
    w: &'a mut W,
}
impl<'a> B470_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B471` reader - B471"]
pub struct B471_R(crate::FieldReader<bool, bool>);
impl B471_R {
    pub(crate) fn new(bits: bool) -> Self {
        B471_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B471_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B471` writer - B471"]
pub struct B471_W<'a> {
    w: &'a mut W,
}
impl<'a> B471_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B472` reader - B472"]
pub struct B472_R(crate::FieldReader<bool, bool>);
impl B472_R {
    pub(crate) fn new(bits: bool) -> Self {
        B472_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B472_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B472` writer - B472"]
pub struct B472_W<'a> {
    w: &'a mut W,
}
impl<'a> B472_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B473` reader - B473"]
pub struct B473_R(crate::FieldReader<bool, bool>);
impl B473_R {
    pub(crate) fn new(bits: bool) -> Self {
        B473_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B473_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B473` writer - B473"]
pub struct B473_W<'a> {
    w: &'a mut W,
}
impl<'a> B473_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B474` reader - B474"]
pub struct B474_R(crate::FieldReader<bool, bool>);
impl B474_R {
    pub(crate) fn new(bits: bool) -> Self {
        B474_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B474_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B474` writer - B474"]
pub struct B474_W<'a> {
    w: &'a mut W,
}
impl<'a> B474_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B475` reader - B475"]
pub struct B475_R(crate::FieldReader<bool, bool>);
impl B475_R {
    pub(crate) fn new(bits: bool) -> Self {
        B475_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B475_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B475` writer - B475"]
pub struct B475_W<'a> {
    w: &'a mut W,
}
impl<'a> B475_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B476` reader - B476"]
pub struct B476_R(crate::FieldReader<bool, bool>);
impl B476_R {
    pub(crate) fn new(bits: bool) -> Self {
        B476_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B476_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B476` writer - B476"]
pub struct B476_W<'a> {
    w: &'a mut W,
}
impl<'a> B476_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B477` reader - B477"]
pub struct B477_R(crate::FieldReader<bool, bool>);
impl B477_R {
    pub(crate) fn new(bits: bool) -> Self {
        B477_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B477_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B477` writer - B477"]
pub struct B477_W<'a> {
    w: &'a mut W,
}
impl<'a> B477_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B478` reader - B478"]
pub struct B478_R(crate::FieldReader<bool, bool>);
impl B478_R {
    pub(crate) fn new(bits: bool) -> Self {
        B478_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B478_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B478` writer - B478"]
pub struct B478_W<'a> {
    w: &'a mut W,
}
impl<'a> B478_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B479` reader - B479"]
pub struct B479_R(crate::FieldReader<bool, bool>);
impl B479_R {
    pub(crate) fn new(bits: bool) -> Self {
        B479_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B479_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B479` writer - B479"]
pub struct B479_W<'a> {
    w: &'a mut W,
}
impl<'a> B479_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B448"]
    #[inline(always)]
    pub fn b448(&self) -> B448_R {
        B448_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B449"]
    #[inline(always)]
    pub fn b449(&self) -> B449_R {
        B449_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B450"]
    #[inline(always)]
    pub fn b450(&self) -> B450_R {
        B450_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B451"]
    #[inline(always)]
    pub fn b451(&self) -> B451_R {
        B451_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B452"]
    #[inline(always)]
    pub fn b452(&self) -> B452_R {
        B452_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B453"]
    #[inline(always)]
    pub fn b453(&self) -> B453_R {
        B453_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B454"]
    #[inline(always)]
    pub fn b454(&self) -> B454_R {
        B454_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B455"]
    #[inline(always)]
    pub fn b455(&self) -> B455_R {
        B455_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B456"]
    #[inline(always)]
    pub fn b456(&self) -> B456_R {
        B456_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B457"]
    #[inline(always)]
    pub fn b457(&self) -> B457_R {
        B457_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B458"]
    #[inline(always)]
    pub fn b458(&self) -> B458_R {
        B458_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B459"]
    #[inline(always)]
    pub fn b459(&self) -> B459_R {
        B459_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B460"]
    #[inline(always)]
    pub fn b460(&self) -> B460_R {
        B460_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B461"]
    #[inline(always)]
    pub fn b461(&self) -> B461_R {
        B461_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B462"]
    #[inline(always)]
    pub fn b462(&self) -> B462_R {
        B462_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B463"]
    #[inline(always)]
    pub fn b463(&self) -> B463_R {
        B463_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B464"]
    #[inline(always)]
    pub fn b464(&self) -> B464_R {
        B464_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B465"]
    #[inline(always)]
    pub fn b465(&self) -> B465_R {
        B465_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B466"]
    #[inline(always)]
    pub fn b466(&self) -> B466_R {
        B466_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B467"]
    #[inline(always)]
    pub fn b467(&self) -> B467_R {
        B467_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B468"]
    #[inline(always)]
    pub fn b468(&self) -> B468_R {
        B468_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B469"]
    #[inline(always)]
    pub fn b469(&self) -> B469_R {
        B469_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B470"]
    #[inline(always)]
    pub fn b470(&self) -> B470_R {
        B470_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B471"]
    #[inline(always)]
    pub fn b471(&self) -> B471_R {
        B471_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B472"]
    #[inline(always)]
    pub fn b472(&self) -> B472_R {
        B472_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B473"]
    #[inline(always)]
    pub fn b473(&self) -> B473_R {
        B473_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B474"]
    #[inline(always)]
    pub fn b474(&self) -> B474_R {
        B474_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B475"]
    #[inline(always)]
    pub fn b475(&self) -> B475_R {
        B475_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B476"]
    #[inline(always)]
    pub fn b476(&self) -> B476_R {
        B476_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B477"]
    #[inline(always)]
    pub fn b477(&self) -> B477_R {
        B477_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B478"]
    #[inline(always)]
    pub fn b478(&self) -> B478_R {
        B478_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B479"]
    #[inline(always)]
    pub fn b479(&self) -> B479_R {
        B479_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B448"]
    #[inline(always)]
    pub fn b448(&mut self) -> B448_W {
        B448_W { w: self }
    }
    #[doc = "Bit 1 - B449"]
    #[inline(always)]
    pub fn b449(&mut self) -> B449_W {
        B449_W { w: self }
    }
    #[doc = "Bit 2 - B450"]
    #[inline(always)]
    pub fn b450(&mut self) -> B450_W {
        B450_W { w: self }
    }
    #[doc = "Bit 3 - B451"]
    #[inline(always)]
    pub fn b451(&mut self) -> B451_W {
        B451_W { w: self }
    }
    #[doc = "Bit 4 - B452"]
    #[inline(always)]
    pub fn b452(&mut self) -> B452_W {
        B452_W { w: self }
    }
    #[doc = "Bit 5 - B453"]
    #[inline(always)]
    pub fn b453(&mut self) -> B453_W {
        B453_W { w: self }
    }
    #[doc = "Bit 6 - B454"]
    #[inline(always)]
    pub fn b454(&mut self) -> B454_W {
        B454_W { w: self }
    }
    #[doc = "Bit 7 - B455"]
    #[inline(always)]
    pub fn b455(&mut self) -> B455_W {
        B455_W { w: self }
    }
    #[doc = "Bit 8 - B456"]
    #[inline(always)]
    pub fn b456(&mut self) -> B456_W {
        B456_W { w: self }
    }
    #[doc = "Bit 9 - B457"]
    #[inline(always)]
    pub fn b457(&mut self) -> B457_W {
        B457_W { w: self }
    }
    #[doc = "Bit 10 - B458"]
    #[inline(always)]
    pub fn b458(&mut self) -> B458_W {
        B458_W { w: self }
    }
    #[doc = "Bit 11 - B459"]
    #[inline(always)]
    pub fn b459(&mut self) -> B459_W {
        B459_W { w: self }
    }
    #[doc = "Bit 12 - B460"]
    #[inline(always)]
    pub fn b460(&mut self) -> B460_W {
        B460_W { w: self }
    }
    #[doc = "Bit 13 - B461"]
    #[inline(always)]
    pub fn b461(&mut self) -> B461_W {
        B461_W { w: self }
    }
    #[doc = "Bit 14 - B462"]
    #[inline(always)]
    pub fn b462(&mut self) -> B462_W {
        B462_W { w: self }
    }
    #[doc = "Bit 15 - B463"]
    #[inline(always)]
    pub fn b463(&mut self) -> B463_W {
        B463_W { w: self }
    }
    #[doc = "Bit 16 - B464"]
    #[inline(always)]
    pub fn b464(&mut self) -> B464_W {
        B464_W { w: self }
    }
    #[doc = "Bit 17 - B465"]
    #[inline(always)]
    pub fn b465(&mut self) -> B465_W {
        B465_W { w: self }
    }
    #[doc = "Bit 18 - B466"]
    #[inline(always)]
    pub fn b466(&mut self) -> B466_W {
        B466_W { w: self }
    }
    #[doc = "Bit 19 - B467"]
    #[inline(always)]
    pub fn b467(&mut self) -> B467_W {
        B467_W { w: self }
    }
    #[doc = "Bit 20 - B468"]
    #[inline(always)]
    pub fn b468(&mut self) -> B468_W {
        B468_W { w: self }
    }
    #[doc = "Bit 21 - B469"]
    #[inline(always)]
    pub fn b469(&mut self) -> B469_W {
        B469_W { w: self }
    }
    #[doc = "Bit 22 - B470"]
    #[inline(always)]
    pub fn b470(&mut self) -> B470_W {
        B470_W { w: self }
    }
    #[doc = "Bit 23 - B471"]
    #[inline(always)]
    pub fn b471(&mut self) -> B471_W {
        B471_W { w: self }
    }
    #[doc = "Bit 24 - B472"]
    #[inline(always)]
    pub fn b472(&mut self) -> B472_W {
        B472_W { w: self }
    }
    #[doc = "Bit 25 - B473"]
    #[inline(always)]
    pub fn b473(&mut self) -> B473_W {
        B473_W { w: self }
    }
    #[doc = "Bit 26 - B474"]
    #[inline(always)]
    pub fn b474(&mut self) -> B474_W {
        B474_W { w: self }
    }
    #[doc = "Bit 27 - B475"]
    #[inline(always)]
    pub fn b475(&mut self) -> B475_W {
        B475_W { w: self }
    }
    #[doc = "Bit 28 - B476"]
    #[inline(always)]
    pub fn b476(&mut self) -> B476_W {
        B476_W { w: self }
    }
    #[doc = "Bit 29 - B477"]
    #[inline(always)]
    pub fn b477(&mut self) -> B477_W {
        B477_W { w: self }
    }
    #[doc = "Bit 30 - B478"]
    #[inline(always)]
    pub fn b478(&mut self) -> B478_W {
        B478_W { w: self }
    }
    #[doc = "Bit 31 - B479"]
    #[inline(always)]
    pub fn b479(&mut self) -> B479_W {
        B479_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr14](index.html) module"]
pub struct MPCBB1_VCTR14_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr14::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr14::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR14_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR14 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR14_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
