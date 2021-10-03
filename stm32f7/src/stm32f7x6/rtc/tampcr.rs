#[doc = "Register `TAMPCR` reader"]
pub struct R(crate::R<TAMPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMPCR` writer"]
pub struct W(crate::W<TAMPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMPCR_SPEC>;
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
impl From<crate::W<TAMPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
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
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
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
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
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
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
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
#[doc = "Field `TAMP2E` reader - Tamper 2 detection enable"]
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
#[doc = "Field `TAMP2E` writer - Tamper 2 detection enable"]
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
#[doc = "Field `TAMP2TRG` reader - Active level for tamper 2"]
pub struct TAMP2TRG_R(crate::FieldReader<bool, bool>);
impl TAMP2TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2TRG` writer - Active level for tamper 2"]
pub struct TAMP2TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2TRG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3E` reader - Tamper 3 detection enable"]
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
#[doc = "Field `TAMP3E` writer - Tamper 3 detection enable"]
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
#[doc = "Field `TAMP3TRG` reader - Active level for tamper 3"]
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
#[doc = "Field `TAMP3TRG` writer - Active level for tamper 3"]
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
#[doc = "Field `TAMPFLT` reader - Tamper filter count"]
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
#[doc = "Field `TAMPFLT` writer - Tamper filter count"]
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
#[doc = "Field `TAMPPRCH` reader - Tamper precharge duration"]
pub struct TAMPPRCH_R(crate::FieldReader<u8, u8>);
impl TAMPPRCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TAMPPRCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPPRCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPRCH` writer - Tamper precharge duration"]
pub struct TAMPPRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `TAMPPUDIS` reader - TAMPER pull-up disable"]
pub struct TAMPPUDIS_R(crate::FieldReader<bool, bool>);
impl TAMPPUDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPPUDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPPUDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPPUDIS` writer - TAMPER pull-up disable"]
pub struct TAMPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPPUDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub struct TAMP1IE_R(crate::FieldReader<bool, bool>);
impl TAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub struct TAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1IE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase"]
pub struct TAMP1NOERASE_R(crate::FieldReader<bool, bool>);
impl TAMP1NOERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1NOERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1NOERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase"]
pub struct TAMP1NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1NOERASE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag"]
pub struct TAMP1MF_R(crate::FieldReader<bool, bool>);
impl TAMP1MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag"]
pub struct TAMP1MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1MF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub struct TAMP2IE_R(crate::FieldReader<bool, bool>);
impl TAMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub struct TAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2IE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP2NOERASE` reader - Tamper 2 no erase"]
pub struct TAMP2NOERASE_R(crate::FieldReader<bool, bool>);
impl TAMP2NOERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2NOERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2NOERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2NOERASE` writer - Tamper 2 no erase"]
pub struct TAMP2NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2NOERASE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP2MF` reader - Tamper 2 mask flag"]
pub struct TAMP2MF_R(crate::FieldReader<bool, bool>);
impl TAMP2MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2MF` writer - Tamper 2 mask flag"]
pub struct TAMP2MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2MF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub struct TAMP3IE_R(crate::FieldReader<bool, bool>);
impl TAMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub struct TAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3IE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3NOERASE` reader - Tamper 3 no erase"]
pub struct TAMP3NOERASE_R(crate::FieldReader<bool, bool>);
impl TAMP3NOERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3NOERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3NOERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3NOERASE` writer - Tamper 3 no erase"]
pub struct TAMP3NOERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3NOERASE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TAMP3MF` reader - Tamper 3 mask flag"]
pub struct TAMP3MF_R(crate::FieldReader<bool, bool>);
impl TAMP3MF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3MF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3MF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3MF` writer - Tamper 3 mask flag"]
pub struct TAMP3MF_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3MF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> TAMP1NOERASE_R {
        TAMP1NOERASE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&self) -> TAMP2NOERASE_R {
        TAMP2NOERASE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&self) -> TAMP3NOERASE_R {
        TAMP3NOERASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W {
        TAMPIE_W { w: self }
    }
    #[doc = "Bit 3 - Tamper 2 detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    #[doc = "Bit 4 - Active level for tamper 2"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W {
        TAMP2TRG_W { w: self }
    }
    #[doc = "Bit 5 - Tamper 3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W {
        TAMP3E_W { w: self }
    }
    #[doc = "Bit 6 - Active level for tamper 3"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W {
        TAMP3TRG_W { w: self }
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W {
        TAMPTS_W { w: self }
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W {
        TAMPFREQ_W { w: self }
    }
    #[doc = "Bits 11:12 - Tamper filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W {
        TAMPFLT_W { w: self }
    }
    #[doc = "Bits 13:14 - Tamper precharge duration"]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W {
        TAMPPRCH_W { w: self }
    }
    #[doc = "Bit 15 - TAMPER pull-up disable"]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W {
        TAMPPUDIS_W { w: self }
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W {
        TAMP1IE_W { w: self }
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> TAMP1NOERASE_W {
        TAMP1NOERASE_W { w: self }
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> TAMP1MF_W {
        TAMP1MF_W { w: self }
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W {
        TAMP2IE_W { w: self }
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> TAMP2NOERASE_W {
        TAMP2NOERASE_W { w: self }
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> TAMP2MF_W {
        TAMP2MF_W { w: self }
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W {
        TAMP3IE_W { w: self }
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> TAMP3NOERASE_W {
        TAMP3NOERASE_W { w: self }
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> TAMP3MF_W {
        TAMP3MF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tampcr](index.html) module"]
pub struct TAMPCR_SPEC;
impl crate::RegisterSpec for TAMPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tampcr::R](R) reader structure"]
impl crate::Readable for TAMPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tampcr::W](W) writer structure"]
impl crate::Writable for TAMPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMPCR to value 0"]
impl crate::Resettable for TAMPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
