#[doc = "Register `MPCBB1_VCTR60` reader"]
pub struct R(crate::R<MPCBB1_VCTR60_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR60_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR60_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR60_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR60` writer"]
pub struct W(crate::W<MPCBB1_VCTR60_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR60_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR60_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR60_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1920` reader - B1920"]
pub struct B1920_R(crate::FieldReader<bool, bool>);
impl B1920_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1920_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1920_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1920` writer - B1920"]
pub struct B1920_W<'a> {
    w: &'a mut W,
}
impl<'a> B1920_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1921` reader - B1921"]
pub struct B1921_R(crate::FieldReader<bool, bool>);
impl B1921_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1921_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1921_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1921` writer - B1921"]
pub struct B1921_W<'a> {
    w: &'a mut W,
}
impl<'a> B1921_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1922` reader - B1922"]
pub struct B1922_R(crate::FieldReader<bool, bool>);
impl B1922_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1922_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1922_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1922` writer - B1922"]
pub struct B1922_W<'a> {
    w: &'a mut W,
}
impl<'a> B1922_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1923` reader - B1923"]
pub struct B1923_R(crate::FieldReader<bool, bool>);
impl B1923_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1923_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1923_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1923` writer - B1923"]
pub struct B1923_W<'a> {
    w: &'a mut W,
}
impl<'a> B1923_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1924` reader - B1924"]
pub struct B1924_R(crate::FieldReader<bool, bool>);
impl B1924_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1924_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1924_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1924` writer - B1924"]
pub struct B1924_W<'a> {
    w: &'a mut W,
}
impl<'a> B1924_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1925` reader - B1925"]
pub struct B1925_R(crate::FieldReader<bool, bool>);
impl B1925_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1925_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1925_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1925` writer - B1925"]
pub struct B1925_W<'a> {
    w: &'a mut W,
}
impl<'a> B1925_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1926` reader - B1926"]
pub struct B1926_R(crate::FieldReader<bool, bool>);
impl B1926_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1926_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1926_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1926` writer - B1926"]
pub struct B1926_W<'a> {
    w: &'a mut W,
}
impl<'a> B1926_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1927` reader - B1927"]
pub struct B1927_R(crate::FieldReader<bool, bool>);
impl B1927_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1927_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1927_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1927` writer - B1927"]
pub struct B1927_W<'a> {
    w: &'a mut W,
}
impl<'a> B1927_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1928` reader - B1928"]
pub struct B1928_R(crate::FieldReader<bool, bool>);
impl B1928_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1928_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1928_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1928` writer - B1928"]
pub struct B1928_W<'a> {
    w: &'a mut W,
}
impl<'a> B1928_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1929` reader - B1929"]
pub struct B1929_R(crate::FieldReader<bool, bool>);
impl B1929_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1929_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1929_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1929` writer - B1929"]
pub struct B1929_W<'a> {
    w: &'a mut W,
}
impl<'a> B1929_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1930` reader - B1930"]
pub struct B1930_R(crate::FieldReader<bool, bool>);
impl B1930_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1930_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1930_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1930` writer - B1930"]
pub struct B1930_W<'a> {
    w: &'a mut W,
}
impl<'a> B1930_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1931` reader - B1931"]
pub struct B1931_R(crate::FieldReader<bool, bool>);
impl B1931_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1931_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1931_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1931` writer - B1931"]
pub struct B1931_W<'a> {
    w: &'a mut W,
}
impl<'a> B1931_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1932` reader - B1932"]
pub struct B1932_R(crate::FieldReader<bool, bool>);
impl B1932_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1932_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1932_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1932` writer - B1932"]
pub struct B1932_W<'a> {
    w: &'a mut W,
}
impl<'a> B1932_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1933` reader - B1933"]
pub struct B1933_R(crate::FieldReader<bool, bool>);
impl B1933_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1933_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1933_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1933` writer - B1933"]
pub struct B1933_W<'a> {
    w: &'a mut W,
}
impl<'a> B1933_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1934` reader - B1934"]
pub struct B1934_R(crate::FieldReader<bool, bool>);
impl B1934_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1934_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1934_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1934` writer - B1934"]
pub struct B1934_W<'a> {
    w: &'a mut W,
}
impl<'a> B1934_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1935` reader - B1935"]
pub struct B1935_R(crate::FieldReader<bool, bool>);
impl B1935_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1935_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1935_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1935` writer - B1935"]
pub struct B1935_W<'a> {
    w: &'a mut W,
}
impl<'a> B1935_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1936` reader - B1936"]
pub struct B1936_R(crate::FieldReader<bool, bool>);
impl B1936_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1936_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1936_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1936` writer - B1936"]
pub struct B1936_W<'a> {
    w: &'a mut W,
}
impl<'a> B1936_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1937` reader - B1937"]
pub struct B1937_R(crate::FieldReader<bool, bool>);
impl B1937_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1937_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1937_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1937` writer - B1937"]
pub struct B1937_W<'a> {
    w: &'a mut W,
}
impl<'a> B1937_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1938` reader - B1938"]
pub struct B1938_R(crate::FieldReader<bool, bool>);
impl B1938_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1938_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1938_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1938` writer - B1938"]
pub struct B1938_W<'a> {
    w: &'a mut W,
}
impl<'a> B1938_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1939` reader - B1939"]
pub struct B1939_R(crate::FieldReader<bool, bool>);
impl B1939_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1939_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1939_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1939` writer - B1939"]
pub struct B1939_W<'a> {
    w: &'a mut W,
}
impl<'a> B1939_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1940` reader - B1940"]
pub struct B1940_R(crate::FieldReader<bool, bool>);
impl B1940_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1940_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1940_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1940` writer - B1940"]
pub struct B1940_W<'a> {
    w: &'a mut W,
}
impl<'a> B1940_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1941` reader - B1941"]
pub struct B1941_R(crate::FieldReader<bool, bool>);
impl B1941_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1941_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1941_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1941` writer - B1941"]
pub struct B1941_W<'a> {
    w: &'a mut W,
}
impl<'a> B1941_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1942` reader - B1942"]
pub struct B1942_R(crate::FieldReader<bool, bool>);
impl B1942_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1942_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1942_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1942` writer - B1942"]
pub struct B1942_W<'a> {
    w: &'a mut W,
}
impl<'a> B1942_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1943` reader - B1943"]
pub struct B1943_R(crate::FieldReader<bool, bool>);
impl B1943_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1943_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1943_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1943` writer - B1943"]
pub struct B1943_W<'a> {
    w: &'a mut W,
}
impl<'a> B1943_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1944` reader - B1944"]
pub struct B1944_R(crate::FieldReader<bool, bool>);
impl B1944_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1944_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1944_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1944` writer - B1944"]
pub struct B1944_W<'a> {
    w: &'a mut W,
}
impl<'a> B1944_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1945` reader - B1945"]
pub struct B1945_R(crate::FieldReader<bool, bool>);
impl B1945_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1945_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1945_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1945` writer - B1945"]
pub struct B1945_W<'a> {
    w: &'a mut W,
}
impl<'a> B1945_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1946` reader - B1946"]
pub struct B1946_R(crate::FieldReader<bool, bool>);
impl B1946_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1946_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1946_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1946` writer - B1946"]
pub struct B1946_W<'a> {
    w: &'a mut W,
}
impl<'a> B1946_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1947` reader - B1947"]
pub struct B1947_R(crate::FieldReader<bool, bool>);
impl B1947_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1947_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1947_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1947` writer - B1947"]
pub struct B1947_W<'a> {
    w: &'a mut W,
}
impl<'a> B1947_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1948` reader - B1948"]
pub struct B1948_R(crate::FieldReader<bool, bool>);
impl B1948_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1948_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1948_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1948` writer - B1948"]
pub struct B1948_W<'a> {
    w: &'a mut W,
}
impl<'a> B1948_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1949` reader - B1949"]
pub struct B1949_R(crate::FieldReader<bool, bool>);
impl B1949_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1949_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1949_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1949` writer - B1949"]
pub struct B1949_W<'a> {
    w: &'a mut W,
}
impl<'a> B1949_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1950` reader - B1950"]
pub struct B1950_R(crate::FieldReader<bool, bool>);
impl B1950_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1950_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1950_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1950` writer - B1950"]
pub struct B1950_W<'a> {
    w: &'a mut W,
}
impl<'a> B1950_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1951` reader - B1951"]
pub struct B1951_R(crate::FieldReader<bool, bool>);
impl B1951_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1951_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1951_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1951` writer - B1951"]
pub struct B1951_W<'a> {
    w: &'a mut W,
}
impl<'a> B1951_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1920"]
    #[inline(always)]
    pub fn b1920(&self) -> B1920_R {
        B1920_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1921"]
    #[inline(always)]
    pub fn b1921(&self) -> B1921_R {
        B1921_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1922"]
    #[inline(always)]
    pub fn b1922(&self) -> B1922_R {
        B1922_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1923"]
    #[inline(always)]
    pub fn b1923(&self) -> B1923_R {
        B1923_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1924"]
    #[inline(always)]
    pub fn b1924(&self) -> B1924_R {
        B1924_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1925"]
    #[inline(always)]
    pub fn b1925(&self) -> B1925_R {
        B1925_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1926"]
    #[inline(always)]
    pub fn b1926(&self) -> B1926_R {
        B1926_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1927"]
    #[inline(always)]
    pub fn b1927(&self) -> B1927_R {
        B1927_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1928"]
    #[inline(always)]
    pub fn b1928(&self) -> B1928_R {
        B1928_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1929"]
    #[inline(always)]
    pub fn b1929(&self) -> B1929_R {
        B1929_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1930"]
    #[inline(always)]
    pub fn b1930(&self) -> B1930_R {
        B1930_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1931"]
    #[inline(always)]
    pub fn b1931(&self) -> B1931_R {
        B1931_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1932"]
    #[inline(always)]
    pub fn b1932(&self) -> B1932_R {
        B1932_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1933"]
    #[inline(always)]
    pub fn b1933(&self) -> B1933_R {
        B1933_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1934"]
    #[inline(always)]
    pub fn b1934(&self) -> B1934_R {
        B1934_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1935"]
    #[inline(always)]
    pub fn b1935(&self) -> B1935_R {
        B1935_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1936"]
    #[inline(always)]
    pub fn b1936(&self) -> B1936_R {
        B1936_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1937"]
    #[inline(always)]
    pub fn b1937(&self) -> B1937_R {
        B1937_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1938"]
    #[inline(always)]
    pub fn b1938(&self) -> B1938_R {
        B1938_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1939"]
    #[inline(always)]
    pub fn b1939(&self) -> B1939_R {
        B1939_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1940"]
    #[inline(always)]
    pub fn b1940(&self) -> B1940_R {
        B1940_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1941"]
    #[inline(always)]
    pub fn b1941(&self) -> B1941_R {
        B1941_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1942"]
    #[inline(always)]
    pub fn b1942(&self) -> B1942_R {
        B1942_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1943"]
    #[inline(always)]
    pub fn b1943(&self) -> B1943_R {
        B1943_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1944"]
    #[inline(always)]
    pub fn b1944(&self) -> B1944_R {
        B1944_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1945"]
    #[inline(always)]
    pub fn b1945(&self) -> B1945_R {
        B1945_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1946"]
    #[inline(always)]
    pub fn b1946(&self) -> B1946_R {
        B1946_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1947"]
    #[inline(always)]
    pub fn b1947(&self) -> B1947_R {
        B1947_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1948"]
    #[inline(always)]
    pub fn b1948(&self) -> B1948_R {
        B1948_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1949"]
    #[inline(always)]
    pub fn b1949(&self) -> B1949_R {
        B1949_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1950"]
    #[inline(always)]
    pub fn b1950(&self) -> B1950_R {
        B1950_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1951"]
    #[inline(always)]
    pub fn b1951(&self) -> B1951_R {
        B1951_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1920"]
    #[inline(always)]
    pub fn b1920(&mut self) -> B1920_W {
        B1920_W { w: self }
    }
    #[doc = "Bit 1 - B1921"]
    #[inline(always)]
    pub fn b1921(&mut self) -> B1921_W {
        B1921_W { w: self }
    }
    #[doc = "Bit 2 - B1922"]
    #[inline(always)]
    pub fn b1922(&mut self) -> B1922_W {
        B1922_W { w: self }
    }
    #[doc = "Bit 3 - B1923"]
    #[inline(always)]
    pub fn b1923(&mut self) -> B1923_W {
        B1923_W { w: self }
    }
    #[doc = "Bit 4 - B1924"]
    #[inline(always)]
    pub fn b1924(&mut self) -> B1924_W {
        B1924_W { w: self }
    }
    #[doc = "Bit 5 - B1925"]
    #[inline(always)]
    pub fn b1925(&mut self) -> B1925_W {
        B1925_W { w: self }
    }
    #[doc = "Bit 6 - B1926"]
    #[inline(always)]
    pub fn b1926(&mut self) -> B1926_W {
        B1926_W { w: self }
    }
    #[doc = "Bit 7 - B1927"]
    #[inline(always)]
    pub fn b1927(&mut self) -> B1927_W {
        B1927_W { w: self }
    }
    #[doc = "Bit 8 - B1928"]
    #[inline(always)]
    pub fn b1928(&mut self) -> B1928_W {
        B1928_W { w: self }
    }
    #[doc = "Bit 9 - B1929"]
    #[inline(always)]
    pub fn b1929(&mut self) -> B1929_W {
        B1929_W { w: self }
    }
    #[doc = "Bit 10 - B1930"]
    #[inline(always)]
    pub fn b1930(&mut self) -> B1930_W {
        B1930_W { w: self }
    }
    #[doc = "Bit 11 - B1931"]
    #[inline(always)]
    pub fn b1931(&mut self) -> B1931_W {
        B1931_W { w: self }
    }
    #[doc = "Bit 12 - B1932"]
    #[inline(always)]
    pub fn b1932(&mut self) -> B1932_W {
        B1932_W { w: self }
    }
    #[doc = "Bit 13 - B1933"]
    #[inline(always)]
    pub fn b1933(&mut self) -> B1933_W {
        B1933_W { w: self }
    }
    #[doc = "Bit 14 - B1934"]
    #[inline(always)]
    pub fn b1934(&mut self) -> B1934_W {
        B1934_W { w: self }
    }
    #[doc = "Bit 15 - B1935"]
    #[inline(always)]
    pub fn b1935(&mut self) -> B1935_W {
        B1935_W { w: self }
    }
    #[doc = "Bit 16 - B1936"]
    #[inline(always)]
    pub fn b1936(&mut self) -> B1936_W {
        B1936_W { w: self }
    }
    #[doc = "Bit 17 - B1937"]
    #[inline(always)]
    pub fn b1937(&mut self) -> B1937_W {
        B1937_W { w: self }
    }
    #[doc = "Bit 18 - B1938"]
    #[inline(always)]
    pub fn b1938(&mut self) -> B1938_W {
        B1938_W { w: self }
    }
    #[doc = "Bit 19 - B1939"]
    #[inline(always)]
    pub fn b1939(&mut self) -> B1939_W {
        B1939_W { w: self }
    }
    #[doc = "Bit 20 - B1940"]
    #[inline(always)]
    pub fn b1940(&mut self) -> B1940_W {
        B1940_W { w: self }
    }
    #[doc = "Bit 21 - B1941"]
    #[inline(always)]
    pub fn b1941(&mut self) -> B1941_W {
        B1941_W { w: self }
    }
    #[doc = "Bit 22 - B1942"]
    #[inline(always)]
    pub fn b1942(&mut self) -> B1942_W {
        B1942_W { w: self }
    }
    #[doc = "Bit 23 - B1943"]
    #[inline(always)]
    pub fn b1943(&mut self) -> B1943_W {
        B1943_W { w: self }
    }
    #[doc = "Bit 24 - B1944"]
    #[inline(always)]
    pub fn b1944(&mut self) -> B1944_W {
        B1944_W { w: self }
    }
    #[doc = "Bit 25 - B1945"]
    #[inline(always)]
    pub fn b1945(&mut self) -> B1945_W {
        B1945_W { w: self }
    }
    #[doc = "Bit 26 - B1946"]
    #[inline(always)]
    pub fn b1946(&mut self) -> B1946_W {
        B1946_W { w: self }
    }
    #[doc = "Bit 27 - B1947"]
    #[inline(always)]
    pub fn b1947(&mut self) -> B1947_W {
        B1947_W { w: self }
    }
    #[doc = "Bit 28 - B1948"]
    #[inline(always)]
    pub fn b1948(&mut self) -> B1948_W {
        B1948_W { w: self }
    }
    #[doc = "Bit 29 - B1949"]
    #[inline(always)]
    pub fn b1949(&mut self) -> B1949_W {
        B1949_W { w: self }
    }
    #[doc = "Bit 30 - B1950"]
    #[inline(always)]
    pub fn b1950(&mut self) -> B1950_W {
        B1950_W { w: self }
    }
    #[doc = "Bit 31 - B1951"]
    #[inline(always)]
    pub fn b1951(&mut self) -> B1951_W {
        B1951_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr60](index.html) module"]
pub struct MPCBB1_VCTR60_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR60_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr60::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR60_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr60::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR60_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR60 to value 0"]
impl crate::Resettable for MPCBB1_VCTR60_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
