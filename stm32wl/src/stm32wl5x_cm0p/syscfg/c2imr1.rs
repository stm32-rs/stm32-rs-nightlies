#[doc = "Register `C2IMR1` reader"]
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR1` writer"]
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` reader - RTCSTAMPTAMPLSECSSIM"]
pub struct RTCSTAMPTAMPLSECSSIM_R(crate::FieldReader<bool, bool>);
impl RTCSTAMPTAMPLSECSSIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCSTAMPTAMPLSECSSIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCSTAMPTAMPLSECSSIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` writer - RTCSTAMPTAMPLSECSSIM"]
pub struct RTCSTAMPTAMPLSECSSIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSTAMPTAMPLSECSSIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RTCALARMIM` reader - RTCALARMIM"]
pub struct RTCALARMIM_R(crate::FieldReader<bool, bool>);
impl RTCALARMIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCALARMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCALARMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCALARMIM` writer - RTCALARMIM"]
pub struct RTCALARMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALARMIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RTCSSRUIM` reader - RTCSSRUIM"]
pub struct RTCSSRUIM_R(crate::FieldReader<bool, bool>);
impl RTCSSRUIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCSSRUIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCSSRUIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSSRUIM` writer - RTCSSRUIM"]
pub struct RTCSSRUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSRUIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RTCWKUPIM` reader - RTCWKUPIM"]
pub struct RTCWKUPIM_R(crate::FieldReader<bool, bool>);
impl RTCWKUPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCWKUPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCWKUPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCWKUPIM` writer - RTCWKUPIM"]
pub struct RTCWKUPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWKUPIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RCCIM` reader - RCCIM"]
pub struct RCCIM_R(crate::FieldReader<bool, bool>);
impl RCCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCCIM` writer - RCCIM"]
pub struct RCCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FLASHIM` reader - FLASHIM"]
pub struct FLASHIM_R(crate::FieldReader<bool, bool>);
impl FLASHIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASHIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHIM` writer - FLASHIM"]
pub struct FLASHIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PKAIM` reader - PKAIM"]
pub struct PKAIM_R(crate::FieldReader<bool, bool>);
impl PKAIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKAIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKAIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKAIM` writer - PKAIM"]
pub struct PKAIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `AESIM` reader - AESIM"]
pub struct AESIM_R(crate::FieldReader<bool, bool>);
impl AESIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESIM` writer - AESIM"]
pub struct AESIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AESIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `COMPIM` reader - COMPIM"]
pub struct COMPIM_R(crate::FieldReader<bool, bool>);
impl COMPIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPIM` writer - COMPIM"]
pub struct COMPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ADCIM` reader - ADCIM"]
pub struct ADCIM_R(crate::FieldReader<bool, bool>);
impl ADCIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIM` writer - ADCIM"]
pub struct ADCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DACIM` reader - DACIM"]
pub struct DACIM_R(crate::FieldReader<bool, bool>);
impl DACIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACIM` writer - DACIM"]
pub struct DACIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DACIM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI0IM` reader - EXTI0IM"]
pub struct EXTI0IM_R(crate::FieldReader<bool, bool>);
impl EXTI0IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI0IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI0IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI0IM` writer - EXTI0IM"]
pub struct EXTI0IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI1IM` reader - EXTI1IM"]
pub struct EXTI1IM_R(crate::FieldReader<bool, bool>);
impl EXTI1IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI1IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI1IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI1IM` writer - EXTI1IM"]
pub struct EXTI1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI2IM` reader - EXTI2IM"]
pub struct EXTI2IM_R(crate::FieldReader<bool, bool>);
impl EXTI2IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI2IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI2IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI2IM` writer - EXTI2IM"]
pub struct EXTI2IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI3IM` reader - EXTI3IM"]
pub struct EXTI3IM_R(crate::FieldReader<bool, bool>);
impl EXTI3IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI3IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI3IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI3IM` writer - EXTI3IM"]
pub struct EXTI3IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI4IM` reader - EXTI4IM"]
pub struct EXTI4IM_R(crate::FieldReader<bool, bool>);
impl EXTI4IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI4IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI4IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI4IM` writer - EXTI4IM"]
pub struct EXTI4IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI5IM` reader - EXTI5IM"]
pub struct EXTI5IM_R(crate::FieldReader<bool, bool>);
impl EXTI5IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI5IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI5IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI5IM` writer - EXTI5IM"]
pub struct EXTI5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI6IM` reader - EXTI6IM"]
pub struct EXTI6IM_R(crate::FieldReader<bool, bool>);
impl EXTI6IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI6IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI6IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI6IM` writer - EXTI6IM"]
pub struct EXTI6IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI7IM` reader - EXTI7IM"]
pub struct EXTI7IM_R(crate::FieldReader<bool, bool>);
impl EXTI7IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI7IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI7IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI7IM` writer - EXTI7IM"]
pub struct EXTI7IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI8IM` reader - EXTI8IM"]
pub struct EXTI8IM_R(crate::FieldReader<bool, bool>);
impl EXTI8IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI8IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI8IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI8IM` writer - EXTI8IM"]
pub struct EXTI8IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI9IM` reader - EXTI9IM"]
pub struct EXTI9IM_R(crate::FieldReader<bool, bool>);
impl EXTI9IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI9IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI9IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI9IM` writer - EXTI9IM"]
pub struct EXTI9IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI10IM` reader - EXTI10IM"]
pub struct EXTI10IM_R(crate::FieldReader<bool, bool>);
impl EXTI10IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI10IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI10IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI10IM` writer - EXTI10IM"]
pub struct EXTI10IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI11IM` reader - EXTI11IM"]
pub struct EXTI11IM_R(crate::FieldReader<bool, bool>);
impl EXTI11IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI11IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI11IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI11IM` writer - EXTI11IM"]
pub struct EXTI11IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI12IM` reader - EXTI12IM"]
pub struct EXTI12IM_R(crate::FieldReader<bool, bool>);
impl EXTI12IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI12IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI12IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI12IM` writer - EXTI12IM"]
pub struct EXTI12IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI13IM` reader - EXTI13IM"]
pub struct EXTI13IM_R(crate::FieldReader<bool, bool>);
impl EXTI13IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI13IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI13IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI13IM` writer - EXTI13IM"]
pub struct EXTI13IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI14IM` reader - EXTI14IM"]
pub struct EXTI14IM_R(crate::FieldReader<bool, bool>);
impl EXTI14IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI14IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI14IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI14IM` writer - EXTI14IM"]
pub struct EXTI14IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `EXTI15IM` reader - EXTI15IM"]
pub struct EXTI15IM_R(crate::FieldReader<bool, bool>);
impl EXTI15IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTI15IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI15IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI15IM` writer - EXTI15IM"]
pub struct EXTI15IM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15IM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&self) -> RTCSTAMPTAMPLSECSSIM_R {
        RTCSTAMPTAMPLSECSSIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTCALARMIM"]
    #[inline(always)]
    pub fn rtcalarmim(&self) -> RTCALARMIM_R {
        RTCALARMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&self) -> RTCSSRUIM_R {
        RTCSSRUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTCWKUPIM"]
    #[inline(always)]
    pub fn rtcwkupim(&self) -> RTCWKUPIM_R {
        RTCWKUPIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RCCIM"]
    #[inline(always)]
    pub fn rccim(&self) -> RCCIM_R {
        RCCIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIM"]
    #[inline(always)]
    pub fn flashim(&self) -> FLASHIM_R {
        FLASHIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PKAIM"]
    #[inline(always)]
    pub fn pkaim(&self) -> PKAIM_R {
        PKAIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AESIM"]
    #[inline(always)]
    pub fn aesim(&self) -> AESIM_R {
        AESIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - COMPIM"]
    #[inline(always)]
    pub fn compim(&self) -> COMPIM_R {
        COMPIM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADCIM"]
    #[inline(always)]
    pub fn adcim(&self) -> ADCIM_R {
        ADCIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DACIM"]
    #[inline(always)]
    pub fn dacim(&self) -> DACIM_R {
        DACIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EXTI0IM"]
    #[inline(always)]
    pub fn exti0im(&self) -> EXTI0IM_R {
        EXTI0IM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - EXTI1IM"]
    #[inline(always)]
    pub fn exti1im(&self) -> EXTI1IM_R {
        EXTI1IM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - EXTI2IM"]
    #[inline(always)]
    pub fn exti2im(&self) -> EXTI2IM_R {
        EXTI2IM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - EXTI3IM"]
    #[inline(always)]
    pub fn exti3im(&self) -> EXTI3IM_R {
        EXTI3IM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - EXTI4IM"]
    #[inline(always)]
    pub fn exti4im(&self) -> EXTI4IM_R {
        EXTI4IM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    pub fn exti5im(&self) -> EXTI5IM_R {
        EXTI5IM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    pub fn exti6im(&self) -> EXTI6IM_R {
        EXTI6IM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    pub fn exti7im(&self) -> EXTI7IM_R {
        EXTI7IM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    pub fn exti8im(&self) -> EXTI8IM_R {
        EXTI8IM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    pub fn exti9im(&self) -> EXTI9IM_R {
        EXTI9IM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    pub fn exti10im(&self) -> EXTI10IM_R {
        EXTI10IM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    pub fn exti11im(&self) -> EXTI11IM_R {
        EXTI11IM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    pub fn exti12im(&self) -> EXTI12IM_R {
        EXTI12IM_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    pub fn exti13im(&self) -> EXTI13IM_R {
        EXTI13IM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    pub fn exti14im(&self) -> EXTI14IM_R {
        EXTI14IM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    pub fn exti15im(&self) -> EXTI15IM_R {
        EXTI15IM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&mut self) -> RTCSTAMPTAMPLSECSSIM_W {
        RTCSTAMPTAMPLSECSSIM_W { w: self }
    }
    #[doc = "Bit 1 - RTCALARMIM"]
    #[inline(always)]
    pub fn rtcalarmim(&mut self) -> RTCALARMIM_W {
        RTCALARMIM_W { w: self }
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&mut self) -> RTCSSRUIM_W {
        RTCSSRUIM_W { w: self }
    }
    #[doc = "Bit 3 - RTCWKUPIM"]
    #[inline(always)]
    pub fn rtcwkupim(&mut self) -> RTCWKUPIM_W {
        RTCWKUPIM_W { w: self }
    }
    #[doc = "Bit 5 - RCCIM"]
    #[inline(always)]
    pub fn rccim(&mut self) -> RCCIM_W {
        RCCIM_W { w: self }
    }
    #[doc = "Bit 6 - FLASHIM"]
    #[inline(always)]
    pub fn flashim(&mut self) -> FLASHIM_W {
        FLASHIM_W { w: self }
    }
    #[doc = "Bit 8 - PKAIM"]
    #[inline(always)]
    pub fn pkaim(&mut self) -> PKAIM_W {
        PKAIM_W { w: self }
    }
    #[doc = "Bit 10 - AESIM"]
    #[inline(always)]
    pub fn aesim(&mut self) -> AESIM_W {
        AESIM_W { w: self }
    }
    #[doc = "Bit 11 - COMPIM"]
    #[inline(always)]
    pub fn compim(&mut self) -> COMPIM_W {
        COMPIM_W { w: self }
    }
    #[doc = "Bit 12 - ADCIM"]
    #[inline(always)]
    pub fn adcim(&mut self) -> ADCIM_W {
        ADCIM_W { w: self }
    }
    #[doc = "Bit 13 - DACIM"]
    #[inline(always)]
    pub fn dacim(&mut self) -> DACIM_W {
        DACIM_W { w: self }
    }
    #[doc = "Bit 16 - EXTI0IM"]
    #[inline(always)]
    pub fn exti0im(&mut self) -> EXTI0IM_W {
        EXTI0IM_W { w: self }
    }
    #[doc = "Bit 17 - EXTI1IM"]
    #[inline(always)]
    pub fn exti1im(&mut self) -> EXTI1IM_W {
        EXTI1IM_W { w: self }
    }
    #[doc = "Bit 18 - EXTI2IM"]
    #[inline(always)]
    pub fn exti2im(&mut self) -> EXTI2IM_W {
        EXTI2IM_W { w: self }
    }
    #[doc = "Bit 19 - EXTI3IM"]
    #[inline(always)]
    pub fn exti3im(&mut self) -> EXTI3IM_W {
        EXTI3IM_W { w: self }
    }
    #[doc = "Bit 20 - EXTI4IM"]
    #[inline(always)]
    pub fn exti4im(&mut self) -> EXTI4IM_W {
        EXTI4IM_W { w: self }
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    pub fn exti5im(&mut self) -> EXTI5IM_W {
        EXTI5IM_W { w: self }
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    pub fn exti6im(&mut self) -> EXTI6IM_W {
        EXTI6IM_W { w: self }
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    pub fn exti7im(&mut self) -> EXTI7IM_W {
        EXTI7IM_W { w: self }
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    pub fn exti8im(&mut self) -> EXTI8IM_W {
        EXTI8IM_W { w: self }
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    pub fn exti9im(&mut self) -> EXTI9IM_W {
        EXTI9IM_W { w: self }
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    pub fn exti10im(&mut self) -> EXTI10IM_W {
        EXTI10IM_W { w: self }
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    pub fn exti11im(&mut self) -> EXTI11IM_W {
        EXTI11IM_W { w: self }
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    pub fn exti12im(&mut self) -> EXTI12IM_W {
        EXTI12IM_W { w: self }
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    pub fn exti13im(&mut self) -> EXTI13IM_W {
        EXTI13IM_W { w: self }
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    pub fn exti14im(&mut self) -> EXTI14IM_W {
        EXTI14IM_W { w: self }
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    pub fn exti15im(&mut self) -> EXTI15IM_W {
        EXTI15IM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG CPU2 interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](index.html) module"]
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr1::R](R) reader structure"]
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](W) writer structure"]
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2IMR1 to value 0"]
impl crate::Resettable for C2IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
