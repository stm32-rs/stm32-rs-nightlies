#[doc = "Register `GPIOI_HWCFGR4` reader"]
pub type R = crate::R<GPIOI_HWCFGR4rs>;
#[doc = "Field `OSPEED_RES` reader - OSPEED_RES"]
pub type OSPEED_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OSPEED_RES"]
    #[inline(always)]
    pub fn ospeed_res(&self) -> OSPEED_RES_R {
        OSPEED_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioi_hwcfgr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOI_HWCFGR4rs;
impl crate::RegisterSpec for GPIOI_HWCFGR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioi_hwcfgr4::R`](R) reader structure"]
impl crate::Readable for GPIOI_HWCFGR4rs {}
#[doc = "`reset()` method sets GPIOI_HWCFGR4 to value 0"]
impl crate::Resettable for GPIOI_HWCFGR4rs {
    const RESET_VALUE: u32 = 0;
}
