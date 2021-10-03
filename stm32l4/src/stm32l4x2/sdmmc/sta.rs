#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61"]
pub struct CEATAEND_R(crate::FieldReader<bool, bool>);
impl CEATAEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEATAEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEATAEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub struct SDIOIT_R(crate::FieldReader<bool, bool>);
impl SDIOIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub struct RXDAVL_R(crate::FieldReader<bool, bool>);
impl RXDAVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDAVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDAVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub struct TXDAVL_R(crate::FieldReader<bool, bool>);
impl TXDAVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDAVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDAVL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub struct RXFIFOE_R(crate::FieldReader<bool, bool>);
impl RXFIFOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub struct TXFIFOE_R(crate::FieldReader<bool, bool>);
impl TXFIFOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub struct RXFIFOF_R(crate::FieldReader<bool, bool>);
impl RXFIFOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub struct TXFIFOF_R(crate::FieldReader<bool, bool>);
impl TXFIFOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub struct RXFIFOHF_R(crate::FieldReader<bool, bool>);
impl RXFIFOHF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOHF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOHF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub struct TXFIFOHE_R(crate::FieldReader<bool, bool>);
impl TXFIFOHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFIFOHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub struct RXACT_R(crate::FieldReader<bool, bool>);
impl RXACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub struct TXACT_R(crate::FieldReader<bool, bool>);
impl TXACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub struct CMDACT_R(crate::FieldReader<bool, bool>);
impl CMDACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub struct DBCKEND_R(crate::FieldReader<bool, bool>);
impl DBCKEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBCKEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBCKEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode"]
pub struct STBITERR_R(crate::FieldReader<bool, bool>);
impl STBITERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STBITERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STBITERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub struct DATAEND_R(crate::FieldReader<bool, bool>);
impl DATAEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub struct CMDSENT_R(crate::FieldReader<bool, bool>);
impl CMDSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub struct CMDREND_R(crate::FieldReader<bool, bool>);
impl CMDREND_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDREND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDREND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub struct RXOVERR_R(crate::FieldReader<bool, bool>);
impl RXOVERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub struct TXUNDERR_R(crate::FieldReader<bool, bool>);
impl TXUNDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub struct DTIMEOUT_R(crate::FieldReader<bool, bool>);
impl DTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub struct CTIMEOUT_R(crate::FieldReader<bool, bool>);
impl CTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub struct DCRCFAIL_R(crate::FieldReader<bool, bool>);
impl DCRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub struct CCRCFAIL_R(crate::FieldReader<bool, bool>);
impl CCRCFAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRCFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRCFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
