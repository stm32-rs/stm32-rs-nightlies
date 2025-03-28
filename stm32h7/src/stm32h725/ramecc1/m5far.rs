///Register `M5FAR` reader
pub type R = crate::R<M5FARrs>;
///Register `M5FAR` writer
pub type W = crate::W<M5FARrs>;
///Field `FEC` reader - Failing error code
pub type FEC_R = crate::FieldReader<u32>;
///Field `FEC` writer - Failing error code
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FAR").field("fec", &self.fec()).finish()
    }
}
impl W {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<M5FARrs> {
        FEC_W::new(self, 0)
    }
}
/**RAMECC monitor x failing address register

You can [`read`](crate::Reg::read) this register and get [`m5far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#RAMECC1:M5FAR)*/
pub struct M5FARrs;
impl crate::RegisterSpec for M5FARrs {
    type Ux = u32;
}
///`read()` method returns [`m5far::R`](R) reader structure
impl crate::Readable for M5FARrs {}
///`write(|w| ..)` method takes [`m5far::W`](W) writer structure
impl crate::Writable for M5FARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5FAR to value 0
impl crate::Resettable for M5FARrs {}
