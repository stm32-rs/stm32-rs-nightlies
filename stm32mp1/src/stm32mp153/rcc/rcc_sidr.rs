///Register `RCC_SIDR` reader
pub type R = crate::R<RCC_SIDRrs>;
///Field `SID` reader - SID
pub type SID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SID
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_SIDR")
            .field("sid", &self.sid())
            .finish()
    }
}
/**This register gives the decoding space, which is for the RCC of 4 kB.

You can [`read`](crate::Reg::read) this register and get [`rcc_sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:RCC_SIDR)*/
pub struct RCC_SIDRrs;
impl crate::RegisterSpec for RCC_SIDRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_sidr::R`](R) reader structure
impl crate::Readable for RCC_SIDRrs {}
///`reset()` method sets RCC_SIDR to value 0xa3c5_dd04
impl crate::Resettable for RCC_SIDRrs {
    const RESET_VALUE: u32 = 0xa3c5_dd04;
}
