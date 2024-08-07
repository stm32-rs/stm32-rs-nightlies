///Register `GPIOD_SIDR` reader
pub type R = crate::R<GPIOD_SIDRrs>;
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
        f.debug_struct("GPIOD_SIDR")
            .field("sidr", &self.sidr())
            .finish()
    }
}
/**GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpiod_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOD:GPIOD_SIDR)*/
pub struct GPIOD_SIDRrs;
impl crate::RegisterSpec for GPIOD_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpiod_sidr::R`](R) reader structure
impl crate::Readable for GPIOD_SIDRrs {}
///`reset()` method sets GPIOD_SIDR to value 0xa3c5_dd01
impl crate::Resettable for GPIOD_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
