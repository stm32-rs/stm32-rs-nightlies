#[doc = "Register `ITLINE15` reader"]
pub type R = crate::R<ITLINE15rs>;
#[doc = "Field `TIM2` reader - TIM2"]
pub type TIM2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM2"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 15 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline15::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE15rs;
impl crate::RegisterSpec for ITLINE15rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline15::R`](R) reader structure"]
impl crate::Readable for ITLINE15rs {}
#[doc = "`reset()` method sets ITLINE15 to value 0"]
impl crate::Resettable for ITLINE15rs {
    const RESET_VALUE: u32 = 0;
}
