#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `ITR1_RMP` reader - Internal trigger 1 remap"]
pub type ITR1_RMP_R = crate::FieldReader;
#[doc = "Field `ITR1_RMP` writer - Internal trigger 1 remap"]
pub type ITR1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 10:11 - Internal trigger 1 remap"]
    #[inline(always)]
    pub fn itr1_rmp(&self) -> ITR1_RMP_R {
        ITR1_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 10:11 - Internal trigger 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn itr1_rmp(&mut self) -> ITR1_RMP_W<ORrs> {
        ITR1_RMP_W::new(self, 10)
    }
}
#[doc = "TIM2 option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
