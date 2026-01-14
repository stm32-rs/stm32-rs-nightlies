///Register `HWCFGR2` reader
pub type R = crate::R<HWCFGR2rs>;
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
        f.debug_struct("HWCFGR2")
            .field("event_trg", &self.event_trg())
            .finish()
    }
}
/**EXTI hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:HWCFGR2)*/
pub struct HWCFGR2rs;
impl crate::RegisterSpec for HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr2::R`](R) reader structure
impl crate::Readable for HWCFGR2rs {}
///`reset()` method sets HWCFGR2 to value 0x0001_ffff
impl crate::Resettable for HWCFGR2rs {
    const RESET_VALUE: u32 = 0x0001_ffff;
}
