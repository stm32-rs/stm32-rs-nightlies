///Register `STAR` reader
pub type R = crate::R<STARrs>;
///Field `CCRCFAIL` reader - Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CCRCFAIL_R = crate::BitReader;
///Field `DCRCFAIL` reader - Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DCRCFAIL_R = crate::BitReader;
///Field `CTIMEOUT` reader - Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
pub type CTIMEOUT_R = crate::BitReader;
///Field `DTIMEOUT` reader - Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DTIMEOUT_R = crate::BitReader;
///Field `TXUNDERR` reader - Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type TXUNDERR_R = crate::BitReader;
///Field `RXOVERR` reader - Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type RXOVERR_R = crate::BitReader;
///Field `CMDREND` reader - Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDREND_R = crate::BitReader;
///Field `CMDSENT` reader - Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CMDSENT_R = crate::BitReader;
///Field `DATAEND` reader - Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DATAEND_R = crate::BitReader;
///Field `DHOLD` reader - Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DHOLD_R = crate::BitReader;
///Field `DBCKEND` reader - Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DBCKEND_R = crate::BitReader;
///Field `DABORT` reader - Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type DABORT_R = crate::BitReader;
///Field `DPSMACT` reader - Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
pub type DPSMACT_R = crate::BitReader;
///Field `CPSMACT` reader - Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
pub type CPSMACT_R = crate::BitReader;
///Field `TXFIFOHE` reader - Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
pub type TXFIFOHE_R = crate::BitReader;
///Field `RXFIFOHF` reader - Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
pub type RXFIFOHF_R = crate::BitReader;
///Field `TXFIFOF` reader - Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
pub type TXFIFOF_R = crate::BitReader;
///Field `RXFIFOF` reader - Receive FIFO full This bit is cleared when one FIFO location becomes empty.
pub type RXFIFOF_R = crate::BitReader;
///Field `TXFIFOE` reader - Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
pub type TXFIFOE_R = crate::BitReader;
///Field `RXFIFOE` reader - Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
pub type RXFIFOE_R = crate::BitReader;
///Field `BUSYD0` reader - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
pub type BUSYD0_R = crate::BitReader;
///Field `BUSYD0END` reader - end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type BUSYD0END_R = crate::BitReader;
///Field `SDIOIT` reader - SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type SDIOIT_R = crate::BitReader;
///Field `ACKFAIL` reader - Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type ACKFAIL_R = crate::BitReader;
///Field `ACKTIMEOUT` reader - Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type ACKTIMEOUT_R = crate::BitReader;
///Field `VSWEND` reader - Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type VSWEND_R = crate::BitReader;
///Field `CKSTOP` reader - SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type CKSTOP_R = crate::BitReader;
///Field `IDMATE` reader - IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type IDMATE_R = crate::BitReader;
///Field `IDMABTC` reader - IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
pub type IDMABTC_R = crate::BitReader;
impl R {
    ///Bit 0 - Command response received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data block sent/received (CRC check failed). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Command response timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR. The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Data timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit FIFO underrun error or IDMA read transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Received FIFO overrun error or IDMA write transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Command response received (CRC check passed, or no CRC). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Command sent (no response required). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Data transfer ended correctly. (data counter, DATACOUNT is zero and no errors occur). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Data transfer Hold. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dhold(&self) -> DHOLD_R {
        DHOLD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data block sent/received. (CRC check passed) and DPSM moves to the READWAIT state. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data transfer aborted by CMD12. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn dabort(&self) -> DABORT_R {
        DABORT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn dpsmact(&self) -> DPSMACT_R {
        DPSMACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Command path state machine active, i.e. not in Idle state. This is a hardware status flag only, does not generate an interrupt.
    #[inline(always)]
    pub fn cpsmact(&self) -> CPSMACT_R {
        CPSMACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Transmit FIFO half empty At least half the number of words can be written into the FIFO. This bit is cleared when the FIFO becomes half+1 full.
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive FIFO half full There are at least half the number of words in the FIFO. This bit is cleared when the FIFO becomes half+1 empty.
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Transmit FIFO full This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes empty.
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Receive FIFO full This bit is cleared when one FIFO location becomes empty.
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Transmit FIFO empty This bit is cleared when one FIFO location becomes full.
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receive FIFO empty This is a hardware status flag only, does not generate an interrupt. This bit is cleared when one FIFO location becomes full.
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response. This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit does not signal busy due to data transfer. This is a hardware status flag only, it does not generate an interrupt.
    #[inline(always)]
    pub fn busyd0(&self) -> BUSYD0_R {
        BUSYD0_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - end of SDMMC_D0 Busy following a CMD response detected. This indicates only end of busy following a CMD response. This bit does not signal busy due to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn busyd0end(&self) -> BUSYD0END_R {
        BUSYD0END_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIO interrupt received. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Boot acknowledgment received (boot acknowledgment check fail). Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Boot acknowledgment timeout. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn acktimeout(&self) -> ACKTIMEOUT_R {
        ACKTIMEOUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Voltage switch critical timing section completion. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn vswend(&self) -> VSWEND_R {
        VSWEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SDMMC_CK stopped in Voltage switch procedure. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn ckstop(&self) -> CKSTOP_R {
        CKSTOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMA transfer error. Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
    #[inline(always)]
    pub fn idmate(&self) -> IDMATE_R {
        IDMATE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMA buffer transfer complete. interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
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
/**The SDMMC_STAR register is a read-only register. It contains two types of flag:Static flags (bits \[29,21,11:0\]): these bits remain asserted until they are cleared by writing to the SDMMC interrupt Clear register (see SDMMC_ICR)Dynamic flags (bits \[20:12\]): these bits change state depending on the state of the underlying logic (for example, FIFO full and empty flags are asserted and de-asserted as data while written to the FIFO)

You can [`read`](crate::Reg::read) this register and get [`star::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#SDMMC1:STAR)*/
pub struct STARrs;
impl crate::RegisterSpec for STARrs {
    type Ux = u32;
}
///`read()` method returns [`star::R`](R) reader structure
impl crate::Readable for STARrs {}
///`reset()` method sets STAR to value 0
impl crate::Resettable for STARrs {}
