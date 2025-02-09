///Register `M5FDRL` reader
pub type R = crate::R<M5FDRLrs>;
///Register `M5FDRL` writer
pub type W = crate::W<M5FDRLrs>;
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
        f.debug_struct("M5FDRL")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
impl W {}
/**RAMECC monitor 5 failing data low register

You can [`read`](crate::Reg::read) this register and get [`m5fdrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5fdrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#RAMECC1:M5FDRL)*/
pub struct M5FDRLrs;
impl crate::RegisterSpec for M5FDRLrs {
    type Ux = u32;
}
///`read()` method returns [`m5fdrl::R`](R) reader structure
impl crate::Readable for M5FDRLrs {}
///`write(|w| ..)` method takes [`m5fdrl::W`](W) writer structure
impl crate::Writable for M5FDRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M5FDRL to value 0
impl crate::Resettable for M5FDRLrs {
    const RESET_VALUE: u32 = 0;
}
