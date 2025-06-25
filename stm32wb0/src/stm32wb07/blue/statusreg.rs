///Register `STATUSREG` reader
pub type R = crate::R<STATUSREGrs>;
///Field `AESONFLYBUSY` reader - AES on the fligh encryption busy status
pub type AESONFLYBUSY_R = crate::BitReader;
///Field `NOTSUPPORTED_FUNCTION` reader - indicates the SW requests an unsupported feature.
pub type NOTSUPPORTED_FUNCTION_R = crate::BitReader;
///Field `ADDPOINTERROR` reader - Address Pointer Error status
pub type ADDPOINTERROR_R = crate::BitReader;
///Field `RXOVERFLOWERROR` reader - AHB arbiter is full and there is no more storage capability available in RX datapath
pub type RXOVERFLOWERROR_R = crate::BitReader;
///Field `PREVTRANSMIT` reader - Previous event was a Transmission (1) or Reception (0) status
pub type PREVTRANSMIT_R = crate::BitReader;
///Field `SEQDONE` reader - Sequencer end of task status.
pub type SEQDONE_R = crate::BitReader;
///Field `TXERROR_0` reader - Transmission error 0 status: Transmit block missing data error.
pub type TXERROR_0_R = crate::BitReader;
///Field `TXERROR_1` reader - Transmission error 1 status
pub type TXERROR_1_R = crate::BitReader;
///Field `TXERROR_2` reader - Transmission error 2 status.
pub type TXERROR_2_R = crate::BitReader;
///Field `TXERROR_3` reader - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state (timeout defined in GlobalStatMach.
pub type TXERROR_3_R = crate::BitReader;
///Field `TXERROR_4` reader - Transmission error 4 status
pub type TXERROR_4_R = crate::BitReader;
///Field `ENCERROR` reader - Encryption error on receive status
pub type ENCERROR_R = crate::BitReader;
///Field `ALLTABLEREADYERROR` reader - All RAM Table not ready status
pub type ALLTABLEREADYERROR_R = crate::BitReader;
///Field `TXDATAREADYERROR` reader - Transmit data pack not ready status.
pub type TXDATAREADYERROR_R = crate::BitReader;
///Field `NOACTIVELERROR` reader - GlobalStatMach.
pub type NOACTIVELERROR_R = crate::BitReader;
///Field `RCVLENGTHERROR` reader - Receive length error status
pub type RCVLENGTHERROR_R = crate::BitReader;
///Field `SEMATIMEOUTERROR` reader - Semaphore timeout error status
pub type SEMATIMEOUTERROR_R = crate::BitReader;
///Field `TXRXSKIP` reader - Transmission/Reception skip status.
pub type TXRXSKIP_R = crate::BitReader;
///Field `ACTIVE2ERROR` reader - Indicates the Radio FSM was not in ACTIVE2 state when the Sequencer reaches the end of 1st INIT step.
pub type ACTIVE2ERROR_R = crate::BitReader;
///Field `CONFIGERROR` reader - Data pointer configuration error status
pub type CONFIGERROR_R = crate::BitReader;
///Field `TXOK` reader - Previous transmitted packet received OK by the peer device status.
pub type TXOK_R = crate::BitReader;
///Field `DONE` reader - Receive/Transmit done status.
pub type DONE_R = crate::BitReader;
///Field `RCVTIMEOUT` reader - Receive timeout status (no access address found)
pub type RCVTIMEOUT_R = crate::BitReader;
///Field `RCVNOMD` reader - Received MD bit status (valid only on Data Physical Channel PDU reception)
pub type RCVNOMD_R = crate::BitReader;
///Field `RCVCMD` reader - Received command status (valid only on Data Physical Channel PDU reception).
pub type RCVCMD_R = crate::BitReader;
///Field `TIMECAPTURETRIG` reader - indicates a time has been captured in TIMERCAPTUREREG when set.
pub type TIMECAPTURETRIG_R = crate::BitReader;
///Field `RCVCRCERR` reader - Receive data fail (CRC error or invalid CI field) status.
pub type RCVCRCERR_R = crate::BitReader;
///Field `RCVOK` reader - Receive data OK status
pub type RCVOK_R = crate::BitReader;
impl R {
    ///Bit 0 - AES on the fligh encryption busy status
    #[inline(always)]
    pub fn aesonflybusy(&self) -> AESONFLYBUSY_R {
        AESONFLYBUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - indicates the SW requests an unsupported feature.
    #[inline(always)]
    pub fn notsupported_function(&self) -> NOTSUPPORTED_FUNCTION_R {
        NOTSUPPORTED_FUNCTION_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Address Pointer Error status
    #[inline(always)]
    pub fn addpointerror(&self) -> ADDPOINTERROR_R {
        ADDPOINTERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHB arbiter is full and there is no more storage capability available in RX datapath
    #[inline(always)]
    pub fn rxoverflowerror(&self) -> RXOVERFLOWERROR_R {
        RXOVERFLOWERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Previous event was a Transmission (1) or Reception (0) status
    #[inline(always)]
    pub fn prevtransmit(&self) -> PREVTRANSMIT_R {
        PREVTRANSMIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Sequencer end of task status.
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission error 0 status: Transmit block missing data error.
    #[inline(always)]
    pub fn txerror_0(&self) -> TXERROR_0_R {
        TXERROR_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission error 1 status
    #[inline(always)]
    pub fn txerror_1(&self) -> TXERROR_1_R {
        TXERROR_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission error 2 status.
    #[inline(always)]
    pub fn txerror_2(&self) -> TXERROR_2_R {
        TXERROR_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state (timeout defined in GlobalStatMach.
    #[inline(always)]
    pub fn txerror_3(&self) -> TXERROR_3_R {
        TXERROR_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmission error 4 status
    #[inline(always)]
    pub fn txerror_4(&self) -> TXERROR_4_R {
        TXERROR_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Encryption error on receive status
    #[inline(always)]
    pub fn encerror(&self) -> ENCERROR_R {
        ENCERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - All RAM Table not ready status
    #[inline(always)]
    pub fn alltablereadyerror(&self) -> ALLTABLEREADYERROR_R {
        ALLTABLEREADYERROR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmit data pack not ready status.
    #[inline(always)]
    pub fn txdatareadyerror(&self) -> TXDATAREADYERROR_R {
        TXDATAREADYERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GlobalStatMach.
    #[inline(always)]
    pub fn noactivelerror(&self) -> NOACTIVELERROR_R {
        NOACTIVELERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Receive length error status
    #[inline(always)]
    pub fn rcvlengtherror(&self) -> RCVLENGTHERROR_R {
        RCVLENGTHERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Semaphore timeout error status
    #[inline(always)]
    pub fn sematimeouterror(&self) -> SEMATIMEOUTERROR_R {
        SEMATIMEOUTERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Transmission/Reception skip status.
    #[inline(always)]
    pub fn txrxskip(&self) -> TXRXSKIP_R {
        TXRXSKIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Indicates the Radio FSM was not in ACTIVE2 state when the Sequencer reaches the end of 1st INIT step.
    #[inline(always)]
    pub fn active2error(&self) -> ACTIVE2ERROR_R {
        ACTIVE2ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Data pointer configuration error status
    #[inline(always)]
    pub fn configerror(&self) -> CONFIGERROR_R {
        CONFIGERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Previous transmitted packet received OK by the peer device status.
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive/Transmit done status.
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive timeout status (no access address found)
    #[inline(always)]
    pub fn rcvtimeout(&self) -> RCVTIMEOUT_R {
        RCVTIMEOUT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Received MD bit status (valid only on Data Physical Channel PDU reception)
    #[inline(always)]
    pub fn rcvnomd(&self) -> RCVNOMD_R {
        RCVNOMD_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Received command status (valid only on Data Physical Channel PDU reception).
    #[inline(always)]
    pub fn rcvcmd(&self) -> RCVCMD_R {
        RCVCMD_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - indicates a time has been captured in TIMERCAPTUREREG when set.
    #[inline(always)]
    pub fn timecapturetrig(&self) -> TIMECAPTURETRIG_R {
        TIMECAPTURETRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Receive data fail (CRC error or invalid CI field) status.
    #[inline(always)]
    pub fn rcvcrcerr(&self) -> RCVCRCERR_R {
        RCVCRCERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Receive data OK status
    #[inline(always)]
    pub fn rcvok(&self) -> RCVOK_R {
        RCVOK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUSREG")
            .field("aesonflybusy", &self.aesonflybusy())
            .field("notsupported_function", &self.notsupported_function())
            .field("addpointerror", &self.addpointerror())
            .field("rxoverflowerror", &self.rxoverflowerror())
            .field("prevtransmit", &self.prevtransmit())
            .field("seqdone", &self.seqdone())
            .field("txerror_0", &self.txerror_0())
            .field("txerror_1", &self.txerror_1())
            .field("txerror_2", &self.txerror_2())
            .field("txerror_3", &self.txerror_3())
            .field("txerror_4", &self.txerror_4())
            .field("encerror", &self.encerror())
            .field("alltablereadyerror", &self.alltablereadyerror())
            .field("txdatareadyerror", &self.txdatareadyerror())
            .field("noactivelerror", &self.noactivelerror())
            .field("rcvlengtherror", &self.rcvlengtherror())
            .field("sematimeouterror", &self.sematimeouterror())
            .field("txrxskip", &self.txrxskip())
            .field("active2error", &self.active2error())
            .field("configerror", &self.configerror())
            .field("txok", &self.txok())
            .field("done", &self.done())
            .field("rcvtimeout", &self.rcvtimeout())
            .field("rcvnomd", &self.rcvnomd())
            .field("rcvcmd", &self.rcvcmd())
            .field("timecapturetrig", &self.timecapturetrig())
            .field("rcvcrcerr", &self.rcvcrcerr())
            .field("rcvok", &self.rcvok())
            .finish()
    }
}
/**STATUSREG register

You can [`read`](crate::Reg::read) this register and get [`statusreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:STATUSREG)*/
pub struct STATUSREGrs;
impl crate::RegisterSpec for STATUSREGrs {
    type Ux = u32;
}
///`read()` method returns [`statusreg::R`](R) reader structure
impl crate::Readable for STATUSREGrs {}
///`reset()` method sets STATUSREG to value 0
impl crate::Resettable for STATUSREGrs {}
