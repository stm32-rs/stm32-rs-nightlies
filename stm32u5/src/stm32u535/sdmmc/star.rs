#[doc = "Register `STAR` reader"]
pub type R = crate::R<STARrs>;
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CCRCFAIL_R = crate::BitReader;
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DCRCFAIL_R = crate::BitReader;
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CTIMEOUT_R = crate::BitReader;
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DTIMEOUT_R = crate::BitReader;
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error (masked by hardware when IDMA is enabled)"]
pub type TXUNDERR_R = crate::BitReader;
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error (masked by hardware when IDMA is enabled)"]
pub type RXOVERR_R = crate::BitReader;
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed, or no CRC)"]
pub type CMDREND_R = crate::BitReader;
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CMDSENT_R = crate::BitReader;
#[doc = "Field `DATAEND` reader - Data transfer ended correctly"]
pub type DATAEND_R = crate::BitReader;
#[doc = "Field `DHOLD` reader - Data transfer Hold"]
pub type DHOLD_R = crate::BitReader;
#[doc = "Field `DBCKEND` reader - Data block sent/received"]
pub type DBCKEND_R = crate::BitReader;
#[doc = "Field `DABORT` reader - Data transfer aborted by CMD12"]
pub type DABORT_R = crate::BitReader;
#[doc = "Field `DPSMACT` reader - Data path state machine active, i.e. not in Idle state"]
pub type DPSMACT_R = crate::BitReader;
#[doc = "Field `CPSMACT` reader - Command path state machine active, i.e. not in Idle state"]
pub type CPSMACT_R = crate::BitReader;
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty"]
pub type TXFIFOHE_R = crate::BitReader;
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full"]
pub type RXFIFOHF_R = crate::BitReader;
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TXFIFOF_R = crate::BitReader;
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RXFIFOF_R = crate::BitReader;
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TXFIFOE_R = crate::BitReader;
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RXFIFOE_R = crate::BitReader;
#[doc = "Field `BUSYD0` reader - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response"]
pub type BUSYD0_R = crate::BitReader;
#[doc = "Field `BUSYD0END` reader - end of SDMMC_D0 Busy following a CMD response detected"]
pub type BUSYD0END_R = crate::BitReader;
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SDIOIT_R = crate::BitReader;
#[doc = "Field `ACKFAIL` reader - Boot acknowledgment received (boot acknowledgment check fail)"]
pub type ACKFAIL_R = crate::BitReader;
#[doc = "Field `ACKTIMEOUT` reader - Boot acknowledgment timeout"]
pub type ACKTIMEOUT_R = crate::BitReader;
#[doc = "Field `VSWEND` reader - Voltage switch critical timing section completion"]
pub type VSWEND_R = crate::BitReader;
#[doc = "Field `CKSTOP` reader - SDMMC_CK stopped in Voltage switch procedure"]
pub type CKSTOP_R = crate::BitReader;
#[doc = "Field `IDMATE` reader - IDMA transfer error"]
pub type IDMATE_R = crate::BitReader;
#[doc = "Field `IDMABTC` reader - IDMA buffer transfer complete"]
pub type IDMABTC_R = crate::BitReader;
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
    #[doc = "Bit 4 - Transmit FIFO underrun error (masked by hardware when IDMA is enabled)"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error (masked by hardware when IDMA is enabled)"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed, or no CRC)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data transfer ended correctly"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data transfer Hold"]
    #[inline(always)]
    pub fn dhold(&self) -> DHOLD_R {
        DHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted by CMD12"]
    #[inline(always)]
    pub fn dabort(&self) -> DABORT_R {
        DABORT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data path state machine active, i.e. not in Idle state"]
    #[inline(always)]
    pub fn dpsmact(&self) -> DPSMACT_R {
        DPSMACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Command path state machine active, i.e. not in Idle state"]
    #[inline(always)]
    pub fn cpsmact(&self) -> CPSMACT_R {
        CPSMACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full"]
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
    #[doc = "Bit 20 - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response"]
    #[inline(always)]
    pub fn busyd0(&self) -> BUSYD0_R {
        BUSYD0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - end of SDMMC_D0 Busy following a CMD response detected"]
    #[inline(always)]
    pub fn busyd0end(&self) -> BUSYD0END_R {
        BUSYD0END_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Boot acknowledgment received (boot acknowledgment check fail)"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Boot acknowledgment timeout"]
    #[inline(always)]
    pub fn acktimeout(&self) -> ACKTIMEOUT_R {
        ACKTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion"]
    #[inline(always)]
    pub fn vswend(&self) -> VSWEND_R {
        VSWEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SDMMC_CK stopped in Voltage switch procedure"]
    #[inline(always)]
    pub fn ckstop(&self) -> CKSTOP_R {
        CKSTOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IDMA transfer error"]
    #[inline(always)]
    pub fn idmate(&self) -> IDMATE_R {
        IDMATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete"]
    #[inline(always)]
    pub fn idmabtc(&self) -> IDMABTC_R {
        IDMABTC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`star::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STARrs;
impl crate::RegisterSpec for STARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`star::R`](R) reader structure"]
impl crate::Readable for STARrs {}
#[doc = "`reset()` method sets STAR to value 0"]
impl crate::Resettable for STARrs {
    const RESET_VALUE: u32 = 0;
}
