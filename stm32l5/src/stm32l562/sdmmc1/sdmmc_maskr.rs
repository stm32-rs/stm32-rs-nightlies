#[doc = "Register `SDMMC_MASKR` reader"]
pub struct R(crate::R<SDMMC_MASKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_MASKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_MASKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_MASKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMMC_MASKR` writer"]
pub struct W(crate::W<SDMMC_MASKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_MASKR_SPEC>;
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
impl From<crate::W<SDMMC_MASKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_MASKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub struct CCRCFAILIE_R(crate::FieldReader<bool, bool>);
impl CCRCFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAILIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRCFAILIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub struct CCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCFAILIE_W<'a> {
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
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub struct DCRCFAILIE_R(crate::FieldReader<bool, bool>);
impl DCRCFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCFAILIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCFAILIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub struct DCRCFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCFAILIE_W<'a> {
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
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub struct CTIMEOUTIE_R(crate::FieldReader<bool, bool>);
impl CTIMEOUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMEOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMEOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub struct CTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMEOUTIE_W<'a> {
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
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub struct DTIMEOUTIE_R(crate::FieldReader<bool, bool>);
impl DTIMEOUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIMEOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIMEOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub struct DTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIMEOUTIE_W<'a> {
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
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub struct TXUNDERRIE_R(crate::FieldReader<bool, bool>);
impl TXUNDERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub struct TXUNDERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRIE_W<'a> {
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
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub struct RXOVERRIE_R(crate::FieldReader<bool, bool>);
impl RXOVERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub struct RXOVERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRIE_W<'a> {
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
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub struct CMDRENDIE_R(crate::FieldReader<bool, bool>);
impl CMDRENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDRENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub struct CMDRENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRENDIE_W<'a> {
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
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub struct CMDSENTIE_R(crate::FieldReader<bool, bool>);
impl CMDSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub struct CMDSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENTIE_W<'a> {
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
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub struct DATAENDIE_R(crate::FieldReader<bool, bool>);
impl DATAENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub struct DATAENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAENDIE_W<'a> {
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
#[doc = "Field `DHOLDIE` reader - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub struct DHOLDIE_R(crate::FieldReader<bool, bool>);
impl DHOLDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DHOLDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DHOLDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DHOLDIE` writer - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub struct DHOLDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DHOLDIE_W<'a> {
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
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub struct DBCKENDIE_R(crate::FieldReader<bool, bool>);
impl DBCKENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCKENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCKENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub struct DBCKENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBCKENDIE_W<'a> {
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
#[doc = "Field `DABORTIE` reader - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub struct DABORTIE_R(crate::FieldReader<bool, bool>);
impl DABORTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DABORTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DABORTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DABORTIE` writer - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub struct DABORTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DABORTIE_W<'a> {
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
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub struct TXFIFOHEIE_R(crate::FieldReader<bool, bool>);
impl TXFIFOHEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOHEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOHEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub struct TXFIFOHEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOHEIE_W<'a> {
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
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub struct RXFIFOHFIE_R(crate::FieldReader<bool, bool>);
impl RXFIFOHFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOHFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOHFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub struct RXFIFOHFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOHFIE_W<'a> {
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
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub struct RXFIFOFIE_R(crate::FieldReader<bool, bool>);
impl RXFIFOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub struct RXFIFOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOFIE_W<'a> {
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
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub struct TXFIFOEIE_R(crate::FieldReader<bool, bool>);
impl TXFIFOEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub struct TXFIFOEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOEIE_W<'a> {
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
#[doc = "Field `BUSYD0ENDIE` reader - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub struct BUSYD0ENDIE_R(crate::FieldReader<bool, bool>);
impl BUSYD0ENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSYD0ENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYD0ENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSYD0ENDIE` writer - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub struct BUSYD0ENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSYD0ENDIE_W<'a> {
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
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub struct SDIOITIE_R(crate::FieldReader<bool, bool>);
impl SDIOITIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOITIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOITIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub struct SDIOITIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOITIE_W<'a> {
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
#[doc = "Field `ACKFAILIE` reader - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub struct ACKFAILIE_R(crate::FieldReader<bool, bool>);
impl ACKFAILIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKFAILIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKFAILIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKFAILIE` writer - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub struct ACKFAILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKFAILIE_W<'a> {
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
#[doc = "Field `ACKTIMEOUTIE` reader - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub struct ACKTIMEOUTIE_R(crate::FieldReader<bool, bool>);
impl ACKTIMEOUTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKTIMEOUTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKTIMEOUTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKTIMEOUTIE` writer - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub struct ACKTIMEOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKTIMEOUTIE_W<'a> {
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
#[doc = "Field `VSWENDIE` reader - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub struct VSWENDIE_R(crate::FieldReader<bool, bool>);
impl VSWENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSWENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSWENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSWENDIE` writer - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub struct VSWENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWENDIE_W<'a> {
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
#[doc = "Field `CKSTOPIE` reader - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub struct CKSTOPIE_R(crate::FieldReader<bool, bool>);
impl CKSTOPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSTOPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSTOPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSTOPIE` writer - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub struct CKSTOPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTOPIE_W<'a> {
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
#[doc = "Field `IDMABTCIE` reader - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub struct IDMABTCIE_R(crate::FieldReader<bool, bool>);
impl IDMABTCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDMABTCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMABTCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMABTCIE` writer - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub struct IDMABTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMABTCIE_W<'a> {
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
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W {
        CCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W {
        DCRCFAILIE_W { w: self }
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W {
        CTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W {
        DTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W {
        TXUNDERRIE_W { w: self }
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W {
        RXOVERRIE_W { w: self }
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W {
        CMDRENDIE_W { w: self }
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W {
        CMDSENTIE_W { w: self }
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DATAENDIE_W {
        DATAENDIE_W { w: self }
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    pub fn dholdie(&mut self) -> DHOLDIE_W {
        DHOLDIE_W { w: self }
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W {
        DBCKENDIE_W { w: self }
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn dabortie(&mut self) -> DABORTIE_W {
        DABORTIE_W { w: self }
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W {
        TXFIFOHEIE_W { w: self }
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W {
        RXFIFOHFIE_W { w: self }
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W {
        RXFIFOFIE_W { w: self }
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W {
        TXFIFOEIE_W { w: self }
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W {
        BUSYD0ENDIE_W { w: self }
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SDIOITIE_W {
        SDIOITIE_W { w: self }
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W {
        ACKFAILIE_W { w: self }
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W {
        ACKTIMEOUTIE_W { w: self }
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    pub fn vswendie(&mut self) -> VSWENDIE_W {
        VSWENDIE_W { w: self }
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W {
        CKSTOPIE_W { w: self }
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W {
        IDMABTCIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmmc_maskr](index.html) module"]
pub struct SDMMC_MASKR_SPEC;
impl crate::RegisterSpec for SDMMC_MASKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmmc_maskr::R](R) reader structure"]
impl crate::Readable for SDMMC_MASKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmmc_maskr::W](W) writer structure"]
impl crate::Writable for SDMMC_MASKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMMC_MASKR to value 0"]
impl crate::Resettable for SDMMC_MASKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
