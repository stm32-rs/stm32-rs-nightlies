///Register `HTR1` reader
pub type R = crate::R<HTR1rs>;
///Register `HTR1` writer
pub type W = crate::W<HTR1rs>;
///Field `HTR1` reader - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy) Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type HTR1_R = crate::FieldReader<u32>;
///Field `HTR1` writer - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy) Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
pub type HTR1_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32, crate::Safe>;
impl R {
    ///Bits 0:25 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy) Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HTR1").field("htr1", &self.htr1()).finish()
    }
}
impl W {
    ///Bits 0:25 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog 1. Refer to AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy) Note: The software is allowed to write these bits only when ADSTART=0 and JADSTART=0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn htr1(&mut self) -> HTR1_W<'_, HTR1rs> {
        HTR1_W::new(self, 0)
    }
}
/**ADC watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`htr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#ADC1:HTR1)*/
pub struct HTR1rs;
impl crate::RegisterSpec for HTR1rs {
    type Ux = u32;
}
///`read()` method returns [`htr1::R`](R) reader structure
impl crate::Readable for HTR1rs {}
///`write(|w| ..)` method takes [`htr1::W`](W) writer structure
impl crate::Writable for HTR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HTR1 to value 0x03ff_ffff
impl crate::Resettable for HTR1rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
