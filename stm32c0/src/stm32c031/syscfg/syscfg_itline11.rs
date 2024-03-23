#[doc = "Register `SYSCFG_ITLINE11` reader"]
pub type R = crate::R<SYSCFG_ITLINE11rs>;
#[doc = "Field `DMAMUX` reader - DMAMUX interrupt request pending"]
pub type DMAMUX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMAMUX interrupt request pending"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 11 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE11rs;
impl crate::RegisterSpec for SYSCFG_ITLINE11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline11::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE11rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE11 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE11rs {
    const RESET_VALUE: u32 = 0;
}
