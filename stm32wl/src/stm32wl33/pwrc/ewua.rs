///Register `EWUA` reader
pub type R = crate::R<EWUArs>;
///Register `EWUA` writer
pub type W = crate::W<EWUArs>;
///Field `EWUA` reader - EWUA\[x\] Enable WakeUp line PA\[x\] When this bit is set the PA\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PA\[x\] will trigger a CPU wakeup event depending on CR7.WUPA\[x\] bit.
pub type EWUA_R = crate::FieldReader<u16>;
///Field `EWUA` writer - EWUA\[x\] Enable WakeUp line PA\[x\] When this bit is set the PA\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PA\[x\] will trigger a CPU wakeup event depending on CR7.WUPA\[x\] bit.
pub type EWUA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EWUA\[x\] Enable WakeUp line PA\[x\] When this bit is set the PA\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PA\[x\] will trigger a CPU wakeup event depending on CR7.WUPA\[x\] bit.
    #[inline(always)]
    pub fn ewua(&self) -> EWUA_R {
        EWUA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWUA").field("ewua", &self.ewua()).finish()
    }
}
impl W {
    ///Bits 0:15 - EWUA\[x\] Enable WakeUp line PA\[x\] When this bit is set the PA\[x\] wakeup line is enabled and a rising or falling edge on wakeup line PA\[x\] will trigger a CPU wakeup event depending on CR7.WUPA\[x\] bit.
    #[inline(always)]
    pub fn ewua(&mut self) -> EWUA_W<'_, EWUArs> {
        EWUA_W::new(self, 0)
    }
}
/**EWUA register

You can [`read`](crate::Reg::read) this register and get [`ewua::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewua::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#PWRC:EWUA)*/
pub struct EWUArs;
impl crate::RegisterSpec for EWUArs {
    type Ux = u32;
}
///`read()` method returns [`ewua::R`](R) reader structure
impl crate::Readable for EWUArs {}
///`write(|w| ..)` method takes [`ewua::W`](W) writer structure
impl crate::Writable for EWUArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EWUA to value 0
impl crate::Resettable for EWUArs {}
