///Register `DMAMUX_IPIDR` reader
pub type R = crate::R<DMAMUX_IPIDRrs>;
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
        f.debug_struct("DMAMUX_IPIDR")
            .field("id", &self.id())
            .finish()
    }
}
/**This register identifies the IP.

You can [`read`](crate::Reg::read) this register and get [`dmamux_ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DMAMUX1:DMAMUX_IPIDR)*/
pub struct DMAMUX_IPIDRrs;
impl crate::RegisterSpec for DMAMUX_IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`dmamux_ipidr::R`](R) reader structure
impl crate::Readable for DMAMUX_IPIDRrs {}
///`reset()` method sets DMAMUX_IPIDR to value 0x0010_0011
impl crate::Resettable for DMAMUX_IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0011;
}
