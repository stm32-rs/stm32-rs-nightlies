///Register `M1FECR` reader
pub type R = crate::R<M1FECRrs>;
///Register `M1FECR` writer
pub type W = crate::W<M1FECRrs>;
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
        f.debug_struct("M1FECR").field("fec", &self.fec()).finish()
    }
}
impl W {
    ///Bits 0:31 - Failing error code
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<M1FECRrs> {
        FEC_W::new(self, 0)
    }
}
/**RAMECC monitor x failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`m1fecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#RAMECC2:M1FECR)*/
pub struct M1FECRrs;
impl crate::RegisterSpec for M1FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m1fecr::R`](R) reader structure
impl crate::Readable for M1FECRrs {}
///`write(|w| ..)` method takes [`m1fecr::W`](W) writer structure
impl crate::Writable for M1FECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M1FECR to value 0
impl crate::Resettable for M1FECRrs {}
