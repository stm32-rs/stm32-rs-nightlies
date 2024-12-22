///Register `SYSCFG_ITLINE2` reader
pub type R = crate::R<SYSCFG_ITLINE2rs>;
///Field `TAMP` reader - Tamper interrupt request pending (EXTI line 21)
pub type TAMP_R = crate::BitReader;
///Field `RTC` reader - RTC interrupt request pending (EXTI line 19)
pub type RTC_R = crate::BitReader;
impl R {
    ///Bit 0 - Tamper interrupt request pending (EXTI line 21)
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC interrupt request pending (EXTI line 19)
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE2")
            .field("tamp", &self.tamp())
            .field("rtc", &self.rtc())
            .finish()
    }
}
/**SYSCFG interrupt line 2 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#SYSCFG:SYSCFG_ITLINE2)*/
pub struct SYSCFG_ITLINE2rs;
impl crate::RegisterSpec for SYSCFG_ITLINE2rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline2::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE2rs {}
///`reset()` method sets SYSCFG_ITLINE2 to value 0
impl crate::Resettable for SYSCFG_ITLINE2rs {
    const RESET_VALUE: u32 = 0;
}
