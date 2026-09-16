///Register `P0PPM0AR2` reader
pub type R = crate::R<P0PPM0AR2rs>;
///Register `P0PPM0AR2` writer
pub type W = crate::W<P0PPM0AR2rs>;
///Field `M0A` reader - Memory0 address Base address of memory area 0, to whom data are written. It is assumed to be a multiple of 16, hence its bits 3:0 are always at 0x0.
pub type M0A_R = crate::FieldReader<u32>;
///Field `M0A` writer - Memory0 address Base address of memory area 0, to whom data are written. It is assumed to be a multiple of 16, hence its bits 3:0 are always at 0x0.
pub type M0A_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Memory0 address Base address of memory area 0, to whom data are written. It is assumed to be a multiple of 16, hence its bits 3:0 are always at 0x0.
    #[inline(always)]
    pub fn m0a(&self) -> M0A_R {
        M0A_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0PPM0AR2")
            .field("m0a", &self.m0a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Memory0 address Base address of memory area 0, to whom data are written. It is assumed to be a multiple of 16, hence its bits 3:0 are always at 0x0.
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, P0PPM0AR2rs> {
        M0A_W::new(self, 0)
    }
}
/**DCMIPP Pipe0 pixel packer Memory0 address register 2

You can [`read`](crate::Reg::read) this register and get [`p0ppm0ar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0ppm0ar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0PPM0AR2)*/
pub struct P0PPM0AR2rs;
impl crate::RegisterSpec for P0PPM0AR2rs {
    type Ux = u32;
}
///`read()` method returns [`p0ppm0ar2::R`](R) reader structure
impl crate::Readable for P0PPM0AR2rs {}
///`write(|w| ..)` method takes [`p0ppm0ar2::W`](W) writer structure
impl crate::Writable for P0PPM0AR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0PPM0AR2 to value 0
impl crate::Resettable for P0PPM0AR2rs {}
