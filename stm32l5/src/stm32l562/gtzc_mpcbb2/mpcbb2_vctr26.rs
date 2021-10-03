#[doc = "Register `MPCBB2_VCTR26` reader"]
pub struct R(crate::R<MPCBB2_VCTR26_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR26_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR26_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR26_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR26` writer"]
pub struct W(crate::W<MPCBB2_VCTR26_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR26_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR26_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR26_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B832` reader - B832"]
pub struct B832_R(crate::FieldReader<bool, bool>);
impl B832_R {
    pub(crate) fn new(bits: bool) -> Self {
        B832_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B832_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B832` writer - B832"]
pub struct B832_W<'a> {
    w: &'a mut W,
}
impl<'a> B832_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B833` reader - B833"]
pub struct B833_R(crate::FieldReader<bool, bool>);
impl B833_R {
    pub(crate) fn new(bits: bool) -> Self {
        B833_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B833_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B833` writer - B833"]
pub struct B833_W<'a> {
    w: &'a mut W,
}
impl<'a> B833_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B834` reader - B834"]
pub struct B834_R(crate::FieldReader<bool, bool>);
impl B834_R {
    pub(crate) fn new(bits: bool) -> Self {
        B834_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B834_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B834` writer - B834"]
pub struct B834_W<'a> {
    w: &'a mut W,
}
impl<'a> B834_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B835` reader - B835"]
pub struct B835_R(crate::FieldReader<bool, bool>);
impl B835_R {
    pub(crate) fn new(bits: bool) -> Self {
        B835_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B835_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B835` writer - B835"]
pub struct B835_W<'a> {
    w: &'a mut W,
}
impl<'a> B835_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B836` reader - B836"]
pub struct B836_R(crate::FieldReader<bool, bool>);
impl B836_R {
    pub(crate) fn new(bits: bool) -> Self {
        B836_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B836_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B836` writer - B836"]
pub struct B836_W<'a> {
    w: &'a mut W,
}
impl<'a> B836_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B837` reader - B837"]
pub struct B837_R(crate::FieldReader<bool, bool>);
impl B837_R {
    pub(crate) fn new(bits: bool) -> Self {
        B837_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B837_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B837` writer - B837"]
pub struct B837_W<'a> {
    w: &'a mut W,
}
impl<'a> B837_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B838` reader - B838"]
pub struct B838_R(crate::FieldReader<bool, bool>);
impl B838_R {
    pub(crate) fn new(bits: bool) -> Self {
        B838_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B838_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B838` writer - B838"]
pub struct B838_W<'a> {
    w: &'a mut W,
}
impl<'a> B838_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B839` reader - B839"]
pub struct B839_R(crate::FieldReader<bool, bool>);
impl B839_R {
    pub(crate) fn new(bits: bool) -> Self {
        B839_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B839_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B839` writer - B839"]
pub struct B839_W<'a> {
    w: &'a mut W,
}
impl<'a> B839_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B840` reader - B840"]
pub struct B840_R(crate::FieldReader<bool, bool>);
impl B840_R {
    pub(crate) fn new(bits: bool) -> Self {
        B840_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B840_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B840` writer - B840"]
pub struct B840_W<'a> {
    w: &'a mut W,
}
impl<'a> B840_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B841` reader - B841"]
pub struct B841_R(crate::FieldReader<bool, bool>);
impl B841_R {
    pub(crate) fn new(bits: bool) -> Self {
        B841_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B841_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B841` writer - B841"]
pub struct B841_W<'a> {
    w: &'a mut W,
}
impl<'a> B841_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B842` reader - B842"]
pub struct B842_R(crate::FieldReader<bool, bool>);
impl B842_R {
    pub(crate) fn new(bits: bool) -> Self {
        B842_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B842_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B842` writer - B842"]
pub struct B842_W<'a> {
    w: &'a mut W,
}
impl<'a> B842_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B843` reader - B843"]
pub struct B843_R(crate::FieldReader<bool, bool>);
impl B843_R {
    pub(crate) fn new(bits: bool) -> Self {
        B843_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B843_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B843` writer - B843"]
pub struct B843_W<'a> {
    w: &'a mut W,
}
impl<'a> B843_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B844` reader - B844"]
pub struct B844_R(crate::FieldReader<bool, bool>);
impl B844_R {
    pub(crate) fn new(bits: bool) -> Self {
        B844_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B844_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B844` writer - B844"]
pub struct B844_W<'a> {
    w: &'a mut W,
}
impl<'a> B844_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B845` reader - B845"]
pub struct B845_R(crate::FieldReader<bool, bool>);
impl B845_R {
    pub(crate) fn new(bits: bool) -> Self {
        B845_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B845_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B845` writer - B845"]
pub struct B845_W<'a> {
    w: &'a mut W,
}
impl<'a> B845_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B846` reader - B846"]
pub struct B846_R(crate::FieldReader<bool, bool>);
impl B846_R {
    pub(crate) fn new(bits: bool) -> Self {
        B846_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B846_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B846` writer - B846"]
pub struct B846_W<'a> {
    w: &'a mut W,
}
impl<'a> B846_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B847` reader - B847"]
pub struct B847_R(crate::FieldReader<bool, bool>);
impl B847_R {
    pub(crate) fn new(bits: bool) -> Self {
        B847_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B847_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B847` writer - B847"]
pub struct B847_W<'a> {
    w: &'a mut W,
}
impl<'a> B847_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B848` reader - B848"]
pub struct B848_R(crate::FieldReader<bool, bool>);
impl B848_R {
    pub(crate) fn new(bits: bool) -> Self {
        B848_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B848_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B848` writer - B848"]
pub struct B848_W<'a> {
    w: &'a mut W,
}
impl<'a> B848_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B849` reader - B849"]
pub struct B849_R(crate::FieldReader<bool, bool>);
impl B849_R {
    pub(crate) fn new(bits: bool) -> Self {
        B849_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B849_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B849` writer - B849"]
pub struct B849_W<'a> {
    w: &'a mut W,
}
impl<'a> B849_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B850` reader - B850"]
pub struct B850_R(crate::FieldReader<bool, bool>);
impl B850_R {
    pub(crate) fn new(bits: bool) -> Self {
        B850_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B850_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B850` writer - B850"]
pub struct B850_W<'a> {
    w: &'a mut W,
}
impl<'a> B850_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B851` reader - B851"]
pub struct B851_R(crate::FieldReader<bool, bool>);
impl B851_R {
    pub(crate) fn new(bits: bool) -> Self {
        B851_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B851_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B851` writer - B851"]
pub struct B851_W<'a> {
    w: &'a mut W,
}
impl<'a> B851_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B852` reader - B852"]
pub struct B852_R(crate::FieldReader<bool, bool>);
impl B852_R {
    pub(crate) fn new(bits: bool) -> Self {
        B852_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B852_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B852` writer - B852"]
pub struct B852_W<'a> {
    w: &'a mut W,
}
impl<'a> B852_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B853` reader - B853"]
pub struct B853_R(crate::FieldReader<bool, bool>);
impl B853_R {
    pub(crate) fn new(bits: bool) -> Self {
        B853_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B853_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B853` writer - B853"]
pub struct B853_W<'a> {
    w: &'a mut W,
}
impl<'a> B853_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B854` reader - B854"]
pub struct B854_R(crate::FieldReader<bool, bool>);
impl B854_R {
    pub(crate) fn new(bits: bool) -> Self {
        B854_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B854_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B854` writer - B854"]
pub struct B854_W<'a> {
    w: &'a mut W,
}
impl<'a> B854_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B855` reader - B855"]
pub struct B855_R(crate::FieldReader<bool, bool>);
impl B855_R {
    pub(crate) fn new(bits: bool) -> Self {
        B855_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B855_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B855` writer - B855"]
pub struct B855_W<'a> {
    w: &'a mut W,
}
impl<'a> B855_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B856` reader - B856"]
pub struct B856_R(crate::FieldReader<bool, bool>);
impl B856_R {
    pub(crate) fn new(bits: bool) -> Self {
        B856_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B856_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B856` writer - B856"]
pub struct B856_W<'a> {
    w: &'a mut W,
}
impl<'a> B856_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B857` reader - B857"]
pub struct B857_R(crate::FieldReader<bool, bool>);
impl B857_R {
    pub(crate) fn new(bits: bool) -> Self {
        B857_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B857_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B857` writer - B857"]
pub struct B857_W<'a> {
    w: &'a mut W,
}
impl<'a> B857_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B858` reader - B858"]
pub struct B858_R(crate::FieldReader<bool, bool>);
impl B858_R {
    pub(crate) fn new(bits: bool) -> Self {
        B858_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B858_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B858` writer - B858"]
pub struct B858_W<'a> {
    w: &'a mut W,
}
impl<'a> B858_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B859` reader - B859"]
pub struct B859_R(crate::FieldReader<bool, bool>);
impl B859_R {
    pub(crate) fn new(bits: bool) -> Self {
        B859_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B859_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B859` writer - B859"]
pub struct B859_W<'a> {
    w: &'a mut W,
}
impl<'a> B859_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B860` reader - B860"]
pub struct B860_R(crate::FieldReader<bool, bool>);
impl B860_R {
    pub(crate) fn new(bits: bool) -> Self {
        B860_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B860_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B860` writer - B860"]
pub struct B860_W<'a> {
    w: &'a mut W,
}
impl<'a> B860_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B861` reader - B861"]
pub struct B861_R(crate::FieldReader<bool, bool>);
impl B861_R {
    pub(crate) fn new(bits: bool) -> Self {
        B861_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B861_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B861` writer - B861"]
pub struct B861_W<'a> {
    w: &'a mut W,
}
impl<'a> B861_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B862` reader - B862"]
pub struct B862_R(crate::FieldReader<bool, bool>);
impl B862_R {
    pub(crate) fn new(bits: bool) -> Self {
        B862_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B862_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B862` writer - B862"]
pub struct B862_W<'a> {
    w: &'a mut W,
}
impl<'a> B862_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B863` reader - B863"]
pub struct B863_R(crate::FieldReader<bool, bool>);
impl B863_R {
    pub(crate) fn new(bits: bool) -> Self {
        B863_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B863_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B863` writer - B863"]
pub struct B863_W<'a> {
    w: &'a mut W,
}
impl<'a> B863_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B832"]
    #[inline(always)]
    pub fn b832(&self) -> B832_R {
        B832_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B833"]
    #[inline(always)]
    pub fn b833(&self) -> B833_R {
        B833_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B834"]
    #[inline(always)]
    pub fn b834(&self) -> B834_R {
        B834_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B835"]
    #[inline(always)]
    pub fn b835(&self) -> B835_R {
        B835_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B836"]
    #[inline(always)]
    pub fn b836(&self) -> B836_R {
        B836_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B837"]
    #[inline(always)]
    pub fn b837(&self) -> B837_R {
        B837_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B838"]
    #[inline(always)]
    pub fn b838(&self) -> B838_R {
        B838_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B839"]
    #[inline(always)]
    pub fn b839(&self) -> B839_R {
        B839_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B840"]
    #[inline(always)]
    pub fn b840(&self) -> B840_R {
        B840_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B841"]
    #[inline(always)]
    pub fn b841(&self) -> B841_R {
        B841_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B842"]
    #[inline(always)]
    pub fn b842(&self) -> B842_R {
        B842_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B843"]
    #[inline(always)]
    pub fn b843(&self) -> B843_R {
        B843_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B844"]
    #[inline(always)]
    pub fn b844(&self) -> B844_R {
        B844_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B845"]
    #[inline(always)]
    pub fn b845(&self) -> B845_R {
        B845_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B846"]
    #[inline(always)]
    pub fn b846(&self) -> B846_R {
        B846_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B847"]
    #[inline(always)]
    pub fn b847(&self) -> B847_R {
        B847_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B848"]
    #[inline(always)]
    pub fn b848(&self) -> B848_R {
        B848_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B849"]
    #[inline(always)]
    pub fn b849(&self) -> B849_R {
        B849_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B850"]
    #[inline(always)]
    pub fn b850(&self) -> B850_R {
        B850_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B851"]
    #[inline(always)]
    pub fn b851(&self) -> B851_R {
        B851_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B852"]
    #[inline(always)]
    pub fn b852(&self) -> B852_R {
        B852_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B853"]
    #[inline(always)]
    pub fn b853(&self) -> B853_R {
        B853_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B854"]
    #[inline(always)]
    pub fn b854(&self) -> B854_R {
        B854_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B855"]
    #[inline(always)]
    pub fn b855(&self) -> B855_R {
        B855_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B856"]
    #[inline(always)]
    pub fn b856(&self) -> B856_R {
        B856_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B857"]
    #[inline(always)]
    pub fn b857(&self) -> B857_R {
        B857_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B858"]
    #[inline(always)]
    pub fn b858(&self) -> B858_R {
        B858_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B859"]
    #[inline(always)]
    pub fn b859(&self) -> B859_R {
        B859_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B860"]
    #[inline(always)]
    pub fn b860(&self) -> B860_R {
        B860_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B861"]
    #[inline(always)]
    pub fn b861(&self) -> B861_R {
        B861_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B862"]
    #[inline(always)]
    pub fn b862(&self) -> B862_R {
        B862_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B863"]
    #[inline(always)]
    pub fn b863(&self) -> B863_R {
        B863_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B832"]
    #[inline(always)]
    pub fn b832(&mut self) -> B832_W {
        B832_W { w: self }
    }
    #[doc = "Bit 1 - B833"]
    #[inline(always)]
    pub fn b833(&mut self) -> B833_W {
        B833_W { w: self }
    }
    #[doc = "Bit 2 - B834"]
    #[inline(always)]
    pub fn b834(&mut self) -> B834_W {
        B834_W { w: self }
    }
    #[doc = "Bit 3 - B835"]
    #[inline(always)]
    pub fn b835(&mut self) -> B835_W {
        B835_W { w: self }
    }
    #[doc = "Bit 4 - B836"]
    #[inline(always)]
    pub fn b836(&mut self) -> B836_W {
        B836_W { w: self }
    }
    #[doc = "Bit 5 - B837"]
    #[inline(always)]
    pub fn b837(&mut self) -> B837_W {
        B837_W { w: self }
    }
    #[doc = "Bit 6 - B838"]
    #[inline(always)]
    pub fn b838(&mut self) -> B838_W {
        B838_W { w: self }
    }
    #[doc = "Bit 7 - B839"]
    #[inline(always)]
    pub fn b839(&mut self) -> B839_W {
        B839_W { w: self }
    }
    #[doc = "Bit 8 - B840"]
    #[inline(always)]
    pub fn b840(&mut self) -> B840_W {
        B840_W { w: self }
    }
    #[doc = "Bit 9 - B841"]
    #[inline(always)]
    pub fn b841(&mut self) -> B841_W {
        B841_W { w: self }
    }
    #[doc = "Bit 10 - B842"]
    #[inline(always)]
    pub fn b842(&mut self) -> B842_W {
        B842_W { w: self }
    }
    #[doc = "Bit 11 - B843"]
    #[inline(always)]
    pub fn b843(&mut self) -> B843_W {
        B843_W { w: self }
    }
    #[doc = "Bit 12 - B844"]
    #[inline(always)]
    pub fn b844(&mut self) -> B844_W {
        B844_W { w: self }
    }
    #[doc = "Bit 13 - B845"]
    #[inline(always)]
    pub fn b845(&mut self) -> B845_W {
        B845_W { w: self }
    }
    #[doc = "Bit 14 - B846"]
    #[inline(always)]
    pub fn b846(&mut self) -> B846_W {
        B846_W { w: self }
    }
    #[doc = "Bit 15 - B847"]
    #[inline(always)]
    pub fn b847(&mut self) -> B847_W {
        B847_W { w: self }
    }
    #[doc = "Bit 16 - B848"]
    #[inline(always)]
    pub fn b848(&mut self) -> B848_W {
        B848_W { w: self }
    }
    #[doc = "Bit 17 - B849"]
    #[inline(always)]
    pub fn b849(&mut self) -> B849_W {
        B849_W { w: self }
    }
    #[doc = "Bit 18 - B850"]
    #[inline(always)]
    pub fn b850(&mut self) -> B850_W {
        B850_W { w: self }
    }
    #[doc = "Bit 19 - B851"]
    #[inline(always)]
    pub fn b851(&mut self) -> B851_W {
        B851_W { w: self }
    }
    #[doc = "Bit 20 - B852"]
    #[inline(always)]
    pub fn b852(&mut self) -> B852_W {
        B852_W { w: self }
    }
    #[doc = "Bit 21 - B853"]
    #[inline(always)]
    pub fn b853(&mut self) -> B853_W {
        B853_W { w: self }
    }
    #[doc = "Bit 22 - B854"]
    #[inline(always)]
    pub fn b854(&mut self) -> B854_W {
        B854_W { w: self }
    }
    #[doc = "Bit 23 - B855"]
    #[inline(always)]
    pub fn b855(&mut self) -> B855_W {
        B855_W { w: self }
    }
    #[doc = "Bit 24 - B856"]
    #[inline(always)]
    pub fn b856(&mut self) -> B856_W {
        B856_W { w: self }
    }
    #[doc = "Bit 25 - B857"]
    #[inline(always)]
    pub fn b857(&mut self) -> B857_W {
        B857_W { w: self }
    }
    #[doc = "Bit 26 - B858"]
    #[inline(always)]
    pub fn b858(&mut self) -> B858_W {
        B858_W { w: self }
    }
    #[doc = "Bit 27 - B859"]
    #[inline(always)]
    pub fn b859(&mut self) -> B859_W {
        B859_W { w: self }
    }
    #[doc = "Bit 28 - B860"]
    #[inline(always)]
    pub fn b860(&mut self) -> B860_W {
        B860_W { w: self }
    }
    #[doc = "Bit 29 - B861"]
    #[inline(always)]
    pub fn b861(&mut self) -> B861_W {
        B861_W { w: self }
    }
    #[doc = "Bit 30 - B862"]
    #[inline(always)]
    pub fn b862(&mut self) -> B862_W {
        B862_W { w: self }
    }
    #[doc = "Bit 31 - B863"]
    #[inline(always)]
    pub fn b863(&mut self) -> B863_W {
        B863_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr26](index.html) module"]
pub struct MPCBB2_VCTR26_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR26_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr26::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR26_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr26::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR26_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR26 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR26_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
