///Register `SYSCFG_ITLINE22` reader
pub type R = crate::R<SYSCFG_ITLINE22rs>;
///Field `LCD` reader - LCD interrupt request pending
pub type LCD_R = crate::BitReader;
impl R {
    ///Bit 0 - LCD interrupt request pending
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE22")
            .field("lcd", &self.lcd())
            .finish()
    }
}
/**SYSCFG interrupt line 22 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_ITLINE22)*/
pub struct SYSCFG_ITLINE22rs;
impl crate::RegisterSpec for SYSCFG_ITLINE22rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline22::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE22rs {}
///`reset()` method sets SYSCFG_ITLINE22 to value 0
impl crate::Resettable for SYSCFG_ITLINE22rs {
    const RESET_VALUE: u32 = 0;
}
