///Register `HWCFGR13` reader
pub type R = crate::R<HWCFGR13rs>;
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
        f.debug_struct("HWCFGR13").field("tz", &self.tz()).finish()
    }
}
/**EXTI hardware configuration register 13

You can [`read`](crate::Reg::read) this register and get [`hwcfgr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#EXTI:HWCFGR13)*/
pub struct HWCFGR13rs;
impl crate::RegisterSpec for HWCFGR13rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr13::R`](R) reader structure
impl crate::Readable for HWCFGR13rs {}
///`reset()` method sets HWCFGR13 to value 0x050e_ffff
impl crate::Resettable for HWCFGR13rs {
    const RESET_VALUE: u32 = 0x050e_ffff;
}
