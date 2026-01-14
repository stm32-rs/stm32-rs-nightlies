///Register `BOOTSR` reader
pub type R = crate::R<BOOTSRrs>;
///Field `INITVTOR` reader - initial vector for Cortex-M7 This register includes the physical boot address used by the Cortex-M7 after reset
pub type INITVTOR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - initial vector for Cortex-M7 This register includes the physical boot address used by the Cortex-M7 after reset
    #[inline(always)]
    pub fn initvtor(&self) -> INITVTOR_R {
        INITVTOR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTSR")
            .field("initvtor", &self.initvtor())
            .finish()
    }
}
/**SBS boot status register

You can [`read`](crate::Reg::read) this register and get [`bootsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:BOOTSR)*/
pub struct BOOTSRrs;
impl crate::RegisterSpec for BOOTSRrs {
    type Ux = u32;
}
///`read()` method returns [`bootsr::R`](R) reader structure
impl crate::Readable for BOOTSRrs {}
///`reset()` method sets BOOTSR to value 0
impl crate::Resettable for BOOTSRrs {}
