///Register `P1PPM2AR1` reader
pub type R = crate::R<P1PPM2AR1rs>;
///Register `P1PPM2AR1` writer
pub type W = crate::W<P1PPM2AR1rs>;
///Field `M2A` reader - Memory 2 address
pub type M2A_R = crate::FieldReader<u32>;
///Field `M2A` writer - Memory 2 address
pub type M2A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory 2 address
    #[inline(always)]
    pub fn m2a(&self) -> M2A_R {
        M2A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1PPM2AR1")
            .field("m2a", &self.m2a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Memory 2 address
    #[inline(always)]
    pub fn m2a(&mut self) -> M2A_W<'_, P1PPM2AR1rs> {
        M2A_W::new(self, 0)
    }
}
/**DCMIPP Pipex pixel packer memory2 address register 1

You can [`read`](crate::Reg::read) this register and get [`p1ppm2ar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm2ar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1PPM2AR1)*/
pub struct P1PPM2AR1rs;
impl crate::RegisterSpec for P1PPM2AR1rs {
    type Ux = u32;
}
///`read()` method returns [`p1ppm2ar1::R`](R) reader structure
impl crate::Readable for P1PPM2AR1rs {}
///`write(|w| ..)` method takes [`p1ppm2ar1::W`](W) writer structure
impl crate::Writable for P1PPM2AR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1PPM2AR1 to value 0
impl crate::Resettable for P1PPM2AR1rs {}
