///Register `HRCR` reader
pub type R = crate::R<HRCRrs>;
///Field `HRC` reader - hot reset counter
pub type HRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - hot reset counter
    #[inline(always)]
    pub fn hrc(&self) -> HRC_R {
        HRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HRCR").field("hrc", &self.hrc()).finish()
    }
}
/**BSEC hot reset count register

You can [`read`](crate::Reg::read) this register and get [`hrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:HRCR)*/
pub struct HRCRrs;
impl crate::RegisterSpec for HRCRrs {
    type Ux = u32;
}
///`read()` method returns [`hrcr::R`](R) reader structure
impl crate::Readable for HRCRrs {}
///`reset()` method sets HRCR to value 0
impl crate::Resettable for HRCRrs {}
