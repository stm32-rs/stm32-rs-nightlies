///Register `SYSCFG_ITLINE3` reader
pub type R = crate::R<SYSCFG_ITLINE3rs>;
///Field `FLASH_ITF` reader - Flash interface interrupt request pending
pub type FLASH_ITF_R = crate::BitReader;
///Field `FLASH_ECC` reader - Flash interface ECC interrupt request pending
pub type FLASH_ECC_R = crate::BitReader;
impl R {
    ///Bit 0 - Flash interface interrupt request pending
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Flash interface ECC interrupt request pending
    #[inline(always)]
    pub fn flash_ecc(&self) -> FLASH_ECC_R {
        FLASH_ECC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_ITLINE3")
            .field("flash_itf", &self.flash_itf())
            .field("flash_ecc", &self.flash_ecc())
            .finish()
    }
}
/**SYSCFG interrupt line 3 status register

You can [`read`](crate::Reg::read) this register and get [`syscfg_itline3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#SYSCFG:SYSCFG_ITLINE3)*/
pub struct SYSCFG_ITLINE3rs;
impl crate::RegisterSpec for SYSCFG_ITLINE3rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_itline3::R`](R) reader structure
impl crate::Readable for SYSCFG_ITLINE3rs {}
///`reset()` method sets SYSCFG_ITLINE3 to value 0
impl crate::Resettable for SYSCFG_ITLINE3rs {
    const RESET_VALUE: u32 = 0;
}
