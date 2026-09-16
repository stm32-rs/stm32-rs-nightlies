///Register `IPIDR` reader
pub type R = crate::R<IPIDRrs>;
///Field `ID` reader - Identification Code
pub type ID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Identification Code
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPIDR").field("id", &self.id()).finish()
    }
}
/**Graphic MMU identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#GFXMMU:IPIDR)*/
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`ipidr::R`](R) reader structure
impl crate::Readable for IPIDRrs {}
///`reset()` method sets IPIDR to value 0x0016_0061
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x0016_0061;
}
