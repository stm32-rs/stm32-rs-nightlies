#[doc = "Register `ETZPC_DECPROT_LOCK0` reader"]
pub struct R(crate::R<ETZPC_DECPROT_LOCK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_DECPROT_LOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_DECPROT_LOCK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_DECPROT_LOCK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_DECPROT_LOCK0` writer"]
pub struct W(crate::W<ETZPC_DECPROT_LOCK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_DECPROT_LOCK0_SPEC>;
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
impl From<crate::W<ETZPC_DECPROT_LOCK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_DECPROT_LOCK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - LOCK0"]
pub struct LOCK0_R(crate::FieldReader<bool, bool>);
impl LOCK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK0` writer - LOCK0"]
pub struct LOCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK1` reader - LOCK1"]
pub struct LOCK1_R(crate::FieldReader<bool, bool>);
impl LOCK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK1` writer - LOCK1"]
pub struct LOCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK2` reader - LOCK2"]
pub struct LOCK2_R(crate::FieldReader<bool, bool>);
impl LOCK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK2` writer - LOCK2"]
pub struct LOCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK3` reader - LOCK3"]
pub struct LOCK3_R(crate::FieldReader<bool, bool>);
impl LOCK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK3` writer - LOCK3"]
pub struct LOCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK4` reader - LOCK4"]
pub struct LOCK4_R(crate::FieldReader<bool, bool>);
impl LOCK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK4` writer - LOCK4"]
pub struct LOCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK5` reader - LOCK5"]
pub struct LOCK5_R(crate::FieldReader<bool, bool>);
impl LOCK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK5` writer - LOCK5"]
pub struct LOCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK6` reader - LOCK6"]
pub struct LOCK6_R(crate::FieldReader<bool, bool>);
impl LOCK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK6` writer - LOCK6"]
pub struct LOCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK7` reader - LOCK7"]
pub struct LOCK7_R(crate::FieldReader<bool, bool>);
impl LOCK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK7` writer - LOCK7"]
pub struct LOCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK8` reader - LOCK8"]
pub struct LOCK8_R(crate::FieldReader<bool, bool>);
impl LOCK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK8` writer - LOCK8"]
pub struct LOCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK9` reader - LOCK9"]
pub struct LOCK9_R(crate::FieldReader<bool, bool>);
impl LOCK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK9` writer - LOCK9"]
pub struct LOCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK10` reader - LOCK10"]
pub struct LOCK10_R(crate::FieldReader<bool, bool>);
impl LOCK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK10` writer - LOCK10"]
pub struct LOCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK11` reader - LOCK11"]
pub struct LOCK11_R(crate::FieldReader<bool, bool>);
impl LOCK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK11` writer - LOCK11"]
pub struct LOCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK12` reader - LOCK12"]
pub struct LOCK12_R(crate::FieldReader<bool, bool>);
impl LOCK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK12` writer - LOCK12"]
pub struct LOCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK13` reader - LOCK13"]
pub struct LOCK13_R(crate::FieldReader<bool, bool>);
impl LOCK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK13` writer - LOCK13"]
pub struct LOCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK14` reader - LOCK14"]
pub struct LOCK14_R(crate::FieldReader<bool, bool>);
impl LOCK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK14` writer - LOCK14"]
pub struct LOCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK15` reader - LOCK15"]
pub struct LOCK15_R(crate::FieldReader<bool, bool>);
impl LOCK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK15` writer - LOCK15"]
pub struct LOCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK16` reader - LOCK16"]
pub struct LOCK16_R(crate::FieldReader<bool, bool>);
impl LOCK16_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK16` writer - LOCK16"]
pub struct LOCK16_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK17` reader - LOCK17"]
pub struct LOCK17_R(crate::FieldReader<bool, bool>);
impl LOCK17_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK17` writer - LOCK17"]
pub struct LOCK17_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK18` reader - LOCK18"]
pub struct LOCK18_R(crate::FieldReader<bool, bool>);
impl LOCK18_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK18` writer - LOCK18"]
pub struct LOCK18_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK19` reader - LOCK19"]
pub struct LOCK19_R(crate::FieldReader<bool, bool>);
impl LOCK19_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK19` writer - LOCK19"]
pub struct LOCK19_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK20` reader - LOCK20"]
pub struct LOCK20_R(crate::FieldReader<bool, bool>);
impl LOCK20_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK20` writer - LOCK20"]
pub struct LOCK20_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK21` reader - LOCK21"]
pub struct LOCK21_R(crate::FieldReader<bool, bool>);
impl LOCK21_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK21` writer - LOCK21"]
pub struct LOCK21_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK22` reader - LOCK22"]
pub struct LOCK22_R(crate::FieldReader<bool, bool>);
impl LOCK22_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK22` writer - LOCK22"]
pub struct LOCK22_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK23` reader - LOCK23"]
pub struct LOCK23_R(crate::FieldReader<bool, bool>);
impl LOCK23_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK23` writer - LOCK23"]
pub struct LOCK23_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK24` reader - LOCK24"]
pub struct LOCK24_R(crate::FieldReader<bool, bool>);
impl LOCK24_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK24` writer - LOCK24"]
pub struct LOCK24_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK25` reader - LOCK25"]
pub struct LOCK25_R(crate::FieldReader<bool, bool>);
impl LOCK25_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK25` writer - LOCK25"]
pub struct LOCK25_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK26` reader - LOCK26"]
pub struct LOCK26_R(crate::FieldReader<bool, bool>);
impl LOCK26_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK26` writer - LOCK26"]
pub struct LOCK26_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK27` reader - LOCK27"]
pub struct LOCK27_R(crate::FieldReader<bool, bool>);
impl LOCK27_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK27` writer - LOCK27"]
pub struct LOCK27_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK28` reader - LOCK28"]
pub struct LOCK28_R(crate::FieldReader<bool, bool>);
impl LOCK28_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK28` writer - LOCK28"]
pub struct LOCK28_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK29` reader - LOCK29"]
pub struct LOCK29_R(crate::FieldReader<bool, bool>);
impl LOCK29_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK29` writer - LOCK29"]
pub struct LOCK29_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK30` reader - LOCK30"]
pub struct LOCK30_R(crate::FieldReader<bool, bool>);
impl LOCK30_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK30` writer - LOCK30"]
pub struct LOCK30_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LOCK31` reader - LOCK31"]
pub struct LOCK31_R(crate::FieldReader<bool, bool>);
impl LOCK31_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK31` writer - LOCK31"]
pub struct LOCK31_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    pub fn lock16(&self) -> LOCK16_R {
        LOCK16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    pub fn lock17(&self) -> LOCK17_R {
        LOCK17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    pub fn lock18(&self) -> LOCK18_R {
        LOCK18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    pub fn lock19(&self) -> LOCK19_R {
        LOCK19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    pub fn lock20(&self) -> LOCK20_R {
        LOCK20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    pub fn lock21(&self) -> LOCK21_R {
        LOCK21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    pub fn lock22(&self) -> LOCK22_R {
        LOCK22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    pub fn lock23(&self) -> LOCK23_R {
        LOCK23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    pub fn lock24(&self) -> LOCK24_R {
        LOCK24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    pub fn lock25(&self) -> LOCK25_R {
        LOCK25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    pub fn lock26(&self) -> LOCK26_R {
        LOCK26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    pub fn lock27(&self) -> LOCK27_R {
        LOCK27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    pub fn lock28(&self) -> LOCK28_R {
        LOCK28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    pub fn lock29(&self) -> LOCK29_R {
        LOCK29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    pub fn lock30(&self) -> LOCK30_R {
        LOCK30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    pub fn lock31(&self) -> LOCK31_R {
        LOCK31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&mut self) -> LOCK0_W {
        LOCK0_W { w: self }
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&mut self) -> LOCK1_W {
        LOCK1_W { w: self }
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&mut self) -> LOCK2_W {
        LOCK2_W { w: self }
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&mut self) -> LOCK3_W {
        LOCK3_W { w: self }
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&mut self) -> LOCK4_W {
        LOCK4_W { w: self }
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&mut self) -> LOCK5_W {
        LOCK5_W { w: self }
    }
    #[doc = "Bit 6 - LOCK6"]
    #[inline(always)]
    pub fn lock6(&mut self) -> LOCK6_W {
        LOCK6_W { w: self }
    }
    #[doc = "Bit 7 - LOCK7"]
    #[inline(always)]
    pub fn lock7(&mut self) -> LOCK7_W {
        LOCK7_W { w: self }
    }
    #[doc = "Bit 8 - LOCK8"]
    #[inline(always)]
    pub fn lock8(&mut self) -> LOCK8_W {
        LOCK8_W { w: self }
    }
    #[doc = "Bit 9 - LOCK9"]
    #[inline(always)]
    pub fn lock9(&mut self) -> LOCK9_W {
        LOCK9_W { w: self }
    }
    #[doc = "Bit 10 - LOCK10"]
    #[inline(always)]
    pub fn lock10(&mut self) -> LOCK10_W {
        LOCK10_W { w: self }
    }
    #[doc = "Bit 11 - LOCK11"]
    #[inline(always)]
    pub fn lock11(&mut self) -> LOCK11_W {
        LOCK11_W { w: self }
    }
    #[doc = "Bit 12 - LOCK12"]
    #[inline(always)]
    pub fn lock12(&mut self) -> LOCK12_W {
        LOCK12_W { w: self }
    }
    #[doc = "Bit 13 - LOCK13"]
    #[inline(always)]
    pub fn lock13(&mut self) -> LOCK13_W {
        LOCK13_W { w: self }
    }
    #[doc = "Bit 14 - LOCK14"]
    #[inline(always)]
    pub fn lock14(&mut self) -> LOCK14_W {
        LOCK14_W { w: self }
    }
    #[doc = "Bit 15 - LOCK15"]
    #[inline(always)]
    pub fn lock15(&mut self) -> LOCK15_W {
        LOCK15_W { w: self }
    }
    #[doc = "Bit 16 - LOCK16"]
    #[inline(always)]
    pub fn lock16(&mut self) -> LOCK16_W {
        LOCK16_W { w: self }
    }
    #[doc = "Bit 17 - LOCK17"]
    #[inline(always)]
    pub fn lock17(&mut self) -> LOCK17_W {
        LOCK17_W { w: self }
    }
    #[doc = "Bit 18 - LOCK18"]
    #[inline(always)]
    pub fn lock18(&mut self) -> LOCK18_W {
        LOCK18_W { w: self }
    }
    #[doc = "Bit 19 - LOCK19"]
    #[inline(always)]
    pub fn lock19(&mut self) -> LOCK19_W {
        LOCK19_W { w: self }
    }
    #[doc = "Bit 20 - LOCK20"]
    #[inline(always)]
    pub fn lock20(&mut self) -> LOCK20_W {
        LOCK20_W { w: self }
    }
    #[doc = "Bit 21 - LOCK21"]
    #[inline(always)]
    pub fn lock21(&mut self) -> LOCK21_W {
        LOCK21_W { w: self }
    }
    #[doc = "Bit 22 - LOCK22"]
    #[inline(always)]
    pub fn lock22(&mut self) -> LOCK22_W {
        LOCK22_W { w: self }
    }
    #[doc = "Bit 23 - LOCK23"]
    #[inline(always)]
    pub fn lock23(&mut self) -> LOCK23_W {
        LOCK23_W { w: self }
    }
    #[doc = "Bit 24 - LOCK24"]
    #[inline(always)]
    pub fn lock24(&mut self) -> LOCK24_W {
        LOCK24_W { w: self }
    }
    #[doc = "Bit 25 - LOCK25"]
    #[inline(always)]
    pub fn lock25(&mut self) -> LOCK25_W {
        LOCK25_W { w: self }
    }
    #[doc = "Bit 26 - LOCK26"]
    #[inline(always)]
    pub fn lock26(&mut self) -> LOCK26_W {
        LOCK26_W { w: self }
    }
    #[doc = "Bit 27 - LOCK27"]
    #[inline(always)]
    pub fn lock27(&mut self) -> LOCK27_W {
        LOCK27_W { w: self }
    }
    #[doc = "Bit 28 - LOCK28"]
    #[inline(always)]
    pub fn lock28(&mut self) -> LOCK28_W {
        LOCK28_W { w: self }
    }
    #[doc = "Bit 29 - LOCK29"]
    #[inline(always)]
    pub fn lock29(&mut self) -> LOCK29_W {
        LOCK29_W { w: self }
    }
    #[doc = "Bit 30 - LOCK30"]
    #[inline(always)]
    pub fn lock30(&mut self) -> LOCK30_W {
        LOCK30_W { w: self }
    }
    #[doc = "Bit 31 - LOCK31"]
    #[inline(always)]
    pub fn lock31(&mut self) -> LOCK31_W {
        LOCK31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETZPC decprot lock 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock0](index.html) module"]
pub struct ETZPC_DECPROT_LOCK0_SPEC;
impl crate::RegisterSpec for ETZPC_DECPROT_LOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_decprot_lock0::R](R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock0::W](W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETZPC_DECPROT_LOCK0 to value 0"]
impl crate::Resettable for ETZPC_DECPROT_LOCK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
