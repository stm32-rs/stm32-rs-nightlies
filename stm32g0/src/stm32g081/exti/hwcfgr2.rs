///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
///Register `HWCFGR2` writer
pub type W = crate::W<HWCFGR2rs>;
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
        f.debug_struct("HWCFGR2")
            .field("event_trg", &self.event_trg())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&mut self) -> EVENT_TRG_W<'_, HWCFGR2rs> {
        EVENT_TRG_W::new(self, 0)
    }
}
/**Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#EXTI:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`write(|w| ..)` method takes [`hwcfgr2::W`](W) writer structure
impl crate::Writable for HWCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HWCFGR2 to value 0x0007_ffff
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x0007_ffff;
}
