///Register `AWD1HTR` reader
pub type R = crate::R<AWD1HTRrs>;
///Register `AWD1HTR` writer
pub type W = crate::W<AWD1HTRrs>;
///Field `HTR` reader - Analog watchdog 1 higher threshold
pub type HTR_R = crate::FieldReader<u32>;
///Field `HTR` writer - Analog watchdog 1 higher threshold
pub type HTR_W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
///Field `AWDFILT` reader - Analog watchdog filtering parameter
pub type AWDFILT_R = crate::FieldReader;
///Field `AWDFILT` writer - Analog watchdog filtering parameter
pub type AWDFILT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:22 - Analog watchdog 1 higher threshold
    #[inline(always)]
    pub fn htr(&self) -> HTR_R {
        HTR_R::new(self.bits & 0x007f_ffff)
    }
    ///Bits 29:31 - Analog watchdog filtering parameter
    #[inline(always)]
    pub fn awdfilt(&self) -> AWDFILT_R {
        AWDFILT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD1HTR")
            .field("htr", &self.htr())
            .field("awdfilt", &self.awdfilt())
            .finish()
    }
}
impl W {
    ///Bits 0:22 - Analog watchdog 1 higher threshold
    #[inline(always)]
    pub fn htr(&mut self) -> HTR_W<'_, AWD1HTRrs> {
        HTR_W::new(self, 0)
    }
    ///Bits 29:31 - Analog watchdog filtering parameter
    #[inline(always)]
    pub fn awdfilt(&mut self) -> AWDFILT_W<'_, AWD1HTRrs> {
        AWDFILT_W::new(self, 29)
    }
}
/**ADC analog watchdog 1 higher threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1htr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1htr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ADC1:AWD1HTR)*/
pub struct AWD1HTRrs;
impl crate::RegisterSpec for AWD1HTRrs {
    type Ux = u32;
}
///`read()` method returns [`awd1htr::R`](R) reader structure
impl crate::Readable for AWD1HTRrs {}
///`write(|w| ..)` method takes [`awd1htr::W`](W) writer structure
impl crate::Writable for AWD1HTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWD1HTR to value 0x003f_ffff
impl crate::Resettable for AWD1HTRrs {
    const RESET_VALUE: u32 = 0x003f_ffff;
}
