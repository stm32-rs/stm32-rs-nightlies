///Register `DLYB_IPIDR` reader
pub type R = crate::R<DLYB_IPIDRrs>;
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
        f.debug_struct("DLYB_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**DLYB IP identification register

You can [`read`](crate::Reg::read) this register and get [`dlyb_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DLYBSD1:DLYB_IPIDR)*/
pub struct DLYB_IPIDRrs;
impl crate::RegisterSpec for DLYB_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dlyb_ipidr::R`](R) reader structure
impl crate::Readable for DLYB_IPIDRrs {}
///`reset()` method sets DLYB_IPIDR to value 0x0014_0051
impl crate::Resettable for DLYB_IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0051;
}
