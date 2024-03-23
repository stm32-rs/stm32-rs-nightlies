#[doc = "Register `DDRCTRL_SCHED1` reader"]
pub type R = crate::R<DDRCTRL_SCHED1rs>;
#[doc = "Register `DDRCTRL_SCHED1` writer"]
pub type W = crate::W<DDRCTRL_SCHED1rs>;
#[doc = "Field `PAGECLOSE_TIMER` reader - PAGECLOSE_TIMER"]
pub type PAGECLOSE_TIMER_R = crate::FieldReader;
#[doc = "Field `PAGECLOSE_TIMER` writer - PAGECLOSE_TIMER"]
pub type PAGECLOSE_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    pub fn pageclose_timer(&self) -> PAGECLOSE_TIMER_R {
        PAGECLOSE_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    #[must_use]
    pub fn pageclose_timer(&mut self) -> PAGECLOSE_TIMER_W<DDRCTRL_SCHED1rs> {
        PAGECLOSE_TIMER_W::new(self, 0)
    }
}
#[doc = "DDRCTRL scheduler control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_sched1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_sched1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_SCHED1rs;
impl crate::RegisterSpec for DDRCTRL_SCHED1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_sched1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_SCHED1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_sched1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_SCHED1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_SCHED1 to value 0"]
impl crate::Resettable for DDRCTRL_SCHED1rs {
    const RESET_VALUE: u32 = 0;
}
