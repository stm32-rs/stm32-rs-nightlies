#[doc = "Register `COMP_ICFR` reader"]
pub type R = crate::R<COMP_ICFRrs>;
#[doc = "Register `COMP_ICFR` writer"]
pub type W = crate::W<COMP_ICFRrs>;
#[doc = "Field `CC1IF` reader - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_R = crate::BitReader;
#[doc = "Field `CC1IF` writer - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clear COMP Channel1 interrupt flag Writing 1 clears the C1IF flag in the COMP_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<COMP_ICFRrs> {
        CC1IF_W::new(self, 16)
    }
}
#[doc = "Comparator interrupt clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_icfr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_icfr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP_ICFRrs;
impl crate::RegisterSpec for COMP_ICFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_icfr::R`](R) reader structure"]
impl crate::Readable for COMP_ICFRrs {}
#[doc = "`write(|w| ..)` method takes [`comp_icfr::W`](W) writer structure"]
impl crate::Writable for COMP_ICFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP_ICFR to value 0"]
impl crate::Resettable for COMP_ICFRrs {
    const RESET_VALUE: u32 = 0;
}
