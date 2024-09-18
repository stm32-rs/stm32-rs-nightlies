///Register `IWDG_HWCFGR` reader
pub type R = crate::R<IWDG_HWCFGRrs>;
///Field `WINDOW` reader - WINDOW
pub type WINDOW_R = crate::FieldReader;
///Field `PR_DEFAULT` reader - PR_DEFAULT
pub type PR_DEFAULT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - WINDOW
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PR_DEFAULT
    #[inline(always)]
    pub fn pr_default(&self) -> PR_DEFAULT_R {
        PR_DEFAULT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_HWCFGR")
            .field("window", &self.window())
            .field("pr_default", &self.pr_default())
            .finish()
    }
}
/**IWDG hardware configuration register

You can [`read`](crate::Reg::read) this register and get [`iwdg_hwcfgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#IWDG1:IWDG_HWCFGR)*/
pub struct IWDG_HWCFGRrs;
impl crate::RegisterSpec for IWDG_HWCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_hwcfgr::R`](R) reader structure
impl crate::Readable for IWDG_HWCFGRrs {}
///`reset()` method sets IWDG_HWCFGR to value 0x71
impl crate::Resettable for IWDG_HWCFGRrs {
    const RESET_VALUE: u32 = 0x71;
}
