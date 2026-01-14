///Register `WUFA` reader
pub type R = crate::R<WUFArs>;
///Register `WUFA` writer
pub type W = crate::W<WUFArs>;
///Field `WUFA` reader - WUFA\[x\] WakeUp Flag PA\[x\] This bit is set when a wakeup is detected on wakeup line PA\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
pub type WUFA_R = crate::FieldReader<u16>;
///Field `WUFA` writer - WUFA\[x\] WakeUp Flag PA\[x\] This bit is set when a wakeup is detected on wakeup line PA\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
pub type WUFA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - WUFA\[x\] WakeUp Flag PA\[x\] This bit is set when a wakeup is detected on wakeup line PA\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
    #[inline(always)]
    pub fn wufa(&self) -> WUFA_R {
        WUFA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WUFA").field("wufa", &self.wufa()).finish()
    }
}
impl W {
    ///Bits 0:15 - WUFA\[x\] WakeUp Flag PA\[x\] This bit is set when a wakeup is detected on wakeup line PA\[x\]. It is cleared by a reset pad or by writing 1 in this bit field. Writing 1 this bit, clears the interrupt:
    #[inline(always)]
    pub fn wufa(&mut self) -> WUFA_W<'_, WUFArs> {
        WUFA_W::new(self, 0)
    }
}
/**WUFA register

You can [`read`](crate::Reg::read) this register and get [`wufa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wufa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:WUFA)*/
pub struct WUFArs;
impl crate::RegisterSpec for WUFArs {
    type Ux = u32;
}
///`read()` method returns [`wufa::R`](R) reader structure
impl crate::Readable for WUFArs {}
///`write(|w| ..)` method takes [`wufa::W`](W) writer structure
impl crate::Writable for WUFArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WUFA to value 0
impl crate::Resettable for WUFArs {}
