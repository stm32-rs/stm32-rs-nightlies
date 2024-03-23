#[doc = "Register `GPIOB_HWCFGR2` reader"]
pub type R = crate::R<GPIOB_HWCFGR2rs>;
#[doc = "Field `AFRL_RES` reader - AFRL_RES"]
pub type AFRL_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AFRL_RES"]
    #[inline(always)]
    pub fn afrl_res(&self) -> AFRL_RES_R {
        AFRL_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_hwcfgr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOB_HWCFGR2rs;
impl crate::RegisterSpec for GPIOB_HWCFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_hwcfgr2::R`](R) reader structure"]
impl crate::Readable for GPIOB_HWCFGR2rs {}
#[doc = "`reset()` method sets GPIOB_HWCFGR2 to value 0"]
impl crate::Resettable for GPIOB_HWCFGR2rs {
    const RESET_VALUE: u32 = 0;
}
