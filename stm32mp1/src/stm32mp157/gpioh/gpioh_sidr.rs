///Register `GPIOH_SIDR` reader
pub type R = crate::R<GPIOH_SIDRrs>;
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
        f.debug_struct("GPIOH_SIDR")
            .field("sidr", &self.sidr())
            .finish()
    }
}
/**GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioh_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOH:GPIOH_SIDR)*/
pub struct GPIOH_SIDRrs;
impl crate::RegisterSpec for GPIOH_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioh_sidr::R`](R) reader structure
impl crate::Readable for GPIOH_SIDRrs {}
///`reset()` method sets GPIOH_SIDR to value 0xa3c5_dd01
impl crate::Resettable for GPIOH_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
