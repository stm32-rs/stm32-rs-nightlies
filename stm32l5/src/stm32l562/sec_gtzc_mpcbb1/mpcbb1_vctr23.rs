#[doc = "Register `MPCBB1_VCTR23` reader"]
pub struct R(crate::R<MPCBB1_VCTR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR23` writer"]
pub struct W(crate::W<MPCBB1_VCTR23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR23_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B736` reader - B736"]
pub struct B736_R(crate::FieldReader<bool, bool>);
impl B736_R {
    pub(crate) fn new(bits: bool) -> Self {
        B736_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B736_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B736` writer - B736"]
pub struct B736_W<'a> {
    w: &'a mut W,
}
impl<'a> B736_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B737` reader - B737"]
pub struct B737_R(crate::FieldReader<bool, bool>);
impl B737_R {
    pub(crate) fn new(bits: bool) -> Self {
        B737_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B737_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B737` writer - B737"]
pub struct B737_W<'a> {
    w: &'a mut W,
}
impl<'a> B737_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B738` reader - B738"]
pub struct B738_R(crate::FieldReader<bool, bool>);
impl B738_R {
    pub(crate) fn new(bits: bool) -> Self {
        B738_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B738_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B738` writer - B738"]
pub struct B738_W<'a> {
    w: &'a mut W,
}
impl<'a> B738_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B739` reader - B739"]
pub struct B739_R(crate::FieldReader<bool, bool>);
impl B739_R {
    pub(crate) fn new(bits: bool) -> Self {
        B739_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B739_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B739` writer - B739"]
pub struct B739_W<'a> {
    w: &'a mut W,
}
impl<'a> B739_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B740` reader - B740"]
pub struct B740_R(crate::FieldReader<bool, bool>);
impl B740_R {
    pub(crate) fn new(bits: bool) -> Self {
        B740_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B740_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B740` writer - B740"]
pub struct B740_W<'a> {
    w: &'a mut W,
}
impl<'a> B740_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B741` reader - B741"]
pub struct B741_R(crate::FieldReader<bool, bool>);
impl B741_R {
    pub(crate) fn new(bits: bool) -> Self {
        B741_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B741_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B741` writer - B741"]
pub struct B741_W<'a> {
    w: &'a mut W,
}
impl<'a> B741_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B742` reader - B742"]
pub struct B742_R(crate::FieldReader<bool, bool>);
impl B742_R {
    pub(crate) fn new(bits: bool) -> Self {
        B742_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B742_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B742` writer - B742"]
pub struct B742_W<'a> {
    w: &'a mut W,
}
impl<'a> B742_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B743` reader - B743"]
pub struct B743_R(crate::FieldReader<bool, bool>);
impl B743_R {
    pub(crate) fn new(bits: bool) -> Self {
        B743_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B743_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B743` writer - B743"]
pub struct B743_W<'a> {
    w: &'a mut W,
}
impl<'a> B743_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B744` reader - B744"]
pub struct B744_R(crate::FieldReader<bool, bool>);
impl B744_R {
    pub(crate) fn new(bits: bool) -> Self {
        B744_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B744_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B744` writer - B744"]
pub struct B744_W<'a> {
    w: &'a mut W,
}
impl<'a> B744_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B745` reader - B745"]
pub struct B745_R(crate::FieldReader<bool, bool>);
impl B745_R {
    pub(crate) fn new(bits: bool) -> Self {
        B745_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B745_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B745` writer - B745"]
pub struct B745_W<'a> {
    w: &'a mut W,
}
impl<'a> B745_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B746` reader - B746"]
pub struct B746_R(crate::FieldReader<bool, bool>);
impl B746_R {
    pub(crate) fn new(bits: bool) -> Self {
        B746_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B746_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B746` writer - B746"]
pub struct B746_W<'a> {
    w: &'a mut W,
}
impl<'a> B746_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B747` reader - B747"]
pub struct B747_R(crate::FieldReader<bool, bool>);
impl B747_R {
    pub(crate) fn new(bits: bool) -> Self {
        B747_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B747_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B747` writer - B747"]
pub struct B747_W<'a> {
    w: &'a mut W,
}
impl<'a> B747_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B748` reader - B748"]
pub struct B748_R(crate::FieldReader<bool, bool>);
impl B748_R {
    pub(crate) fn new(bits: bool) -> Self {
        B748_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B748_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B748` writer - B748"]
pub struct B748_W<'a> {
    w: &'a mut W,
}
impl<'a> B748_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B749` reader - B749"]
pub struct B749_R(crate::FieldReader<bool, bool>);
impl B749_R {
    pub(crate) fn new(bits: bool) -> Self {
        B749_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B749_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B749` writer - B749"]
pub struct B749_W<'a> {
    w: &'a mut W,
}
impl<'a> B749_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B750` reader - B750"]
pub struct B750_R(crate::FieldReader<bool, bool>);
impl B750_R {
    pub(crate) fn new(bits: bool) -> Self {
        B750_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B750_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B750` writer - B750"]
pub struct B750_W<'a> {
    w: &'a mut W,
}
impl<'a> B750_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B751` reader - B751"]
pub struct B751_R(crate::FieldReader<bool, bool>);
impl B751_R {
    pub(crate) fn new(bits: bool) -> Self {
        B751_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B751_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B751` writer - B751"]
pub struct B751_W<'a> {
    w: &'a mut W,
}
impl<'a> B751_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B752` reader - B752"]
pub struct B752_R(crate::FieldReader<bool, bool>);
impl B752_R {
    pub(crate) fn new(bits: bool) -> Self {
        B752_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B752_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B752` writer - B752"]
pub struct B752_W<'a> {
    w: &'a mut W,
}
impl<'a> B752_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B753` reader - B753"]
pub struct B753_R(crate::FieldReader<bool, bool>);
impl B753_R {
    pub(crate) fn new(bits: bool) -> Self {
        B753_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B753_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B753` writer - B753"]
pub struct B753_W<'a> {
    w: &'a mut W,
}
impl<'a> B753_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B754` reader - B754"]
pub struct B754_R(crate::FieldReader<bool, bool>);
impl B754_R {
    pub(crate) fn new(bits: bool) -> Self {
        B754_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B754_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B754` writer - B754"]
pub struct B754_W<'a> {
    w: &'a mut W,
}
impl<'a> B754_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B755` reader - B755"]
pub struct B755_R(crate::FieldReader<bool, bool>);
impl B755_R {
    pub(crate) fn new(bits: bool) -> Self {
        B755_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B755_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B755` writer - B755"]
pub struct B755_W<'a> {
    w: &'a mut W,
}
impl<'a> B755_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B756` reader - B756"]
pub struct B756_R(crate::FieldReader<bool, bool>);
impl B756_R {
    pub(crate) fn new(bits: bool) -> Self {
        B756_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B756_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B756` writer - B756"]
pub struct B756_W<'a> {
    w: &'a mut W,
}
impl<'a> B756_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B757` reader - B757"]
pub struct B757_R(crate::FieldReader<bool, bool>);
impl B757_R {
    pub(crate) fn new(bits: bool) -> Self {
        B757_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B757_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B757` writer - B757"]
pub struct B757_W<'a> {
    w: &'a mut W,
}
impl<'a> B757_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B758` reader - B758"]
pub struct B758_R(crate::FieldReader<bool, bool>);
impl B758_R {
    pub(crate) fn new(bits: bool) -> Self {
        B758_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B758_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B758` writer - B758"]
pub struct B758_W<'a> {
    w: &'a mut W,
}
impl<'a> B758_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B759` reader - B759"]
pub struct B759_R(crate::FieldReader<bool, bool>);
impl B759_R {
    pub(crate) fn new(bits: bool) -> Self {
        B759_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B759_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B759` writer - B759"]
pub struct B759_W<'a> {
    w: &'a mut W,
}
impl<'a> B759_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B760` reader - B760"]
pub struct B760_R(crate::FieldReader<bool, bool>);
impl B760_R {
    pub(crate) fn new(bits: bool) -> Self {
        B760_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B760_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B760` writer - B760"]
pub struct B760_W<'a> {
    w: &'a mut W,
}
impl<'a> B760_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B761` reader - B761"]
pub struct B761_R(crate::FieldReader<bool, bool>);
impl B761_R {
    pub(crate) fn new(bits: bool) -> Self {
        B761_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B761_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B761` writer - B761"]
pub struct B761_W<'a> {
    w: &'a mut W,
}
impl<'a> B761_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B762` reader - B762"]
pub struct B762_R(crate::FieldReader<bool, bool>);
impl B762_R {
    pub(crate) fn new(bits: bool) -> Self {
        B762_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B762_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B762` writer - B762"]
pub struct B762_W<'a> {
    w: &'a mut W,
}
impl<'a> B762_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B763` reader - B763"]
pub struct B763_R(crate::FieldReader<bool, bool>);
impl B763_R {
    pub(crate) fn new(bits: bool) -> Self {
        B763_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B763_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B763` writer - B763"]
pub struct B763_W<'a> {
    w: &'a mut W,
}
impl<'a> B763_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B764` reader - B764"]
pub struct B764_R(crate::FieldReader<bool, bool>);
impl B764_R {
    pub(crate) fn new(bits: bool) -> Self {
        B764_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B764_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B764` writer - B764"]
pub struct B764_W<'a> {
    w: &'a mut W,
}
impl<'a> B764_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B765` reader - B765"]
pub struct B765_R(crate::FieldReader<bool, bool>);
impl B765_R {
    pub(crate) fn new(bits: bool) -> Self {
        B765_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B765_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B765` writer - B765"]
pub struct B765_W<'a> {
    w: &'a mut W,
}
impl<'a> B765_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B766` reader - B766"]
pub struct B766_R(crate::FieldReader<bool, bool>);
impl B766_R {
    pub(crate) fn new(bits: bool) -> Self {
        B766_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B766_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B766` writer - B766"]
pub struct B766_W<'a> {
    w: &'a mut W,
}
impl<'a> B766_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B767` reader - B767"]
pub struct B767_R(crate::FieldReader<bool, bool>);
impl B767_R {
    pub(crate) fn new(bits: bool) -> Self {
        B767_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B767_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B767` writer - B767"]
pub struct B767_W<'a> {
    w: &'a mut W,
}
impl<'a> B767_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B736"]
    #[inline(always)]
    pub fn b736(&self) -> B736_R {
        B736_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B737"]
    #[inline(always)]
    pub fn b737(&self) -> B737_R {
        B737_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B738"]
    #[inline(always)]
    pub fn b738(&self) -> B738_R {
        B738_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B739"]
    #[inline(always)]
    pub fn b739(&self) -> B739_R {
        B739_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B740"]
    #[inline(always)]
    pub fn b740(&self) -> B740_R {
        B740_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B741"]
    #[inline(always)]
    pub fn b741(&self) -> B741_R {
        B741_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B742"]
    #[inline(always)]
    pub fn b742(&self) -> B742_R {
        B742_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B743"]
    #[inline(always)]
    pub fn b743(&self) -> B743_R {
        B743_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B744"]
    #[inline(always)]
    pub fn b744(&self) -> B744_R {
        B744_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B745"]
    #[inline(always)]
    pub fn b745(&self) -> B745_R {
        B745_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B746"]
    #[inline(always)]
    pub fn b746(&self) -> B746_R {
        B746_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B747"]
    #[inline(always)]
    pub fn b747(&self) -> B747_R {
        B747_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B748"]
    #[inline(always)]
    pub fn b748(&self) -> B748_R {
        B748_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B749"]
    #[inline(always)]
    pub fn b749(&self) -> B749_R {
        B749_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B750"]
    #[inline(always)]
    pub fn b750(&self) -> B750_R {
        B750_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B751"]
    #[inline(always)]
    pub fn b751(&self) -> B751_R {
        B751_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B752"]
    #[inline(always)]
    pub fn b752(&self) -> B752_R {
        B752_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B753"]
    #[inline(always)]
    pub fn b753(&self) -> B753_R {
        B753_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B754"]
    #[inline(always)]
    pub fn b754(&self) -> B754_R {
        B754_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B755"]
    #[inline(always)]
    pub fn b755(&self) -> B755_R {
        B755_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B756"]
    #[inline(always)]
    pub fn b756(&self) -> B756_R {
        B756_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B757"]
    #[inline(always)]
    pub fn b757(&self) -> B757_R {
        B757_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B758"]
    #[inline(always)]
    pub fn b758(&self) -> B758_R {
        B758_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B759"]
    #[inline(always)]
    pub fn b759(&self) -> B759_R {
        B759_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B760"]
    #[inline(always)]
    pub fn b760(&self) -> B760_R {
        B760_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B761"]
    #[inline(always)]
    pub fn b761(&self) -> B761_R {
        B761_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B762"]
    #[inline(always)]
    pub fn b762(&self) -> B762_R {
        B762_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B763"]
    #[inline(always)]
    pub fn b763(&self) -> B763_R {
        B763_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B764"]
    #[inline(always)]
    pub fn b764(&self) -> B764_R {
        B764_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B765"]
    #[inline(always)]
    pub fn b765(&self) -> B765_R {
        B765_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B766"]
    #[inline(always)]
    pub fn b766(&self) -> B766_R {
        B766_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B767"]
    #[inline(always)]
    pub fn b767(&self) -> B767_R {
        B767_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B736"]
    #[inline(always)]
    pub fn b736(&mut self) -> B736_W {
        B736_W { w: self }
    }
    #[doc = "Bit 1 - B737"]
    #[inline(always)]
    pub fn b737(&mut self) -> B737_W {
        B737_W { w: self }
    }
    #[doc = "Bit 2 - B738"]
    #[inline(always)]
    pub fn b738(&mut self) -> B738_W {
        B738_W { w: self }
    }
    #[doc = "Bit 3 - B739"]
    #[inline(always)]
    pub fn b739(&mut self) -> B739_W {
        B739_W { w: self }
    }
    #[doc = "Bit 4 - B740"]
    #[inline(always)]
    pub fn b740(&mut self) -> B740_W {
        B740_W { w: self }
    }
    #[doc = "Bit 5 - B741"]
    #[inline(always)]
    pub fn b741(&mut self) -> B741_W {
        B741_W { w: self }
    }
    #[doc = "Bit 6 - B742"]
    #[inline(always)]
    pub fn b742(&mut self) -> B742_W {
        B742_W { w: self }
    }
    #[doc = "Bit 7 - B743"]
    #[inline(always)]
    pub fn b743(&mut self) -> B743_W {
        B743_W { w: self }
    }
    #[doc = "Bit 8 - B744"]
    #[inline(always)]
    pub fn b744(&mut self) -> B744_W {
        B744_W { w: self }
    }
    #[doc = "Bit 9 - B745"]
    #[inline(always)]
    pub fn b745(&mut self) -> B745_W {
        B745_W { w: self }
    }
    #[doc = "Bit 10 - B746"]
    #[inline(always)]
    pub fn b746(&mut self) -> B746_W {
        B746_W { w: self }
    }
    #[doc = "Bit 11 - B747"]
    #[inline(always)]
    pub fn b747(&mut self) -> B747_W {
        B747_W { w: self }
    }
    #[doc = "Bit 12 - B748"]
    #[inline(always)]
    pub fn b748(&mut self) -> B748_W {
        B748_W { w: self }
    }
    #[doc = "Bit 13 - B749"]
    #[inline(always)]
    pub fn b749(&mut self) -> B749_W {
        B749_W { w: self }
    }
    #[doc = "Bit 14 - B750"]
    #[inline(always)]
    pub fn b750(&mut self) -> B750_W {
        B750_W { w: self }
    }
    #[doc = "Bit 15 - B751"]
    #[inline(always)]
    pub fn b751(&mut self) -> B751_W {
        B751_W { w: self }
    }
    #[doc = "Bit 16 - B752"]
    #[inline(always)]
    pub fn b752(&mut self) -> B752_W {
        B752_W { w: self }
    }
    #[doc = "Bit 17 - B753"]
    #[inline(always)]
    pub fn b753(&mut self) -> B753_W {
        B753_W { w: self }
    }
    #[doc = "Bit 18 - B754"]
    #[inline(always)]
    pub fn b754(&mut self) -> B754_W {
        B754_W { w: self }
    }
    #[doc = "Bit 19 - B755"]
    #[inline(always)]
    pub fn b755(&mut self) -> B755_W {
        B755_W { w: self }
    }
    #[doc = "Bit 20 - B756"]
    #[inline(always)]
    pub fn b756(&mut self) -> B756_W {
        B756_W { w: self }
    }
    #[doc = "Bit 21 - B757"]
    #[inline(always)]
    pub fn b757(&mut self) -> B757_W {
        B757_W { w: self }
    }
    #[doc = "Bit 22 - B758"]
    #[inline(always)]
    pub fn b758(&mut self) -> B758_W {
        B758_W { w: self }
    }
    #[doc = "Bit 23 - B759"]
    #[inline(always)]
    pub fn b759(&mut self) -> B759_W {
        B759_W { w: self }
    }
    #[doc = "Bit 24 - B760"]
    #[inline(always)]
    pub fn b760(&mut self) -> B760_W {
        B760_W { w: self }
    }
    #[doc = "Bit 25 - B761"]
    #[inline(always)]
    pub fn b761(&mut self) -> B761_W {
        B761_W { w: self }
    }
    #[doc = "Bit 26 - B762"]
    #[inline(always)]
    pub fn b762(&mut self) -> B762_W {
        B762_W { w: self }
    }
    #[doc = "Bit 27 - B763"]
    #[inline(always)]
    pub fn b763(&mut self) -> B763_W {
        B763_W { w: self }
    }
    #[doc = "Bit 28 - B764"]
    #[inline(always)]
    pub fn b764(&mut self) -> B764_W {
        B764_W { w: self }
    }
    #[doc = "Bit 29 - B765"]
    #[inline(always)]
    pub fn b765(&mut self) -> B765_W {
        B765_W { w: self }
    }
    #[doc = "Bit 30 - B766"]
    #[inline(always)]
    pub fn b766(&mut self) -> B766_W {
        B766_W { w: self }
    }
    #[doc = "Bit 31 - B767"]
    #[inline(always)]
    pub fn b767(&mut self) -> B767_W {
        B767_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr23](index.html) module"]
pub struct MPCBB1_VCTR23_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr23::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr23::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR23 to value 0"]
impl crate::Resettable for MPCBB1_VCTR23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
