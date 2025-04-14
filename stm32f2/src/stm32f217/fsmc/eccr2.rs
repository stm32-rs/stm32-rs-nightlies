///Register `ECCR2` reader
pub type R = crate::R<ECCR2rs>;
///Field `ECC` reader - ECC result
pub type ECC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC result
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECCR2").field("ecc", &self.ecc()).finish()
    }
}
/**ECC result register 2

You can [`read`](crate::Reg::read) this register and get [`eccr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F217.html#FSMC:ECCR2)*/
pub struct ECCR2rs;
impl crate::RegisterSpec for ECCR2rs {
    type Ux = u32;
}
///`read()` method returns [`eccr2::R`](R) reader structure
impl crate::Readable for ECCR2rs {}
///`reset()` method sets ECCR2 to value 0
impl crate::Resettable for ECCR2rs {}
