#[doc = "Register `ITLINE5` reader"]
pub type R = crate::R<ITLINE5rs>;
#[doc = "Field `EXTI0` reader - EXTI0"]
pub type EXTI0_R = crate::BitReader;
#[doc = "Field `EXTI1` reader - EXTI1"]
pub type EXTI1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 5 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE5rs;
impl crate::RegisterSpec for ITLINE5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline5::R`](R) reader structure"]
impl crate::Readable for ITLINE5rs {}
#[doc = "`reset()` method sets ITLINE5 to value 0"]
impl crate::Resettable for ITLINE5rs {
    const RESET_VALUE: u32 = 0;
}
