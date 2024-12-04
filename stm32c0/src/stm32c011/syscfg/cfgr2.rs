///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `LOCKUP_LOCK` reader - Cortex<Superscript><Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript><Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
pub type LOCKUP_LOCK_R = crate::BitReader;
///Field `LOCKUP_LOCK` writer - Cortex<Superscript><Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript><Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Cortex<Superscript><Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript><Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("lockup_lock", &self.lockup_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cortex<Superscript><Default Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript><Default Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.
    #[inline(always)]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
}
/**SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
