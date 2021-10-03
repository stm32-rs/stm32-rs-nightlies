#[doc = "Register `MPCBB1_VCTR16` reader"]
pub struct R(crate::R<MPCBB1_VCTR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR16` writer"]
pub struct W(crate::W<MPCBB1_VCTR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR16_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B512` reader - B512"]
pub struct B512_R(crate::FieldReader<bool, bool>);
impl B512_R {
    pub(crate) fn new(bits: bool) -> Self {
        B512_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B512_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B512` writer - B512"]
pub struct B512_W<'a> {
    w: &'a mut W,
}
impl<'a> B512_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B513` reader - B513"]
pub struct B513_R(crate::FieldReader<bool, bool>);
impl B513_R {
    pub(crate) fn new(bits: bool) -> Self {
        B513_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B513_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B513` writer - B513"]
pub struct B513_W<'a> {
    w: &'a mut W,
}
impl<'a> B513_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B514` reader - B514"]
pub struct B514_R(crate::FieldReader<bool, bool>);
impl B514_R {
    pub(crate) fn new(bits: bool) -> Self {
        B514_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B514_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B514` writer - B514"]
pub struct B514_W<'a> {
    w: &'a mut W,
}
impl<'a> B514_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B515` reader - B515"]
pub struct B515_R(crate::FieldReader<bool, bool>);
impl B515_R {
    pub(crate) fn new(bits: bool) -> Self {
        B515_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B515_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B515` writer - B515"]
pub struct B515_W<'a> {
    w: &'a mut W,
}
impl<'a> B515_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B516` reader - B516"]
pub struct B516_R(crate::FieldReader<bool, bool>);
impl B516_R {
    pub(crate) fn new(bits: bool) -> Self {
        B516_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B516_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B516` writer - B516"]
pub struct B516_W<'a> {
    w: &'a mut W,
}
impl<'a> B516_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B517` reader - B517"]
pub struct B517_R(crate::FieldReader<bool, bool>);
impl B517_R {
    pub(crate) fn new(bits: bool) -> Self {
        B517_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B517_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B517` writer - B517"]
pub struct B517_W<'a> {
    w: &'a mut W,
}
impl<'a> B517_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B518` reader - B518"]
pub struct B518_R(crate::FieldReader<bool, bool>);
impl B518_R {
    pub(crate) fn new(bits: bool) -> Self {
        B518_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B518_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B518` writer - B518"]
pub struct B518_W<'a> {
    w: &'a mut W,
}
impl<'a> B518_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B519` reader - B519"]
pub struct B519_R(crate::FieldReader<bool, bool>);
impl B519_R {
    pub(crate) fn new(bits: bool) -> Self {
        B519_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B519_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B519` writer - B519"]
pub struct B519_W<'a> {
    w: &'a mut W,
}
impl<'a> B519_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B520` reader - B520"]
pub struct B520_R(crate::FieldReader<bool, bool>);
impl B520_R {
    pub(crate) fn new(bits: bool) -> Self {
        B520_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B520_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B520` writer - B520"]
pub struct B520_W<'a> {
    w: &'a mut W,
}
impl<'a> B520_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B521` reader - B521"]
pub struct B521_R(crate::FieldReader<bool, bool>);
impl B521_R {
    pub(crate) fn new(bits: bool) -> Self {
        B521_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B521_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B521` writer - B521"]
pub struct B521_W<'a> {
    w: &'a mut W,
}
impl<'a> B521_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B522` reader - B522"]
pub struct B522_R(crate::FieldReader<bool, bool>);
impl B522_R {
    pub(crate) fn new(bits: bool) -> Self {
        B522_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B522_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B522` writer - B522"]
pub struct B522_W<'a> {
    w: &'a mut W,
}
impl<'a> B522_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B523` reader - B523"]
pub struct B523_R(crate::FieldReader<bool, bool>);
impl B523_R {
    pub(crate) fn new(bits: bool) -> Self {
        B523_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B523_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B523` writer - B523"]
pub struct B523_W<'a> {
    w: &'a mut W,
}
impl<'a> B523_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B524` reader - B524"]
pub struct B524_R(crate::FieldReader<bool, bool>);
impl B524_R {
    pub(crate) fn new(bits: bool) -> Self {
        B524_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B524_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B524` writer - B524"]
pub struct B524_W<'a> {
    w: &'a mut W,
}
impl<'a> B524_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B525` reader - B525"]
pub struct B525_R(crate::FieldReader<bool, bool>);
impl B525_R {
    pub(crate) fn new(bits: bool) -> Self {
        B525_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B525_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B525` writer - B525"]
pub struct B525_W<'a> {
    w: &'a mut W,
}
impl<'a> B525_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B526` reader - B526"]
pub struct B526_R(crate::FieldReader<bool, bool>);
impl B526_R {
    pub(crate) fn new(bits: bool) -> Self {
        B526_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B526_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B526` writer - B526"]
pub struct B526_W<'a> {
    w: &'a mut W,
}
impl<'a> B526_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B527` reader - B527"]
pub struct B527_R(crate::FieldReader<bool, bool>);
impl B527_R {
    pub(crate) fn new(bits: bool) -> Self {
        B527_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B527_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B527` writer - B527"]
pub struct B527_W<'a> {
    w: &'a mut W,
}
impl<'a> B527_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B528` reader - B528"]
pub struct B528_R(crate::FieldReader<bool, bool>);
impl B528_R {
    pub(crate) fn new(bits: bool) -> Self {
        B528_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B528_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B528` writer - B528"]
pub struct B528_W<'a> {
    w: &'a mut W,
}
impl<'a> B528_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B529` reader - B529"]
pub struct B529_R(crate::FieldReader<bool, bool>);
impl B529_R {
    pub(crate) fn new(bits: bool) -> Self {
        B529_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B529_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B529` writer - B529"]
pub struct B529_W<'a> {
    w: &'a mut W,
}
impl<'a> B529_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B530` reader - B530"]
pub struct B530_R(crate::FieldReader<bool, bool>);
impl B530_R {
    pub(crate) fn new(bits: bool) -> Self {
        B530_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B530_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B530` writer - B530"]
pub struct B530_W<'a> {
    w: &'a mut W,
}
impl<'a> B530_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B531` reader - B531"]
pub struct B531_R(crate::FieldReader<bool, bool>);
impl B531_R {
    pub(crate) fn new(bits: bool) -> Self {
        B531_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B531_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B531` writer - B531"]
pub struct B531_W<'a> {
    w: &'a mut W,
}
impl<'a> B531_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B532` reader - B532"]
pub struct B532_R(crate::FieldReader<bool, bool>);
impl B532_R {
    pub(crate) fn new(bits: bool) -> Self {
        B532_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B532_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B532` writer - B532"]
pub struct B532_W<'a> {
    w: &'a mut W,
}
impl<'a> B532_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B533` reader - B533"]
pub struct B533_R(crate::FieldReader<bool, bool>);
impl B533_R {
    pub(crate) fn new(bits: bool) -> Self {
        B533_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B533_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B533` writer - B533"]
pub struct B533_W<'a> {
    w: &'a mut W,
}
impl<'a> B533_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B534` reader - B534"]
pub struct B534_R(crate::FieldReader<bool, bool>);
impl B534_R {
    pub(crate) fn new(bits: bool) -> Self {
        B534_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B534_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B534` writer - B534"]
pub struct B534_W<'a> {
    w: &'a mut W,
}
impl<'a> B534_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B535` reader - B535"]
pub struct B535_R(crate::FieldReader<bool, bool>);
impl B535_R {
    pub(crate) fn new(bits: bool) -> Self {
        B535_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B535_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B535` writer - B535"]
pub struct B535_W<'a> {
    w: &'a mut W,
}
impl<'a> B535_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B536` reader - B536"]
pub struct B536_R(crate::FieldReader<bool, bool>);
impl B536_R {
    pub(crate) fn new(bits: bool) -> Self {
        B536_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B536_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B536` writer - B536"]
pub struct B536_W<'a> {
    w: &'a mut W,
}
impl<'a> B536_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B537` reader - B537"]
pub struct B537_R(crate::FieldReader<bool, bool>);
impl B537_R {
    pub(crate) fn new(bits: bool) -> Self {
        B537_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B537_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B537` writer - B537"]
pub struct B537_W<'a> {
    w: &'a mut W,
}
impl<'a> B537_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B538` reader - B538"]
pub struct B538_R(crate::FieldReader<bool, bool>);
impl B538_R {
    pub(crate) fn new(bits: bool) -> Self {
        B538_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B538_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B538` writer - B538"]
pub struct B538_W<'a> {
    w: &'a mut W,
}
impl<'a> B538_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B539` reader - B539"]
pub struct B539_R(crate::FieldReader<bool, bool>);
impl B539_R {
    pub(crate) fn new(bits: bool) -> Self {
        B539_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B539_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B539` writer - B539"]
pub struct B539_W<'a> {
    w: &'a mut W,
}
impl<'a> B539_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B540` reader - B540"]
pub struct B540_R(crate::FieldReader<bool, bool>);
impl B540_R {
    pub(crate) fn new(bits: bool) -> Self {
        B540_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B540_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B540` writer - B540"]
pub struct B540_W<'a> {
    w: &'a mut W,
}
impl<'a> B540_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B541` reader - B541"]
pub struct B541_R(crate::FieldReader<bool, bool>);
impl B541_R {
    pub(crate) fn new(bits: bool) -> Self {
        B541_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B541_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B541` writer - B541"]
pub struct B541_W<'a> {
    w: &'a mut W,
}
impl<'a> B541_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B542` reader - B542"]
pub struct B542_R(crate::FieldReader<bool, bool>);
impl B542_R {
    pub(crate) fn new(bits: bool) -> Self {
        B542_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B542_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B542` writer - B542"]
pub struct B542_W<'a> {
    w: &'a mut W,
}
impl<'a> B542_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B543` reader - B543"]
pub struct B543_R(crate::FieldReader<bool, bool>);
impl B543_R {
    pub(crate) fn new(bits: bool) -> Self {
        B543_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B543_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B543` writer - B543"]
pub struct B543_W<'a> {
    w: &'a mut W,
}
impl<'a> B543_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B512"]
    #[inline(always)]
    pub fn b512(&self) -> B512_R {
        B512_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B513"]
    #[inline(always)]
    pub fn b513(&self) -> B513_R {
        B513_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B514"]
    #[inline(always)]
    pub fn b514(&self) -> B514_R {
        B514_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B515"]
    #[inline(always)]
    pub fn b515(&self) -> B515_R {
        B515_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B516"]
    #[inline(always)]
    pub fn b516(&self) -> B516_R {
        B516_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B517"]
    #[inline(always)]
    pub fn b517(&self) -> B517_R {
        B517_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B518"]
    #[inline(always)]
    pub fn b518(&self) -> B518_R {
        B518_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B519"]
    #[inline(always)]
    pub fn b519(&self) -> B519_R {
        B519_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B520"]
    #[inline(always)]
    pub fn b520(&self) -> B520_R {
        B520_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B521"]
    #[inline(always)]
    pub fn b521(&self) -> B521_R {
        B521_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B522"]
    #[inline(always)]
    pub fn b522(&self) -> B522_R {
        B522_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B523"]
    #[inline(always)]
    pub fn b523(&self) -> B523_R {
        B523_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B524"]
    #[inline(always)]
    pub fn b524(&self) -> B524_R {
        B524_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B525"]
    #[inline(always)]
    pub fn b525(&self) -> B525_R {
        B525_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B526"]
    #[inline(always)]
    pub fn b526(&self) -> B526_R {
        B526_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B527"]
    #[inline(always)]
    pub fn b527(&self) -> B527_R {
        B527_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B528"]
    #[inline(always)]
    pub fn b528(&self) -> B528_R {
        B528_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B529"]
    #[inline(always)]
    pub fn b529(&self) -> B529_R {
        B529_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B530"]
    #[inline(always)]
    pub fn b530(&self) -> B530_R {
        B530_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B531"]
    #[inline(always)]
    pub fn b531(&self) -> B531_R {
        B531_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B532"]
    #[inline(always)]
    pub fn b532(&self) -> B532_R {
        B532_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B533"]
    #[inline(always)]
    pub fn b533(&self) -> B533_R {
        B533_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B534"]
    #[inline(always)]
    pub fn b534(&self) -> B534_R {
        B534_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B535"]
    #[inline(always)]
    pub fn b535(&self) -> B535_R {
        B535_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B536"]
    #[inline(always)]
    pub fn b536(&self) -> B536_R {
        B536_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B537"]
    #[inline(always)]
    pub fn b537(&self) -> B537_R {
        B537_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B538"]
    #[inline(always)]
    pub fn b538(&self) -> B538_R {
        B538_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B539"]
    #[inline(always)]
    pub fn b539(&self) -> B539_R {
        B539_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B540"]
    #[inline(always)]
    pub fn b540(&self) -> B540_R {
        B540_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B541"]
    #[inline(always)]
    pub fn b541(&self) -> B541_R {
        B541_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B542"]
    #[inline(always)]
    pub fn b542(&self) -> B542_R {
        B542_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B543"]
    #[inline(always)]
    pub fn b543(&self) -> B543_R {
        B543_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B512"]
    #[inline(always)]
    pub fn b512(&mut self) -> B512_W {
        B512_W { w: self }
    }
    #[doc = "Bit 1 - B513"]
    #[inline(always)]
    pub fn b513(&mut self) -> B513_W {
        B513_W { w: self }
    }
    #[doc = "Bit 2 - B514"]
    #[inline(always)]
    pub fn b514(&mut self) -> B514_W {
        B514_W { w: self }
    }
    #[doc = "Bit 3 - B515"]
    #[inline(always)]
    pub fn b515(&mut self) -> B515_W {
        B515_W { w: self }
    }
    #[doc = "Bit 4 - B516"]
    #[inline(always)]
    pub fn b516(&mut self) -> B516_W {
        B516_W { w: self }
    }
    #[doc = "Bit 5 - B517"]
    #[inline(always)]
    pub fn b517(&mut self) -> B517_W {
        B517_W { w: self }
    }
    #[doc = "Bit 6 - B518"]
    #[inline(always)]
    pub fn b518(&mut self) -> B518_W {
        B518_W { w: self }
    }
    #[doc = "Bit 7 - B519"]
    #[inline(always)]
    pub fn b519(&mut self) -> B519_W {
        B519_W { w: self }
    }
    #[doc = "Bit 8 - B520"]
    #[inline(always)]
    pub fn b520(&mut self) -> B520_W {
        B520_W { w: self }
    }
    #[doc = "Bit 9 - B521"]
    #[inline(always)]
    pub fn b521(&mut self) -> B521_W {
        B521_W { w: self }
    }
    #[doc = "Bit 10 - B522"]
    #[inline(always)]
    pub fn b522(&mut self) -> B522_W {
        B522_W { w: self }
    }
    #[doc = "Bit 11 - B523"]
    #[inline(always)]
    pub fn b523(&mut self) -> B523_W {
        B523_W { w: self }
    }
    #[doc = "Bit 12 - B524"]
    #[inline(always)]
    pub fn b524(&mut self) -> B524_W {
        B524_W { w: self }
    }
    #[doc = "Bit 13 - B525"]
    #[inline(always)]
    pub fn b525(&mut self) -> B525_W {
        B525_W { w: self }
    }
    #[doc = "Bit 14 - B526"]
    #[inline(always)]
    pub fn b526(&mut self) -> B526_W {
        B526_W { w: self }
    }
    #[doc = "Bit 15 - B527"]
    #[inline(always)]
    pub fn b527(&mut self) -> B527_W {
        B527_W { w: self }
    }
    #[doc = "Bit 16 - B528"]
    #[inline(always)]
    pub fn b528(&mut self) -> B528_W {
        B528_W { w: self }
    }
    #[doc = "Bit 17 - B529"]
    #[inline(always)]
    pub fn b529(&mut self) -> B529_W {
        B529_W { w: self }
    }
    #[doc = "Bit 18 - B530"]
    #[inline(always)]
    pub fn b530(&mut self) -> B530_W {
        B530_W { w: self }
    }
    #[doc = "Bit 19 - B531"]
    #[inline(always)]
    pub fn b531(&mut self) -> B531_W {
        B531_W { w: self }
    }
    #[doc = "Bit 20 - B532"]
    #[inline(always)]
    pub fn b532(&mut self) -> B532_W {
        B532_W { w: self }
    }
    #[doc = "Bit 21 - B533"]
    #[inline(always)]
    pub fn b533(&mut self) -> B533_W {
        B533_W { w: self }
    }
    #[doc = "Bit 22 - B534"]
    #[inline(always)]
    pub fn b534(&mut self) -> B534_W {
        B534_W { w: self }
    }
    #[doc = "Bit 23 - B535"]
    #[inline(always)]
    pub fn b535(&mut self) -> B535_W {
        B535_W { w: self }
    }
    #[doc = "Bit 24 - B536"]
    #[inline(always)]
    pub fn b536(&mut self) -> B536_W {
        B536_W { w: self }
    }
    #[doc = "Bit 25 - B537"]
    #[inline(always)]
    pub fn b537(&mut self) -> B537_W {
        B537_W { w: self }
    }
    #[doc = "Bit 26 - B538"]
    #[inline(always)]
    pub fn b538(&mut self) -> B538_W {
        B538_W { w: self }
    }
    #[doc = "Bit 27 - B539"]
    #[inline(always)]
    pub fn b539(&mut self) -> B539_W {
        B539_W { w: self }
    }
    #[doc = "Bit 28 - B540"]
    #[inline(always)]
    pub fn b540(&mut self) -> B540_W {
        B540_W { w: self }
    }
    #[doc = "Bit 29 - B541"]
    #[inline(always)]
    pub fn b541(&mut self) -> B541_W {
        B541_W { w: self }
    }
    #[doc = "Bit 30 - B542"]
    #[inline(always)]
    pub fn b542(&mut self) -> B542_W {
        B542_W { w: self }
    }
    #[doc = "Bit 31 - B543"]
    #[inline(always)]
    pub fn b543(&mut self) -> B543_W {
        B543_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr16](index.html) module"]
pub struct MPCBB1_VCTR16_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr16::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr16::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR16 to value 0"]
impl crate::Resettable for MPCBB1_VCTR16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
