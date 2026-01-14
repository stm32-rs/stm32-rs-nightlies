///Register `ECCR%s` reader
pub type R = crate::R<ECCRrs>;
///Field `ECC` reader - ECCx
pub type ECC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECCx
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR").field("ecc", &self.ecc()).finish()
    }
}
/**ECC result register %s

You can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#FMC:ECCR[2])*/
pub struct ECCRrs;
impl crate::RegisterSpec for ECCRrs {
    type Ux = u32;
}
///`read()` method returns [`eccr::R`](R) reader structure
impl crate::Readable for ECCRrs {}
///`reset()` method sets ECCR%s to value 0
impl crate::Resettable for ECCRrs {}
