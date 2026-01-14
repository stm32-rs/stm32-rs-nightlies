///Register `WDGPAR` reader
pub type R = crate::R<WDGPARrs>;
///Register `WDGPAR` writer
pub type W = crate::W<WDGPARrs>;
///Field `PREALARM` reader - pre-alarm value
pub type PREALARM_R = crate::FieldReader<u16>;
///Field `PREALARM` writer - pre-alarm value
pub type PREALARM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - pre-alarm value
    #[inline(always)]
    pub fn prealarm(&self) -> PREALARM_R {
        PREALARM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDGPAR")
            .field("prealarm", &self.prealarm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - pre-alarm value
    #[inline(always)]
    pub fn prealarm(&mut self) -> PREALARM_W<'_, WDGPARrs> {
        PREALARM_W::new(self, 0)
    }
}
/**GFXTIM watchdog pre-alarm register

You can [`read`](crate::Reg::read) this register and get [`wdgpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GFXTIM:WDGPAR)*/
pub struct WDGPARrs;
impl crate::RegisterSpec for WDGPARrs {
    type Ux = u32;
}
///`read()` method returns [`wdgpar::R`](R) reader structure
impl crate::Readable for WDGPARrs {}
///`write(|w| ..)` method takes [`wdgpar::W`](W) writer structure
impl crate::Writable for WDGPARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDGPAR to value 0
impl crate::Resettable for WDGPARrs {}
