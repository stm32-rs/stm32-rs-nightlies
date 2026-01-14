///Register `INTERRUPT1REG` reader
pub type R = crate::R<INTERRUPT1REGrs>;
///Register `INTERRUPT1REG` writer
pub type W = crate::W<INTERRUPT1REGrs>;
///Field `ADDPOINTERROR` reader - Address Pointer Error.
pub type ADDPOINTERROR_R = crate::BitReader;
///Field `ADDPOINTERROR` writer - Address Pointer Error.
pub type ADDPOINTERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVERFLOWERROR` reader - Receive Overflow error.
pub type RXOVERFLOWERROR_R = crate::BitReader;
///Field `RXOVERFLOWERROR` writer - Receive Overflow error.
pub type RXOVERFLOWERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEQDONE` reader - Sequencer end of task.
pub type SEQDONE_R = crate::BitReader;
///Field `SEQDONE` writer - Sequencer end of task.
pub type SEQDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERROR_0` reader - Transmission error 0: transmit block missing data error.
pub type TXERROR_0_R = crate::BitReader;
///Field `TXERROR_0` writer - Transmission error 0: transmit block missing data error.
pub type TXERROR_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERROR_1` reader - Transmission error 1: a TX skip happened during an on-going transmission.
pub type TXERROR_1_R = crate::BitReader;
///Field `TXERROR_1` writer - Transmission error 1: a TX skip happened during an on-going transmission.
pub type TXERROR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERROR_2` reader - Transmission error 2: channel index is greater than 39.
pub type TXERROR_2_R = crate::BitReader;
///Field `TXERROR_2` writer - Transmission error 2: channel index is greater than 39.
pub type TXERROR_2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERROR_3` reader - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state.
pub type TXERROR_3_R = crate::BitReader;
///Field `TXERROR_3` writer - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state.
pub type TXERROR_3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERROR_4` reader - Transmission error 4: a CTE issue occurred.
pub type TXERROR_4_R = crate::BitReader;
///Field `TXERROR_4` writer - Transmission error 4: a CTE issue occurred.
pub type TXERROR_4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENCERROR` reader - Encryption error on reception.
pub type ENCERROR_R = crate::BitReader;
///Field `ENCERROR` writer - Encryption error on reception.
pub type ENCERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALLTABLEREADYERROR` reader - All RAM Table not ready on time.
pub type ALLTABLEREADYERROR_R = crate::BitReader;
///Field `ALLTABLEREADYERROR` writer - All RAM Table not ready on time.
pub type ALLTABLEREADYERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDATAREADYERROR` reader - Transmit data pack not ready error
pub type TXDATAREADYERROR_R = crate::BitReader;
///Field `TXDATAREADYERROR` writer - Transmit data pack not ready error
pub type TXDATAREADYERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOACTIVELERROR` reader - GlobalStatMach.
pub type NOACTIVELERROR_R = crate::BitReader;
///Field `NOACTIVELERROR` writer - GlobalStatMach.
pub type NOACTIVELERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVLENGTHERROR` reader - Receive length error.
pub type RCVLENGTHERROR_R = crate::BitReader;
///Field `RCVLENGTHERROR` writer - Receive length error.
pub type RCVLENGTHERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEMATIMEOUTERROR` reader - Semaphore timeout error
pub type SEMATIMEOUTERROR_R = crate::BitReader;
///Field `SEMATIMEOUTERROR` writer - Semaphore timeout error
pub type SEMATIMEOUTERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXRXSKIP` reader - Transmission/Reception skip.
pub type TXRXSKIP_R = crate::BitReader;
///Field `TXRXSKIP` writer - Transmission/Reception skip.
pub type TXRXSKIP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACTIVE2ERROR` reader - Active2 Radio state error.
pub type ACTIVE2ERROR_R = crate::BitReader;
///Field `ACTIVE2ERROR` writer - Active2 Radio state error.
pub type ACTIVE2ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONFIGERROR` reader - Data pointer configuration error.
pub type CONFIGERROR_R = crate::BitReader;
///Field `CONFIGERROR` writer - Data pointer configuration error.
pub type CONFIGERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXOK` reader - Previous transmitted packet received OK by the peer device.
pub type TXOK_R = crate::BitReader;
///Field `TXOK` writer - Previous transmitted packet received OK by the peer device.
pub type TXOK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DONE` reader - Receive/Transmit done.
pub type DONE_R = crate::BitReader;
///Field `DONE` writer - Receive/Transmit done.
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVTIMEOUT` reader - Receive timeout (no preamble found).
pub type RCVTIMEOUT_R = crate::BitReader;
///Field `RCVTIMEOUT` writer - Receive timeout (no preamble found).
pub type RCVTIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVNOMD` reader - Received low MD bit.
pub type RCVNOMD_R = crate::BitReader;
///Field `RCVNOMD` writer - Received low MD bit.
pub type RCVNOMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVCMD` reader - Received command
pub type RCVCMD_R = crate::BitReader;
///Field `RCVCMD` writer - Received command
pub type RCVCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMECAPTURETRIG` reader - A time has been captured in TIMERCAPTUREREG.
pub type TIMECAPTURETRIG_R = crate::BitReader;
///Field `TIMECAPTURETRIG` writer - A time has been captured in TIMERCAPTUREREG.
pub type TIMECAPTURETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVCRCERR` reader - Receive data fail
pub type RCVCRCERR_R = crate::BitReader;
///Field `RCVCRCERR` writer - Receive data fail
pub type RCVCRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RCVOK` reader - Receive data OK.
pub type RCVOK_R = crate::BitReader;
///Field `RCVOK` writer - Receive data OK.
pub type RCVOK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - Address Pointer Error.
    #[inline(always)]
    pub fn addpointerror(&self) -> ADDPOINTERROR_R {
        ADDPOINTERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive Overflow error.
    #[inline(always)]
    pub fn rxoverflowerror(&self) -> RXOVERFLOWERROR_R {
        RXOVERFLOWERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Sequencer end of task.
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission error 0: transmit block missing data error.
    #[inline(always)]
    pub fn txerror_0(&self) -> TXERROR_0_R {
        TXERROR_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transmission error 1: a TX skip happened during an on-going transmission.
    #[inline(always)]
    pub fn txerror_1(&self) -> TXERROR_1_R {
        TXERROR_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transmission error 2: channel index is greater than 39.
    #[inline(always)]
    pub fn txerror_2(&self) -> TXERROR_2_R {
        TXERROR_2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state.
    #[inline(always)]
    pub fn txerror_3(&self) -> TXERROR_3_R {
        TXERROR_3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmission error 4: a CTE issue occurred.
    #[inline(always)]
    pub fn txerror_4(&self) -> TXERROR_4_R {
        TXERROR_4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Encryption error on reception.
    #[inline(always)]
    pub fn encerror(&self) -> ENCERROR_R {
        ENCERROR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - All RAM Table not ready on time.
    #[inline(always)]
    pub fn alltablereadyerror(&self) -> ALLTABLEREADYERROR_R {
        ALLTABLEREADYERROR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Transmit data pack not ready error
    #[inline(always)]
    pub fn txdatareadyerror(&self) -> TXDATAREADYERROR_R {
        TXDATAREADYERROR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GlobalStatMach.
    #[inline(always)]
    pub fn noactivelerror(&self) -> NOACTIVELERROR_R {
        NOACTIVELERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Receive length error.
    #[inline(always)]
    pub fn rcvlengtherror(&self) -> RCVLENGTHERROR_R {
        RCVLENGTHERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Semaphore timeout error
    #[inline(always)]
    pub fn sematimeouterror(&self) -> SEMATIMEOUTERROR_R {
        SEMATIMEOUTERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Transmission/Reception skip.
    #[inline(always)]
    pub fn txrxskip(&self) -> TXRXSKIP_R {
        TXRXSKIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Active2 Radio state error.
    #[inline(always)]
    pub fn active2error(&self) -> ACTIVE2ERROR_R {
        ACTIVE2ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Data pointer configuration error.
    #[inline(always)]
    pub fn configerror(&self) -> CONFIGERROR_R {
        CONFIGERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Previous transmitted packet received OK by the peer device.
    #[inline(always)]
    pub fn txok(&self) -> TXOK_R {
        TXOK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Receive/Transmit done.
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Receive timeout (no preamble found).
    #[inline(always)]
    pub fn rcvtimeout(&self) -> RCVTIMEOUT_R {
        RCVTIMEOUT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Received low MD bit.
    #[inline(always)]
    pub fn rcvnomd(&self) -> RCVNOMD_R {
        RCVNOMD_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Received command
    #[inline(always)]
    pub fn rcvcmd(&self) -> RCVCMD_R {
        RCVCMD_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - A time has been captured in TIMERCAPTUREREG.
    #[inline(always)]
    pub fn timecapturetrig(&self) -> TIMECAPTURETRIG_R {
        TIMECAPTURETRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Receive data fail
    #[inline(always)]
    pub fn rcvcrcerr(&self) -> RCVCRCERR_R {
        RCVCRCERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Receive data OK.
    #[inline(always)]
    pub fn rcvok(&self) -> RCVOK_R {
        RCVOK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT1REG")
            .field("addpointerror", &self.addpointerror())
            .field("rxoverflowerror", &self.rxoverflowerror())
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
impl W {
    ///Bit 4 - Address Pointer Error.
    #[inline(always)]
    pub fn addpointerror(&mut self) -> ADDPOINTERROR_W<'_, INTERRUPT1REGrs> {
        ADDPOINTERROR_W::new(self, 4)
    }
    ///Bit 5 - Receive Overflow error.
    #[inline(always)]
    pub fn rxoverflowerror(&mut self) -> RXOVERFLOWERROR_W<'_, INTERRUPT1REGrs> {
        RXOVERFLOWERROR_W::new(self, 5)
    }
    ///Bit 7 - Sequencer end of task.
    #[inline(always)]
    pub fn seqdone(&mut self) -> SEQDONE_W<'_, INTERRUPT1REGrs> {
        SEQDONE_W::new(self, 7)
    }
    ///Bit 8 - Transmission error 0: transmit block missing data error.
    #[inline(always)]
    pub fn txerror_0(&mut self) -> TXERROR_0_W<'_, INTERRUPT1REGrs> {
        TXERROR_0_W::new(self, 8)
    }
    ///Bit 9 - Transmission error 1: a TX skip happened during an on-going transmission.
    #[inline(always)]
    pub fn txerror_1(&mut self) -> TXERROR_1_W<'_, INTERRUPT1REGrs> {
        TXERROR_1_W::new(self, 9)
    }
    ///Bit 10 - Transmission error 2: channel index is greater than 39.
    #[inline(always)]
    pub fn txerror_2(&mut self) -> TXERROR_2_W<'_, INTERRUPT1REGrs> {
        TXERROR_2_W::new(self, 10)
    }
    ///Bit 11 - Transmission error 3: error while waiting for the confirmation the Radio FSM is in TX state.
    #[inline(always)]
    pub fn txerror_3(&mut self) -> TXERROR_3_W<'_, INTERRUPT1REGrs> {
        TXERROR_3_W::new(self, 11)
    }
    ///Bit 12 - Transmission error 4: a CTE issue occurred.
    #[inline(always)]
    pub fn txerror_4(&mut self) -> TXERROR_4_W<'_, INTERRUPT1REGrs> {
        TXERROR_4_W::new(self, 12)
    }
    ///Bit 13 - Encryption error on reception.
    #[inline(always)]
    pub fn encerror(&mut self) -> ENCERROR_W<'_, INTERRUPT1REGrs> {
        ENCERROR_W::new(self, 13)
    }
    ///Bit 14 - All RAM Table not ready on time.
    #[inline(always)]
    pub fn alltablereadyerror(&mut self) -> ALLTABLEREADYERROR_W<'_, INTERRUPT1REGrs> {
        ALLTABLEREADYERROR_W::new(self, 14)
    }
    ///Bit 15 - Transmit data pack not ready error
    #[inline(always)]
    pub fn txdatareadyerror(&mut self) -> TXDATAREADYERROR_W<'_, INTERRUPT1REGrs> {
        TXDATAREADYERROR_W::new(self, 15)
    }
    ///Bit 16 - GlobalStatMach.
    #[inline(always)]
    pub fn noactivelerror(&mut self) -> NOACTIVELERROR_W<'_, INTERRUPT1REGrs> {
        NOACTIVELERROR_W::new(self, 16)
    }
    ///Bit 18 - Receive length error.
    #[inline(always)]
    pub fn rcvlengtherror(&mut self) -> RCVLENGTHERROR_W<'_, INTERRUPT1REGrs> {
        RCVLENGTHERROR_W::new(self, 18)
    }
    ///Bit 19 - Semaphore timeout error
    #[inline(always)]
    pub fn sematimeouterror(&mut self) -> SEMATIMEOUTERROR_W<'_, INTERRUPT1REGrs> {
        SEMATIMEOUTERROR_W::new(self, 19)
    }
    ///Bit 21 - Transmission/Reception skip.
    #[inline(always)]
    pub fn txrxskip(&mut self) -> TXRXSKIP_W<'_, INTERRUPT1REGrs> {
        TXRXSKIP_W::new(self, 21)
    }
    ///Bit 22 - Active2 Radio state error.
    #[inline(always)]
    pub fn active2error(&mut self) -> ACTIVE2ERROR_W<'_, INTERRUPT1REGrs> {
        ACTIVE2ERROR_W::new(self, 22)
    }
    ///Bit 23 - Data pointer configuration error.
    #[inline(always)]
    pub fn configerror(&mut self) -> CONFIGERROR_W<'_, INTERRUPT1REGrs> {
        CONFIGERROR_W::new(self, 23)
    }
    ///Bit 24 - Previous transmitted packet received OK by the peer device.
    #[inline(always)]
    pub fn txok(&mut self) -> TXOK_W<'_, INTERRUPT1REGrs> {
        TXOK_W::new(self, 24)
    }
    ///Bit 25 - Receive/Transmit done.
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W<'_, INTERRUPT1REGrs> {
        DONE_W::new(self, 25)
    }
    ///Bit 26 - Receive timeout (no preamble found).
    #[inline(always)]
    pub fn rcvtimeout(&mut self) -> RCVTIMEOUT_W<'_, INTERRUPT1REGrs> {
        RCVTIMEOUT_W::new(self, 26)
    }
    ///Bit 27 - Received low MD bit.
    #[inline(always)]
    pub fn rcvnomd(&mut self) -> RCVNOMD_W<'_, INTERRUPT1REGrs> {
        RCVNOMD_W::new(self, 27)
    }
    ///Bit 28 - Received command
    #[inline(always)]
    pub fn rcvcmd(&mut self) -> RCVCMD_W<'_, INTERRUPT1REGrs> {
        RCVCMD_W::new(self, 28)
    }
    ///Bit 29 - A time has been captured in TIMERCAPTUREREG.
    #[inline(always)]
    pub fn timecapturetrig(&mut self) -> TIMECAPTURETRIG_W<'_, INTERRUPT1REGrs> {
        TIMECAPTURETRIG_W::new(self, 29)
    }
    ///Bit 30 - Receive data fail
    #[inline(always)]
    pub fn rcvcrcerr(&mut self) -> RCVCRCERR_W<'_, INTERRUPT1REGrs> {
        RCVCRCERR_W::new(self, 30)
    }
    ///Bit 31 - Receive data OK.
    #[inline(always)]
    pub fn rcvok(&mut self) -> RCVOK_W<'_, INTERRUPT1REGrs> {
        RCVOK_W::new(self, 31)
    }
}
/**INTERRUPT1REG register

You can [`read`](crate::Reg::read) this register and get [`interrupt1reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt1reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#BLUE:INTERRUPT1REG)*/
pub struct INTERRUPT1REGrs;
impl crate::RegisterSpec for INTERRUPT1REGrs {
    type Ux = u32;
}
///`read()` method returns [`interrupt1reg::R`](R) reader structure
impl crate::Readable for INTERRUPT1REGrs {}
///`write(|w| ..)` method takes [`interrupt1reg::W`](W) writer structure
impl crate::Writable for INTERRUPT1REGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INTERRUPT1REG to value 0
impl crate::Resettable for INTERRUPT1REGrs {}
