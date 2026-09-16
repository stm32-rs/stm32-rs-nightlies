///Register `P1PPM1AR1` reader
pub type R = crate::R<P1PPM1AR1rs>;
///Register `P1PPM1AR1` writer
pub type W = crate::W<P1PPM1AR1rs>;
///Field `M1A` reader - Memory1 address
pub type M1A_R = crate::FieldReader<u32>;
///Field `M1A` writer - Memory1 address
pub type M1A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory1 address
    #[inline(always)]
    pub fn m1a(&self) -> M1A_R {
        M1A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1PPM1AR1")
            .field("m1a", &self.m1a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Memory1 address
    #[inline(always)]
    pub fn m1a(&mut self) -> M1A_W<'_, P1PPM1AR1rs> {
        M1A_W::new(self, 0)
    }
}
/**DCMIPP Pipex pixel packer Memory1 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1ppm1ar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm1ar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1PPM1AR1)*/
pub struct P1PPM1AR1rs;
impl crate::RegisterSpec for P1PPM1AR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ppm1ar1::R`](R) reader structure
impl crate::Readable for P1PPM1AR1rs {}
///`write(|w| ..)` method takes [`p1ppm1ar1::W`](W) writer structure
impl crate::Writable for P1PPM1AR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1PPM1AR1 to value 0
impl crate::Resettable for P1PPM1AR1rs {}
