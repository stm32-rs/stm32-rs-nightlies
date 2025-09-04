///Register `HTR2` reader
pub type R = crate::R<HTR2rs>;
///Register `HTR2` writer
pub type W = crate::W<HTR2rs>;
///Field `HTR2` reader - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy). Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type HTR2_R = crate::FieldReader<u32>;
///Field `HTR2` writer - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy). Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type HTR2_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32, crate::Safe>;
impl R {
    ///Bits 0:25 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy). Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTR2").field("htr2", &self.htr2()).finish()
    }
}
impl W {
    ///Bits 0:25 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 2. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy). Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn htr2(&mut self) -> HTR2_W<HTR2rs> {
        HTR2_W::new(self, 0)
    }
}
/**ADC watchdog higher threshold register 2

You can [`read`](crate::Reg::read) this register and get [`htr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#ADC1:HTR2)*/
pub struct HTR2rs;
impl crate::RegisterSpec for HTR2rs {
    type Ux = u32;
}
///`read()` method returns [`htr2::R`](R) reader structure
impl crate::Readable for HTR2rs {}
///`write(|w| ..)` method takes [`htr2::W`](W) writer structure
impl crate::Writable for HTR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTR2 to value 0x03ff_ffff
impl crate::Resettable for HTR2rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
