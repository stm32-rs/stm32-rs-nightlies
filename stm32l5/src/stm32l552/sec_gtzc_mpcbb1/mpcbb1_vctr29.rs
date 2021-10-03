#[doc = "Register `MPCBB1_VCTR29` reader"]
pub struct R(crate::R<MPCBB1_VCTR29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR29` writer"]
pub struct W(crate::W<MPCBB1_VCTR29_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR29_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR29_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR29_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B928` reader - B928"]
pub struct B928_R(crate::FieldReader<bool, bool>);
impl B928_R {
    pub(crate) fn new(bits: bool) -> Self {
        B928_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B928_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B928` writer - B928"]
pub struct B928_W<'a> {
    w: &'a mut W,
}
impl<'a> B928_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B929` reader - B929"]
pub struct B929_R(crate::FieldReader<bool, bool>);
impl B929_R {
    pub(crate) fn new(bits: bool) -> Self {
        B929_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B929_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B929` writer - B929"]
pub struct B929_W<'a> {
    w: &'a mut W,
}
impl<'a> B929_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B930` reader - B930"]
pub struct B930_R(crate::FieldReader<bool, bool>);
impl B930_R {
    pub(crate) fn new(bits: bool) -> Self {
        B930_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B930_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B930` writer - B930"]
pub struct B930_W<'a> {
    w: &'a mut W,
}
impl<'a> B930_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B931` reader - B931"]
pub struct B931_R(crate::FieldReader<bool, bool>);
impl B931_R {
    pub(crate) fn new(bits: bool) -> Self {
        B931_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B931_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B931` writer - B931"]
pub struct B931_W<'a> {
    w: &'a mut W,
}
impl<'a> B931_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B932` reader - B932"]
pub struct B932_R(crate::FieldReader<bool, bool>);
impl B932_R {
    pub(crate) fn new(bits: bool) -> Self {
        B932_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B932_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B932` writer - B932"]
pub struct B932_W<'a> {
    w: &'a mut W,
}
impl<'a> B932_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B933` reader - B933"]
pub struct B933_R(crate::FieldReader<bool, bool>);
impl B933_R {
    pub(crate) fn new(bits: bool) -> Self {
        B933_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B933_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B933` writer - B933"]
pub struct B933_W<'a> {
    w: &'a mut W,
}
impl<'a> B933_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B934` reader - B934"]
pub struct B934_R(crate::FieldReader<bool, bool>);
impl B934_R {
    pub(crate) fn new(bits: bool) -> Self {
        B934_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B934_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B934` writer - B934"]
pub struct B934_W<'a> {
    w: &'a mut W,
}
impl<'a> B934_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B935` reader - B935"]
pub struct B935_R(crate::FieldReader<bool, bool>);
impl B935_R {
    pub(crate) fn new(bits: bool) -> Self {
        B935_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B935_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B935` writer - B935"]
pub struct B935_W<'a> {
    w: &'a mut W,
}
impl<'a> B935_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B936` reader - B936"]
pub struct B936_R(crate::FieldReader<bool, bool>);
impl B936_R {
    pub(crate) fn new(bits: bool) -> Self {
        B936_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B936_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B936` writer - B936"]
pub struct B936_W<'a> {
    w: &'a mut W,
}
impl<'a> B936_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B937` reader - B937"]
pub struct B937_R(crate::FieldReader<bool, bool>);
impl B937_R {
    pub(crate) fn new(bits: bool) -> Self {
        B937_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B937_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B937` writer - B937"]
pub struct B937_W<'a> {
    w: &'a mut W,
}
impl<'a> B937_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B938` reader - B938"]
pub struct B938_R(crate::FieldReader<bool, bool>);
impl B938_R {
    pub(crate) fn new(bits: bool) -> Self {
        B938_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B938_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B938` writer - B938"]
pub struct B938_W<'a> {
    w: &'a mut W,
}
impl<'a> B938_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B939` reader - B939"]
pub struct B939_R(crate::FieldReader<bool, bool>);
impl B939_R {
    pub(crate) fn new(bits: bool) -> Self {
        B939_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B939_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B939` writer - B939"]
pub struct B939_W<'a> {
    w: &'a mut W,
}
impl<'a> B939_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B940` reader - B940"]
pub struct B940_R(crate::FieldReader<bool, bool>);
impl B940_R {
    pub(crate) fn new(bits: bool) -> Self {
        B940_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B940_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B940` writer - B940"]
pub struct B940_W<'a> {
    w: &'a mut W,
}
impl<'a> B940_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B941` reader - B941"]
pub struct B941_R(crate::FieldReader<bool, bool>);
impl B941_R {
    pub(crate) fn new(bits: bool) -> Self {
        B941_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B941_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B941` writer - B941"]
pub struct B941_W<'a> {
    w: &'a mut W,
}
impl<'a> B941_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B942` reader - B942"]
pub struct B942_R(crate::FieldReader<bool, bool>);
impl B942_R {
    pub(crate) fn new(bits: bool) -> Self {
        B942_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B942_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B942` writer - B942"]
pub struct B942_W<'a> {
    w: &'a mut W,
}
impl<'a> B942_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B943` reader - B943"]
pub struct B943_R(crate::FieldReader<bool, bool>);
impl B943_R {
    pub(crate) fn new(bits: bool) -> Self {
        B943_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B943_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B943` writer - B943"]
pub struct B943_W<'a> {
    w: &'a mut W,
}
impl<'a> B943_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B944` reader - B944"]
pub struct B944_R(crate::FieldReader<bool, bool>);
impl B944_R {
    pub(crate) fn new(bits: bool) -> Self {
        B944_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B944_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B944` writer - B944"]
pub struct B944_W<'a> {
    w: &'a mut W,
}
impl<'a> B944_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B945` reader - B945"]
pub struct B945_R(crate::FieldReader<bool, bool>);
impl B945_R {
    pub(crate) fn new(bits: bool) -> Self {
        B945_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B945_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B945` writer - B945"]
pub struct B945_W<'a> {
    w: &'a mut W,
}
impl<'a> B945_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B946` reader - B946"]
pub struct B946_R(crate::FieldReader<bool, bool>);
impl B946_R {
    pub(crate) fn new(bits: bool) -> Self {
        B946_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B946_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B946` writer - B946"]
pub struct B946_W<'a> {
    w: &'a mut W,
}
impl<'a> B946_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B947` reader - B947"]
pub struct B947_R(crate::FieldReader<bool, bool>);
impl B947_R {
    pub(crate) fn new(bits: bool) -> Self {
        B947_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B947_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B947` writer - B947"]
pub struct B947_W<'a> {
    w: &'a mut W,
}
impl<'a> B947_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B948` reader - B948"]
pub struct B948_R(crate::FieldReader<bool, bool>);
impl B948_R {
    pub(crate) fn new(bits: bool) -> Self {
        B948_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B948_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B948` writer - B948"]
pub struct B948_W<'a> {
    w: &'a mut W,
}
impl<'a> B948_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B949` reader - B949"]
pub struct B949_R(crate::FieldReader<bool, bool>);
impl B949_R {
    pub(crate) fn new(bits: bool) -> Self {
        B949_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B949_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B949` writer - B949"]
pub struct B949_W<'a> {
    w: &'a mut W,
}
impl<'a> B949_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B950` reader - B950"]
pub struct B950_R(crate::FieldReader<bool, bool>);
impl B950_R {
    pub(crate) fn new(bits: bool) -> Self {
        B950_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B950_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B950` writer - B950"]
pub struct B950_W<'a> {
    w: &'a mut W,
}
impl<'a> B950_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B951` reader - B951"]
pub struct B951_R(crate::FieldReader<bool, bool>);
impl B951_R {
    pub(crate) fn new(bits: bool) -> Self {
        B951_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B951_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B951` writer - B951"]
pub struct B951_W<'a> {
    w: &'a mut W,
}
impl<'a> B951_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B952` reader - B952"]
pub struct B952_R(crate::FieldReader<bool, bool>);
impl B952_R {
    pub(crate) fn new(bits: bool) -> Self {
        B952_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B952_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B952` writer - B952"]
pub struct B952_W<'a> {
    w: &'a mut W,
}
impl<'a> B952_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B953` reader - B953"]
pub struct B953_R(crate::FieldReader<bool, bool>);
impl B953_R {
    pub(crate) fn new(bits: bool) -> Self {
        B953_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B953_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B953` writer - B953"]
pub struct B953_W<'a> {
    w: &'a mut W,
}
impl<'a> B953_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B954` reader - B954"]
pub struct B954_R(crate::FieldReader<bool, bool>);
impl B954_R {
    pub(crate) fn new(bits: bool) -> Self {
        B954_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B954_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B954` writer - B954"]
pub struct B954_W<'a> {
    w: &'a mut W,
}
impl<'a> B954_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B955` reader - B955"]
pub struct B955_R(crate::FieldReader<bool, bool>);
impl B955_R {
    pub(crate) fn new(bits: bool) -> Self {
        B955_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B955_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B955` writer - B955"]
pub struct B955_W<'a> {
    w: &'a mut W,
}
impl<'a> B955_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B956` reader - B956"]
pub struct B956_R(crate::FieldReader<bool, bool>);
impl B956_R {
    pub(crate) fn new(bits: bool) -> Self {
        B956_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B956_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B956` writer - B956"]
pub struct B956_W<'a> {
    w: &'a mut W,
}
impl<'a> B956_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B957` reader - B957"]
pub struct B957_R(crate::FieldReader<bool, bool>);
impl B957_R {
    pub(crate) fn new(bits: bool) -> Self {
        B957_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B957_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B957` writer - B957"]
pub struct B957_W<'a> {
    w: &'a mut W,
}
impl<'a> B957_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B958` reader - B958"]
pub struct B958_R(crate::FieldReader<bool, bool>);
impl B958_R {
    pub(crate) fn new(bits: bool) -> Self {
        B958_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B958_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B958` writer - B958"]
pub struct B958_W<'a> {
    w: &'a mut W,
}
impl<'a> B958_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B959` reader - B959"]
pub struct B959_R(crate::FieldReader<bool, bool>);
impl B959_R {
    pub(crate) fn new(bits: bool) -> Self {
        B959_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B959_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B959` writer - B959"]
pub struct B959_W<'a> {
    w: &'a mut W,
}
impl<'a> B959_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B928"]
    #[inline(always)]
    pub fn b928(&self) -> B928_R {
        B928_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B929"]
    #[inline(always)]
    pub fn b929(&self) -> B929_R {
        B929_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B930"]
    #[inline(always)]
    pub fn b930(&self) -> B930_R {
        B930_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B931"]
    #[inline(always)]
    pub fn b931(&self) -> B931_R {
        B931_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B932"]
    #[inline(always)]
    pub fn b932(&self) -> B932_R {
        B932_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B933"]
    #[inline(always)]
    pub fn b933(&self) -> B933_R {
        B933_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B934"]
    #[inline(always)]
    pub fn b934(&self) -> B934_R {
        B934_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B935"]
    #[inline(always)]
    pub fn b935(&self) -> B935_R {
        B935_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B936"]
    #[inline(always)]
    pub fn b936(&self) -> B936_R {
        B936_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B937"]
    #[inline(always)]
    pub fn b937(&self) -> B937_R {
        B937_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B938"]
    #[inline(always)]
    pub fn b938(&self) -> B938_R {
        B938_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B939"]
    #[inline(always)]
    pub fn b939(&self) -> B939_R {
        B939_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B940"]
    #[inline(always)]
    pub fn b940(&self) -> B940_R {
        B940_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B941"]
    #[inline(always)]
    pub fn b941(&self) -> B941_R {
        B941_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B942"]
    #[inline(always)]
    pub fn b942(&self) -> B942_R {
        B942_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B943"]
    #[inline(always)]
    pub fn b943(&self) -> B943_R {
        B943_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B944"]
    #[inline(always)]
    pub fn b944(&self) -> B944_R {
        B944_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B945"]
    #[inline(always)]
    pub fn b945(&self) -> B945_R {
        B945_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B946"]
    #[inline(always)]
    pub fn b946(&self) -> B946_R {
        B946_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B947"]
    #[inline(always)]
    pub fn b947(&self) -> B947_R {
        B947_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B948"]
    #[inline(always)]
    pub fn b948(&self) -> B948_R {
        B948_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B949"]
    #[inline(always)]
    pub fn b949(&self) -> B949_R {
        B949_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B950"]
    #[inline(always)]
    pub fn b950(&self) -> B950_R {
        B950_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B951"]
    #[inline(always)]
    pub fn b951(&self) -> B951_R {
        B951_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B952"]
    #[inline(always)]
    pub fn b952(&self) -> B952_R {
        B952_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B953"]
    #[inline(always)]
    pub fn b953(&self) -> B953_R {
        B953_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B954"]
    #[inline(always)]
    pub fn b954(&self) -> B954_R {
        B954_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B955"]
    #[inline(always)]
    pub fn b955(&self) -> B955_R {
        B955_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B956"]
    #[inline(always)]
    pub fn b956(&self) -> B956_R {
        B956_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B957"]
    #[inline(always)]
    pub fn b957(&self) -> B957_R {
        B957_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B958"]
    #[inline(always)]
    pub fn b958(&self) -> B958_R {
        B958_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B959"]
    #[inline(always)]
    pub fn b959(&self) -> B959_R {
        B959_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B928"]
    #[inline(always)]
    pub fn b928(&mut self) -> B928_W {
        B928_W { w: self }
    }
    #[doc = "Bit 1 - B929"]
    #[inline(always)]
    pub fn b929(&mut self) -> B929_W {
        B929_W { w: self }
    }
    #[doc = "Bit 2 - B930"]
    #[inline(always)]
    pub fn b930(&mut self) -> B930_W {
        B930_W { w: self }
    }
    #[doc = "Bit 3 - B931"]
    #[inline(always)]
    pub fn b931(&mut self) -> B931_W {
        B931_W { w: self }
    }
    #[doc = "Bit 4 - B932"]
    #[inline(always)]
    pub fn b932(&mut self) -> B932_W {
        B932_W { w: self }
    }
    #[doc = "Bit 5 - B933"]
    #[inline(always)]
    pub fn b933(&mut self) -> B933_W {
        B933_W { w: self }
    }
    #[doc = "Bit 6 - B934"]
    #[inline(always)]
    pub fn b934(&mut self) -> B934_W {
        B934_W { w: self }
    }
    #[doc = "Bit 7 - B935"]
    #[inline(always)]
    pub fn b935(&mut self) -> B935_W {
        B935_W { w: self }
    }
    #[doc = "Bit 8 - B936"]
    #[inline(always)]
    pub fn b936(&mut self) -> B936_W {
        B936_W { w: self }
    }
    #[doc = "Bit 9 - B937"]
    #[inline(always)]
    pub fn b937(&mut self) -> B937_W {
        B937_W { w: self }
    }
    #[doc = "Bit 10 - B938"]
    #[inline(always)]
    pub fn b938(&mut self) -> B938_W {
        B938_W { w: self }
    }
    #[doc = "Bit 11 - B939"]
    #[inline(always)]
    pub fn b939(&mut self) -> B939_W {
        B939_W { w: self }
    }
    #[doc = "Bit 12 - B940"]
    #[inline(always)]
    pub fn b940(&mut self) -> B940_W {
        B940_W { w: self }
    }
    #[doc = "Bit 13 - B941"]
    #[inline(always)]
    pub fn b941(&mut self) -> B941_W {
        B941_W { w: self }
    }
    #[doc = "Bit 14 - B942"]
    #[inline(always)]
    pub fn b942(&mut self) -> B942_W {
        B942_W { w: self }
    }
    #[doc = "Bit 15 - B943"]
    #[inline(always)]
    pub fn b943(&mut self) -> B943_W {
        B943_W { w: self }
    }
    #[doc = "Bit 16 - B944"]
    #[inline(always)]
    pub fn b944(&mut self) -> B944_W {
        B944_W { w: self }
    }
    #[doc = "Bit 17 - B945"]
    #[inline(always)]
    pub fn b945(&mut self) -> B945_W {
        B945_W { w: self }
    }
    #[doc = "Bit 18 - B946"]
    #[inline(always)]
    pub fn b946(&mut self) -> B946_W {
        B946_W { w: self }
    }
    #[doc = "Bit 19 - B947"]
    #[inline(always)]
    pub fn b947(&mut self) -> B947_W {
        B947_W { w: self }
    }
    #[doc = "Bit 20 - B948"]
    #[inline(always)]
    pub fn b948(&mut self) -> B948_W {
        B948_W { w: self }
    }
    #[doc = "Bit 21 - B949"]
    #[inline(always)]
    pub fn b949(&mut self) -> B949_W {
        B949_W { w: self }
    }
    #[doc = "Bit 22 - B950"]
    #[inline(always)]
    pub fn b950(&mut self) -> B950_W {
        B950_W { w: self }
    }
    #[doc = "Bit 23 - B951"]
    #[inline(always)]
    pub fn b951(&mut self) -> B951_W {
        B951_W { w: self }
    }
    #[doc = "Bit 24 - B952"]
    #[inline(always)]
    pub fn b952(&mut self) -> B952_W {
        B952_W { w: self }
    }
    #[doc = "Bit 25 - B953"]
    #[inline(always)]
    pub fn b953(&mut self) -> B953_W {
        B953_W { w: self }
    }
    #[doc = "Bit 26 - B954"]
    #[inline(always)]
    pub fn b954(&mut self) -> B954_W {
        B954_W { w: self }
    }
    #[doc = "Bit 27 - B955"]
    #[inline(always)]
    pub fn b955(&mut self) -> B955_W {
        B955_W { w: self }
    }
    #[doc = "Bit 28 - B956"]
    #[inline(always)]
    pub fn b956(&mut self) -> B956_W {
        B956_W { w: self }
    }
    #[doc = "Bit 29 - B957"]
    #[inline(always)]
    pub fn b957(&mut self) -> B957_W {
        B957_W { w: self }
    }
    #[doc = "Bit 30 - B958"]
    #[inline(always)]
    pub fn b958(&mut self) -> B958_W {
        B958_W { w: self }
    }
    #[doc = "Bit 31 - B959"]
    #[inline(always)]
    pub fn b959(&mut self) -> B959_W {
        B959_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr29](index.html) module"]
pub struct MPCBB1_VCTR29_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr29::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR29_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr29::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR29_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR29 to value 0"]
impl crate::Resettable for MPCBB1_VCTR29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
