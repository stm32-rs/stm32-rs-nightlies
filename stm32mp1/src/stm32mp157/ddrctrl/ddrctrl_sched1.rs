///Register `DDRCTRL_SCHED1` reader
pub type R = crate::R<DDRCTRL_SCHED1rs>;
///Register `DDRCTRL_SCHED1` writer
pub type W = crate::W<DDRCTRL_SCHED1rs>;
///Field `PAGECLOSE_TIMER` reader - PAGECLOSE_TIMER
pub type PAGECLOSE_TIMER_R = crate::FieldReader;
///Field `PAGECLOSE_TIMER` writer - PAGECLOSE_TIMER
pub type PAGECLOSE_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PAGECLOSE_TIMER
    #[inline(always)]
    pub fn pageclose_timer(&self) -> PAGECLOSE_TIMER_R {
        PAGECLOSE_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDRCTRL_SCHED1")
            .field("pageclose_timer", &self.pageclose_timer())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - PAGECLOSE_TIMER
    #[inline(always)]
    #[must_use]
    pub fn pageclose_timer(&mut self) -> PAGECLOSE_TIMER_W<DDRCTRL_SCHED1rs> {
        PAGECLOSE_TIMER_W::new(self, 0)
    }
}
/**DDRCTRL scheduler control register 1

You can [`read`](crate::Reg::read) this register and get [`ddrctrl_sched1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddrctrl_sched1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRCTRL:DDRCTRL_SCHED1)*/
pub struct DDRCTRL_SCHED1rs;
impl crate::RegisterSpec for DDRCTRL_SCHED1rs {
    type Ux = u32;
}
///`read()` method returns [`ddrctrl_sched1::R`](R) reader structure
impl crate::Readable for DDRCTRL_SCHED1rs {}
///`write(|w| ..)` method takes [`ddrctrl_sched1::W`](W) writer structure
impl crate::Writable for DDRCTRL_SCHED1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DDRCTRL_SCHED1 to value 0
impl crate::Resettable for DDRCTRL_SCHED1rs {
    const RESET_VALUE: u32 = 0;
}
