#[doc = "Register `TZC_ACTION` reader"]
pub type R = crate::R<TZC_ACTIONrs>;
#[doc = "Register `TZC_ACTION` writer"]
pub type W = crate::W<TZC_ACTIONrs>;
#[doc = "Field `REACTION_VALUE` reader - REACTION_VALUE"]
pub type REACTION_VALUE_R = crate::FieldReader;
#[doc = "Field `REACTION_VALUE` writer - REACTION_VALUE"]
pub type REACTION_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    pub fn reaction_value(&self) -> REACTION_VALUE_R {
        REACTION_VALUE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - REACTION_VALUE"]
    #[inline(always)]
    #[must_use]
    pub fn reaction_value(&mut self) -> REACTION_VALUE_W<TZC_ACTIONrs> {
        REACTION_VALUE_W::new(self, 0)
    }
}
#[doc = "Controls interrupt and bus error response behavior when regions permission failures occur.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_action::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_action::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_ACTIONrs;
impl crate::RegisterSpec for TZC_ACTIONrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_action::R`](R) reader structure"]
impl crate::Readable for TZC_ACTIONrs {}
#[doc = "`write(|w| ..)` method takes [`tzc_action::W`](W) writer structure"]
impl crate::Writable for TZC_ACTIONrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_ACTION to value 0"]
impl crate::Resettable for TZC_ACTIONrs {
    const RESET_VALUE: u32 = 0;
}
