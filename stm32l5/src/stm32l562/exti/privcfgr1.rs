#[doc = "Register `PRIVCFGR1` reader"]
pub struct R(crate::R<PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVCFGR1` writer"]
pub struct W(crate::W<PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR1_SPEC>;
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
impl From<crate::W<PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIV0` reader - Security enable on event input x"]
pub struct PRIV0_R(crate::FieldReader<bool, bool>);
impl PRIV0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV0` writer - Security enable on event input x"]
pub struct PRIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV1` reader - Security enable on event input x"]
pub struct PRIV1_R(crate::FieldReader<bool, bool>);
impl PRIV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV1` writer - Security enable on event input x"]
pub struct PRIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV2` reader - Security enable on event input x"]
pub struct PRIV2_R(crate::FieldReader<bool, bool>);
impl PRIV2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV2` writer - Security enable on event input x"]
pub struct PRIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV3` reader - Security enable on event input x"]
pub struct PRIV3_R(crate::FieldReader<bool, bool>);
impl PRIV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV3` writer - Security enable on event input x"]
pub struct PRIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV4` reader - Security enable on event input x"]
pub struct PRIV4_R(crate::FieldReader<bool, bool>);
impl PRIV4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV4` writer - Security enable on event input x"]
pub struct PRIV4_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV5` reader - Security enable on event input x"]
pub struct PRIV5_R(crate::FieldReader<bool, bool>);
impl PRIV5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV5` writer - Security enable on event input x"]
pub struct PRIV5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV6` reader - Security enable on event input x"]
pub struct PRIV6_R(crate::FieldReader<bool, bool>);
impl PRIV6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV6` writer - Security enable on event input x"]
pub struct PRIV6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV7` reader - Security enable on event input x"]
pub struct PRIV7_R(crate::FieldReader<bool, bool>);
impl PRIV7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV7` writer - Security enable on event input x"]
pub struct PRIV7_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV8` reader - Security enable on event input x"]
pub struct PRIV8_R(crate::FieldReader<bool, bool>);
impl PRIV8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV8` writer - Security enable on event input x"]
pub struct PRIV8_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV9` reader - Security enable on event input x"]
pub struct PRIV9_R(crate::FieldReader<bool, bool>);
impl PRIV9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV9` writer - Security enable on event input x"]
pub struct PRIV9_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV10` reader - Security enable on event input x"]
pub struct PRIV10_R(crate::FieldReader<bool, bool>);
impl PRIV10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV10` writer - Security enable on event input x"]
pub struct PRIV10_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV11` reader - Security enable on event input x"]
pub struct PRIV11_R(crate::FieldReader<bool, bool>);
impl PRIV11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV11` writer - Security enable on event input x"]
pub struct PRIV11_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV12` reader - Security enable on event input x"]
pub struct PRIV12_R(crate::FieldReader<bool, bool>);
impl PRIV12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV12` writer - Security enable on event input x"]
pub struct PRIV12_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV13` reader - Security enable on event input x"]
pub struct PRIV13_R(crate::FieldReader<bool, bool>);
impl PRIV13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV13` writer - Security enable on event input x"]
pub struct PRIV13_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV14` reader - Security enable on event input x"]
pub struct PRIV14_R(crate::FieldReader<bool, bool>);
impl PRIV14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV14` writer - Security enable on event input x"]
pub struct PRIV14_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV15` reader - Security enable on event input x"]
pub struct PRIV15_R(crate::FieldReader<bool, bool>);
impl PRIV15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV15` writer - Security enable on event input x"]
pub struct PRIV15_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV16` reader - Security enable on event input x"]
pub struct PRIV16_R(crate::FieldReader<bool, bool>);
impl PRIV16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV16` writer - Security enable on event input x"]
pub struct PRIV16_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV17` reader - Security enable on event input x"]
pub struct PRIV17_R(crate::FieldReader<bool, bool>);
impl PRIV17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV17` writer - Security enable on event input x"]
pub struct PRIV17_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV18` reader - Security enable on event input x"]
pub struct PRIV18_R(crate::FieldReader<bool, bool>);
impl PRIV18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV18` writer - Security enable on event input x"]
pub struct PRIV18_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV19` reader - Security enable on event input x"]
pub struct PRIV19_R(crate::FieldReader<bool, bool>);
impl PRIV19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV19` writer - Security enable on event input x"]
pub struct PRIV19_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV20` reader - Security enable on event input x"]
pub struct PRIV20_R(crate::FieldReader<bool, bool>);
impl PRIV20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV20` writer - Security enable on event input x"]
pub struct PRIV20_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV21` reader - Security enable on event input x"]
pub struct PRIV21_R(crate::FieldReader<bool, bool>);
impl PRIV21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV21` writer - Security enable on event input x"]
pub struct PRIV21_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV22` reader - Security enable on event input x"]
pub struct PRIV22_R(crate::FieldReader<bool, bool>);
impl PRIV22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV22` writer - Security enable on event input x"]
pub struct PRIV22_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV23` reader - Security enable on event input x"]
pub struct PRIV23_R(crate::FieldReader<bool, bool>);
impl PRIV23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV23` writer - Security enable on event input x"]
pub struct PRIV23_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV24` reader - Security enable on event input x"]
pub struct PRIV24_R(crate::FieldReader<bool, bool>);
impl PRIV24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV24` writer - Security enable on event input x"]
pub struct PRIV24_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV25` reader - Security enable on event input x"]
pub struct PRIV25_R(crate::FieldReader<bool, bool>);
impl PRIV25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV25` writer - Security enable on event input x"]
pub struct PRIV25_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV26` reader - Security enable on event input x"]
pub struct PRIV26_R(crate::FieldReader<bool, bool>);
impl PRIV26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV26` writer - Security enable on event input x"]
pub struct PRIV26_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV27` reader - Security enable on event input x"]
pub struct PRIV27_R(crate::FieldReader<bool, bool>);
impl PRIV27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV27` writer - Security enable on event input x"]
pub struct PRIV27_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV28` reader - Security enable on event input x"]
pub struct PRIV28_R(crate::FieldReader<bool, bool>);
impl PRIV28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV28` writer - Security enable on event input x"]
pub struct PRIV28_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV29` reader - Security enable on event input x"]
pub struct PRIV29_R(crate::FieldReader<bool, bool>);
impl PRIV29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV29` writer - Security enable on event input x"]
pub struct PRIV29_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV30` reader - Security enable on event input x"]
pub struct PRIV30_R(crate::FieldReader<bool, bool>);
impl PRIV30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV30` writer - Security enable on event input x"]
pub struct PRIV30_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PRIV31` reader - Security enable on event input x"]
pub struct PRIV31_R(crate::FieldReader<bool, bool>);
impl PRIV31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIV31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIV31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIV31` writer - Security enable on event input x"]
pub struct PRIV31_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv0(&self) -> PRIV0_R {
        PRIV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv1(&self) -> PRIV1_R {
        PRIV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv2(&self) -> PRIV2_R {
        PRIV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv3(&self) -> PRIV3_R {
        PRIV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv4(&self) -> PRIV4_R {
        PRIV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv5(&self) -> PRIV5_R {
        PRIV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv6(&self) -> PRIV6_R {
        PRIV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv7(&self) -> PRIV7_R {
        PRIV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv8(&self) -> PRIV8_R {
        PRIV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv9(&self) -> PRIV9_R {
        PRIV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv10(&self) -> PRIV10_R {
        PRIV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv11(&self) -> PRIV11_R {
        PRIV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv12(&self) -> PRIV12_R {
        PRIV12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv13(&self) -> PRIV13_R {
        PRIV13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv14(&self) -> PRIV14_R {
        PRIV14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv15(&self) -> PRIV15_R {
        PRIV15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv16(&self) -> PRIV16_R {
        PRIV16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv17(&self) -> PRIV17_R {
        PRIV17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv18(&self) -> PRIV18_R {
        PRIV18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv19(&self) -> PRIV19_R {
        PRIV19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv20(&self) -> PRIV20_R {
        PRIV20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv21(&self) -> PRIV21_R {
        PRIV21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv22(&self) -> PRIV22_R {
        PRIV22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv23(&self) -> PRIV23_R {
        PRIV23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv24(&self) -> PRIV24_R {
        PRIV24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv25(&self) -> PRIV25_R {
        PRIV25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv26(&self) -> PRIV26_R {
        PRIV26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv27(&self) -> PRIV27_R {
        PRIV27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv28(&self) -> PRIV28_R {
        PRIV28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv29(&self) -> PRIV29_R {
        PRIV29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv30(&self) -> PRIV30_R {
        PRIV30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv31(&self) -> PRIV31_R {
        PRIV31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv0(&mut self) -> PRIV0_W {
        PRIV0_W { w: self }
    }
    #[doc = "Bit 1 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv1(&mut self) -> PRIV1_W {
        PRIV1_W { w: self }
    }
    #[doc = "Bit 2 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv2(&mut self) -> PRIV2_W {
        PRIV2_W { w: self }
    }
    #[doc = "Bit 3 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv3(&mut self) -> PRIV3_W {
        PRIV3_W { w: self }
    }
    #[doc = "Bit 4 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv4(&mut self) -> PRIV4_W {
        PRIV4_W { w: self }
    }
    #[doc = "Bit 5 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv5(&mut self) -> PRIV5_W {
        PRIV5_W { w: self }
    }
    #[doc = "Bit 6 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv6(&mut self) -> PRIV6_W {
        PRIV6_W { w: self }
    }
    #[doc = "Bit 7 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv7(&mut self) -> PRIV7_W {
        PRIV7_W { w: self }
    }
    #[doc = "Bit 8 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv8(&mut self) -> PRIV8_W {
        PRIV8_W { w: self }
    }
    #[doc = "Bit 9 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv9(&mut self) -> PRIV9_W {
        PRIV9_W { w: self }
    }
    #[doc = "Bit 10 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv10(&mut self) -> PRIV10_W {
        PRIV10_W { w: self }
    }
    #[doc = "Bit 11 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv11(&mut self) -> PRIV11_W {
        PRIV11_W { w: self }
    }
    #[doc = "Bit 12 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv12(&mut self) -> PRIV12_W {
        PRIV12_W { w: self }
    }
    #[doc = "Bit 13 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv13(&mut self) -> PRIV13_W {
        PRIV13_W { w: self }
    }
    #[doc = "Bit 14 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv14(&mut self) -> PRIV14_W {
        PRIV14_W { w: self }
    }
    #[doc = "Bit 15 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv15(&mut self) -> PRIV15_W {
        PRIV15_W { w: self }
    }
    #[doc = "Bit 16 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv16(&mut self) -> PRIV16_W {
        PRIV16_W { w: self }
    }
    #[doc = "Bit 17 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv17(&mut self) -> PRIV17_W {
        PRIV17_W { w: self }
    }
    #[doc = "Bit 18 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv18(&mut self) -> PRIV18_W {
        PRIV18_W { w: self }
    }
    #[doc = "Bit 19 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv19(&mut self) -> PRIV19_W {
        PRIV19_W { w: self }
    }
    #[doc = "Bit 20 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv20(&mut self) -> PRIV20_W {
        PRIV20_W { w: self }
    }
    #[doc = "Bit 21 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv21(&mut self) -> PRIV21_W {
        PRIV21_W { w: self }
    }
    #[doc = "Bit 22 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv22(&mut self) -> PRIV22_W {
        PRIV22_W { w: self }
    }
    #[doc = "Bit 23 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv23(&mut self) -> PRIV23_W {
        PRIV23_W { w: self }
    }
    #[doc = "Bit 24 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv24(&mut self) -> PRIV24_W {
        PRIV24_W { w: self }
    }
    #[doc = "Bit 25 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv25(&mut self) -> PRIV25_W {
        PRIV25_W { w: self }
    }
    #[doc = "Bit 26 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv26(&mut self) -> PRIV26_W {
        PRIV26_W { w: self }
    }
    #[doc = "Bit 27 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv27(&mut self) -> PRIV27_W {
        PRIV27_W { w: self }
    }
    #[doc = "Bit 28 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv28(&mut self) -> PRIV28_W {
        PRIV28_W { w: self }
    }
    #[doc = "Bit 29 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv29(&mut self) -> PRIV29_W {
        PRIV29_W { w: self }
    }
    #[doc = "Bit 30 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv30(&mut self) -> PRIV30_W {
        PRIV30_W { w: self }
    }
    #[doc = "Bit 31 - Security enable on event input x"]
    #[inline(always)]
    pub fn priv31(&mut self) -> PRIV31_W {
        PRIV31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI privilege configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privcfgr1](index.html) module"]
pub struct PRIVCFGR1_SPEC;
impl crate::RegisterSpec for PRIVCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privcfgr1::R](R) reader structure"]
impl crate::Readable for PRIVCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privcfgr1::W](W) writer structure"]
impl crate::Writable for PRIVCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRIVCFGR1 to value 0"]
impl crate::Resettable for PRIVCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
