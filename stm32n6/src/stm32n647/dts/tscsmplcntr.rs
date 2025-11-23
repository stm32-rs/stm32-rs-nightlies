///Register `TSCSMPLCNTR` reader
pub type R = crate::R<TSCSMPLCNTRrs>;
///Field `SMPL_COUNT` reader - Sample counter
pub type SMPL_COUNT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Sample counter
    #[inline(always)]
    pub fn smpl_count(&self) -> SMPL_COUNT_R {
        SMPL_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCSMPLCNTR")
            .field("smpl_count", &self.smpl_count())
            .finish()
    }
}
/**DTS TSC sample count register

You can [`read`](crate::Reg::read) this register and get [`tscsmplcntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DTS:TSCSMPLCNTR)*/
pub struct TSCSMPLCNTRrs;
impl crate::RegisterSpec for TSCSMPLCNTRrs {
    type Ux = u32;
}
///`read()` method returns [`tscsmplcntr::R`](R) reader structure
impl crate::Readable for TSCSMPLCNTRrs {}
///`reset()` method sets TSCSMPLCNTR to value 0
impl crate::Resettable for TSCSMPLCNTRrs {}
