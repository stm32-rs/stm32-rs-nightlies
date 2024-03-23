#[doc = "Register `ITLINE19` reader"]
pub type R = crate::R<ITLINE19rs>;
#[doc = "Field `TIM14` reader - TIM14"]
pub type TIM14_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM14"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 19 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline19::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE19rs;
impl crate::RegisterSpec for ITLINE19rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline19::R`](R) reader structure"]
impl crate::Readable for ITLINE19rs {}
#[doc = "`reset()` method sets ITLINE19 to value 0"]
impl crate::Resettable for ITLINE19rs {
    const RESET_VALUE: u32 = 0;
}
