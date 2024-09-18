///Register `HDP_IPIDR` reader
pub type R = crate::R<HDP_IPIDRrs>;
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
        f.debug_struct("HDP_IPIDR").field("id", &self.id()).finish()
    }
}
/**HDP IP identification register

You can [`read`](crate::Reg::read) this register and get [`hdp_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDP:HDP_IPIDR)*/
pub struct HDP_IPIDRrs;
impl crate::RegisterSpec for HDP_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`hdp_ipidr::R`](R) reader structure
impl crate::Readable for HDP_IPIDRrs {}
///`reset()` method sets HDP_IPIDR to value 0x0003_0002
impl crate::Resettable for HDP_IPIDRrs {
    const RESET_VALUE: u32 = 0x0003_0002;
}
