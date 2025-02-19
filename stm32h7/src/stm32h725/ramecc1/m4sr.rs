///Register `M4SR` reader
pub type R = crate::R<M4SRrs>;
///Field `FDATAL` reader - Failing data low
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M4SR")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x status register

You can [`read`](crate::Reg::read) this register and get [`m4sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H725.html#RAMECC1:M4SR)*/
pub struct M4SRrs;
impl crate::RegisterSpec for M4SRrs {
    type Ux = u32;
}
///`read()` method returns [`m4sr::R`](R) reader structure
impl crate::Readable for M4SRrs {}
///`reset()` method sets M4SR to value 0
impl crate::Resettable for M4SRrs {
    const RESET_VALUE: u32 = 0;
}
