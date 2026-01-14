///Register `HTR` reader
pub type R = crate::R<HTRrs>;
///Register `HTR` writer
pub type W = crate::W<HTRrs>;
///Field `HT` reader - Analog watchdog higher threshold
pub type HT_R = crate::FieldReader<u16>;
///Field `HT` writer - Analog watchdog higher threshold
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTR").field("ht", &self.ht()).finish()
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<'_, HTRrs> {
        HT_W::new(self, 0)
    }
}
/**watchdog higher threshold register

You can [`read`](crate::Reg::read) this register and get [`htr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F750.html#ADC1:HTR)*/
pub struct HTRrs;
impl crate::RegisterSpec for HTRrs {
    type Ux = u32;
}
///`read()` method returns [`htr::R`](R) reader structure
impl crate::Readable for HTRrs {}
///`write(|w| ..)` method takes [`htr::W`](W) writer structure
impl crate::Writable for HTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTR to value 0x0fff
impl crate::Resettable for HTRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
