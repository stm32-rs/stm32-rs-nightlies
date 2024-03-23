#[doc = "Register `ITLINE16` reader"]
pub type R = crate::R<ITLINE16rs>;
#[doc = "Field `TIM3` reader - TIM3"]
pub type TIM3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 16 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE16rs;
impl crate::RegisterSpec for ITLINE16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline16::R`](R) reader structure"]
impl crate::Readable for ITLINE16rs {}
#[doc = "`reset()` method sets ITLINE16 to value 0"]
impl crate::Resettable for ITLINE16rs {
    const RESET_VALUE: u32 = 0;
}
