///Register `AWD2CR` reader
pub type R = crate::R<AWD2CRrs>;
///Register `AWD2CR` writer
pub type W = crate::W<AWD2CRrs>;
///Field `AWD2CH` reader - Analog watchdog 2 channel selection
pub type AWD2CH_R = crate::FieldReader<u32>;
///Field `AWD2CH` writer - Analog watchdog 2 channel selection
pub type AWD2CH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch(&self) -> AWD2CH_R {
        AWD2CH_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD2CR")
            .field("awd2ch", &self.awd2ch())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Analog watchdog 2 channel selection
    #[inline(always)]
    pub fn awd2ch(&mut self) -> AWD2CH_W<'_, AWD2CRrs> {
        AWD2CH_W::new(self, 0)
    }
}
/**ADC Analog Watchdog 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:AWD2CR)*/
pub struct AWD2CRrs;
impl crate::RegisterSpec for AWD2CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd2cr::R`](R) reader structure
impl crate::Readable for AWD2CRrs {}
///`write(|w| ..)` method takes [`awd2cr::W`](W) writer structure
impl crate::Writable for AWD2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD2CR to value 0
impl crate::Resettable for AWD2CRrs {}
