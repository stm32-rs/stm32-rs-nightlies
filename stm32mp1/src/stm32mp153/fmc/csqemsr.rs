///Register `CSQEMSR` reader
pub type R = crate::R<CSQEMSRrs>;
///Field `SEM` reader - SEM
pub type SEM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - SEM
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQEMSR").field("sem", &self.sem()).finish()
    }
}
/**This register holds a sector error mapping status when the whole transfer is complete.

You can [`read`](crate::Reg::read) this register and get [`csqemsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:CSQEMSR)*/
pub struct CSQEMSRrs;
impl crate::RegisterSpec for CSQEMSRrs {
    type Ux = u32;
}
///`read()` method returns [`csqemsr::R`](R) reader structure
impl crate::Readable for CSQEMSRrs {}
///`reset()` method sets CSQEMSR to value 0
impl crate::Resettable for CSQEMSRrs {}
