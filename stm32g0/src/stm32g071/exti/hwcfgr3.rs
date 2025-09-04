///Register `HWCFGR3` reader
pub type R = crate::R<HWCFGR3rs>;
///Register `HWCFGR3` writer
pub type W = crate::W<HWCFGR3rs>;
///Field `EVENT_TRG` reader - HW configuration event trigger type
pub type EVENT_TRG_R = crate::FieldReader<u32>;
///Field `EVENT_TRG` writer - HW configuration event trigger type
pub type EVENT_TRG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR3")
            .field("event_trg", &self.event_trg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&mut self) -> EVENT_TRG_W<HWCFGR3rs> {
        EVENT_TRG_W::new(self, 0)
    }
}
/**Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#EXTI:HWCFGR3)*/
pub struct HWCFGR3rs;
impl crate::RegisterSpec for HWCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr3::R`](R) reader structure
impl crate::Readable for HWCFGR3rs {}
///`write(|w| ..)` method takes [`hwcfgr3::W`](W) writer structure
impl crate::Writable for HWCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR3 to value 0
impl crate::Resettable for HWCFGR3rs {}
