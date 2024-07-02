///Register `EXTI_HWCFGR11` reader
pub type R = crate::R<EXTI_HWCFGR11rs>;
///Field `TZ` reader - TZ
pub type TZ_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TZ
    #[inline(always)]
    pub fn tz(&self) -> TZ_R {
        TZ_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_HWCFGR11")
            .field("tz", &self.tz())
            .finish()
    }
}
/**EXTI hardware configuration register 11

You can [`read`](crate::Reg::read) this register and get [`exti_hwcfgr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:EXTI_HWCFGR11)*/
pub struct EXTI_HWCFGR11rs;
impl crate::RegisterSpec for EXTI_HWCFGR11rs {
    type Ux = u32;
}
///`read()` method returns [`exti_hwcfgr11::R`](R) reader structure
impl crate::Readable for EXTI_HWCFGR11rs {}
///`reset()` method sets EXTI_HWCFGR11 to value 0x050e_ffff
impl crate::Resettable for EXTI_HWCFGR11rs {
    const RESET_VALUE: u32 = 0x050e_ffff;
}
