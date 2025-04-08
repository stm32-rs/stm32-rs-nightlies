///Register `M4CR` reader
pub type R = crate::R<M4CRrs>;
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
        f.debug_struct("M4CR")
            .field("fdatal", &self.fdatal())
            .finish()
    }
}
/**RAMECC monitor x configuration register

You can [`read`](crate::Reg::read) this register and get [`m4cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#RAMECC1:M4CR)*/
pub struct M4CRrs;
impl crate::RegisterSpec for M4CRrs {
    type Ux = u32;
}
///`read()` method returns [`m4cr::R`](R) reader structure
impl crate::Readable for M4CRrs {}
///`reset()` method sets M4CR to value 0
impl crate::Resettable for M4CRrs {}
