///Register `GPIOG_HWCFGR2` reader
pub type R = crate::R<GPIOG_HWCFGR2rs>;
///Field `AFRL_RES` reader - AFRL_RES
pub type AFRL_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - AFRL_RES
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOG_HWCFGR2")
            .field("afrl_res", &self.afrl_res())
            .finish()
    }
}
/**GPIO hardware configuration register 2

You can [`read`](crate::Reg::read) this register and get [`gpiog_hwcfgr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GPIOG:GPIOG_HWCFGR2)*/
pub struct GPIOG_HWCFGR2rs;
impl crate::RegisterSpec for GPIOG_HWCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`gpiog_hwcfgr2::R`](R) reader structure
impl crate::Readable for GPIOG_HWCFGR2rs {}
///`reset()` method sets GPIOG_HWCFGR2 to value 0
impl crate::Resettable for GPIOG_HWCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
