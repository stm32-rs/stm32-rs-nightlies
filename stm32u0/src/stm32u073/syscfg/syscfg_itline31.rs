///Register `SYSCFG_ITLINE31` reader
pub type R = crate::R<SYSCFG_ITLINE31rs>;
///Field `RNG` reader - RNG interrupt request pending
pub type RNG_R = crate::BitReader;
impl R {
    ///Bit 0 - RNG interrupt request pending
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE31")
            .field("rng", &self.rng())
            .finish()
    }
}
/**SYSCFG interrupt line 31 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:SYSCFG_ITLINE31)*/
pub struct SYSCFG_ITLINE31rs;
impl crate::RegisterSpec for SYSCFG_ITLINE31rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline31::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE31rs {}
///`reset()` method sets SYSCFG_ITLINE31 to value 0
impl crate::Resettable for SYSCFG_ITLINE31rs {
    const RESET_VALUE: u32 = 0;
}