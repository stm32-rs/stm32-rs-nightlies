///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `CCRCFAILC` reader - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_R = crate::BitReader;
///Field `CCRCFAILC` writer - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCRCFAILC` reader - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub type DCRCFAILC_R = crate::BitReader;
///Field `DCRCFAILC` writer - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub type DCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub type CTIMEOUTC_R = crate::BitReader;
///Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub type CTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub type DTIMEOUTC_R = crate::BitReader;
///Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub type DTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUNDERRC` reader - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub type TXUNDERRC_R = crate::BitReader;
///Field `TXUNDERRC` writer - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub type TXUNDERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVERRC` reader - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub type RXOVERRC_R = crate::BitReader;
///Field `RXOVERRC` writer - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub type RXOVERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDRENDC` reader - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub type CMDRENDC_R = crate::BitReader;
///Field `CMDRENDC` writer - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub type CMDRENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDSENTC` reader - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub type CMDSENTC_R = crate::BitReader;
///Field `CMDSENTC` writer - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub type CMDSENTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAENDC` reader - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub type DATAENDC_R = crate::BitReader;
///Field `DATAENDC` writer - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub type DATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DHOLDC` reader - DHOLD flag clear bit Set by software to clear the DHOLD flag.
pub type DHOLDC_R = crate::BitReader;
///Field `DHOLDC` writer - DHOLD flag clear bit Set by software to clear the DHOLD flag.
pub type DHOLDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBCKENDC` reader - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub type DBCKENDC_R = crate::BitReader;
///Field `DBCKENDC` writer - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub type DBCKENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DABORTC` reader - DABORT flag clear bit Set by software to clear the DABORT flag.
pub type DABORTC_R = crate::BitReader;
///Field `DABORTC` writer - DABORT flag clear bit Set by software to clear the DABORT flag.
pub type DABORTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUSYD0ENDC` reader - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
pub type BUSYD0ENDC_R = crate::BitReader;
///Field `BUSYD0ENDC` writer - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
pub type BUSYD0ENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIOITC` reader - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub type SDIOITC_R = crate::BitReader;
///Field `SDIOITC` writer - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub type SDIOITC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKFAILC` reader - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
pub type ACKFAILC_R = crate::BitReader;
///Field `ACKFAILC` writer - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
pub type ACKFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACKTIMEOUTC` reader - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
pub type ACKTIMEOUTC_R = crate::BitReader;
///Field `ACKTIMEOUTC` writer - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
pub type ACKTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSWENDC` reader - VSWEND flag clear bit Set by software to clear the VSWEND flag.
pub type VSWENDC_R = crate::BitReader;
///Field `VSWENDC` writer - VSWEND flag clear bit Set by software to clear the VSWEND flag.
pub type VSWENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKSTOPC` reader - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
pub type CKSTOPC_R = crate::BitReader;
///Field `CKSTOPC` writer - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
pub type CKSTOPC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDMATEC` reader - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub type IDMATEC_R = crate::BitReader;
///Field `IDMATEC` writer - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub type IDMATEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDMABTCC` reader - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
pub type IDMABTCC_R = crate::BitReader;
///Field `IDMABTCC` writer - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
pub type IDMABTCC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag.
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag.
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag.
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag.
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag.
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("ccrcfailc", &self.ccrcfailc())
            .field("dcrcfailc", &self.dcrcfailc())
            .field("ctimeoutc", &self.ctimeoutc())
            .field("dtimeoutc", &self.dtimeoutc())
            .field("txunderrc", &self.txunderrc())
            .field("rxoverrc", &self.rxoverrc())
            .field("cmdrendc", &self.cmdrendc())
            .field("cmdsentc", &self.cmdsentc())
            .field("dataendc", &self.dataendc())
            .field("dholdc", &self.dholdc())
            .field("dbckendc", &self.dbckendc())
            .field("dabortc", &self.dabortc())
            .field("busyd0endc", &self.busyd0endc())
            .field("sdioitc", &self.sdioitc())
            .field("ackfailc", &self.ackfailc())
            .field("acktimeoutc", &self.acktimeoutc())
            .field("vswendc", &self.vswendc())
            .field("ckstopc", &self.ckstopc())
            .field("idmatec", &self.idmatec())
            .field("idmabtcc", &self.idmabtcc())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<'_, ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    ///Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<'_, ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    ///Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<'_, ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    ///Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<'_, ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    ///Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<'_, ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    ///Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<'_, ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    ///Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag.
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<'_, ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    ///Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<'_, ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    ///Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag.
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W<'_, ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    ///Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag.
    #[inline(always)]
    pub fn dholdc(&mut self) -> DHOLDC_W<'_, ICRrs> {
        DHOLDC_W::new(self, 9)
    }
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<'_, ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    ///Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag.
    #[inline(always)]
    pub fn dabortc(&mut self) -> DABORTC_W<'_, ICRrs> {
        DABORTC_W::new(self, 11)
    }
    ///Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
    #[inline(always)]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W<'_, ICRrs> {
        BUSYD0ENDC_W::new(self, 21)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W<'_, ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    ///Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
    #[inline(always)]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<'_, ICRrs> {
        ACKFAILC_W::new(self, 23)
    }
    ///Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
    #[inline(always)]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W<'_, ICRrs> {
        ACKTIMEOUTC_W::new(self, 24)
    }
    ///Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag.
    #[inline(always)]
    pub fn vswendc(&mut self) -> VSWENDC_W<'_, ICRrs> {
        VSWENDC_W::new(self, 25)
    }
    ///Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
    #[inline(always)]
    pub fn ckstopc(&mut self) -> CKSTOPC_W<'_, ICRrs> {
        CKSTOPC_W::new(self, 26)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    pub fn idmatec(&mut self) -> IDMATEC_W<'_, ICRrs> {
        IDMATEC_W::new(self, 27)
    }
    ///Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
    #[inline(always)]
    pub fn idmabtcc(&mut self) -> IDMABTCC_W<'_, ICRrs> {
        IDMABTCC_W::new(self, 28)
    }
}
/**The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#SDMMC1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
