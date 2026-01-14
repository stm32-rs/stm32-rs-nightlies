///Register `P2PPM0AR1` reader
pub type R = crate::R<P2PPM0AR1rs>;
///Register `P2PPM0AR1` writer
pub type W = crate::W<P2PPM0AR1rs>;
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
        f.debug_struct("P2PPM0AR1")
            .field("m0a", &self.m0a())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Memory0 address
    #[inline(always)]
    pub fn m0a(&mut self) -> M0A_W<'_, P2PPM0AR1rs> {
        M0A_W::new(self, 0)
    }
}
/**DCMIPP Pipe2 pixel packer Memory0 address register 1

You can [`read`](crate::Reg::read) this register and get [`p2ppm0ar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2ppm0ar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P2PPM0AR1)*/
pub struct P2PPM0AR1rs;
impl crate::RegisterSpec for P2PPM0AR1rs {
    type Ux = u32;
}
///`read()` method returns [`p2ppm0ar1::R`](R) reader structure
impl crate::Readable for P2PPM0AR1rs {}
///`write(|w| ..)` method takes [`p2ppm0ar1::W`](W) writer structure
impl crate::Writable for P2PPM0AR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2PPM0AR1 to value 0
impl crate::Resettable for P2PPM0AR1rs {}
