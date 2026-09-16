///Register `HWCFGR7` reader
pub type R = crate::R<HWCFGR7rs>;
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
        f.debug_struct("HWCFGR7")
            .field("cpuevent", &self.cpuevent())
            .finish()
    }
}
/**EXTI hardware configuration register 7

You can [`read`](crate::Reg::read) this register and get [`hwcfgr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR7)*/
pub struct HWCFGR7rs;
impl crate::RegisterSpec for HWCFGR7rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr7::R`](R) reader structure
impl crate::Readable for HWCFGR7rs {}
///`reset()` method sets HWCFGR7 to value 0x000e_ffff
impl crate::Resettable for HWCFGR7rs {
    const RESET_VALUE: u32 = 0x000e_ffff;
}
