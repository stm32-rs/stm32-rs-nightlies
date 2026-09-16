///Register `HWCFGR8` reader
pub type R = crate::R<HWCFGR8rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**EXTI hardware configuration register 8

You can [`read`](crate::Reg::read) this register and get [`hwcfgr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR8)*/
pub struct HWCFGR8rs;
impl crate::RegisterSpec for HWCFGR8rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr8::R`](R) reader structure
impl crate::Readable for HWCFGR8rs {}
///`reset()` method sets HWCFGR8 to value 0
impl crate::Resettable for HWCFGR8rs {}
