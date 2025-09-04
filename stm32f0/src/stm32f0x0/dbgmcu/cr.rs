///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_STOP` reader - Debug Stop Mode
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - Debug Stop Mode
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - Debug Standby Mode
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - Debug Standby Mode
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .finish()
    }
}
impl W {
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
}
/**Debug MCU Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x0.html#DBGMCU:CR)*/
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
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
