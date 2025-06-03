///Register `HECCR` reader
pub type R = crate::R<HECCRrs>;
///Field `HECC` reader - ECC result
pub type HECC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC result
    #[inline(always)]
    pub fn hecc(&self) -> HECC_R {
        HECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HECCR").field("hecc", &self.hecc()).finish()
    }
}
/**FMC Hamming code ECC result register

You can [`read`](crate::Reg::read) this register and get [`heccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#FMC1:HECCR)*/
pub struct HECCRrs;
impl crate::RegisterSpec for HECCRrs {
    type Ux = u32;
}
///`read()` method returns [`heccr::R`](R) reader structure
impl crate::Readable for HECCRrs {}
///`reset()` method sets HECCR to value 0
impl crate::Resettable for HECCRrs {}
