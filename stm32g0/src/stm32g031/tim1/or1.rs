#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1rs>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1rs>;
#[doc = "Field `OCREF_CLR` reader - Ocref_clr source selection"]
pub type OCREF_CLR_R = crate::BitReader;
#[doc = "Field `OCREF_CLR` writer - Ocref_clr source selection"]
pub type OCREF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W<OR1rs> {
        OCREF_CLR_W::new(self, 0)
    }
}
#[doc = "option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1rs {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1rs {
    const RESET_VALUE: u32 = 0;
}
