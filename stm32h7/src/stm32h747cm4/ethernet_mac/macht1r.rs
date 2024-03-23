#[doc = "Register `MACHT1R` reader"]
pub type R = crate::R<MACHT1Rrs>;
#[doc = "Register `MACHT1R` writer"]
pub type W = crate::W<MACHT1Rrs>;
#[doc = "Field `HT63T32` reader - MAC Hash Table Second 32 Bits"]
pub type HT63T32_R = crate::FieldReader<u32>;
#[doc = "Field `HT63T32` writer - MAC Hash Table Second 32 Bits"]
pub type HT63T32_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits"]
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits"]
    #[inline(always)]
    #[must_use]
    pub fn ht63t32(&mut self) -> HT63T32_W<MACHT1Rrs> {
        HT63T32_W::new(self, 0)
    }
}
#[doc = "Hash Table 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macht1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macht1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHT1Rrs;
impl crate::RegisterSpec for MACHT1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht1r::R`](R) reader structure"]
impl crate::Readable for MACHT1Rrs {}
#[doc = "`write(|w| ..)` method takes [`macht1r::W`](W) writer structure"]
impl crate::Writable for MACHT1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHT1R to value 0"]
impl crate::Resettable for MACHT1Rrs {
    const RESET_VALUE: u32 = 0;
}
