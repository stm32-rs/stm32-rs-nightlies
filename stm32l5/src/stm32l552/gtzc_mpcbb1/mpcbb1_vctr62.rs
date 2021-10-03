#[doc = "Register `MPCBB1_VCTR62` reader"]
pub struct R(crate::R<MPCBB1_VCTR62_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR62_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR62_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR62_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR62` writer"]
pub struct W(crate::W<MPCBB1_VCTR62_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR62_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR62_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR62_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1984` reader - B1984"]
pub struct B1984_R(crate::FieldReader<bool, bool>);
impl B1984_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1984_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1984_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1984` writer - B1984"]
pub struct B1984_W<'a> {
    w: &'a mut W,
}
impl<'a> B1984_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1985` reader - B1985"]
pub struct B1985_R(crate::FieldReader<bool, bool>);
impl B1985_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1985_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1985_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1985` writer - B1985"]
pub struct B1985_W<'a> {
    w: &'a mut W,
}
impl<'a> B1985_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1986` reader - B1986"]
pub struct B1986_R(crate::FieldReader<bool, bool>);
impl B1986_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1986_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1986_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1986` writer - B1986"]
pub struct B1986_W<'a> {
    w: &'a mut W,
}
impl<'a> B1986_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1987` reader - B1987"]
pub struct B1987_R(crate::FieldReader<bool, bool>);
impl B1987_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1987_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1987_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1987` writer - B1987"]
pub struct B1987_W<'a> {
    w: &'a mut W,
}
impl<'a> B1987_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1988` reader - B1988"]
pub struct B1988_R(crate::FieldReader<bool, bool>);
impl B1988_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1988_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1988_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1988` writer - B1988"]
pub struct B1988_W<'a> {
    w: &'a mut W,
}
impl<'a> B1988_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1989` reader - B1989"]
pub struct B1989_R(crate::FieldReader<bool, bool>);
impl B1989_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1989_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1989_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1989` writer - B1989"]
pub struct B1989_W<'a> {
    w: &'a mut W,
}
impl<'a> B1989_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1990` reader - B1990"]
pub struct B1990_R(crate::FieldReader<bool, bool>);
impl B1990_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1990_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1990_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1990` writer - B1990"]
pub struct B1990_W<'a> {
    w: &'a mut W,
}
impl<'a> B1990_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1991` reader - B1991"]
pub struct B1991_R(crate::FieldReader<bool, bool>);
impl B1991_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1991_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1991_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1991` writer - B1991"]
pub struct B1991_W<'a> {
    w: &'a mut W,
}
impl<'a> B1991_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1992` reader - B1992"]
pub struct B1992_R(crate::FieldReader<bool, bool>);
impl B1992_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1992_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1992_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1992` writer - B1992"]
pub struct B1992_W<'a> {
    w: &'a mut W,
}
impl<'a> B1992_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1993` reader - B1993"]
pub struct B1993_R(crate::FieldReader<bool, bool>);
impl B1993_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1993_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1993_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1993` writer - B1993"]
pub struct B1993_W<'a> {
    w: &'a mut W,
}
impl<'a> B1993_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1994` reader - B1994"]
pub struct B1994_R(crate::FieldReader<bool, bool>);
impl B1994_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1994_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1994_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1994` writer - B1994"]
pub struct B1994_W<'a> {
    w: &'a mut W,
}
impl<'a> B1994_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1995` reader - B1995"]
pub struct B1995_R(crate::FieldReader<bool, bool>);
impl B1995_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1995_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1995_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1995` writer - B1995"]
pub struct B1995_W<'a> {
    w: &'a mut W,
}
impl<'a> B1995_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1996` reader - B1996"]
pub struct B1996_R(crate::FieldReader<bool, bool>);
impl B1996_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1996_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1996_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1996` writer - B1996"]
pub struct B1996_W<'a> {
    w: &'a mut W,
}
impl<'a> B1996_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1997` reader - B1997"]
pub struct B1997_R(crate::FieldReader<bool, bool>);
impl B1997_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1997_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1997_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1997` writer - B1997"]
pub struct B1997_W<'a> {
    w: &'a mut W,
}
impl<'a> B1997_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1998` reader - B1998"]
pub struct B1998_R(crate::FieldReader<bool, bool>);
impl B1998_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1998_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1998_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1998` writer - B1998"]
pub struct B1998_W<'a> {
    w: &'a mut W,
}
impl<'a> B1998_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1999` reader - B1999"]
pub struct B1999_R(crate::FieldReader<bool, bool>);
impl B1999_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1999_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1999_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1999` writer - B1999"]
pub struct B1999_W<'a> {
    w: &'a mut W,
}
impl<'a> B1999_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2000` reader - B2000"]
pub struct B2000_R(crate::FieldReader<bool, bool>);
impl B2000_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2000_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2000_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2000` writer - B2000"]
pub struct B2000_W<'a> {
    w: &'a mut W,
}
impl<'a> B2000_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2001` reader - B2001"]
pub struct B2001_R(crate::FieldReader<bool, bool>);
impl B2001_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2001_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2001_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2001` writer - B2001"]
pub struct B2001_W<'a> {
    w: &'a mut W,
}
impl<'a> B2001_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2002` reader - B2002"]
pub struct B2002_R(crate::FieldReader<bool, bool>);
impl B2002_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2002_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2002_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2002` writer - B2002"]
pub struct B2002_W<'a> {
    w: &'a mut W,
}
impl<'a> B2002_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2003` reader - B2003"]
pub struct B2003_R(crate::FieldReader<bool, bool>);
impl B2003_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2003_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2003_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2003` writer - B2003"]
pub struct B2003_W<'a> {
    w: &'a mut W,
}
impl<'a> B2003_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2004` reader - B2004"]
pub struct B2004_R(crate::FieldReader<bool, bool>);
impl B2004_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2004_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2004_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2004` writer - B2004"]
pub struct B2004_W<'a> {
    w: &'a mut W,
}
impl<'a> B2004_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2005` reader - B2005"]
pub struct B2005_R(crate::FieldReader<bool, bool>);
impl B2005_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2005_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2005_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2005` writer - B2005"]
pub struct B2005_W<'a> {
    w: &'a mut W,
}
impl<'a> B2005_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2006` reader - B2006"]
pub struct B2006_R(crate::FieldReader<bool, bool>);
impl B2006_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2006_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2006_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2006` writer - B2006"]
pub struct B2006_W<'a> {
    w: &'a mut W,
}
impl<'a> B2006_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2007` reader - B2007"]
pub struct B2007_R(crate::FieldReader<bool, bool>);
impl B2007_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2007_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2007_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2007` writer - B2007"]
pub struct B2007_W<'a> {
    w: &'a mut W,
}
impl<'a> B2007_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2008` reader - B2008"]
pub struct B2008_R(crate::FieldReader<bool, bool>);
impl B2008_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2008_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2008_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2008` writer - B2008"]
pub struct B2008_W<'a> {
    w: &'a mut W,
}
impl<'a> B2008_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2009` reader - B2009"]
pub struct B2009_R(crate::FieldReader<bool, bool>);
impl B2009_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2009_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2009_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2009` writer - B2009"]
pub struct B2009_W<'a> {
    w: &'a mut W,
}
impl<'a> B2009_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2010` reader - B2010"]
pub struct B2010_R(crate::FieldReader<bool, bool>);
impl B2010_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2010_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2010_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2010` writer - B2010"]
pub struct B2010_W<'a> {
    w: &'a mut W,
}
impl<'a> B2010_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2011` reader - B2011"]
pub struct B2011_R(crate::FieldReader<bool, bool>);
impl B2011_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2011_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2011_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2011` writer - B2011"]
pub struct B2011_W<'a> {
    w: &'a mut W,
}
impl<'a> B2011_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2012` reader - B2012"]
pub struct B2012_R(crate::FieldReader<bool, bool>);
impl B2012_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2012_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2012_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2012` writer - B2012"]
pub struct B2012_W<'a> {
    w: &'a mut W,
}
impl<'a> B2012_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2013` reader - B2013"]
pub struct B2013_R(crate::FieldReader<bool, bool>);
impl B2013_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2013_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2013_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2013` writer - B2013"]
pub struct B2013_W<'a> {
    w: &'a mut W,
}
impl<'a> B2013_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2014` reader - B2014"]
pub struct B2014_R(crate::FieldReader<bool, bool>);
impl B2014_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2014_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2014_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2014` writer - B2014"]
pub struct B2014_W<'a> {
    w: &'a mut W,
}
impl<'a> B2014_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2015` reader - B2015"]
pub struct B2015_R(crate::FieldReader<bool, bool>);
impl B2015_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2015_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2015_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2015` writer - B2015"]
pub struct B2015_W<'a> {
    w: &'a mut W,
}
impl<'a> B2015_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1984"]
    #[inline(always)]
    pub fn b1984(&self) -> B1984_R {
        B1984_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1985"]
    #[inline(always)]
    pub fn b1985(&self) -> B1985_R {
        B1985_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1986"]
    #[inline(always)]
    pub fn b1986(&self) -> B1986_R {
        B1986_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1987"]
    #[inline(always)]
    pub fn b1987(&self) -> B1987_R {
        B1987_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1988"]
    #[inline(always)]
    pub fn b1988(&self) -> B1988_R {
        B1988_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1989"]
    #[inline(always)]
    pub fn b1989(&self) -> B1989_R {
        B1989_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1990"]
    #[inline(always)]
    pub fn b1990(&self) -> B1990_R {
        B1990_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1991"]
    #[inline(always)]
    pub fn b1991(&self) -> B1991_R {
        B1991_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1992"]
    #[inline(always)]
    pub fn b1992(&self) -> B1992_R {
        B1992_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1993"]
    #[inline(always)]
    pub fn b1993(&self) -> B1993_R {
        B1993_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1994"]
    #[inline(always)]
    pub fn b1994(&self) -> B1994_R {
        B1994_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1995"]
    #[inline(always)]
    pub fn b1995(&self) -> B1995_R {
        B1995_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1996"]
    #[inline(always)]
    pub fn b1996(&self) -> B1996_R {
        B1996_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1997"]
    #[inline(always)]
    pub fn b1997(&self) -> B1997_R {
        B1997_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1998"]
    #[inline(always)]
    pub fn b1998(&self) -> B1998_R {
        B1998_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1999"]
    #[inline(always)]
    pub fn b1999(&self) -> B1999_R {
        B1999_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B2000"]
    #[inline(always)]
    pub fn b2000(&self) -> B2000_R {
        B2000_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B2001"]
    #[inline(always)]
    pub fn b2001(&self) -> B2001_R {
        B2001_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B2002"]
    #[inline(always)]
    pub fn b2002(&self) -> B2002_R {
        B2002_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B2003"]
    #[inline(always)]
    pub fn b2003(&self) -> B2003_R {
        B2003_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B2004"]
    #[inline(always)]
    pub fn b2004(&self) -> B2004_R {
        B2004_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B2005"]
    #[inline(always)]
    pub fn b2005(&self) -> B2005_R {
        B2005_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B2006"]
    #[inline(always)]
    pub fn b2006(&self) -> B2006_R {
        B2006_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B2007"]
    #[inline(always)]
    pub fn b2007(&self) -> B2007_R {
        B2007_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B2008"]
    #[inline(always)]
    pub fn b2008(&self) -> B2008_R {
        B2008_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B2009"]
    #[inline(always)]
    pub fn b2009(&self) -> B2009_R {
        B2009_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B2010"]
    #[inline(always)]
    pub fn b2010(&self) -> B2010_R {
        B2010_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B2011"]
    #[inline(always)]
    pub fn b2011(&self) -> B2011_R {
        B2011_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B2012"]
    #[inline(always)]
    pub fn b2012(&self) -> B2012_R {
        B2012_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B2013"]
    #[inline(always)]
    pub fn b2013(&self) -> B2013_R {
        B2013_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B2014"]
    #[inline(always)]
    pub fn b2014(&self) -> B2014_R {
        B2014_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B2015"]
    #[inline(always)]
    pub fn b2015(&self) -> B2015_R {
        B2015_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1984"]
    #[inline(always)]
    pub fn b1984(&mut self) -> B1984_W {
        B1984_W { w: self }
    }
    #[doc = "Bit 1 - B1985"]
    #[inline(always)]
    pub fn b1985(&mut self) -> B1985_W {
        B1985_W { w: self }
    }
    #[doc = "Bit 2 - B1986"]
    #[inline(always)]
    pub fn b1986(&mut self) -> B1986_W {
        B1986_W { w: self }
    }
    #[doc = "Bit 3 - B1987"]
    #[inline(always)]
    pub fn b1987(&mut self) -> B1987_W {
        B1987_W { w: self }
    }
    #[doc = "Bit 4 - B1988"]
    #[inline(always)]
    pub fn b1988(&mut self) -> B1988_W {
        B1988_W { w: self }
    }
    #[doc = "Bit 5 - B1989"]
    #[inline(always)]
    pub fn b1989(&mut self) -> B1989_W {
        B1989_W { w: self }
    }
    #[doc = "Bit 6 - B1990"]
    #[inline(always)]
    pub fn b1990(&mut self) -> B1990_W {
        B1990_W { w: self }
    }
    #[doc = "Bit 7 - B1991"]
    #[inline(always)]
    pub fn b1991(&mut self) -> B1991_W {
        B1991_W { w: self }
    }
    #[doc = "Bit 8 - B1992"]
    #[inline(always)]
    pub fn b1992(&mut self) -> B1992_W {
        B1992_W { w: self }
    }
    #[doc = "Bit 9 - B1993"]
    #[inline(always)]
    pub fn b1993(&mut self) -> B1993_W {
        B1993_W { w: self }
    }
    #[doc = "Bit 10 - B1994"]
    #[inline(always)]
    pub fn b1994(&mut self) -> B1994_W {
        B1994_W { w: self }
    }
    #[doc = "Bit 11 - B1995"]
    #[inline(always)]
    pub fn b1995(&mut self) -> B1995_W {
        B1995_W { w: self }
    }
    #[doc = "Bit 12 - B1996"]
    #[inline(always)]
    pub fn b1996(&mut self) -> B1996_W {
        B1996_W { w: self }
    }
    #[doc = "Bit 13 - B1997"]
    #[inline(always)]
    pub fn b1997(&mut self) -> B1997_W {
        B1997_W { w: self }
    }
    #[doc = "Bit 14 - B1998"]
    #[inline(always)]
    pub fn b1998(&mut self) -> B1998_W {
        B1998_W { w: self }
    }
    #[doc = "Bit 15 - B1999"]
    #[inline(always)]
    pub fn b1999(&mut self) -> B1999_W {
        B1999_W { w: self }
    }
    #[doc = "Bit 16 - B2000"]
    #[inline(always)]
    pub fn b2000(&mut self) -> B2000_W {
        B2000_W { w: self }
    }
    #[doc = "Bit 17 - B2001"]
    #[inline(always)]
    pub fn b2001(&mut self) -> B2001_W {
        B2001_W { w: self }
    }
    #[doc = "Bit 18 - B2002"]
    #[inline(always)]
    pub fn b2002(&mut self) -> B2002_W {
        B2002_W { w: self }
    }
    #[doc = "Bit 19 - B2003"]
    #[inline(always)]
    pub fn b2003(&mut self) -> B2003_W {
        B2003_W { w: self }
    }
    #[doc = "Bit 20 - B2004"]
    #[inline(always)]
    pub fn b2004(&mut self) -> B2004_W {
        B2004_W { w: self }
    }
    #[doc = "Bit 21 - B2005"]
    #[inline(always)]
    pub fn b2005(&mut self) -> B2005_W {
        B2005_W { w: self }
    }
    #[doc = "Bit 22 - B2006"]
    #[inline(always)]
    pub fn b2006(&mut self) -> B2006_W {
        B2006_W { w: self }
    }
    #[doc = "Bit 23 - B2007"]
    #[inline(always)]
    pub fn b2007(&mut self) -> B2007_W {
        B2007_W { w: self }
    }
    #[doc = "Bit 24 - B2008"]
    #[inline(always)]
    pub fn b2008(&mut self) -> B2008_W {
        B2008_W { w: self }
    }
    #[doc = "Bit 25 - B2009"]
    #[inline(always)]
    pub fn b2009(&mut self) -> B2009_W {
        B2009_W { w: self }
    }
    #[doc = "Bit 26 - B2010"]
    #[inline(always)]
    pub fn b2010(&mut self) -> B2010_W {
        B2010_W { w: self }
    }
    #[doc = "Bit 27 - B2011"]
    #[inline(always)]
    pub fn b2011(&mut self) -> B2011_W {
        B2011_W { w: self }
    }
    #[doc = "Bit 28 - B2012"]
    #[inline(always)]
    pub fn b2012(&mut self) -> B2012_W {
        B2012_W { w: self }
    }
    #[doc = "Bit 29 - B2013"]
    #[inline(always)]
    pub fn b2013(&mut self) -> B2013_W {
        B2013_W { w: self }
    }
    #[doc = "Bit 30 - B2014"]
    #[inline(always)]
    pub fn b2014(&mut self) -> B2014_W {
        B2014_W { w: self }
    }
    #[doc = "Bit 31 - B2015"]
    #[inline(always)]
    pub fn b2015(&mut self) -> B2015_W {
        B2015_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr62](index.html) module"]
pub struct MPCBB1_VCTR62_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR62_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr62::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR62_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr62::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR62_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR62 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR62_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
