///Register `GPIOH_HWCFGR6` reader
pub type R = crate::R<GPIOH_HWCFGR6rs>;
///Field `MODER_RES` reader - MODER_RES
pub type MODER_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - MODER_RES
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOH_HWCFGR6")
            .field("moder_res", &self.moder_res())
            .finish()
    }
}
/**GPIO hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`gpioh_hwcfgr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GPIOH:GPIOH_HWCFGR6)*/
pub struct GPIOH_HWCFGR6rs;
impl crate::RegisterSpec for GPIOH_HWCFGR6rs {
    type Ux = u32;
}
///`read()` method returns [`gpioh_hwcfgr6::R`](R) reader structure
impl crate::Readable for GPIOH_HWCFGR6rs {}
///`reset()` method sets GPIOH_HWCFGR6 to value 0xffff_ffff
impl crate::Resettable for GPIOH_HWCFGR6rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
