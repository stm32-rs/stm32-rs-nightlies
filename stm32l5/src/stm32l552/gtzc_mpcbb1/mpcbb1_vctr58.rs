#[doc = "Register `MPCBB1_VCTR58` reader"]
pub struct R(crate::R<MPCBB1_VCTR58_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR58_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR58_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR58_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR58` writer"]
pub struct W(crate::W<MPCBB1_VCTR58_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR58_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR58_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR58_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1856` reader - B1856"]
pub struct B1856_R(crate::FieldReader<bool, bool>);
impl B1856_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1856_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1856_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1856` writer - B1856"]
pub struct B1856_W<'a> {
    w: &'a mut W,
}
impl<'a> B1856_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1857` reader - B1857"]
pub struct B1857_R(crate::FieldReader<bool, bool>);
impl B1857_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1857_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1857_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1857` writer - B1857"]
pub struct B1857_W<'a> {
    w: &'a mut W,
}
impl<'a> B1857_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1858` reader - B1858"]
pub struct B1858_R(crate::FieldReader<bool, bool>);
impl B1858_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1858_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1858_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1858` writer - B1858"]
pub struct B1858_W<'a> {
    w: &'a mut W,
}
impl<'a> B1858_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1859` reader - B1859"]
pub struct B1859_R(crate::FieldReader<bool, bool>);
impl B1859_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1859_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1859_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1859` writer - B1859"]
pub struct B1859_W<'a> {
    w: &'a mut W,
}
impl<'a> B1859_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1860` reader - B1860"]
pub struct B1860_R(crate::FieldReader<bool, bool>);
impl B1860_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1860_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1860_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1860` writer - B1860"]
pub struct B1860_W<'a> {
    w: &'a mut W,
}
impl<'a> B1860_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1861` reader - B1861"]
pub struct B1861_R(crate::FieldReader<bool, bool>);
impl B1861_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1861_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1861_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1861` writer - B1861"]
pub struct B1861_W<'a> {
    w: &'a mut W,
}
impl<'a> B1861_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1862` reader - B1862"]
pub struct B1862_R(crate::FieldReader<bool, bool>);
impl B1862_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1862_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1862_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1862` writer - B1862"]
pub struct B1862_W<'a> {
    w: &'a mut W,
}
impl<'a> B1862_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1863` reader - B1863"]
pub struct B1863_R(crate::FieldReader<bool, bool>);
impl B1863_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1863_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1863_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1863` writer - B1863"]
pub struct B1863_W<'a> {
    w: &'a mut W,
}
impl<'a> B1863_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1864` reader - B1864"]
pub struct B1864_R(crate::FieldReader<bool, bool>);
impl B1864_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1864_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1864_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1864` writer - B1864"]
pub struct B1864_W<'a> {
    w: &'a mut W,
}
impl<'a> B1864_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1865` reader - B1865"]
pub struct B1865_R(crate::FieldReader<bool, bool>);
impl B1865_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1865_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1865_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1865` writer - B1865"]
pub struct B1865_W<'a> {
    w: &'a mut W,
}
impl<'a> B1865_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1866` reader - B1866"]
pub struct B1866_R(crate::FieldReader<bool, bool>);
impl B1866_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1866_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1866_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1866` writer - B1866"]
pub struct B1866_W<'a> {
    w: &'a mut W,
}
impl<'a> B1866_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1867` reader - B1867"]
pub struct B1867_R(crate::FieldReader<bool, bool>);
impl B1867_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1867_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1867_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1867` writer - B1867"]
pub struct B1867_W<'a> {
    w: &'a mut W,
}
impl<'a> B1867_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1868` reader - B1868"]
pub struct B1868_R(crate::FieldReader<bool, bool>);
impl B1868_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1868_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1868_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1868` writer - B1868"]
pub struct B1868_W<'a> {
    w: &'a mut W,
}
impl<'a> B1868_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1869` reader - B1869"]
pub struct B1869_R(crate::FieldReader<bool, bool>);
impl B1869_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1869_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1869_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1869` writer - B1869"]
pub struct B1869_W<'a> {
    w: &'a mut W,
}
impl<'a> B1869_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1870` reader - B1870"]
pub struct B1870_R(crate::FieldReader<bool, bool>);
impl B1870_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1870_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1870_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1870` writer - B1870"]
pub struct B1870_W<'a> {
    w: &'a mut W,
}
impl<'a> B1870_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1871` reader - B1871"]
pub struct B1871_R(crate::FieldReader<bool, bool>);
impl B1871_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1871_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1871_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1871` writer - B1871"]
pub struct B1871_W<'a> {
    w: &'a mut W,
}
impl<'a> B1871_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1872` reader - B1872"]
pub struct B1872_R(crate::FieldReader<bool, bool>);
impl B1872_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1872_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1872_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1872` writer - B1872"]
pub struct B1872_W<'a> {
    w: &'a mut W,
}
impl<'a> B1872_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1873` reader - B1873"]
pub struct B1873_R(crate::FieldReader<bool, bool>);
impl B1873_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1873_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1873_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1873` writer - B1873"]
pub struct B1873_W<'a> {
    w: &'a mut W,
}
impl<'a> B1873_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1874` reader - B1874"]
pub struct B1874_R(crate::FieldReader<bool, bool>);
impl B1874_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1874_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1874_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1874` writer - B1874"]
pub struct B1874_W<'a> {
    w: &'a mut W,
}
impl<'a> B1874_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1875` reader - B1875"]
pub struct B1875_R(crate::FieldReader<bool, bool>);
impl B1875_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1875_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1875_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1875` writer - B1875"]
pub struct B1875_W<'a> {
    w: &'a mut W,
}
impl<'a> B1875_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1876` reader - B1876"]
pub struct B1876_R(crate::FieldReader<bool, bool>);
impl B1876_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1876_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1876_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1876` writer - B1876"]
pub struct B1876_W<'a> {
    w: &'a mut W,
}
impl<'a> B1876_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1877` reader - B1877"]
pub struct B1877_R(crate::FieldReader<bool, bool>);
impl B1877_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1877_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1877_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1877` writer - B1877"]
pub struct B1877_W<'a> {
    w: &'a mut W,
}
impl<'a> B1877_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1878` reader - B1878"]
pub struct B1878_R(crate::FieldReader<bool, bool>);
impl B1878_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1878_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1878_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1878` writer - B1878"]
pub struct B1878_W<'a> {
    w: &'a mut W,
}
impl<'a> B1878_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1879` reader - B1879"]
pub struct B1879_R(crate::FieldReader<bool, bool>);
impl B1879_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1879_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1879_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1879` writer - B1879"]
pub struct B1879_W<'a> {
    w: &'a mut W,
}
impl<'a> B1879_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1880` reader - B1880"]
pub struct B1880_R(crate::FieldReader<bool, bool>);
impl B1880_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1880_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1880_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1880` writer - B1880"]
pub struct B1880_W<'a> {
    w: &'a mut W,
}
impl<'a> B1880_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1881` reader - B1881"]
pub struct B1881_R(crate::FieldReader<bool, bool>);
impl B1881_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1881_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1881_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1881` writer - B1881"]
pub struct B1881_W<'a> {
    w: &'a mut W,
}
impl<'a> B1881_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1882` reader - B1882"]
pub struct B1882_R(crate::FieldReader<bool, bool>);
impl B1882_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1882_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1882_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1882` writer - B1882"]
pub struct B1882_W<'a> {
    w: &'a mut W,
}
impl<'a> B1882_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1883` reader - B1883"]
pub struct B1883_R(crate::FieldReader<bool, bool>);
impl B1883_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1883_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1883_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1883` writer - B1883"]
pub struct B1883_W<'a> {
    w: &'a mut W,
}
impl<'a> B1883_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1884` reader - B1884"]
pub struct B1884_R(crate::FieldReader<bool, bool>);
impl B1884_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1884_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1884_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1884` writer - B1884"]
pub struct B1884_W<'a> {
    w: &'a mut W,
}
impl<'a> B1884_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1885` reader - B1885"]
pub struct B1885_R(crate::FieldReader<bool, bool>);
impl B1885_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1885_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1885_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1885` writer - B1885"]
pub struct B1885_W<'a> {
    w: &'a mut W,
}
impl<'a> B1885_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1886` reader - B1886"]
pub struct B1886_R(crate::FieldReader<bool, bool>);
impl B1886_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1886_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1886_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1886` writer - B1886"]
pub struct B1886_W<'a> {
    w: &'a mut W,
}
impl<'a> B1886_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1887` reader - B1887"]
pub struct B1887_R(crate::FieldReader<bool, bool>);
impl B1887_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1887_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1887_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1887` writer - B1887"]
pub struct B1887_W<'a> {
    w: &'a mut W,
}
impl<'a> B1887_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1856"]
    #[inline(always)]
    pub fn b1856(&self) -> B1856_R {
        B1856_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1857"]
    #[inline(always)]
    pub fn b1857(&self) -> B1857_R {
        B1857_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1858"]
    #[inline(always)]
    pub fn b1858(&self) -> B1858_R {
        B1858_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1859"]
    #[inline(always)]
    pub fn b1859(&self) -> B1859_R {
        B1859_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1860"]
    #[inline(always)]
    pub fn b1860(&self) -> B1860_R {
        B1860_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1861"]
    #[inline(always)]
    pub fn b1861(&self) -> B1861_R {
        B1861_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1862"]
    #[inline(always)]
    pub fn b1862(&self) -> B1862_R {
        B1862_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1863"]
    #[inline(always)]
    pub fn b1863(&self) -> B1863_R {
        B1863_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1864"]
    #[inline(always)]
    pub fn b1864(&self) -> B1864_R {
        B1864_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1865"]
    #[inline(always)]
    pub fn b1865(&self) -> B1865_R {
        B1865_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1866"]
    #[inline(always)]
    pub fn b1866(&self) -> B1866_R {
        B1866_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1867"]
    #[inline(always)]
    pub fn b1867(&self) -> B1867_R {
        B1867_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1868"]
    #[inline(always)]
    pub fn b1868(&self) -> B1868_R {
        B1868_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1869"]
    #[inline(always)]
    pub fn b1869(&self) -> B1869_R {
        B1869_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1870"]
    #[inline(always)]
    pub fn b1870(&self) -> B1870_R {
        B1870_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1871"]
    #[inline(always)]
    pub fn b1871(&self) -> B1871_R {
        B1871_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1872"]
    #[inline(always)]
    pub fn b1872(&self) -> B1872_R {
        B1872_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1873"]
    #[inline(always)]
    pub fn b1873(&self) -> B1873_R {
        B1873_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1874"]
    #[inline(always)]
    pub fn b1874(&self) -> B1874_R {
        B1874_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1875"]
    #[inline(always)]
    pub fn b1875(&self) -> B1875_R {
        B1875_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1876"]
    #[inline(always)]
    pub fn b1876(&self) -> B1876_R {
        B1876_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1877"]
    #[inline(always)]
    pub fn b1877(&self) -> B1877_R {
        B1877_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1878"]
    #[inline(always)]
    pub fn b1878(&self) -> B1878_R {
        B1878_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1879"]
    #[inline(always)]
    pub fn b1879(&self) -> B1879_R {
        B1879_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1880"]
    #[inline(always)]
    pub fn b1880(&self) -> B1880_R {
        B1880_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1881"]
    #[inline(always)]
    pub fn b1881(&self) -> B1881_R {
        B1881_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1882"]
    #[inline(always)]
    pub fn b1882(&self) -> B1882_R {
        B1882_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1883"]
    #[inline(always)]
    pub fn b1883(&self) -> B1883_R {
        B1883_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1884"]
    #[inline(always)]
    pub fn b1884(&self) -> B1884_R {
        B1884_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1885"]
    #[inline(always)]
    pub fn b1885(&self) -> B1885_R {
        B1885_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1886"]
    #[inline(always)]
    pub fn b1886(&self) -> B1886_R {
        B1886_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1887"]
    #[inline(always)]
    pub fn b1887(&self) -> B1887_R {
        B1887_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1856"]
    #[inline(always)]
    pub fn b1856(&mut self) -> B1856_W {
        B1856_W { w: self }
    }
    #[doc = "Bit 1 - B1857"]
    #[inline(always)]
    pub fn b1857(&mut self) -> B1857_W {
        B1857_W { w: self }
    }
    #[doc = "Bit 2 - B1858"]
    #[inline(always)]
    pub fn b1858(&mut self) -> B1858_W {
        B1858_W { w: self }
    }
    #[doc = "Bit 3 - B1859"]
    #[inline(always)]
    pub fn b1859(&mut self) -> B1859_W {
        B1859_W { w: self }
    }
    #[doc = "Bit 4 - B1860"]
    #[inline(always)]
    pub fn b1860(&mut self) -> B1860_W {
        B1860_W { w: self }
    }
    #[doc = "Bit 5 - B1861"]
    #[inline(always)]
    pub fn b1861(&mut self) -> B1861_W {
        B1861_W { w: self }
    }
    #[doc = "Bit 6 - B1862"]
    #[inline(always)]
    pub fn b1862(&mut self) -> B1862_W {
        B1862_W { w: self }
    }
    #[doc = "Bit 7 - B1863"]
    #[inline(always)]
    pub fn b1863(&mut self) -> B1863_W {
        B1863_W { w: self }
    }
    #[doc = "Bit 8 - B1864"]
    #[inline(always)]
    pub fn b1864(&mut self) -> B1864_W {
        B1864_W { w: self }
    }
    #[doc = "Bit 9 - B1865"]
    #[inline(always)]
    pub fn b1865(&mut self) -> B1865_W {
        B1865_W { w: self }
    }
    #[doc = "Bit 10 - B1866"]
    #[inline(always)]
    pub fn b1866(&mut self) -> B1866_W {
        B1866_W { w: self }
    }
    #[doc = "Bit 11 - B1867"]
    #[inline(always)]
    pub fn b1867(&mut self) -> B1867_W {
        B1867_W { w: self }
    }
    #[doc = "Bit 12 - B1868"]
    #[inline(always)]
    pub fn b1868(&mut self) -> B1868_W {
        B1868_W { w: self }
    }
    #[doc = "Bit 13 - B1869"]
    #[inline(always)]
    pub fn b1869(&mut self) -> B1869_W {
        B1869_W { w: self }
    }
    #[doc = "Bit 14 - B1870"]
    #[inline(always)]
    pub fn b1870(&mut self) -> B1870_W {
        B1870_W { w: self }
    }
    #[doc = "Bit 15 - B1871"]
    #[inline(always)]
    pub fn b1871(&mut self) -> B1871_W {
        B1871_W { w: self }
    }
    #[doc = "Bit 16 - B1872"]
    #[inline(always)]
    pub fn b1872(&mut self) -> B1872_W {
        B1872_W { w: self }
    }
    #[doc = "Bit 17 - B1873"]
    #[inline(always)]
    pub fn b1873(&mut self) -> B1873_W {
        B1873_W { w: self }
    }
    #[doc = "Bit 18 - B1874"]
    #[inline(always)]
    pub fn b1874(&mut self) -> B1874_W {
        B1874_W { w: self }
    }
    #[doc = "Bit 19 - B1875"]
    #[inline(always)]
    pub fn b1875(&mut self) -> B1875_W {
        B1875_W { w: self }
    }
    #[doc = "Bit 20 - B1876"]
    #[inline(always)]
    pub fn b1876(&mut self) -> B1876_W {
        B1876_W { w: self }
    }
    #[doc = "Bit 21 - B1877"]
    #[inline(always)]
    pub fn b1877(&mut self) -> B1877_W {
        B1877_W { w: self }
    }
    #[doc = "Bit 22 - B1878"]
    #[inline(always)]
    pub fn b1878(&mut self) -> B1878_W {
        B1878_W { w: self }
    }
    #[doc = "Bit 23 - B1879"]
    #[inline(always)]
    pub fn b1879(&mut self) -> B1879_W {
        B1879_W { w: self }
    }
    #[doc = "Bit 24 - B1880"]
    #[inline(always)]
    pub fn b1880(&mut self) -> B1880_W {
        B1880_W { w: self }
    }
    #[doc = "Bit 25 - B1881"]
    #[inline(always)]
    pub fn b1881(&mut self) -> B1881_W {
        B1881_W { w: self }
    }
    #[doc = "Bit 26 - B1882"]
    #[inline(always)]
    pub fn b1882(&mut self) -> B1882_W {
        B1882_W { w: self }
    }
    #[doc = "Bit 27 - B1883"]
    #[inline(always)]
    pub fn b1883(&mut self) -> B1883_W {
        B1883_W { w: self }
    }
    #[doc = "Bit 28 - B1884"]
    #[inline(always)]
    pub fn b1884(&mut self) -> B1884_W {
        B1884_W { w: self }
    }
    #[doc = "Bit 29 - B1885"]
    #[inline(always)]
    pub fn b1885(&mut self) -> B1885_W {
        B1885_W { w: self }
    }
    #[doc = "Bit 30 - B1886"]
    #[inline(always)]
    pub fn b1886(&mut self) -> B1886_W {
        B1886_W { w: self }
    }
    #[doc = "Bit 31 - B1887"]
    #[inline(always)]
    pub fn b1887(&mut self) -> B1887_W {
        B1887_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr58](index.html) module"]
pub struct MPCBB1_VCTR58_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR58_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr58::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR58_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr58::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR58_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR58 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR58_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
