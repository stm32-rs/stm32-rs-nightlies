///Register `TTCSM` reader
pub type R = crate::R<TTCSMrs>;
///Field `CSM` reader - Cycle Sync Mark
pub type CSM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Cycle Sync Mark
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTCSM").field("csm", &self.csm()).finish()
    }
}
/**FDCAN TT Cycle Sync Mark Register

You can [`read`](crate::Reg::read) this register and get [`ttcsm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#FDCAN1:TTCSM)*/
pub struct TTCSMrs;
impl crate::RegisterSpec for TTCSMrs {
    type Ux = u32;
}
///`read()` method returns [`ttcsm::R`](R) reader structure
impl crate::Readable for TTCSMrs {}
///`reset()` method sets TTCSM to value 0
impl crate::Resettable for TTCSMrs {}
