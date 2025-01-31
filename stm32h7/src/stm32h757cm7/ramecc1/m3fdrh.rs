///Register `M3FDRH` reader
pub type R = crate::R<M3FDRHrs>;
///Register `M3FDRH` writer
pub type W = crate::W<M3FDRHrs>;
///Field `FDATAH` reader - ECC failing data high
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC failing data high
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3FDRH")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
impl W {}
/**RAMECC monitor 3 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m3fdrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3fdrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#RAMECC1:M3FDRH)*/
pub struct M3FDRHrs;
impl crate::RegisterSpec for M3FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m3fdrh::R`](R) reader structure
impl crate::Readable for M3FDRHrs {}
///`write(|w| ..)` method takes [`m3fdrh::W`](W) writer structure
impl crate::Writable for M3FDRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M3FDRH to value 0
impl crate::Resettable for M3FDRHrs {
    const RESET_VALUE: u32 = 0;
}
