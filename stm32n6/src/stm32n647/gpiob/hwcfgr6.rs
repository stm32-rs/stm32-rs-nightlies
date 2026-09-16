///Register `HWCFGR6` reader
pub type R = crate::R<HWCFGR6rs>;
///Field `MODER_RES` reader - MODER register reset value
pub type MODER_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - MODER register reset value
    #[inline(always)]
    pub fn moder_res(&self) -> MODER_RES_R {
        MODER_RES_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWCFGR6")
            .field("moder_res", &self.moder_res())
            .finish()
    }
}
/**GPIO port B hardware configuration register 6

You can [`read`](crate::Reg::read) this register and get [`hwcfgr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#GPIOB:HWCFGR6)*/
pub struct HWCFGR6rs;
impl crate::RegisterSpec for HWCFGR6rs {
    type Ux = u32;
}
///`read()` method returns [`hwcfgr6::R`](R) reader structure
impl crate::Readable for HWCFGR6rs {}
///`reset()` method sets HWCFGR6 to value 0xffff_febf
impl crate::Resettable for HWCFGR6rs {
    const RESET_VALUE: u32 = 0xffff_febf;
}
