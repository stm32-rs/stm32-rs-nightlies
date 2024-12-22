///Register `FDCAN_TTCSM` reader
pub type R = crate::R<FDCAN_TTCSMrs>;
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
        f.debug_struct("FDCAN_TTCSM")
            .field("csm", &self.csm())
            .finish()
    }
}
/**FDCAN TT Cycle Sync Mark Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ttcsm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TTCSM)*/
pub struct FDCAN_TTCSMrs;
impl crate::RegisterSpec for FDCAN_TTCSMrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ttcsm::R`](R) reader structure
impl crate::Readable for FDCAN_TTCSMrs {}
///`reset()` method sets FDCAN_TTCSM to value 0
impl crate::Resettable for FDCAN_TTCSMrs {
    const RESET_VALUE: u32 = 0;
}
