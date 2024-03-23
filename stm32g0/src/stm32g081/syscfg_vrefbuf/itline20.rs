#[doc = "Register `ITLINE20` reader"]
pub type R = crate::R<ITLINE20rs>;
#[doc = "Field `TIM15` reader - TIM15"]
pub type TIM15_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM15"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 20 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline20::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE20rs;
impl crate::RegisterSpec for ITLINE20rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline20::R`](R) reader structure"]
impl crate::Readable for ITLINE20rs {}
#[doc = "`reset()` method sets ITLINE20 to value 0"]
impl crate::Resettable for ITLINE20rs {
    const RESET_VALUE: u32 = 0;
}
