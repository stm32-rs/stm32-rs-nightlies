#[doc = "Register `MPCBB2_VCTR9` reader"]
pub struct R(crate::R<MPCBB2_VCTR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR9` writer"]
pub struct W(crate::W<MPCBB2_VCTR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR9_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B288` reader - B288"]
pub struct B288_R(crate::FieldReader<bool, bool>);
impl B288_R {
    pub(crate) fn new(bits: bool) -> Self {
        B288_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B288_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B288` writer - B288"]
pub struct B288_W<'a> {
    w: &'a mut W,
}
impl<'a> B288_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B289` reader - B289"]
pub struct B289_R(crate::FieldReader<bool, bool>);
impl B289_R {
    pub(crate) fn new(bits: bool) -> Self {
        B289_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B289_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B289` writer - B289"]
pub struct B289_W<'a> {
    w: &'a mut W,
}
impl<'a> B289_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B290` reader - B290"]
pub struct B290_R(crate::FieldReader<bool, bool>);
impl B290_R {
    pub(crate) fn new(bits: bool) -> Self {
        B290_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B290_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B290` writer - B290"]
pub struct B290_W<'a> {
    w: &'a mut W,
}
impl<'a> B290_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B291` reader - B291"]
pub struct B291_R(crate::FieldReader<bool, bool>);
impl B291_R {
    pub(crate) fn new(bits: bool) -> Self {
        B291_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B291_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B291` writer - B291"]
pub struct B291_W<'a> {
    w: &'a mut W,
}
impl<'a> B291_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B292` reader - B292"]
pub struct B292_R(crate::FieldReader<bool, bool>);
impl B292_R {
    pub(crate) fn new(bits: bool) -> Self {
        B292_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B292_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B292` writer - B292"]
pub struct B292_W<'a> {
    w: &'a mut W,
}
impl<'a> B292_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B293` reader - B293"]
pub struct B293_R(crate::FieldReader<bool, bool>);
impl B293_R {
    pub(crate) fn new(bits: bool) -> Self {
        B293_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B293_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B293` writer - B293"]
pub struct B293_W<'a> {
    w: &'a mut W,
}
impl<'a> B293_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B294` reader - B294"]
pub struct B294_R(crate::FieldReader<bool, bool>);
impl B294_R {
    pub(crate) fn new(bits: bool) -> Self {
        B294_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B294_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B294` writer - B294"]
pub struct B294_W<'a> {
    w: &'a mut W,
}
impl<'a> B294_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B295` reader - B295"]
pub struct B295_R(crate::FieldReader<bool, bool>);
impl B295_R {
    pub(crate) fn new(bits: bool) -> Self {
        B295_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B295_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B295` writer - B295"]
pub struct B295_W<'a> {
    w: &'a mut W,
}
impl<'a> B295_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B296` reader - B296"]
pub struct B296_R(crate::FieldReader<bool, bool>);
impl B296_R {
    pub(crate) fn new(bits: bool) -> Self {
        B296_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B296_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B296` writer - B296"]
pub struct B296_W<'a> {
    w: &'a mut W,
}
impl<'a> B296_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B297` reader - B297"]
pub struct B297_R(crate::FieldReader<bool, bool>);
impl B297_R {
    pub(crate) fn new(bits: bool) -> Self {
        B297_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B297_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B297` writer - B297"]
pub struct B297_W<'a> {
    w: &'a mut W,
}
impl<'a> B297_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B298` reader - B298"]
pub struct B298_R(crate::FieldReader<bool, bool>);
impl B298_R {
    pub(crate) fn new(bits: bool) -> Self {
        B298_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B298_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B298` writer - B298"]
pub struct B298_W<'a> {
    w: &'a mut W,
}
impl<'a> B298_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B299` reader - B299"]
pub struct B299_R(crate::FieldReader<bool, bool>);
impl B299_R {
    pub(crate) fn new(bits: bool) -> Self {
        B299_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B299_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B299` writer - B299"]
pub struct B299_W<'a> {
    w: &'a mut W,
}
impl<'a> B299_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B300` reader - B300"]
pub struct B300_R(crate::FieldReader<bool, bool>);
impl B300_R {
    pub(crate) fn new(bits: bool) -> Self {
        B300_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B300_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B300` writer - B300"]
pub struct B300_W<'a> {
    w: &'a mut W,
}
impl<'a> B300_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B301` reader - B301"]
pub struct B301_R(crate::FieldReader<bool, bool>);
impl B301_R {
    pub(crate) fn new(bits: bool) -> Self {
        B301_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B301_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B301` writer - B301"]
pub struct B301_W<'a> {
    w: &'a mut W,
}
impl<'a> B301_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B302` reader - B302"]
pub struct B302_R(crate::FieldReader<bool, bool>);
impl B302_R {
    pub(crate) fn new(bits: bool) -> Self {
        B302_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B302_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B302` writer - B302"]
pub struct B302_W<'a> {
    w: &'a mut W,
}
impl<'a> B302_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B303` reader - B303"]
pub struct B303_R(crate::FieldReader<bool, bool>);
impl B303_R {
    pub(crate) fn new(bits: bool) -> Self {
        B303_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B303_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B303` writer - B303"]
pub struct B303_W<'a> {
    w: &'a mut W,
}
impl<'a> B303_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B304` reader - B304"]
pub struct B304_R(crate::FieldReader<bool, bool>);
impl B304_R {
    pub(crate) fn new(bits: bool) -> Self {
        B304_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B304_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B304` writer - B304"]
pub struct B304_W<'a> {
    w: &'a mut W,
}
impl<'a> B304_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B305` reader - B305"]
pub struct B305_R(crate::FieldReader<bool, bool>);
impl B305_R {
    pub(crate) fn new(bits: bool) -> Self {
        B305_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B305_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B305` writer - B305"]
pub struct B305_W<'a> {
    w: &'a mut W,
}
impl<'a> B305_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B306` reader - B306"]
pub struct B306_R(crate::FieldReader<bool, bool>);
impl B306_R {
    pub(crate) fn new(bits: bool) -> Self {
        B306_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B306_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B306` writer - B306"]
pub struct B306_W<'a> {
    w: &'a mut W,
}
impl<'a> B306_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B307` reader - B307"]
pub struct B307_R(crate::FieldReader<bool, bool>);
impl B307_R {
    pub(crate) fn new(bits: bool) -> Self {
        B307_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B307_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B307` writer - B307"]
pub struct B307_W<'a> {
    w: &'a mut W,
}
impl<'a> B307_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B308` reader - B308"]
pub struct B308_R(crate::FieldReader<bool, bool>);
impl B308_R {
    pub(crate) fn new(bits: bool) -> Self {
        B308_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B308_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B308` writer - B308"]
pub struct B308_W<'a> {
    w: &'a mut W,
}
impl<'a> B308_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B309` reader - B309"]
pub struct B309_R(crate::FieldReader<bool, bool>);
impl B309_R {
    pub(crate) fn new(bits: bool) -> Self {
        B309_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B309_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B309` writer - B309"]
pub struct B309_W<'a> {
    w: &'a mut W,
}
impl<'a> B309_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B310` reader - B310"]
pub struct B310_R(crate::FieldReader<bool, bool>);
impl B310_R {
    pub(crate) fn new(bits: bool) -> Self {
        B310_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B310_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B310` writer - B310"]
pub struct B310_W<'a> {
    w: &'a mut W,
}
impl<'a> B310_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B311` reader - B311"]
pub struct B311_R(crate::FieldReader<bool, bool>);
impl B311_R {
    pub(crate) fn new(bits: bool) -> Self {
        B311_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B311_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B311` writer - B311"]
pub struct B311_W<'a> {
    w: &'a mut W,
}
impl<'a> B311_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B312` reader - B312"]
pub struct B312_R(crate::FieldReader<bool, bool>);
impl B312_R {
    pub(crate) fn new(bits: bool) -> Self {
        B312_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B312_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B312` writer - B312"]
pub struct B312_W<'a> {
    w: &'a mut W,
}
impl<'a> B312_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B313` reader - B313"]
pub struct B313_R(crate::FieldReader<bool, bool>);
impl B313_R {
    pub(crate) fn new(bits: bool) -> Self {
        B313_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B313_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B313` writer - B313"]
pub struct B313_W<'a> {
    w: &'a mut W,
}
impl<'a> B313_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B314` reader - B314"]
pub struct B314_R(crate::FieldReader<bool, bool>);
impl B314_R {
    pub(crate) fn new(bits: bool) -> Self {
        B314_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B314_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B314` writer - B314"]
pub struct B314_W<'a> {
    w: &'a mut W,
}
impl<'a> B314_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B315` reader - B315"]
pub struct B315_R(crate::FieldReader<bool, bool>);
impl B315_R {
    pub(crate) fn new(bits: bool) -> Self {
        B315_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B315_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B315` writer - B315"]
pub struct B315_W<'a> {
    w: &'a mut W,
}
impl<'a> B315_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B316` reader - B316"]
pub struct B316_R(crate::FieldReader<bool, bool>);
impl B316_R {
    pub(crate) fn new(bits: bool) -> Self {
        B316_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B316_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B316` writer - B316"]
pub struct B316_W<'a> {
    w: &'a mut W,
}
impl<'a> B316_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B317` reader - B317"]
pub struct B317_R(crate::FieldReader<bool, bool>);
impl B317_R {
    pub(crate) fn new(bits: bool) -> Self {
        B317_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B317_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B317` writer - B317"]
pub struct B317_W<'a> {
    w: &'a mut W,
}
impl<'a> B317_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B318` reader - B318"]
pub struct B318_R(crate::FieldReader<bool, bool>);
impl B318_R {
    pub(crate) fn new(bits: bool) -> Self {
        B318_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B318_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B318` writer - B318"]
pub struct B318_W<'a> {
    w: &'a mut W,
}
impl<'a> B318_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B319` reader - B319"]
pub struct B319_R(crate::FieldReader<bool, bool>);
impl B319_R {
    pub(crate) fn new(bits: bool) -> Self {
        B319_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B319_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B319` writer - B319"]
pub struct B319_W<'a> {
    w: &'a mut W,
}
impl<'a> B319_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B288"]
    #[inline(always)]
    pub fn b288(&self) -> B288_R {
        B288_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B289"]
    #[inline(always)]
    pub fn b289(&self) -> B289_R {
        B289_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B290"]
    #[inline(always)]
    pub fn b290(&self) -> B290_R {
        B290_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B291"]
    #[inline(always)]
    pub fn b291(&self) -> B291_R {
        B291_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B292"]
    #[inline(always)]
    pub fn b292(&self) -> B292_R {
        B292_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B293"]
    #[inline(always)]
    pub fn b293(&self) -> B293_R {
        B293_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B294"]
    #[inline(always)]
    pub fn b294(&self) -> B294_R {
        B294_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B295"]
    #[inline(always)]
    pub fn b295(&self) -> B295_R {
        B295_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B296"]
    #[inline(always)]
    pub fn b296(&self) -> B296_R {
        B296_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B297"]
    #[inline(always)]
    pub fn b297(&self) -> B297_R {
        B297_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B298"]
    #[inline(always)]
    pub fn b298(&self) -> B298_R {
        B298_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B299"]
    #[inline(always)]
    pub fn b299(&self) -> B299_R {
        B299_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B300"]
    #[inline(always)]
    pub fn b300(&self) -> B300_R {
        B300_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B301"]
    #[inline(always)]
    pub fn b301(&self) -> B301_R {
        B301_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B302"]
    #[inline(always)]
    pub fn b302(&self) -> B302_R {
        B302_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B303"]
    #[inline(always)]
    pub fn b303(&self) -> B303_R {
        B303_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B304"]
    #[inline(always)]
    pub fn b304(&self) -> B304_R {
        B304_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B305"]
    #[inline(always)]
    pub fn b305(&self) -> B305_R {
        B305_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B306"]
    #[inline(always)]
    pub fn b306(&self) -> B306_R {
        B306_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B307"]
    #[inline(always)]
    pub fn b307(&self) -> B307_R {
        B307_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B308"]
    #[inline(always)]
    pub fn b308(&self) -> B308_R {
        B308_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B309"]
    #[inline(always)]
    pub fn b309(&self) -> B309_R {
        B309_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B310"]
    #[inline(always)]
    pub fn b310(&self) -> B310_R {
        B310_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B311"]
    #[inline(always)]
    pub fn b311(&self) -> B311_R {
        B311_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B312"]
    #[inline(always)]
    pub fn b312(&self) -> B312_R {
        B312_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B313"]
    #[inline(always)]
    pub fn b313(&self) -> B313_R {
        B313_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B314"]
    #[inline(always)]
    pub fn b314(&self) -> B314_R {
        B314_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B315"]
    #[inline(always)]
    pub fn b315(&self) -> B315_R {
        B315_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B316"]
    #[inline(always)]
    pub fn b316(&self) -> B316_R {
        B316_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B317"]
    #[inline(always)]
    pub fn b317(&self) -> B317_R {
        B317_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B318"]
    #[inline(always)]
    pub fn b318(&self) -> B318_R {
        B318_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B319"]
    #[inline(always)]
    pub fn b319(&self) -> B319_R {
        B319_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B288"]
    #[inline(always)]
    pub fn b288(&mut self) -> B288_W {
        B288_W { w: self }
    }
    #[doc = "Bit 1 - B289"]
    #[inline(always)]
    pub fn b289(&mut self) -> B289_W {
        B289_W { w: self }
    }
    #[doc = "Bit 2 - B290"]
    #[inline(always)]
    pub fn b290(&mut self) -> B290_W {
        B290_W { w: self }
    }
    #[doc = "Bit 3 - B291"]
    #[inline(always)]
    pub fn b291(&mut self) -> B291_W {
        B291_W { w: self }
    }
    #[doc = "Bit 4 - B292"]
    #[inline(always)]
    pub fn b292(&mut self) -> B292_W {
        B292_W { w: self }
    }
    #[doc = "Bit 5 - B293"]
    #[inline(always)]
    pub fn b293(&mut self) -> B293_W {
        B293_W { w: self }
    }
    #[doc = "Bit 6 - B294"]
    #[inline(always)]
    pub fn b294(&mut self) -> B294_W {
        B294_W { w: self }
    }
    #[doc = "Bit 7 - B295"]
    #[inline(always)]
    pub fn b295(&mut self) -> B295_W {
        B295_W { w: self }
    }
    #[doc = "Bit 8 - B296"]
    #[inline(always)]
    pub fn b296(&mut self) -> B296_W {
        B296_W { w: self }
    }
    #[doc = "Bit 9 - B297"]
    #[inline(always)]
    pub fn b297(&mut self) -> B297_W {
        B297_W { w: self }
    }
    #[doc = "Bit 10 - B298"]
    #[inline(always)]
    pub fn b298(&mut self) -> B298_W {
        B298_W { w: self }
    }
    #[doc = "Bit 11 - B299"]
    #[inline(always)]
    pub fn b299(&mut self) -> B299_W {
        B299_W { w: self }
    }
    #[doc = "Bit 12 - B300"]
    #[inline(always)]
    pub fn b300(&mut self) -> B300_W {
        B300_W { w: self }
    }
    #[doc = "Bit 13 - B301"]
    #[inline(always)]
    pub fn b301(&mut self) -> B301_W {
        B301_W { w: self }
    }
    #[doc = "Bit 14 - B302"]
    #[inline(always)]
    pub fn b302(&mut self) -> B302_W {
        B302_W { w: self }
    }
    #[doc = "Bit 15 - B303"]
    #[inline(always)]
    pub fn b303(&mut self) -> B303_W {
        B303_W { w: self }
    }
    #[doc = "Bit 16 - B304"]
    #[inline(always)]
    pub fn b304(&mut self) -> B304_W {
        B304_W { w: self }
    }
    #[doc = "Bit 17 - B305"]
    #[inline(always)]
    pub fn b305(&mut self) -> B305_W {
        B305_W { w: self }
    }
    #[doc = "Bit 18 - B306"]
    #[inline(always)]
    pub fn b306(&mut self) -> B306_W {
        B306_W { w: self }
    }
    #[doc = "Bit 19 - B307"]
    #[inline(always)]
    pub fn b307(&mut self) -> B307_W {
        B307_W { w: self }
    }
    #[doc = "Bit 20 - B308"]
    #[inline(always)]
    pub fn b308(&mut self) -> B308_W {
        B308_W { w: self }
    }
    #[doc = "Bit 21 - B309"]
    #[inline(always)]
    pub fn b309(&mut self) -> B309_W {
        B309_W { w: self }
    }
    #[doc = "Bit 22 - B310"]
    #[inline(always)]
    pub fn b310(&mut self) -> B310_W {
        B310_W { w: self }
    }
    #[doc = "Bit 23 - B311"]
    #[inline(always)]
    pub fn b311(&mut self) -> B311_W {
        B311_W { w: self }
    }
    #[doc = "Bit 24 - B312"]
    #[inline(always)]
    pub fn b312(&mut self) -> B312_W {
        B312_W { w: self }
    }
    #[doc = "Bit 25 - B313"]
    #[inline(always)]
    pub fn b313(&mut self) -> B313_W {
        B313_W { w: self }
    }
    #[doc = "Bit 26 - B314"]
    #[inline(always)]
    pub fn b314(&mut self) -> B314_W {
        B314_W { w: self }
    }
    #[doc = "Bit 27 - B315"]
    #[inline(always)]
    pub fn b315(&mut self) -> B315_W {
        B315_W { w: self }
    }
    #[doc = "Bit 28 - B316"]
    #[inline(always)]
    pub fn b316(&mut self) -> B316_W {
        B316_W { w: self }
    }
    #[doc = "Bit 29 - B317"]
    #[inline(always)]
    pub fn b317(&mut self) -> B317_W {
        B317_W { w: self }
    }
    #[doc = "Bit 30 - B318"]
    #[inline(always)]
    pub fn b318(&mut self) -> B318_W {
        B318_W { w: self }
    }
    #[doc = "Bit 31 - B319"]
    #[inline(always)]
    pub fn b319(&mut self) -> B319_W {
        B319_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr9](index.html) module"]
pub struct MPCBB2_VCTR9_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr9::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr9::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR9 to value 0"]
impl crate::Resettable for MPCBB2_VCTR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
