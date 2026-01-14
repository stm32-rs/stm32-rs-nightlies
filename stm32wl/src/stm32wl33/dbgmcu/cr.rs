///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_SLEEP` reader - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state.
pub type DBG_SLEEP_R = crate::BitReader;
///Field `DBG_SLEEP` writer - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state.
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STOP` reader - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state.
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state.
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state.
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state.
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - Allow debug of the CPU in SLEEP mode - 0: Normal operation. All clocks will be disabled automatically in SLEEP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during SLEEP mode, allowing full CPU debug capability. On exit from SLEEP mode, the clock settings will be set to the SLEEP mode exit state.
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<'_, CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    ///Bit 1 - Allow debug of the CPU in DEEPSTOP mode - 0: Normal operation. All clocks will be disabled automatically in STOP mode - 1: Automatic clock stop disabled. All active CPU clocks and oscillators will continue to run during STOP mode, allowing full CPU debug capability. On exit from STOP mode, the clock settings will be set to the STOP mode exit state.
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<'_, CRrs> {
        DBG_STOP_W::new(self, 1)
    }
}
/**CR register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#DBGMCU:CR)*/
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
