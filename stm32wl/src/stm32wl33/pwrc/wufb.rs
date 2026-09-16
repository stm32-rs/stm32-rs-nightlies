///Register `WUFB` reader
pub type R = crate::R<WUFBrs>;
///Register `WUFB` writer
pub type W = crate::W<WUFBrs>;
///Field `WUFB` reader - WUFB\[x\] WakeUp Flag PB\[x\] This bit is set when a wakeup is detected on wakeup line PB\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
pub type WUFB_R = crate::FieldReader<u16>;
///Field `WUFB` writer - WUFB\[x\] WakeUp Flag PB\[x\] This bit is set when a wakeup is detected on wakeup line PB\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
pub type WUFB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WUFB\[x\] WakeUp Flag PB\[x\] This bit is set when a wakeup is detected on wakeup line PB\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
    #[inline(always)]
    pub fn wufb(&self) -> WUFB_R {
        WUFB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUFB").field("wufb", &self.wufb()).finish()
    }
}
impl W {
    ///Bits 0:15 - WUFB\[x\] WakeUp Flag PB\[x\] This bit is set when a wakeup is detected on wakeup line PB\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
    #[inline(always)]
    pub fn wufb(&mut self) -> WUFB_W<'_, WUFBrs> {
        WUFB_W::new(self, 0)
    }
}
/**WUFB register

You can [`read`](crate::Reg::read) this register and get [`wufb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUFB)*/
pub struct WUFBrs;
impl crate::RegisterSpec for WUFBrs {
    type Ux = u32;
}
///`read()` method returns [`wufb::R`](R) reader structure
impl crate::Readable for WUFBrs {}
///`write(|w| ..)` method takes [`wufb::W`](W) writer structure
impl crate::Writable for WUFBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUFB to value 0
impl crate::Resettable for WUFBrs {}
