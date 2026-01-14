///Register `AWD3HTR` reader
pub type R = crate::R<AWD3HTRrs>;
///Register `AWD3HTR` writer
pub type W = crate::W<AWD3HTRrs>;
///Field `HTR` reader - Analog watchdog 3 higher threshold
pub type HTR_R = crate::FieldReader<u32>;
///Field `HTR` writer - Analog watchdog 3 higher threshold
pub type HTR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    ///Bits 0:22 - Analog watchdog 3 higher threshold
    #[inline(always)]
    pub fn htr(&self) -> HTR_R {
        HTR_R::new(self.bits & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3HTR").field("htr", &self.htr()).finish()
    }
}
impl W {
    ///Bits 0:22 - Analog watchdog 3 higher threshold
    #[inline(always)]
    pub fn htr(&mut self) -> HTR_W<'_, AWD3HTRrs> {
        HTR_W::new(self, 0)
    }
}
/**ADC analog watchdog 3 higher threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3htr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3htr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ADC1:AWD3HTR)*/
pub struct AWD3HTRrs;
impl crate::RegisterSpec for AWD3HTRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3htr::R`](R) reader structure
impl crate::Readable for AWD3HTRrs {}
///`write(|w| ..)` method takes [`awd3htr::W`](W) writer structure
impl crate::Writable for AWD3HTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD3HTR to value 0x003f_ffff
impl crate::Resettable for AWD3HTRrs {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
