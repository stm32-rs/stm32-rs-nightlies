#[doc = "Register `ICFR` writer"]
pub type W = crate::W<ICFRrs>;
#[doc = "Field `CC1IF` writer - Clear COMP channel 1 Interrupt Flag"]
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` writer - Clear COMP channel 2 Interrupt Flag"]
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 16 - Clear COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1if(&mut self) -> CC1IF_W<ICFRrs> {
        CC1IF_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2if(&mut self) -> CC2IF_W<ICFRrs> {
        CC2IF_W::new(self, 17)
    }
}
#[doc = "Comparator interrupt clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICFRrs;
impl crate::RegisterSpec for ICFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icfr::W`](W) writer structure"]
impl crate::Writable for ICFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICFR to value 0"]
impl crate::Resettable for ICFRrs {
    const RESET_VALUE: u32 = 0;
}
