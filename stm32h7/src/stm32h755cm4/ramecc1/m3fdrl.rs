///Register `M3FDRL` reader
pub type R = crate::R<M3FDRLrs>;
///Register `M3FDRL` writer
pub type W = crate::W<M3FDRLrs>;
///Field `FDATAL` reader - ECC failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
impl W {}
/**RAMECC monitor 3 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m3fdrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3fdrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#RAMECC1:M3FDRL)*/
pub struct M3FDRLrs;
impl crate::RegisterSpec for M3FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m3fdrl::R`](R) reader structure
impl crate::Readable for M3FDRLrs {}
///`write(|w| ..)` method takes [`m3fdrl::W`](W) writer structure
impl crate::Writable for M3FDRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M3FDRL to value 0
impl crate::Resettable for M3FDRLrs {}
