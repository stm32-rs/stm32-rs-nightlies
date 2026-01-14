///Register `EWUB` reader
pub type R = crate::R<EWUBrs>;
///Register `EWUB` writer
pub type W = crate::W<EWUBrs>;
///Field `EWUB` reader - EWUB\[x\] Enable WakeUp line PB\[x\] When this bit is set the PB\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PB\[x\] will trigger a CPU wakeup event depending on CR9.WUPB\[x\] bit.
pub type EWUB_R = crate::FieldReader<u16>;
///Field `EWUB` writer - EWUB\[x\] Enable WakeUp line PB\[x\] When this bit is set the PB\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PB\[x\] will trigger a CPU wakeup event depending on CR9.WUPB\[x\] bit.
pub type EWUB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EWUB\[x\] Enable WakeUp line PB\[x\] When this bit is set the PB\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PB\[x\] will trigger a CPU wakeup event depending on CR9.WUPB\[x\] bit.
    #[inline(always)]
    pub fn ewub(&self) -> EWUB_R {
        EWUB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWUB").field("ewub", &self.ewub()).finish()
    }
}
impl W {
    ///Bits 0:15 - EWUB\[x\] Enable WakeUp line PB\[x\] When this bit is set the PB\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PB\[x\] will trigger a CPU wakeup event depending on CR9.WUPB\[x\] bit.
    #[inline(always)]
    pub fn ewub(&mut self) -> EWUB_W<'_, EWUBrs> {
        EWUB_W::new(self, 0)
    }
}
/**EWUB register

You can [`read`](crate::Reg::read) this register and get [`ewub::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewub::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:EWUB)*/
pub struct EWUBrs;
impl crate::RegisterSpec for EWUBrs {
    type Ux = u32;
}
///`read()` method returns [`ewub::R`](R) reader structure
impl crate::Readable for EWUBrs {}
///`write(|w| ..)` method takes [`ewub::W`](W) writer structure
impl crate::Writable for EWUBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EWUB to value 0
impl crate::Resettable for EWUBrs {}
