#[doc = "Register `TAFCR` reader"]
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFCR` writer"]
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC15MODE` reader - PC15 mode"]
pub struct PC15MODE_R(crate::FieldReader<bool, bool>);
impl PC15MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC15MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC15MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC15MODE` writer - PC15 mode"]
pub struct PC15MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PC15VALUE` reader - PC15 value"]
pub struct PC15VALUE_R(crate::FieldReader<bool, bool>);
impl PC15VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC15VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC15VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC15VALUE` writer - PC15 value"]
pub struct PC15VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC15VALUE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PC14MODE` reader - PC14 mode"]
pub struct PC14MODE_R(crate::FieldReader<bool, bool>);
impl PC14MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC14MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC14MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC14MODE` writer - PC14 mode"]
pub struct PC14MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PC14VALUE` reader - PC14 value"]
pub struct PC14VALUE_R(crate::FieldReader<bool, bool>);
impl PC14VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC14VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC14VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC14VALUE` writer - PC14 value"]
pub struct PC14VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC14VALUE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PC13MODE` reader - PC13 mode"]
pub struct PC13MODE_R(crate::FieldReader<bool, bool>);
impl PC13MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC13MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC13MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC13MODE` writer - PC13 mode"]
pub struct PC13MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PC13VALUE` reader - RTC_ALARM output type/PC13 value"]
pub struct PC13VALUE_R(crate::FieldReader<bool, bool>);
impl PC13VALUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC13VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC13VALUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC13VALUE` writer - RTC_ALARM output type/PC13 value"]
pub struct PC13VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> PC13VALUE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP_PUDIS` reader - RTC_TAMPx pull-up disable"]
pub struct TAMP_PUDIS_R(crate::FieldReader<bool, bool>);
impl TAMP_PUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP_PUDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP_PUDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP_PUDIS` writer - RTC_TAMPx pull-up disable"]
pub struct TAMP_PUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP_PUDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP_PRCH` reader - RTC_TAMPx precharge duration"]
pub struct TAMP_PRCH_R(crate::FieldReader<u8, u8>);
impl TAMP_PRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMP_PRCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP_PRCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP_PRCH` writer - RTC_TAMPx precharge duration"]
pub struct TAMP_PRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP_PRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count"]
pub struct TAMPFLT_R(crate::FieldReader<u8, u8>);
impl TAMPFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPFLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count"]
pub struct TAMPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub struct TAMPFREQ_R(crate::FieldReader<u8, u8>);
impl TAMPFREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPFREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPFREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub struct TAMPFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub struct TAMPTS_R(crate::FieldReader<bool, bool>);
impl TAMPTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub struct TAMPTS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input"]
pub struct TAMP3TRG_R(crate::FieldReader<bool, bool>);
impl TAMP3TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input"]
pub struct TAMP3TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3TRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub struct TAMP3E_R(crate::FieldReader<bool, bool>);
impl TAMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub struct TAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3E_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP2_TRG` reader - Active level for RTC_TAMP2 input"]
pub struct TAMP2_TRG_R(crate::FieldReader<bool, bool>);
impl TAMP2_TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2_TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2_TRG` writer - Active level for RTC_TAMP2 input"]
pub struct TAMP2_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2_TRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub struct TAMP2E_R(crate::FieldReader<bool, bool>);
impl TAMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub struct TAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2E_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub struct TAMPIE_R(crate::FieldReader<bool, bool>);
impl TAMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub struct TAMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input"]
pub struct TAMP1TRG_R(crate::FieldReader<bool, bool>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input"]
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub struct TAMP1E_R(crate::FieldReader<bool, bool>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&self) -> PC15MODE_R {
        PC15MODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&self) -> PC15VALUE_R {
        PC15VALUE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mode(&self) -> PC14MODE_R {
        PC14MODE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&self) -> PC14VALUE_R {
        PC14VALUE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&self) -> PC13MODE_R {
        PC13MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13value(&self) -> PC13VALUE_R {
        PC13VALUE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamp_pudis(&self) -> TAMP_PUDIS_R {
        TAMP_PUDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tamp_prch(&self) -> TAMP_PRCH_R {
        TAMP_PRCH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2_trg(&self) -> TAMP2_TRG_R {
        TAMP2_TRG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&mut self) -> PC15MODE_W {
        PC15MODE_W { w: self }
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&mut self) -> PC15VALUE_W {
        PC15VALUE_W { w: self }
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mode(&mut self) -> PC14MODE_W {
        PC14MODE_W { w: self }
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&mut self) -> PC14VALUE_W {
        PC14VALUE_W { w: self }
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&mut self) -> PC13MODE_W {
        PC13MODE_W { w: self }
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13value(&mut self) -> PC13VALUE_W {
        PC13VALUE_W { w: self }
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamp_pudis(&mut self) -> TAMP_PUDIS_W {
        TAMP_PUDIS_W { w: self }
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tamp_prch(&mut self) -> TAMP_PRCH_W {
        TAMP_PRCH_W { w: self }
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W {
        TAMP3TRG_W { w: self }
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W {
        TAMP3E_W { w: self }
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2_trg(&mut self) -> TAMP2_TRG_W {
        TAMP2_TRG_W { w: self }
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W {
        TAMPIE_W { w: self }
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](index.html) module"]
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafcr::R](R) reader structure"]
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafcr::W](W) writer structure"]
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
