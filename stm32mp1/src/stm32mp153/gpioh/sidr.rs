///Register `SIDR` reader
pub type R = crate::R<SIDRrs>;
///Field `SIDR` reader - SIDR
pub type SIDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SIDR
    #[inline(always)]
    pub fn sidr(&self) -> SIDR_R {
        SIDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIDR").field("sidr", &self.sidr()).finish()
    }
}
/**GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOH:SIDR)*/
pub struct SIDRrs;
impl crate::RegisterSpec for SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`sidr::R`](R) reader structure
impl crate::Readable for SIDRrs {}
///`reset()` method sets SIDR to value 0xa3c5_dd01
impl crate::Resettable for SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
