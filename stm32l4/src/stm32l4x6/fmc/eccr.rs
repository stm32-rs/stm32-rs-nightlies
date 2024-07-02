///Register `ECCR` reader
pub type R = crate::R<ECCRrs>;
///Field `ECCx` reader - ECCx
pub type ECCX_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECCx
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR").field("eccx", &self.eccx()).finish()
    }
}
/**ECC result register 3

You can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x6.html#FMC:ECCR)*/
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
///`read()` method returns [`eccr::R`](R) reader structure
impl crate::Readable for ECCRrs {}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCRrs {
    const RESET_VALUE: u32 = 0;
}
