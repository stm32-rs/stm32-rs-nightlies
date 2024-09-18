///Register `SPDIFRX_IPIDR` reader
pub type R = crate::R<SPDIFRX_IPIDRrs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDIFRX_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**SPDIFRX identification register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SPDIFRX:SPDIFRX_IPIDR)*/
pub struct SPDIFRX_IPIDRrs;
impl crate::RegisterSpec for SPDIFRX_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`spdifrx_ipidr::R`](R) reader structure
impl crate::Readable for SPDIFRX_IPIDRrs {}
///`reset()` method sets SPDIFRX_IPIDR to value 0x0013_0041
impl crate::Resettable for SPDIFRX_IPIDRrs {
    const RESET_VALUE: u32 = 0x0013_0041;
}
