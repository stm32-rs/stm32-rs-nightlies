///Register `GPIOE_SIDR` reader
pub type R = crate::R<GPIOE_SIDRrs>;
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
        f.debug_struct("GPIOE_SIDR")
            .field("sidr", &self.sidr())
            .finish()
    }
}
/**GPIO size identification register

You can [`read`](crate::Reg::read) this register and get [`gpioe_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOE:GPIOE_SIDR)*/
pub struct GPIOE_SIDRrs;
impl crate::RegisterSpec for GPIOE_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`gpioe_sidr::R`](R) reader structure
impl crate::Readable for GPIOE_SIDRrs {}
///`reset()` method sets GPIOE_SIDR to value 0xa3c5_dd01
impl crate::Resettable for GPIOE_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
