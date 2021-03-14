#[doc = "Reader of register STA"]
pub type R = crate::R<u32, super::STA>;
#[doc = "Reader of field `CCRCFAIL`"]
pub type CCRCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCRCFAIL`"]
pub type DCRCFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTIMEOUT`"]
pub type CTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTIMEOUT`"]
pub type DTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUNDERR`"]
pub type TXUNDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOVERR`"]
pub type RXOVERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDREND`"]
pub type CMDREND_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDSENT`"]
pub type CMDSENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATAEND`"]
pub type DATAEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `STBITERR`"]
pub type STBITERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBCKEND`"]
pub type DBCKEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDACT`"]
pub type CMDACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXACT`"]
pub type TXACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXACT`"]
pub type RXACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOHE`"]
pub type TXFIFOHE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOHF`"]
pub type RXFIFOHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOF`"]
pub type TXFIFOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOF`"]
pub type RXFIFOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXFIFOE`"]
pub type TXFIFOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXFIFOE`"]
pub type RXFIFOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDAVL`"]
pub type TXDAVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXDAVL`"]
pub type RXDAVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIOIT`"]
pub type SDIOIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CEATAEND`"]
pub type CEATAEND_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CCRCFAIL"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RXOVERR"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CMDREND"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CMDSENT"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DATAEND"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STBITERR"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBCKEND"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CMDACT"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TXACT"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RXACT"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHE"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHF"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TXFIFOF"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RXFIFOF"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TXFIFOE"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RXFIFOE"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TXDAVL"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RXDAVL"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SDIOIT"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CEATAEND"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
