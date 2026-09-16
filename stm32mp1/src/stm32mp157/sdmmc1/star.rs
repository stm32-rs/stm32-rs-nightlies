///Register `STAR` reader
pub type R = crate::R<STARrs>;
///Field `CCRCFAIL` reader - CCRCFAIL
pub type CCRCFAIL_R = crate::BitReader;
///Field `DCRCFAIL` reader - DCRCFAIL
pub type DCRCFAIL_R = crate::BitReader;
///Field `CTIMEOUT` reader - CTIMEOUT
pub type CTIMEOUT_R = crate::BitReader;
///Field `DTIMEOUT` reader - DTIMEOUT
pub type DTIMEOUT_R = crate::BitReader;
///Field `TXUNDERR` reader - TXUNDERR
pub type TXUNDERR_R = crate::BitReader;
///Field `RXOVERR` reader - RXOVERR
pub type RXOVERR_R = crate::BitReader;
///Field `CMDREND` reader - CMDREND
pub type CMDREND_R = crate::BitReader;
///Field `CMDSENT` reader - CMDSENT
pub type CMDSENT_R = crate::BitReader;
///Field `DATAEND` reader - DATAEND
pub type DATAEND_R = crate::BitReader;
///Field `DHOLD` reader - DHOLD
pub type DHOLD_R = crate::BitReader;
///Field `DBCKEND` reader - DBCKEND
pub type DBCKEND_R = crate::BitReader;
///Field `DABORT` reader - DABORT
pub type DABORT_R = crate::BitReader;
///Field `DPSMACT` reader - DPSMACT
pub type DPSMACT_R = crate::BitReader;
///Field `CPSMACT` reader - CPSMACT
pub type CPSMACT_R = crate::BitReader;
///Field `TXFIFOHE` reader - TXFIFOHE
pub type TXFIFOHE_R = crate::BitReader;
///Field `RXFIFOHF` reader - RXFIFOHF
pub type RXFIFOHF_R = crate::BitReader;
///Field `TXFIFOF` reader - TXFIFOF
pub type TXFIFOF_R = crate::BitReader;
///Field `RXFIFOF` reader - RXFIFOF
pub type RXFIFOF_R = crate::BitReader;
///Field `TXFIFOE` reader - TXFIFOE
pub type TXFIFOE_R = crate::BitReader;
///Field `RXFIFOE` reader - RXFIFOE
pub type RXFIFOE_R = crate::BitReader;
///Field `BUSYD0` reader - BUSYD0
pub type BUSYD0_R = crate::BitReader;
///Field `BUSYD0END` reader - BUSYD0END
pub type BUSYD0END_R = crate::BitReader;
///Field `SDIOIT` reader - SDIOIT
pub type SDIOIT_R = crate::BitReader;
///Field `ACKFAIL` reader - ACKFAIL
pub type ACKFAIL_R = crate::BitReader;
///Field `ACKTIMEOUT` reader - ACKTIMEOUT
pub type ACKTIMEOUT_R = crate::BitReader;
///Field `VSWEND` reader - VSWEND
pub type VSWEND_R = crate::BitReader;
///Field `CKSTOP` reader - CKSTOP
pub type CKSTOP_R = crate::BitReader;
///Field `IDMATE` reader - IDMATE
pub type IDMATE_R = crate::BitReader;
///Field `IDMABTC` reader - IDMABTC
pub type IDMABTC_R = crate::BitReader;
impl R {
    ///Bit 0 - CCRCFAIL
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAIL
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUT
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUT
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERR
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERR
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDREND
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENT
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAEND
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DHOLD
    #[inline(always)]
    pub fn dhold(&self) -> DHOLD_R {
        DHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKEND
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORT
    #[inline(always)]
    pub fn dabort(&self) -> DABORT_R {
        DABORT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DPSMACT
    #[inline(always)]
    pub fn dpsmact(&self) -> DPSMACT_R {
        DPSMACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPSMACT
    #[inline(always)]
    pub fn cpsmact(&self) -> CPSMACT_R {
        CPSMACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXFIFOHE
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXFIFOHF
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TXFIFOF
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RXFIFOF
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXFIFOE
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RXFIFOE
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - BUSYD0
    #[inline(always)]
    pub fn busyd0(&self) -> BUSYD0_R {
        BUSYD0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - BUSYD0END
    #[inline(always)]
    pub fn busyd0end(&self) -> BUSYD0END_R {
        BUSYD0END_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOIT
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAIL
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUT
    #[inline(always)]
    pub fn acktimeout(&self) -> ACKTIMEOUT_R {
        ACKTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWEND
    #[inline(always)]
    pub fn vswend(&self) -> VSWEND_R {
        VSWEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOP
    #[inline(always)]
    pub fn ckstop(&self) -> CKSTOP_R {
        CKSTOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMATE
    #[inline(always)]
    pub fn idmate(&self) -> IDMATE_R {
        IDMATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMABTC
    #[inline(always)]
    pub fn idmabtc(&self) -> IDMABTC_R {
        IDMABTC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAR")
            .field("ccrcfail", &self.ccrcfail())
            .field("dcrcfail", &self.dcrcfail())
            .field("ctimeout", &self.ctimeout())
            .field("dtimeout", &self.dtimeout())
            .field("txunderr", &self.txunderr())
            .field("rxoverr", &self.rxoverr())
            .field("cmdrend", &self.cmdrend())
            .field("cmdsent", &self.cmdsent())
            .field("dataend", &self.dataend())
            .field("dhold", &self.dhold())
            .field("dbckend", &self.dbckend())
            .field("dabort", &self.dabort())
            .field("dpsmact", &self.dpsmact())
            .field("cpsmact", &self.cpsmact())
            .field("txfifohe", &self.txfifohe())
            .field("rxfifohf", &self.rxfifohf())
            .field("txfifof", &self.txfifof())
            .field("rxfifof", &self.rxfifof())
            .field("txfifoe", &self.txfifoe())
            .field("rxfifoe", &self.rxfifoe())
            .field("busyd0", &self.busyd0())
            .field("busyd0end", &self.busyd0end())
            .field("sdioit", &self.sdioit())
            .field("ackfail", &self.ackfail())
            .field("acktimeout", &self.acktimeout())
            .field("vswend", &self.vswend())
            .field("ckstop", &self.ckstop())
            .field("idmate", &self.idmate())
            .field("idmabtc", &self.idmabtc())
            .finish()
    }
}
/**The SDMMC_STAR register is a read-only register. It contains two types of flag: Static flags (bits \[28, 21, 11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR) Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)

You can [`read`](crate::Reg::read) this register and get [`star::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SDMMC1:STAR)*/
pub struct STARrs;
impl crate::RegisterSpec for STARrs {
    type Ux = u32;
}
///`read()` method returns [`star::R`](R) reader structure
impl crate::Readable for STARrs {}
///`reset()` method sets STAR to value 0
impl crate::Resettable for STARrs {}
