///Register `IOG%sCR` reader
pub type R = crate::R<IOGCRrs>;
///Field `CNT` reader - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across CsubS/sub has reached the threshold).
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Counter value These bits represent the number of charge transfer cycles generated on the analog I/O group x to complete its acquisition (voltage across CsubS/sub has reached the threshold).
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOGCR").field("cnt", &self.cnt()).finish()
    }
}
/**TSC I/O group %s counter register

You can [`read`](crate::Reg::read) this register and get [`iogcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#TSC:IOG[1]CR)*/
pub struct IOGCRrs;
impl crate::RegisterSpec for IOGCRrs {
    type Ux = u32;
}
///`read()` method returns [`iogcr::R`](R) reader structure
impl crate::Readable for IOGCRrs {}
///`reset()` method sets IOG%sCR to value 0
impl crate::Resettable for IOGCRrs {}
