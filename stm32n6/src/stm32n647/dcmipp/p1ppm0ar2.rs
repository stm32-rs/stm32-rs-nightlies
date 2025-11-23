///Register `P1PPM0AR2` reader
pub type R = crate::R<P1PPM0AR2rs>;
///Register `P1PPM0AR2` writer
pub type W = crate::W<P1PPM0AR2rs>;
///Field `M0A` reader - Memory0 address
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - Memory0 address
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory0 address
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1PPM0AR2")
            .field("m0a", &self.m0a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Memory0 address
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, P1PPM0AR2rs> {
        M0A_W::new(self, 0)
    }
}
/**DCMIPP Pipe1 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p1ppm0ar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm0ar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1PPM0AR2)*/
pub struct P1PPM0AR2rs;
impl crate::RegisterSpec for P1PPM0AR2rs {
    type Ux = u32;
}
///`read()` method returns [`p1ppm0ar2::R`](R) reader structure
impl crate::Readable for P1PPM0AR2rs {}
///`write(|w| ..)` method takes [`p1ppm0ar2::W`](W) writer structure
impl crate::Writable for P1PPM0AR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1PPM0AR2 to value 0
impl crate::Resettable for P1PPM0AR2rs {}
