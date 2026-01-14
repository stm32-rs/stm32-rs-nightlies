///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_SLEEP` reader - Allow debug in Sleep mode
pub type DBG_SLEEP_R = crate::BitReader;
///Field `DBG_SLEEP` writer - Allow debug in Sleep mode
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STOP` reader - Allow debug in Stop mode
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - Allow debug in Stop mode
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - Allow debug in Standby mode
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - Allow debug in Standby mode
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGCLKEN` reader - Debug clock enable through software
pub type DBGCLKEN_R = crate::BitReader;
///Field `DBGCLKEN` writer - Debug clock enable through software
pub type DBGCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACECLKEN` reader - TPIU export clock enable through software
pub type TRACECLKEN_R = crate::BitReader;
///Field `TRACECLKEN` writer - TPIU export clock enable through software
pub type TRACECLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBTRGOEN` reader - DBTRGIO connection control
pub type DBTRGOEN_R = crate::BitReader;
///Field `DBTRGOEN` writer - DBTRGIO connection control
pub type DBTRGOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HLT_TSGEN_EN` reader - TSGEN halt enable
pub type HLT_TSGEN_EN_R = crate::BitReader;
///Field `HLT_TSGEN_EN` writer - TSGEN halt enable
pub type HLT_TSGEN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Allow debug in Sleep mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow debug in Stop mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Allow debug in Standby mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 20 - Debug clock enable through software
    #[inline(always)]
    pub fn dbgclken(&self) -> DBGCLKEN_R {
        DBGCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TPIU export clock enable through software
    #[inline(always)]
    pub fn traceclken(&self) -> TRACECLKEN_R {
        TRACECLKEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 28 - DBTRGIO connection control
    #[inline(always)]
    pub fn dbtrgoen(&self) -> DBTRGOEN_R {
        DBTRGOEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 31 - TSGEN halt enable
    #[inline(always)]
    pub fn hlt_tsgen_en(&self) -> HLT_TSGEN_EN_R {
        HLT_TSGEN_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("dbgclken", &self.dbgclken())
            .field("traceclken", &self.traceclken())
            .field("dbtrgoen", &self.dbtrgoen())
            .field("hlt_tsgen_en", &self.hlt_tsgen_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Allow debug in Sleep mode
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<'_, CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    ///Bit 1 - Allow debug in Stop mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<'_, CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - Allow debug in Standby mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<'_, CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    ///Bit 20 - Debug clock enable through software
    #[inline(always)]
    pub fn dbgclken(&mut self) -> DBGCLKEN_W<'_, CRrs> {
        DBGCLKEN_W::new(self, 20)
    }
    ///Bit 21 - TPIU export clock enable through software
    #[inline(always)]
    pub fn traceclken(&mut self) -> TRACECLKEN_W<'_, CRrs> {
        TRACECLKEN_W::new(self, 21)
    }
    ///Bit 28 - DBTRGIO connection control
    #[inline(always)]
    pub fn dbtrgoen(&mut self) -> DBTRGOEN_W<'_, CRrs> {
        DBTRGOEN_W::new(self, 28)
    }
    ///Bit 31 - TSGEN halt enable
    #[inline(always)]
    pub fn hlt_tsgen_en(&mut self) -> HLT_TSGEN_EN_W<'_, CRrs> {
        HLT_TSGEN_EN_W::new(self, 31)
    }
}
/**DBGMCU configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0x8000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
