#[doc = "Register `SYSCFG_CFGR2` reader"]
pub type R = crate::R<SYSCFG_CFGR2rs>;
#[doc = "Register `SYSCFG_CFGR2` writer"]
pub type W = crate::W<SYSCFG_CFGR2rs>;
#[doc = "Field `LOCKUP_LOCK` reader - Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
pub type LOCKUP_LOCK_R = crate::BitReader;
#[doc = "Field `LOCKUP_LOCK` writer - Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
pub type LOCKUP_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
    #[inline(always)]
    pub fn lockup_lock(&self) -> LOCKUP_LOCK_R {
        LOCKUP_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP enable This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex&lt;Superscript>�&lt;Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input."]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lock(&mut self) -> LOCKUP_LOCK_W<SYSCFG_CFGR2rs> {
        LOCKUP_LOCK_W::new(self, 0)
    }
}
#[doc = "SYSCFG configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_CFGR2rs;
impl crate::RegisterSpec for SYSCFG_CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_cfgr2::R`](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`syscfg_cfgr2::W`](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR2 to value 0"]
impl crate::Resettable for SYSCFG_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
