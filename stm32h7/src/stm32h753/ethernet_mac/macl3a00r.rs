#[doc = "Register `MACL3A00R` reader"]
pub type R = crate::R<MACL3A00Rrs>;
#[doc = "Register `MACL3A00R` writer"]
pub type W = crate::W<MACL3A00Rrs>;
#[doc = "Field `L3A00` reader - Layer 3 Address 0 Field"]
pub type L3A00_R = crate::FieldReader<u32>;
#[doc = "Field `L3A00` writer - Layer 3 Address 0 Field"]
pub type L3A00_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 0 Field"]
    #[inline(always)]
    pub fn l3a00(&self) -> L3A00_R {
        L3A00_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 0 Field"]
    #[inline(always)]
    #[must_use]
    pub fn l3a00(&mut self) -> L3A00_W<MACL3A00Rrs> {
        L3A00_W::new(self, 0)
    }
}
#[doc = "MACL3A00R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a00r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a00r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A00Rrs;
impl crate::RegisterSpec for MACL3A00Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a00r::R`](R) reader structure"]
impl crate::Readable for MACL3A00Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl3a00r::W`](W) writer structure"]
impl crate::Writable for MACL3A00Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3A00R to value 0"]
impl crate::Resettable for MACL3A00Rrs {
    const RESET_VALUE: u32 = 0;
}
