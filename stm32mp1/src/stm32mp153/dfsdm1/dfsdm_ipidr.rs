///Register `DFSDM_IPIDR` reader
pub type R = crate::R<DFSDM_IPIDRrs>;
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
        f.debug_struct("DFSDM_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**This register specifies the identification of DFSDM peripheral.

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_IPIDR)*/
pub struct DFSDM_IPIDRrs;
impl crate::RegisterSpec for DFSDM_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ipidr::R`](R) reader structure
impl crate::Readable for DFSDM_IPIDRrs {}
///`reset()` method sets DFSDM_IPIDR to value 0x0011_0031
impl crate::Resettable for DFSDM_IPIDRrs {
    const RESET_VALUE: u32 = 0x0011_0031;
}
