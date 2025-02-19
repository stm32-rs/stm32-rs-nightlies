///Register `HWCFGR4` reader
pub type R = crate::R<HWCFGR4rs>;
///Field `EVENT_TRG` reader - EVENT_TRG
pub type EVENT_TRG_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - EVENT_TRG
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
/**EXTI hardware configuration register 4

You can [`read`](crate::Reg::read) this register and get [`hwcfgr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR4)*/
pub struct HWCFGR4rs;
impl crate::RegisterSpec for HWCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr4::R`](R) reader structure
impl crate::Readable for HWCFGR4rs {}
///`reset()` method sets HWCFGR4 to value 0x0001_ffff
impl crate::Resettable for HWCFGR4rs {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
