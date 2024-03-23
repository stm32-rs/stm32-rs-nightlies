#[doc = "Register `ITLINE22` reader"]
pub type R = crate::R<ITLINE22rs>;
#[doc = "Field `TIM17` reader - TIM17"]
pub type TIM17_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM17"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 22 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline22::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE22rs;
impl crate::RegisterSpec for ITLINE22rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline22::R`](R) reader structure"]
impl crate::Readable for ITLINE22rs {}
#[doc = "`reset()` method sets ITLINE22 to value 0"]
impl crate::Resettable for ITLINE22rs {
    const RESET_VALUE: u32 = 0;
}
