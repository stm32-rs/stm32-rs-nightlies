///Register `HWCFGR4` reader
pub type R = crate::R<HWCFGR4rs>;
///Field `EVENT_TRG` reader - HW configuration event trigger type
pub type EVENT_TRG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR4")
            .field("event_trg", &self.event_trg())
            .finish()
    }
}
/**Hardware configuration registers

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#EXTI:HWCFGR4)*/
pub struct HWCFGR4rs;
impl crate::RegisterSpec for HWCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr4::R`](R) reader structure
impl crate::Readable for HWCFGR4rs {}
///`reset()` method sets HWCFGR4 to value 0
impl crate::Resettable for HWCFGR4rs {}
