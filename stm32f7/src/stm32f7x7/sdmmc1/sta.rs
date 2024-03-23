#[doc = "Register `STA` reader"]
pub type R = crate::R<STArs>;
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CCRCFAIL_R = crate::BitReader;
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DCRCFAIL_R = crate::BitReader;
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DTIMEOUT_R = crate::BitReader;
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub type TXUNDERR_R = crate::BitReader;
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub type RXOVERR_R = crate::BitReader;
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub type CMDREND_R = crate::BitReader;
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CMDSENT_R = crate::BitReader;
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub type DATAEND_R = crate::BitReader;
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub type DBCKEND_R = crate::BitReader;
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub type CMDACT_R = crate::BitReader;
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub type TXACT_R = crate::BitReader;
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub type RXACT_R = crate::BitReader;
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub type TXFIFOHE_R = crate::BitReader;
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub type RXFIFOHF_R = crate::BitReader;
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TXFIFOF_R = crate::BitReader;
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RXFIFOF_R = crate::BitReader;
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TXFIFOE_R = crate::BitReader;
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RXFIFOE_R = crate::BitReader;
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub type TXDAVL_R = crate::BitReader;
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub type RXDAVL_R = crate::BitReader;
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SDIOIT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STArs;
impl crate::RegisterSpec for STArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for STArs {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STArs {
    const RESET_VALUE: u32 = 0;
}
