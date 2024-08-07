///Register `BSEC_IPIDR` reader
pub type R = crate::R<BSEC_IPIDRrs>;
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
        f.debug_struct("BSEC_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**BSEC identification register

You can [`read`](crate::Reg::read) this register and get [`bsec_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:BSEC_IPIDR)*/
pub struct BSEC_IPIDRrs;
impl crate::RegisterSpec for BSEC_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`bsec_ipidr::R`](R) reader structure
impl crate::Readable for BSEC_IPIDRrs {}
///`reset()` method sets BSEC_IPIDR to value 0x0010_0032
impl crate::Resettable for BSEC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0032;
}
