#[doc = "Register `ITLINE6` reader"]
pub type R = crate::R<ITLINE6rs>;
#[doc = "Field `EXTI2` reader - EXTI2"]
pub type EXTI2_R = crate::BitReader;
#[doc = "Field `EXTI3` reader - EXTI3"]
pub type EXTI3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 6 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE6rs;
impl crate::RegisterSpec for ITLINE6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline6::R`](R) reader structure"]
impl crate::Readable for ITLINE6rs {}
#[doc = "`reset()` method sets ITLINE6 to value 0"]
impl crate::Resettable for ITLINE6rs {
    const RESET_VALUE: u32 = 0;
}
