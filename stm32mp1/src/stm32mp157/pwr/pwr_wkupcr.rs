#[doc = "Register `PWR_WKUPCR` reader"]
pub struct R(crate::R<PWR_WKUPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WKUPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WKUPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WKUPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_WKUPCR` writer"]
pub struct W(crate::W<PWR_WKUPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WKUPCR_SPEC>;
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
impl From<crate::W<PWR_WKUPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WKUPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPC1` reader - WKUPC1"]
pub struct WKUPC1_R(crate::FieldReader<bool, bool>);
impl WKUPC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC1` writer - WKUPC1"]
pub struct WKUPC1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPC2` reader - WKUPC2"]
pub struct WKUPC2_R(crate::FieldReader<bool, bool>);
impl WKUPC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC2` writer - WKUPC2"]
pub struct WKUPC2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPC3` reader - WKUPC3"]
pub struct WKUPC3_R(crate::FieldReader<bool, bool>);
impl WKUPC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC3` writer - WKUPC3"]
pub struct WKUPC3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPC4` reader - WKUPC4"]
pub struct WKUPC4_R(crate::FieldReader<bool, bool>);
impl WKUPC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC4` writer - WKUPC4"]
pub struct WKUPC4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPC5` reader - WKUPC5"]
pub struct WKUPC5_R(crate::FieldReader<bool, bool>);
impl WKUPC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC5` writer - WKUPC5"]
pub struct WKUPC5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPC6` reader - WKUPC6"]
pub struct WKUPC6_R(crate::FieldReader<bool, bool>);
impl WKUPC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPC6` writer - WKUPC6"]
pub struct WKUPC6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPC6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP1` reader - WKUPP1"]
pub struct WKUPP1_R(crate::FieldReader<bool, bool>);
impl WKUPP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP1` writer - WKUPP1"]
pub struct WKUPP1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP2` reader - WKUPP2"]
pub struct WKUPP2_R(crate::FieldReader<bool, bool>);
impl WKUPP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP2` writer - WKUPP2"]
pub struct WKUPP2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP3` reader - WKUPP3"]
pub struct WKUPP3_R(crate::FieldReader<bool, bool>);
impl WKUPP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP3` writer - WKUPP3"]
pub struct WKUPP3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP4` reader - WKUPP4"]
pub struct WKUPP4_R(crate::FieldReader<bool, bool>);
impl WKUPP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP4` writer - WKUPP4"]
pub struct WKUPP4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP5` reader - WKUPP5"]
pub struct WKUPP5_R(crate::FieldReader<bool, bool>);
impl WKUPP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP5` writer - WKUPP5"]
pub struct WKUPP5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPP6` reader - WKUPP6"]
pub struct WKUPP6_R(crate::FieldReader<bool, bool>);
impl WKUPP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPP6` writer - WKUPP6"]
pub struct WKUPP6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPP6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `WKUPPUPD1` reader - WKUPPUPD1"]
pub struct WKUPPUPD1_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD1_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD1` writer - WKUPPUPD1"]
pub struct WKUPPUPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `WKUPPUPD2` reader - WKUPPUPD2"]
pub struct WKUPPUPD2_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD2_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD2` writer - WKUPPUPD2"]
pub struct WKUPPUPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `WKUPPUPD3` reader - WKUPPUPD3"]
pub struct WKUPPUPD3_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD3_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD3` writer - WKUPPUPD3"]
pub struct WKUPPUPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `WKUPPUPD4` reader - WKUPPUPD4"]
pub struct WKUPPUPD4_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD4_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD4` writer - WKUPPUPD4"]
pub struct WKUPPUPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `WKUPPUPD5` reader - WKUPPUPD5"]
pub struct WKUPPUPD5_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD5_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD5` writer - WKUPPUPD5"]
pub struct WKUPPUPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `WKUPPUPD6` reader - WKUPPUPD6"]
pub struct WKUPPUPD6_R(crate::FieldReader<u8, u8>);
impl WKUPPUPD6_R {
    pub(crate) fn new(bits: u8) -> Self {
        WKUPPUPD6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WKUPPUPD6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKUPPUPD6` writer - WKUPPUPD6"]
pub struct WKUPPUPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPPUPD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&self) -> WKUPC1_R {
        WKUPC1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&self) -> WKUPC2_R {
        WKUPC2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&self) -> WKUPC3_R {
        WKUPC3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&self) -> WKUPC4_R {
        WKUPC4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&self) -> WKUPC5_R {
        WKUPC5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&self) -> WKUPC6_R {
        WKUPC6_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&self) -> WKUPP1_R {
        WKUPP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&self) -> WKUPP2_R {
        WKUPP2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&self) -> WKUPP3_R {
        WKUPP3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&self) -> WKUPP4_R {
        WKUPP4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&self) -> WKUPP5_R {
        WKUPP5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&self) -> WKUPP6_R {
        WKUPP6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&self) -> WKUPPUPD1_R {
        WKUPPUPD1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&self) -> WKUPPUPD2_R {
        WKUPPUPD2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&self) -> WKUPPUPD3_R {
        WKUPPUPD3_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&self) -> WKUPPUPD4_R {
        WKUPPUPD4_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&self) -> WKUPPUPD5_R {
        WKUPPUPD5_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
    #[inline(always)]
    pub fn wkuppupd6(&self) -> WKUPPUPD6_R {
        WKUPPUPD6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPC1"]
    #[inline(always)]
    pub fn wkupc1(&mut self) -> WKUPC1_W {
        WKUPC1_W { w: self }
    }
    #[doc = "Bit 1 - WKUPC2"]
    #[inline(always)]
    pub fn wkupc2(&mut self) -> WKUPC2_W {
        WKUPC2_W { w: self }
    }
    #[doc = "Bit 2 - WKUPC3"]
    #[inline(always)]
    pub fn wkupc3(&mut self) -> WKUPC3_W {
        WKUPC3_W { w: self }
    }
    #[doc = "Bit 3 - WKUPC4"]
    #[inline(always)]
    pub fn wkupc4(&mut self) -> WKUPC4_W {
        WKUPC4_W { w: self }
    }
    #[doc = "Bit 4 - WKUPC5"]
    #[inline(always)]
    pub fn wkupc5(&mut self) -> WKUPC5_W {
        WKUPC5_W { w: self }
    }
    #[doc = "Bit 5 - WKUPC6"]
    #[inline(always)]
    pub fn wkupc6(&mut self) -> WKUPC6_W {
        WKUPC6_W { w: self }
    }
    #[doc = "Bit 8 - WKUPP1"]
    #[inline(always)]
    pub fn wkupp1(&mut self) -> WKUPP1_W {
        WKUPP1_W { w: self }
    }
    #[doc = "Bit 9 - WKUPP2"]
    #[inline(always)]
    pub fn wkupp2(&mut self) -> WKUPP2_W {
        WKUPP2_W { w: self }
    }
    #[doc = "Bit 10 - WKUPP3"]
    #[inline(always)]
    pub fn wkupp3(&mut self) -> WKUPP3_W {
        WKUPP3_W { w: self }
    }
    #[doc = "Bit 11 - WKUPP4"]
    #[inline(always)]
    pub fn wkupp4(&mut self) -> WKUPP4_W {
        WKUPP4_W { w: self }
    }
    #[doc = "Bit 12 - WKUPP5"]
    #[inline(always)]
    pub fn wkupp5(&mut self) -> WKUPP5_W {
        WKUPP5_W { w: self }
    }
    #[doc = "Bit 13 - WKUPP6"]
    #[inline(always)]
    pub fn wkupp6(&mut self) -> WKUPP6_W {
        WKUPP6_W { w: self }
    }
    #[doc = "Bits 16:17 - WKUPPUPD1"]
    #[inline(always)]
    pub fn wkuppupd1(&mut self) -> WKUPPUPD1_W {
        WKUPPUPD1_W { w: self }
    }
    #[doc = "Bits 18:19 - WKUPPUPD2"]
    #[inline(always)]
    pub fn wkuppupd2(&mut self) -> WKUPPUPD2_W {
        WKUPPUPD2_W { w: self }
    }
    #[doc = "Bits 20:21 - WKUPPUPD3"]
    #[inline(always)]
    pub fn wkuppupd3(&mut self) -> WKUPPUPD3_W {
        WKUPPUPD3_W { w: self }
    }
    #[doc = "Bits 22:23 - WKUPPUPD4"]
    #[inline(always)]
    pub fn wkuppupd4(&mut self) -> WKUPPUPD4_W {
        WKUPPUPD4_W { w: self }
    }
    #[doc = "Bits 24:25 - WKUPPUPD5"]
    #[inline(always)]
    pub fn wkuppupd5(&mut self) -> WKUPPUPD5_W {
        WKUPPUPD5_W { w: self }
    }
    #[doc = "Bits 26:27 - WKUPPUPD6"]
    #[inline(always)]
    pub fn wkuppupd6(&mut self) -> WKUPPUPD6_W {
        WKUPPUPD6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\\[6:1\\], WKUPP\\[6:1\\]
bits and WKUPPUPD\\[6:1\\]
bit pairs are discarded when the corresponding WKUPEN\\[6:1\\]
bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_wkupcr](index.html) module"]
pub struct PWR_WKUPCR_SPEC;
impl crate::RegisterSpec for PWR_WKUPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_wkupcr::R](R) reader structure"]
impl crate::Readable for PWR_WKUPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_wkupcr::W](W) writer structure"]
impl crate::Writable for PWR_WKUPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_WKUPCR to value 0"]
impl crate::Resettable for PWR_WKUPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
