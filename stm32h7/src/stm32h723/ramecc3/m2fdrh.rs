///Register `M2FDRH` reader
pub type R = crate::R<M2FDRHrs>;
///Register `M2FDRH` writer
pub type W = crate::W<M2FDRHrs>;
///Field `FDATAH` reader - Failing data high (64-bit memory)
pub type FDATAH_R = crate::FieldReader<u32>;
///Field `FDATAH` writer - Failing data high (64-bit memory)
pub type FDATAH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2FDRH")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&mut self) -> FDATAH_W<M2FDRHrs> {
        FDATAH_W::new(self, 0)
    }
}
/**RAMECC monitor x failing data high register

You can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#RAMECC3:M2FDRH)*/
pub struct M2FDRHrs;
impl crate::RegisterSpec for M2FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m2fdrh::R`](R) reader structure
impl crate::Readable for M2FDRHrs {}
///`write(|w| ..)` method takes [`m2fdrh::W`](W) writer structure
impl crate::Writable for M2FDRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2FDRH to value 0
impl crate::Resettable for M2FDRHrs {}
