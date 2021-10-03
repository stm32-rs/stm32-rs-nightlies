#[doc = "Register `RSTAR` reader"]
pub struct R(crate::R<RSTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTAR` writer"]
pub struct W(crate::W<RSTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTAR_SPEC>;
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
impl From<crate::W<RSTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMECMP4` reader - Timer E Compare 4"]
pub struct TIMECMP4_R(crate::FieldReader<bool, bool>);
impl TIMECMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMECMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMECMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMECMP4` writer - Timer E Compare 4"]
pub struct TIMECMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMECMP2` reader - Timer E Compare 2"]
pub struct TIMECMP2_R(crate::FieldReader<bool, bool>);
impl TIMECMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMECMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMECMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMECMP2` writer - Timer E Compare 2"]
pub struct TIMECMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMECMP1` reader - Timer E Compare 1"]
pub struct TIMECMP1_R(crate::FieldReader<bool, bool>);
impl TIMECMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMECMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMECMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMECMP1` writer - Timer E Compare 1"]
pub struct TIMECMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMECMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMDCMP4` reader - Timer D Compare 4"]
pub struct TIMDCMP4_R(crate::FieldReader<bool, bool>);
impl TIMDCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMDCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMDCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMDCMP4` writer - Timer D Compare 4"]
pub struct TIMDCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMDCMP2` reader - Timer D Compare 2"]
pub struct TIMDCMP2_R(crate::FieldReader<bool, bool>);
impl TIMDCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMDCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMDCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMDCMP2` writer - Timer D Compare 2"]
pub struct TIMDCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMDCMP1` reader - Timer D Compare 1"]
pub struct TIMDCMP1_R(crate::FieldReader<bool, bool>);
impl TIMDCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMDCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMDCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMDCMP1` writer - Timer D Compare 1"]
pub struct TIMDCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDCMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMCCMP4` reader - Timer C Compare 4"]
pub struct TIMCCMP4_R(crate::FieldReader<bool, bool>);
impl TIMCCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMCCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMCCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMCCMP4` writer - Timer C Compare 4"]
pub struct TIMCCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMCCMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMCCMP2` reader - Timer C Compare 2"]
pub struct TIMCCMP2_R(crate::FieldReader<bool, bool>);
impl TIMCCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMCCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMCCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMCCMP2` writer - Timer C Compare 2"]
pub struct TIMCCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMCCMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMCCMP1` reader - Timer C Compare 1"]
pub struct TIMCCMP1_R(crate::FieldReader<bool, bool>);
impl TIMCCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMCCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMCCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMCCMP1` writer - Timer C Compare 1"]
pub struct TIMCCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMCCMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMBCMP4` reader - Timer B Compare 4"]
pub struct TIMBCMP4_R(crate::FieldReader<bool, bool>);
impl TIMBCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMBCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMBCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMBCMP4` writer - Timer B Compare 4"]
pub struct TIMBCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMBCMP2` reader - Timer B Compare 2"]
pub struct TIMBCMP2_R(crate::FieldReader<bool, bool>);
impl TIMBCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMBCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMBCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMBCMP2` writer - Timer B Compare 2"]
pub struct TIMBCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TIMBCMP1` reader - Timer B Compare 1"]
pub struct TIMBCMP1_R(crate::FieldReader<bool, bool>);
impl TIMBCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMBCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMBCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMBCMP1` writer - Timer B Compare 1"]
pub struct TIMBCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMBCMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT10` reader - External Event 10"]
pub struct EXTEVNT10_R(crate::FieldReader<bool, bool>);
impl EXTEVNT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT10` writer - External Event 10"]
pub struct EXTEVNT10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT9` reader - External Event 9"]
pub struct EXTEVNT9_R(crate::FieldReader<bool, bool>);
impl EXTEVNT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT9` writer - External Event 9"]
pub struct EXTEVNT9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT8` reader - External Event 8"]
pub struct EXTEVNT8_R(crate::FieldReader<bool, bool>);
impl EXTEVNT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT8` writer - External Event 8"]
pub struct EXTEVNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT7` reader - External Event 7"]
pub struct EXTEVNT7_R(crate::FieldReader<bool, bool>);
impl EXTEVNT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT7` writer - External Event 7"]
pub struct EXTEVNT7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT6` reader - External Event 6"]
pub struct EXTEVNT6_R(crate::FieldReader<bool, bool>);
impl EXTEVNT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT6` writer - External Event 6"]
pub struct EXTEVNT6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT5` reader - External Event 5"]
pub struct EXTEVNT5_R(crate::FieldReader<bool, bool>);
impl EXTEVNT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT5` writer - External Event 5"]
pub struct EXTEVNT5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT4` reader - External Event 4"]
pub struct EXTEVNT4_R(crate::FieldReader<bool, bool>);
impl EXTEVNT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT4` writer - External Event 4"]
pub struct EXTEVNT4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT3` reader - External Event 3"]
pub struct EXTEVNT3_R(crate::FieldReader<bool, bool>);
impl EXTEVNT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT3` writer - External Event 3"]
pub struct EXTEVNT3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT2` reader - External Event 2"]
pub struct EXTEVNT2_R(crate::FieldReader<bool, bool>);
impl EXTEVNT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT2` writer - External Event 2"]
pub struct EXTEVNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTEVNT1` reader - External Event 1"]
pub struct EXTEVNT1_R(crate::FieldReader<bool, bool>);
impl EXTEVNT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTEVNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTEVNT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTEVNT1` writer - External Event 1"]
pub struct EXTEVNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTEVNT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MSTCMP4` reader - Master compare 4"]
pub struct MSTCMP4_R(crate::FieldReader<bool, bool>);
impl MSTCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP4` writer - Master compare 4"]
pub struct MSTCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MSTCMP3` reader - Master compare 3"]
pub struct MSTCMP3_R(crate::FieldReader<bool, bool>);
impl MSTCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP3` writer - Master compare 3"]
pub struct MSTCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MSTCMP2` reader - Master compare 2"]
pub struct MSTCMP2_R(crate::FieldReader<bool, bool>);
impl MSTCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP2` writer - Master compare 2"]
pub struct MSTCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MSTCMP1` reader - Master compare 1"]
pub struct MSTCMP1_R(crate::FieldReader<bool, bool>);
impl MSTCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTCMP1` writer - Master compare 1"]
pub struct MSTCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTCMP1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `MSTPER` reader - Master timer Period"]
pub struct MSTPER_R(crate::FieldReader<bool, bool>);
impl MSTPER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTPER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTPER` writer - Master timer Period"]
pub struct MSTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CMP4` reader - Timer A compare 4 reset"]
pub struct CMP4_R(crate::FieldReader<bool, bool>);
impl CMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4` writer - Timer A compare 4 reset"]
pub struct CMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CMP2` reader - Timer A compare 2 reset"]
pub struct CMP2_R(crate::FieldReader<bool, bool>);
impl CMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2` writer - Timer A compare 2 reset"]
pub struct CMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `UPDT` reader - Timer A Update reset"]
pub struct UPDT_R(crate::FieldReader<bool, bool>);
impl UPDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDT` writer - Timer A Update reset"]
pub struct UPDT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&self) -> TIMECMP4_R {
        TIMECMP4_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&self) -> TIMECMP2_R {
        TIMECMP2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&self) -> TIMECMP1_R {
        TIMECMP1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&self) -> TIMDCMP4_R {
        TIMDCMP4_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&self) -> TIMDCMP2_R {
        TIMDCMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&self) -> TIMDCMP1_R {
        TIMDCMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    pub fn timccmp4(&self) -> TIMCCMP4_R {
        TIMCCMP4_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn timccmp2(&self) -> TIMCCMP2_R {
        TIMCCMP2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn timccmp1(&self) -> TIMCCMP1_R {
        TIMCCMP1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&self) -> TIMBCMP4_R {
        TIMBCMP4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&self) -> TIMBCMP2_R {
        TIMBCMP2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&self) -> TIMBCMP1_R {
        TIMBCMP1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&self) -> EXTEVNT10_R {
        EXTEVNT10_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&self) -> EXTEVNT9_R {
        EXTEVNT9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&self) -> EXTEVNT8_R {
        EXTEVNT8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&self) -> EXTEVNT7_R {
        EXTEVNT7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&self) -> EXTEVNT6_R {
        EXTEVNT6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&self) -> EXTEVNT5_R {
        EXTEVNT5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&self) -> EXTEVNT4_R {
        EXTEVNT4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&self) -> EXTEVNT3_R {
        EXTEVNT3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&self) -> EXTEVNT2_R {
        EXTEVNT2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&self) -> EXTEVNT1_R {
        EXTEVNT1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&self) -> MSTCMP4_R {
        MSTCMP4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&self) -> MSTCMP3_R {
        MSTCMP3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&self) -> MSTCMP2_R {
        MSTCMP2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&self) -> MSTCMP1_R {
        MSTCMP1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&self) -> MSTPER_R {
        MSTPER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&self) -> UPDT_R {
        UPDT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Timer E Compare 4"]
    #[inline(always)]
    pub fn timecmp4(&mut self) -> TIMECMP4_W {
        TIMECMP4_W { w: self }
    }
    #[doc = "Bit 29 - Timer E Compare 2"]
    #[inline(always)]
    pub fn timecmp2(&mut self) -> TIMECMP2_W {
        TIMECMP2_W { w: self }
    }
    #[doc = "Bit 28 - Timer E Compare 1"]
    #[inline(always)]
    pub fn timecmp1(&mut self) -> TIMECMP1_W {
        TIMECMP1_W { w: self }
    }
    #[doc = "Bit 27 - Timer D Compare 4"]
    #[inline(always)]
    pub fn timdcmp4(&mut self) -> TIMDCMP4_W {
        TIMDCMP4_W { w: self }
    }
    #[doc = "Bit 26 - Timer D Compare 2"]
    #[inline(always)]
    pub fn timdcmp2(&mut self) -> TIMDCMP2_W {
        TIMDCMP2_W { w: self }
    }
    #[doc = "Bit 25 - Timer D Compare 1"]
    #[inline(always)]
    pub fn timdcmp1(&mut self) -> TIMDCMP1_W {
        TIMDCMP1_W { w: self }
    }
    #[doc = "Bit 24 - Timer C Compare 4"]
    #[inline(always)]
    pub fn timccmp4(&mut self) -> TIMCCMP4_W {
        TIMCCMP4_W { w: self }
    }
    #[doc = "Bit 23 - Timer C Compare 2"]
    #[inline(always)]
    pub fn timccmp2(&mut self) -> TIMCCMP2_W {
        TIMCCMP2_W { w: self }
    }
    #[doc = "Bit 22 - Timer C Compare 1"]
    #[inline(always)]
    pub fn timccmp1(&mut self) -> TIMCCMP1_W {
        TIMCCMP1_W { w: self }
    }
    #[doc = "Bit 21 - Timer B Compare 4"]
    #[inline(always)]
    pub fn timbcmp4(&mut self) -> TIMBCMP4_W {
        TIMBCMP4_W { w: self }
    }
    #[doc = "Bit 20 - Timer B Compare 2"]
    #[inline(always)]
    pub fn timbcmp2(&mut self) -> TIMBCMP2_W {
        TIMBCMP2_W { w: self }
    }
    #[doc = "Bit 19 - Timer B Compare 1"]
    #[inline(always)]
    pub fn timbcmp1(&mut self) -> TIMBCMP1_W {
        TIMBCMP1_W { w: self }
    }
    #[doc = "Bit 18 - External Event 10"]
    #[inline(always)]
    pub fn extevnt10(&mut self) -> EXTEVNT10_W {
        EXTEVNT10_W { w: self }
    }
    #[doc = "Bit 17 - External Event 9"]
    #[inline(always)]
    pub fn extevnt9(&mut self) -> EXTEVNT9_W {
        EXTEVNT9_W { w: self }
    }
    #[doc = "Bit 16 - External Event 8"]
    #[inline(always)]
    pub fn extevnt8(&mut self) -> EXTEVNT8_W {
        EXTEVNT8_W { w: self }
    }
    #[doc = "Bit 15 - External Event 7"]
    #[inline(always)]
    pub fn extevnt7(&mut self) -> EXTEVNT7_W {
        EXTEVNT7_W { w: self }
    }
    #[doc = "Bit 14 - External Event 6"]
    #[inline(always)]
    pub fn extevnt6(&mut self) -> EXTEVNT6_W {
        EXTEVNT6_W { w: self }
    }
    #[doc = "Bit 13 - External Event 5"]
    #[inline(always)]
    pub fn extevnt5(&mut self) -> EXTEVNT5_W {
        EXTEVNT5_W { w: self }
    }
    #[doc = "Bit 12 - External Event 4"]
    #[inline(always)]
    pub fn extevnt4(&mut self) -> EXTEVNT4_W {
        EXTEVNT4_W { w: self }
    }
    #[doc = "Bit 11 - External Event 3"]
    #[inline(always)]
    pub fn extevnt3(&mut self) -> EXTEVNT3_W {
        EXTEVNT3_W { w: self }
    }
    #[doc = "Bit 10 - External Event 2"]
    #[inline(always)]
    pub fn extevnt2(&mut self) -> EXTEVNT2_W {
        EXTEVNT2_W { w: self }
    }
    #[doc = "Bit 9 - External Event 1"]
    #[inline(always)]
    pub fn extevnt1(&mut self) -> EXTEVNT1_W {
        EXTEVNT1_W { w: self }
    }
    #[doc = "Bit 8 - Master compare 4"]
    #[inline(always)]
    pub fn mstcmp4(&mut self) -> MSTCMP4_W {
        MSTCMP4_W { w: self }
    }
    #[doc = "Bit 7 - Master compare 3"]
    #[inline(always)]
    pub fn mstcmp3(&mut self) -> MSTCMP3_W {
        MSTCMP3_W { w: self }
    }
    #[doc = "Bit 6 - Master compare 2"]
    #[inline(always)]
    pub fn mstcmp2(&mut self) -> MSTCMP2_W {
        MSTCMP2_W { w: self }
    }
    #[doc = "Bit 5 - Master compare 1"]
    #[inline(always)]
    pub fn mstcmp1(&mut self) -> MSTCMP1_W {
        MSTCMP1_W { w: self }
    }
    #[doc = "Bit 4 - Master timer Period"]
    #[inline(always)]
    pub fn mstper(&mut self) -> MSTPER_W {
        MSTPER_W { w: self }
    }
    #[doc = "Bit 3 - Timer A compare 4 reset"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> CMP4_W {
        CMP4_W { w: self }
    }
    #[doc = "Bit 2 - Timer A compare 2 reset"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> CMP2_W {
        CMP2_W { w: self }
    }
    #[doc = "Bit 1 - Timer A Update reset"]
    #[inline(always)]
    pub fn updt(&mut self) -> UPDT_W {
        UPDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstar](index.html) module"]
pub struct RSTAR_SPEC;
impl crate::RegisterSpec for RSTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstar::R](R) reader structure"]
impl crate::Readable for RSTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstar::W](W) writer structure"]
impl crate::Writable for RSTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTAR to value 0"]
impl crate::Resettable for RSTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
