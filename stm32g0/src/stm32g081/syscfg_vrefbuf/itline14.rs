#[doc = "Register `ITLINE14` reader"]
pub type R = crate::R<ITLINE14rs>;
#[doc = "Field `TIM1_CC` reader - TIM1_CC"]
pub type TIM1_CC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM1_CC"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 14 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE14rs;
impl crate::RegisterSpec for ITLINE14rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline14::R`](R) reader structure"]
impl crate::Readable for ITLINE14rs {}
#[doc = "`reset()` method sets ITLINE14 to value 0"]
impl crate::Resettable for ITLINE14rs {
    const RESET_VALUE: u32 = 0;
}
