///Register `ICR` reader
pub type R = crate::R<ICRrs>;
///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCRCFAILCW {
    ///1: Clear flag
    Clear = 1,
}
impl From<CCRCFAILCW> for bool {
    #[inline(always)]
    fn from(variant: CCRCFAILCW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCRCFAILC` reader - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_R = crate::BitReader<CCRCFAILCW>;
impl CCRCFAILC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CCRCFAILCW> {
        match self.bits {
            true => Some(CCRCFAILCW::Clear),
            _ => None,
        }
    }
    ///Clear flag
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCRCFAILCW::Clear
    }
}
///Field `CCRCFAILC` writer - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG, CCRCFAILCW>;
impl<'a, REG> CCRCFAILC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCRCFAILCW::Clear)
    }
}
///Field `DCRCFAILC` reader - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub use CCRCFAILC_R as DCRCFAILC_R;
///Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub use CCRCFAILC_R as CTIMEOUTC_R;
///Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub use CCRCFAILC_R as DTIMEOUTC_R;
///Field `TXUNDERRC` reader - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub use CCRCFAILC_R as TXUNDERRC_R;
///Field `RXOVERRC` reader - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub use CCRCFAILC_R as RXOVERRC_R;
///Field `CMDRENDC` reader - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub use CCRCFAILC_R as CMDRENDC_R;
///Field `CMDSENTC` reader - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub use CCRCFAILC_R as CMDSENTC_R;
///Field `DATAENDC` reader - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub use CCRCFAILC_R as DATAENDC_R;
///Field `DBCKENDC` reader - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub use CCRCFAILC_R as DBCKENDC_R;
///Field `SDIOITC` reader - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub use CCRCFAILC_R as SDIOITC_R;
///Field `IDMATEC` reader - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub use CCRCFAILC_R as IDMATEC_R;
///Field `DCRCFAILC` writer - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub use CCRCFAILC_W as DCRCFAILC_W;
///Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub use CCRCFAILC_W as CTIMEOUTC_W;
///Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub use CCRCFAILC_W as DTIMEOUTC_W;
///Field `TXUNDERRC` writer - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub use CCRCFAILC_W as TXUNDERRC_W;
///Field `RXOVERRC` writer - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub use CCRCFAILC_W as RXOVERRC_W;
///Field `CMDRENDC` writer - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub use CCRCFAILC_W as CMDRENDC_W;
///Field `CMDSENTC` writer - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub use CCRCFAILC_W as CMDSENTC_W;
///Field `DATAENDC` writer - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub use CCRCFAILC_W as DATAENDC_W;
///Field `DBCKENDC` writer - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub use CCRCFAILC_W as DBCKENDC_W;
///Field `SDIOITC` writer - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub use CCRCFAILC_W as SDIOITC_W;
///Field `IDMATEC` writer - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub use CCRCFAILC_W as IDMATEC_W;
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
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
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
            .field("dbckendc", &self.dbckendc())
            .field("sdioitc", &self.sdioitc())
            .field("idmatec", &self.idmatec())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    ///Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    ///Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    ///Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    ///Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    ///Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    ///Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag.
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    ///Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    ///Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag.
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    #[must_use]
    pub fn idmatec(&mut self) -> IDMATEC_W<ICRrs> {
        IDMATEC_W::new(self, 27)
    }
}
/**The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.

You can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#SDIO:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`read()` method returns [`icr::R`](R) reader structure
impl crate::Readable for ICRrs {}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
