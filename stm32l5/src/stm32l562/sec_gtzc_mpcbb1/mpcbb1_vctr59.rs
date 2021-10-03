#[doc = "Register `MPCBB1_VCTR59` reader"]
pub struct R(crate::R<MPCBB1_VCTR59_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR59_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR59_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR59_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR59` writer"]
pub struct W(crate::W<MPCBB1_VCTR59_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR59_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR59_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR59_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1888` reader - B1888"]
pub struct B1888_R(crate::FieldReader<bool, bool>);
impl B1888_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1888_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1888_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1888` writer - B1888"]
pub struct B1888_W<'a> {
    w: &'a mut W,
}
impl<'a> B1888_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1889` reader - B1889"]
pub struct B1889_R(crate::FieldReader<bool, bool>);
impl B1889_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1889_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1889_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1889` writer - B1889"]
pub struct B1889_W<'a> {
    w: &'a mut W,
}
impl<'a> B1889_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1890` reader - B1890"]
pub struct B1890_R(crate::FieldReader<bool, bool>);
impl B1890_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1890_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1890_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1890` writer - B1890"]
pub struct B1890_W<'a> {
    w: &'a mut W,
}
impl<'a> B1890_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1891` reader - B1891"]
pub struct B1891_R(crate::FieldReader<bool, bool>);
impl B1891_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1891_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1891_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1891` writer - B1891"]
pub struct B1891_W<'a> {
    w: &'a mut W,
}
impl<'a> B1891_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1892` reader - B1892"]
pub struct B1892_R(crate::FieldReader<bool, bool>);
impl B1892_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1892_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1892_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1892` writer - B1892"]
pub struct B1892_W<'a> {
    w: &'a mut W,
}
impl<'a> B1892_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1893` reader - B1893"]
pub struct B1893_R(crate::FieldReader<bool, bool>);
impl B1893_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1893_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1893_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1893` writer - B1893"]
pub struct B1893_W<'a> {
    w: &'a mut W,
}
impl<'a> B1893_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1894` reader - B1894"]
pub struct B1894_R(crate::FieldReader<bool, bool>);
impl B1894_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1894_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1894_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1894` writer - B1894"]
pub struct B1894_W<'a> {
    w: &'a mut W,
}
impl<'a> B1894_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1895` reader - B1895"]
pub struct B1895_R(crate::FieldReader<bool, bool>);
impl B1895_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1895_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1895_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1895` writer - B1895"]
pub struct B1895_W<'a> {
    w: &'a mut W,
}
impl<'a> B1895_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1896` reader - B1896"]
pub struct B1896_R(crate::FieldReader<bool, bool>);
impl B1896_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1896_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1896_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1896` writer - B1896"]
pub struct B1896_W<'a> {
    w: &'a mut W,
}
impl<'a> B1896_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1897` reader - B1897"]
pub struct B1897_R(crate::FieldReader<bool, bool>);
impl B1897_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1897_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1897_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1897` writer - B1897"]
pub struct B1897_W<'a> {
    w: &'a mut W,
}
impl<'a> B1897_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1898` reader - B1898"]
pub struct B1898_R(crate::FieldReader<bool, bool>);
impl B1898_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1898_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1898_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1898` writer - B1898"]
pub struct B1898_W<'a> {
    w: &'a mut W,
}
impl<'a> B1898_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1899` reader - B1899"]
pub struct B1899_R(crate::FieldReader<bool, bool>);
impl B1899_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1899_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1899_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1899` writer - B1899"]
pub struct B1899_W<'a> {
    w: &'a mut W,
}
impl<'a> B1899_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1900` reader - B1900"]
pub struct B1900_R(crate::FieldReader<bool, bool>);
impl B1900_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1900_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1900_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1900` writer - B1900"]
pub struct B1900_W<'a> {
    w: &'a mut W,
}
impl<'a> B1900_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1901` reader - B1901"]
pub struct B1901_R(crate::FieldReader<bool, bool>);
impl B1901_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1901_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1901_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1901` writer - B1901"]
pub struct B1901_W<'a> {
    w: &'a mut W,
}
impl<'a> B1901_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1902` reader - B1902"]
pub struct B1902_R(crate::FieldReader<bool, bool>);
impl B1902_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1902_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1902_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1902` writer - B1902"]
pub struct B1902_W<'a> {
    w: &'a mut W,
}
impl<'a> B1902_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1903` reader - B1903"]
pub struct B1903_R(crate::FieldReader<bool, bool>);
impl B1903_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1903_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1903_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1903` writer - B1903"]
pub struct B1903_W<'a> {
    w: &'a mut W,
}
impl<'a> B1903_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1904` reader - B1904"]
pub struct B1904_R(crate::FieldReader<bool, bool>);
impl B1904_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1904_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1904_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1904` writer - B1904"]
pub struct B1904_W<'a> {
    w: &'a mut W,
}
impl<'a> B1904_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1905` reader - B1905"]
pub struct B1905_R(crate::FieldReader<bool, bool>);
impl B1905_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1905_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1905_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1905` writer - B1905"]
pub struct B1905_W<'a> {
    w: &'a mut W,
}
impl<'a> B1905_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1906` reader - B1906"]
pub struct B1906_R(crate::FieldReader<bool, bool>);
impl B1906_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1906_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1906_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1906` writer - B1906"]
pub struct B1906_W<'a> {
    w: &'a mut W,
}
impl<'a> B1906_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1907` reader - B1907"]
pub struct B1907_R(crate::FieldReader<bool, bool>);
impl B1907_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1907_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1907_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1907` writer - B1907"]
pub struct B1907_W<'a> {
    w: &'a mut W,
}
impl<'a> B1907_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1908` reader - B1908"]
pub struct B1908_R(crate::FieldReader<bool, bool>);
impl B1908_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1908_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1908_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1908` writer - B1908"]
pub struct B1908_W<'a> {
    w: &'a mut W,
}
impl<'a> B1908_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1909` reader - B1909"]
pub struct B1909_R(crate::FieldReader<bool, bool>);
impl B1909_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1909_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1909_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1909` writer - B1909"]
pub struct B1909_W<'a> {
    w: &'a mut W,
}
impl<'a> B1909_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1910` reader - B1910"]
pub struct B1910_R(crate::FieldReader<bool, bool>);
impl B1910_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1910_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1910_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1910` writer - B1910"]
pub struct B1910_W<'a> {
    w: &'a mut W,
}
impl<'a> B1910_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1911` reader - B1911"]
pub struct B1911_R(crate::FieldReader<bool, bool>);
impl B1911_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1911_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1911_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1911` writer - B1911"]
pub struct B1911_W<'a> {
    w: &'a mut W,
}
impl<'a> B1911_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1912` reader - B1912"]
pub struct B1912_R(crate::FieldReader<bool, bool>);
impl B1912_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1912_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1912_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1912` writer - B1912"]
pub struct B1912_W<'a> {
    w: &'a mut W,
}
impl<'a> B1912_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1913` reader - B1913"]
pub struct B1913_R(crate::FieldReader<bool, bool>);
impl B1913_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1913_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1913_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1913` writer - B1913"]
pub struct B1913_W<'a> {
    w: &'a mut W,
}
impl<'a> B1913_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1914` reader - B1914"]
pub struct B1914_R(crate::FieldReader<bool, bool>);
impl B1914_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1914_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1914_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1914` writer - B1914"]
pub struct B1914_W<'a> {
    w: &'a mut W,
}
impl<'a> B1914_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1915` reader - B1915"]
pub struct B1915_R(crate::FieldReader<bool, bool>);
impl B1915_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1915_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1915_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1915` writer - B1915"]
pub struct B1915_W<'a> {
    w: &'a mut W,
}
impl<'a> B1915_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1916` reader - B1916"]
pub struct B1916_R(crate::FieldReader<bool, bool>);
impl B1916_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1916_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1916_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1916` writer - B1916"]
pub struct B1916_W<'a> {
    w: &'a mut W,
}
impl<'a> B1916_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1917` reader - B1917"]
pub struct B1917_R(crate::FieldReader<bool, bool>);
impl B1917_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1917_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1917_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1917` writer - B1917"]
pub struct B1917_W<'a> {
    w: &'a mut W,
}
impl<'a> B1917_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1918` reader - B1918"]
pub struct B1918_R(crate::FieldReader<bool, bool>);
impl B1918_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1918_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1918_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1918` writer - B1918"]
pub struct B1918_W<'a> {
    w: &'a mut W,
}
impl<'a> B1918_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1919` reader - B1919"]
pub struct B1919_R(crate::FieldReader<bool, bool>);
impl B1919_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1919_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1919_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1919` writer - B1919"]
pub struct B1919_W<'a> {
    w: &'a mut W,
}
impl<'a> B1919_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1888"]
    #[inline(always)]
    pub fn b1888(&self) -> B1888_R {
        B1888_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1889"]
    #[inline(always)]
    pub fn b1889(&self) -> B1889_R {
        B1889_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1890"]
    #[inline(always)]
    pub fn b1890(&self) -> B1890_R {
        B1890_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1891"]
    #[inline(always)]
    pub fn b1891(&self) -> B1891_R {
        B1891_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1892"]
    #[inline(always)]
    pub fn b1892(&self) -> B1892_R {
        B1892_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1893"]
    #[inline(always)]
    pub fn b1893(&self) -> B1893_R {
        B1893_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1894"]
    #[inline(always)]
    pub fn b1894(&self) -> B1894_R {
        B1894_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1895"]
    #[inline(always)]
    pub fn b1895(&self) -> B1895_R {
        B1895_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1896"]
    #[inline(always)]
    pub fn b1896(&self) -> B1896_R {
        B1896_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1897"]
    #[inline(always)]
    pub fn b1897(&self) -> B1897_R {
        B1897_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1898"]
    #[inline(always)]
    pub fn b1898(&self) -> B1898_R {
        B1898_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1899"]
    #[inline(always)]
    pub fn b1899(&self) -> B1899_R {
        B1899_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1900"]
    #[inline(always)]
    pub fn b1900(&self) -> B1900_R {
        B1900_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1901"]
    #[inline(always)]
    pub fn b1901(&self) -> B1901_R {
        B1901_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1902"]
    #[inline(always)]
    pub fn b1902(&self) -> B1902_R {
        B1902_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1903"]
    #[inline(always)]
    pub fn b1903(&self) -> B1903_R {
        B1903_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1904"]
    #[inline(always)]
    pub fn b1904(&self) -> B1904_R {
        B1904_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1905"]
    #[inline(always)]
    pub fn b1905(&self) -> B1905_R {
        B1905_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1906"]
    #[inline(always)]
    pub fn b1906(&self) -> B1906_R {
        B1906_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1907"]
    #[inline(always)]
    pub fn b1907(&self) -> B1907_R {
        B1907_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1908"]
    #[inline(always)]
    pub fn b1908(&self) -> B1908_R {
        B1908_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1909"]
    #[inline(always)]
    pub fn b1909(&self) -> B1909_R {
        B1909_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1910"]
    #[inline(always)]
    pub fn b1910(&self) -> B1910_R {
        B1910_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1911"]
    #[inline(always)]
    pub fn b1911(&self) -> B1911_R {
        B1911_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1912"]
    #[inline(always)]
    pub fn b1912(&self) -> B1912_R {
        B1912_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1913"]
    #[inline(always)]
    pub fn b1913(&self) -> B1913_R {
        B1913_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1914"]
    #[inline(always)]
    pub fn b1914(&self) -> B1914_R {
        B1914_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1915"]
    #[inline(always)]
    pub fn b1915(&self) -> B1915_R {
        B1915_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1916"]
    #[inline(always)]
    pub fn b1916(&self) -> B1916_R {
        B1916_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1917"]
    #[inline(always)]
    pub fn b1917(&self) -> B1917_R {
        B1917_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1918"]
    #[inline(always)]
    pub fn b1918(&self) -> B1918_R {
        B1918_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1919"]
    #[inline(always)]
    pub fn b1919(&self) -> B1919_R {
        B1919_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1888"]
    #[inline(always)]
    pub fn b1888(&mut self) -> B1888_W {
        B1888_W { w: self }
    }
    #[doc = "Bit 1 - B1889"]
    #[inline(always)]
    pub fn b1889(&mut self) -> B1889_W {
        B1889_W { w: self }
    }
    #[doc = "Bit 2 - B1890"]
    #[inline(always)]
    pub fn b1890(&mut self) -> B1890_W {
        B1890_W { w: self }
    }
    #[doc = "Bit 3 - B1891"]
    #[inline(always)]
    pub fn b1891(&mut self) -> B1891_W {
        B1891_W { w: self }
    }
    #[doc = "Bit 4 - B1892"]
    #[inline(always)]
    pub fn b1892(&mut self) -> B1892_W {
        B1892_W { w: self }
    }
    #[doc = "Bit 5 - B1893"]
    #[inline(always)]
    pub fn b1893(&mut self) -> B1893_W {
        B1893_W { w: self }
    }
    #[doc = "Bit 6 - B1894"]
    #[inline(always)]
    pub fn b1894(&mut self) -> B1894_W {
        B1894_W { w: self }
    }
    #[doc = "Bit 7 - B1895"]
    #[inline(always)]
    pub fn b1895(&mut self) -> B1895_W {
        B1895_W { w: self }
    }
    #[doc = "Bit 8 - B1896"]
    #[inline(always)]
    pub fn b1896(&mut self) -> B1896_W {
        B1896_W { w: self }
    }
    #[doc = "Bit 9 - B1897"]
    #[inline(always)]
    pub fn b1897(&mut self) -> B1897_W {
        B1897_W { w: self }
    }
    #[doc = "Bit 10 - B1898"]
    #[inline(always)]
    pub fn b1898(&mut self) -> B1898_W {
        B1898_W { w: self }
    }
    #[doc = "Bit 11 - B1899"]
    #[inline(always)]
    pub fn b1899(&mut self) -> B1899_W {
        B1899_W { w: self }
    }
    #[doc = "Bit 12 - B1900"]
    #[inline(always)]
    pub fn b1900(&mut self) -> B1900_W {
        B1900_W { w: self }
    }
    #[doc = "Bit 13 - B1901"]
    #[inline(always)]
    pub fn b1901(&mut self) -> B1901_W {
        B1901_W { w: self }
    }
    #[doc = "Bit 14 - B1902"]
    #[inline(always)]
    pub fn b1902(&mut self) -> B1902_W {
        B1902_W { w: self }
    }
    #[doc = "Bit 15 - B1903"]
    #[inline(always)]
    pub fn b1903(&mut self) -> B1903_W {
        B1903_W { w: self }
    }
    #[doc = "Bit 16 - B1904"]
    #[inline(always)]
    pub fn b1904(&mut self) -> B1904_W {
        B1904_W { w: self }
    }
    #[doc = "Bit 17 - B1905"]
    #[inline(always)]
    pub fn b1905(&mut self) -> B1905_W {
        B1905_W { w: self }
    }
    #[doc = "Bit 18 - B1906"]
    #[inline(always)]
    pub fn b1906(&mut self) -> B1906_W {
        B1906_W { w: self }
    }
    #[doc = "Bit 19 - B1907"]
    #[inline(always)]
    pub fn b1907(&mut self) -> B1907_W {
        B1907_W { w: self }
    }
    #[doc = "Bit 20 - B1908"]
    #[inline(always)]
    pub fn b1908(&mut self) -> B1908_W {
        B1908_W { w: self }
    }
    #[doc = "Bit 21 - B1909"]
    #[inline(always)]
    pub fn b1909(&mut self) -> B1909_W {
        B1909_W { w: self }
    }
    #[doc = "Bit 22 - B1910"]
    #[inline(always)]
    pub fn b1910(&mut self) -> B1910_W {
        B1910_W { w: self }
    }
    #[doc = "Bit 23 - B1911"]
    #[inline(always)]
    pub fn b1911(&mut self) -> B1911_W {
        B1911_W { w: self }
    }
    #[doc = "Bit 24 - B1912"]
    #[inline(always)]
    pub fn b1912(&mut self) -> B1912_W {
        B1912_W { w: self }
    }
    #[doc = "Bit 25 - B1913"]
    #[inline(always)]
    pub fn b1913(&mut self) -> B1913_W {
        B1913_W { w: self }
    }
    #[doc = "Bit 26 - B1914"]
    #[inline(always)]
    pub fn b1914(&mut self) -> B1914_W {
        B1914_W { w: self }
    }
    #[doc = "Bit 27 - B1915"]
    #[inline(always)]
    pub fn b1915(&mut self) -> B1915_W {
        B1915_W { w: self }
    }
    #[doc = "Bit 28 - B1916"]
    #[inline(always)]
    pub fn b1916(&mut self) -> B1916_W {
        B1916_W { w: self }
    }
    #[doc = "Bit 29 - B1917"]
    #[inline(always)]
    pub fn b1917(&mut self) -> B1917_W {
        B1917_W { w: self }
    }
    #[doc = "Bit 30 - B1918"]
    #[inline(always)]
    pub fn b1918(&mut self) -> B1918_W {
        B1918_W { w: self }
    }
    #[doc = "Bit 31 - B1919"]
    #[inline(always)]
    pub fn b1919(&mut self) -> B1919_W {
        B1919_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr59](index.html) module"]
pub struct MPCBB1_VCTR59_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR59_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr59::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR59_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr59::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR59_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR59 to value 0"]
impl crate::Resettable for MPCBB1_VCTR59_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
