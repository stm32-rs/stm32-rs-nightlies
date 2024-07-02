///Register `EXTI_HWCFGR10` reader
pub type R = crate::R<EXTI_HWCFGR10rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**EXTI hardware configuration register 10

You can [`read`](crate::Reg::read) this register and get [`exti_hwcfgr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#EXTI:EXTI_HWCFGR10)*/
pub struct EXTI_HWCFGR10rs;
impl crate::RegisterSpec for EXTI_HWCFGR10rs {
    type Ux = u32;
}
///`read()` method returns [`exti_hwcfgr10::R`](R) reader structure
impl crate::Readable for EXTI_HWCFGR10rs {}
///`reset()` method sets EXTI_HWCFGR10 to value 0
impl crate::Resettable for EXTI_HWCFGR10rs {
    const RESET_VALUE: u32 = 0;
}
