#[doc = "Register `TIM5_OR` reader"]
pub type R = crate::R<TIM5_ORrs>;
#[doc = "Register `TIM5_OR` writer"]
pub type W = crate::W<TIM5_ORrs>;
#[doc = "Field `IT4_RMP` reader - Timer Input 4 remap"]
pub type IT4_RMP_R = crate::FieldReader;
#[doc = "Field `IT4_RMP` writer - Timer Input 4 remap"]
pub type IT4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    #[must_use]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W<TIM5_ORrs> {
        IT4_RMP_W::new(self, 6)
    }
}
#[doc = "TIM5 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim5_or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim5_or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM5_ORrs;
impl crate::RegisterSpec for TIM5_ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim5_or::R`](R) reader structure"]
impl crate::Readable for TIM5_ORrs {}
#[doc = "`write(|w| ..)` method takes [`tim5_or::W`](W) writer structure"]
impl crate::Writable for TIM5_ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM5_OR to value 0"]
impl crate::Resettable for TIM5_ORrs {
    const RESET_VALUE: u32 = 0;
}
