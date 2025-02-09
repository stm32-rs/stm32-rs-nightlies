///Register `HWCFGR5` reader
pub type R = crate::R<HWCFGR5rs>;
///Field `CPUEVENT` reader - CPUEVENT
pub type CPUEVENT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CPUEVENT
    #[inline(always)]
    pub fn cpuevent(&self) -> CPUEVENT_R {
        CPUEVENT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR5")
            .field("cpuevent", &self.cpuevent())
            .finish()
    }
}
/**EXTI hardware configuration register 5

You can [`read`](crate::Reg::read) this register and get [`hwcfgr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR5)*/
pub struct HWCFGR5rs;
impl crate::RegisterSpec for HWCFGR5rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr5::R`](R) reader structure
impl crate::Readable for HWCFGR5rs {}
///`reset()` method sets HWCFGR5 to value 0x000e_ffff
impl crate::Resettable for HWCFGR5rs {
    const RESET_VALUE: u32 = 0x000e_ffff;
}
