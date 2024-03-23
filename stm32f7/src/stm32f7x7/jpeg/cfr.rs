#[doc = "Register `CFR` writer"]
pub type W = crate::W<CFRrs>;
#[doc = "Field `CEOCF` writer - Clear End of Conversion Flag"]
pub type CEOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPDF` writer - Clear Header Parsing Done Flag"]
pub type CHPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ceocf(&mut self) -> CEOCF_W<CFRrs> {
        CEOCF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chpdf(&mut self) -> CHPDF_W<CFRrs> {
        CHPDF_W::new(self, 6)
    }
}
#[doc = "JPEG clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u32 = 0;
}
