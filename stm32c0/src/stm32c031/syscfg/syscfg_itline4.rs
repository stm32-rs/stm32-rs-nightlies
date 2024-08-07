///Register `SYSCFG_ITLINE4` reader
pub type R = crate::R<SYSCFG_ITLINE4rs>;
///Field `RCC` reader - Reset and clock control interrupt request pending
pub type RCC_R = crate::BitReader;
impl R {
    ///Bit 0 - Reset and clock control interrupt request pending
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE4")
            .field("rcc", &self.rcc())
            .finish()
    }
}
/**SYSCFG interrupt line 4 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#SYSCFG:SYSCFG_ITLINE4)*/
pub struct SYSCFG_ITLINE4rs;
impl crate::RegisterSpec for SYSCFG_ITLINE4rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline4::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE4rs {}
///`reset()` method sets SYSCFG_ITLINE4 to value 0
impl crate::Resettable for SYSCFG_ITLINE4rs {
    const RESET_VALUE: u32 = 0;
}
