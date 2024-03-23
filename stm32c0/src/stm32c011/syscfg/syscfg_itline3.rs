#[doc = "Register `SYSCFG_ITLINE3` reader"]
pub type R = crate::R<SYSCFG_ITLINE3rs>;
#[doc = "Field `FLASH_ITF` reader - Flash interface interrupt request pending"]
pub type FLASH_ITF_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Flash interface interrupt request pending"]
    #[inline(always)]
    pub fn flash_itf(&self) -> FLASH_ITF_R {
        FLASH_ITF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 3 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE3rs;
impl crate::RegisterSpec for SYSCFG_ITLINE3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline3::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE3rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE3 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE3rs {
    const RESET_VALUE: u32 = 0;
}
