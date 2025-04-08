///Register `REGION_TOP_LOW0` reader
pub type R = crate::R<REGION_TOP_LOW0rs>;
///Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32>;
impl R {
    ///Bits 12:31 - TOP_ADDRESS_LOW
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION_TOP_LOW0")
            .field("top_address_low", &self.top_address_low())
            .finish()
    }
}
/**Top address bits \[31:12\] for region 0.

You can [`read`](crate::Reg::read) this register and get [`region_top_low0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:REGION_TOP_LOW0)*/
pub struct REGION_TOP_LOW0rs;
impl crate::RegisterSpec for REGION_TOP_LOW0rs {
    type Ux = u32;
}
///`read()` method returns [`region_top_low0::R`](R) reader structure
impl crate::Readable for REGION_TOP_LOW0rs {}
///`reset()` method sets REGION_TOP_LOW0 to value 0xffff_ffff
impl crate::Resettable for REGION_TOP_LOW0rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
