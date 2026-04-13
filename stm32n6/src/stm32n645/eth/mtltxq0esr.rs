///Register `MTLTXQ0ESR` reader
pub type R = crate::R<MTLTXQ0ESRrs>;
///Field `ABS` reader - Average Bits per Slot
pub type ABS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - Average Bits per Slot
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTXQ0ESR")
            .field("abs", &self.abs())
            .finish()
    }
}
/**T0 queue 0 ETS status Register

You can [`read`](crate::Reg::read) this register and get [`mtltxq0esr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLTXQ0ESR)*/
pub struct MTLTXQ0ESRrs;
impl crate::RegisterSpec for MTLTXQ0ESRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltxq0esr::R`](R) reader structure
impl crate::Readable for MTLTXQ0ESRrs {}
///`reset()` method sets MTLTXQ0ESR to value 0
impl crate::Resettable for MTLTXQ0ESRrs {}
