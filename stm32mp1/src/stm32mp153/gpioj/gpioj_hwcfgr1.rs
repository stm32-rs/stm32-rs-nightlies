#[doc = "Register `GPIOJ_HWCFGR1` reader"]
pub type R = crate::R<GPIOJ_HWCFGR1rs>;
#[doc = "Field `AFRH_RES` reader - AFRH_RES"]
pub type AFRH_RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AFRH_RES"]
    #[inline(always)]
    pub fn afrh_res(&self) -> AFRH_RES_R {
        AFRH_RES_R::new(self.bits)
    }
}
#[doc = "GPIO hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioj_hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOJ_HWCFGR1rs;
impl crate::RegisterSpec for GPIOJ_HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioj_hwcfgr1::R`](R) reader structure"]
impl crate::Readable for GPIOJ_HWCFGR1rs {}
#[doc = "`reset()` method sets GPIOJ_HWCFGR1 to value 0"]
impl crate::Resettable for GPIOJ_HWCFGR1rs {
    const RESET_VALUE: u32 = 0;
}
