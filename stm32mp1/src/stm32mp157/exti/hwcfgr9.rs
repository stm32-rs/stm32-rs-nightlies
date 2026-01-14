///Register `HWCFGR9` reader
pub type R = crate::R<HWCFGR9rs>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/**EXTI hardware configuration register 9

You can [`read`](crate::Reg::read) this register and get [`hwcfgr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR9)*/
pub struct HWCFGR9rs;
impl crate::RegisterSpec for HWCFGR9rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr9::R`](R) reader structure
impl crate::Readable for HWCFGR9rs {}
///`reset()` method sets HWCFGR9 to value 0
impl crate::Resettable for HWCFGR9rs {}
