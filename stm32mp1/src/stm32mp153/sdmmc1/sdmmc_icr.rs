#[doc = "Register `SDMMC_ICR` reader"]
pub struct R(crate::R<SDMMC_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_ICR` writer"]
pub struct W(crate::W<SDMMC_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_ICR_SPEC>;
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
impl From<crate::W<SDMMC_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILC` reader - CCRCFAILC"]
pub struct CCRCFAILC_R(crate::FieldReader<bool, bool>);
impl CCRCFAILC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAILC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRCFAILC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAILC` writer - CCRCFAILC"]
pub struct CCRCFAILC_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCFAILC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DCRCFAILC` reader - DCRCFAILC"]
pub struct DCRCFAILC_R(crate::FieldReader<bool, bool>);
impl DCRCFAILC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCFAILC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCFAILC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCFAILC` writer - DCRCFAILC"]
pub struct DCRCFAILC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCFAILC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUTC"]
pub struct CTIMEOUTC_R(crate::FieldReader<bool, bool>);
impl CTIMEOUTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMEOUTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMEOUTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUTC"]
pub struct CTIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMEOUTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUTC"]
pub struct DTIMEOUTC_R(crate::FieldReader<bool, bool>);
impl DTIMEOUTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIMEOUTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIMEOUTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUTC"]
pub struct DTIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIMEOUTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TXUNDERRC` reader - TXUNDERRC"]
pub struct TXUNDERRC_R(crate::FieldReader<bool, bool>);
impl TXUNDERRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRC` writer - TXUNDERRC"]
pub struct TXUNDERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `RXOVERRC` reader - RXOVERRC"]
pub struct RXOVERRC_R(crate::FieldReader<bool, bool>);
impl RXOVERRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERRC` writer - RXOVERRC"]
pub struct RXOVERRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CMDRENDC` reader - CMDRENDC"]
pub struct CMDRENDC_R(crate::FieldReader<bool, bool>);
impl CMDRENDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDRENDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRENDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDRENDC` writer - CMDRENDC"]
pub struct CMDRENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRENDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CMDSENTC` reader - CMDSENTC"]
pub struct CMDSENTC_R(crate::FieldReader<bool, bool>);
impl CMDSENTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENTC` writer - CMDSENTC"]
pub struct CMDSENTC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DATAENDC` reader - DATAENDC"]
pub struct DATAENDC_R(crate::FieldReader<bool, bool>);
impl DATAENDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAENDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAENDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAENDC` writer - DATAENDC"]
pub struct DATAENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DHOLDC` reader - DHOLDC"]
pub struct DHOLDC_R(crate::FieldReader<bool, bool>);
impl DHOLDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHOLDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHOLDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHOLDC` writer - DHOLDC"]
pub struct DHOLDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DHOLDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DBCKENDC` reader - DBCKENDC"]
pub struct DBCKENDC_R(crate::FieldReader<bool, bool>);
impl DBCKENDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCKENDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCKENDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCKENDC` writer - DBCKENDC"]
pub struct DBCKENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCKENDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `DABORTC` reader - DABORTC"]
pub struct DABORTC_R(crate::FieldReader<bool, bool>);
impl DABORTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DABORTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DABORTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DABORTC` writer - DABORTC"]
pub struct DABORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DABORTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `BUSYD0ENDC` reader - BUSYD0ENDC"]
pub struct BUSYD0ENDC_R(crate::FieldReader<bool, bool>);
impl BUSYD0ENDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSYD0ENDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYD0ENDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYD0ENDC` writer - BUSYD0ENDC"]
pub struct BUSYD0ENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSYD0ENDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SDIOITC` reader - SDIOITC"]
pub struct SDIOITC_R(crate::FieldReader<bool, bool>);
impl SDIOITC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOITC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOITC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOITC` writer - SDIOITC"]
pub struct SDIOITC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOITC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ACKFAILC` reader - ACKFAILC"]
pub struct ACKFAILC_R(crate::FieldReader<bool, bool>);
impl ACKFAILC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKFAILC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKFAILC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKFAILC` writer - ACKFAILC"]
pub struct ACKFAILC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKFAILC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `ACKTIMEOUTC` reader - ACKTIMEOUTC"]
pub struct ACKTIMEOUTC_R(crate::FieldReader<bool, bool>);
impl ACKTIMEOUTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKTIMEOUTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKTIMEOUTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKTIMEOUTC` writer - ACKTIMEOUTC"]
pub struct ACKTIMEOUTC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKTIMEOUTC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `VSWENDC` reader - VSWENDC"]
pub struct VSWENDC_R(crate::FieldReader<bool, bool>);
impl VSWENDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWENDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWENDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWENDC` writer - VSWENDC"]
pub struct VSWENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWENDC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `CKSTOPC` reader - CKSTOPC"]
pub struct CKSTOPC_R(crate::FieldReader<bool, bool>);
impl CKSTOPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSTOPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSTOPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSTOPC` writer - CKSTOPC"]
pub struct CKSTOPC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTOPC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IDMATEC` reader - IDMATEC"]
pub struct IDMATEC_R(crate::FieldReader<bool, bool>);
impl IDMATEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMATEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMATEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMATEC` writer - IDMATEC"]
pub struct IDMATEC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMATEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `IDMABTCC` reader - IDMABTCC"]
pub struct IDMABTCC_R(crate::FieldReader<bool, bool>);
impl IDMABTCC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMABTCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABTCC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABTCC` writer - IDMABTCC"]
pub struct IDMABTCC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABTCC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DHOLDC"]
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DABORTC"]
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BUSYD0ENDC"]
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ACKFAILC"]
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ACKTIMEOUTC"]
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VSWENDC"]
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CKSTOPC"]
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IDMATEC"]
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IDMABTCC"]
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W {
        CCRCFAILC_W { w: self }
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W {
        DCRCFAILC_W { w: self }
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W {
        CTIMEOUTC_W { w: self }
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W {
        DTIMEOUTC_W { w: self }
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W {
        TXUNDERRC_W { w: self }
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W {
        RXOVERRC_W { w: self }
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W {
        CMDRENDC_W { w: self }
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W {
        CMDSENTC_W { w: self }
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W {
        DATAENDC_W { w: self }
    }
    #[doc = "Bit 9 - DHOLDC"]
    #[inline(always)]
    pub fn dholdc(&mut self) -> DHOLDC_W {
        DHOLDC_W { w: self }
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W {
        DBCKENDC_W { w: self }
    }
    #[doc = "Bit 11 - DABORTC"]
    #[inline(always)]
    pub fn dabortc(&mut self) -> DABORTC_W {
        DABORTC_W { w: self }
    }
    #[doc = "Bit 21 - BUSYD0ENDC"]
    #[inline(always)]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W {
        BUSYD0ENDC_W { w: self }
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W {
        SDIOITC_W { w: self }
    }
    #[doc = "Bit 23 - ACKFAILC"]
    #[inline(always)]
    pub fn ackfailc(&mut self) -> ACKFAILC_W {
        ACKFAILC_W { w: self }
    }
    #[doc = "Bit 24 - ACKTIMEOUTC"]
    #[inline(always)]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W {
        ACKTIMEOUTC_W { w: self }
    }
    #[doc = "Bit 25 - VSWENDC"]
    #[inline(always)]
    pub fn vswendc(&mut self) -> VSWENDC_W {
        VSWENDC_W { w: self }
    }
    #[doc = "Bit 26 - CKSTOPC"]
    #[inline(always)]
    pub fn ckstopc(&mut self) -> CKSTOPC_W {
        CKSTOPC_W { w: self }
    }
    #[doc = "Bit 27 - IDMATEC"]
    #[inline(always)]
    pub fn idmatec(&mut self) -> IDMATEC_W {
        IDMATEC_W { w: self }
    }
    #[doc = "Bit 28 - IDMABTCC"]
    #[inline(always)]
    pub fn idmabtcc(&mut self) -> IDMABTCC_W {
        IDMABTCC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_icr](index.html) module"]
pub struct SDMMC_ICR_SPEC;
impl crate::RegisterSpec for SDMMC_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_icr::R](R) reader structure"]
impl crate::Readable for SDMMC_ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_icr::W](W) writer structure"]
impl crate::Writable for SDMMC_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_ICR to value 0"]
impl crate::Resettable for SDMMC_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
