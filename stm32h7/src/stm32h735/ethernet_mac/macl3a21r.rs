#[doc = "Register `MACL3A21R` reader"]
pub type R = crate::R<MACL3A21Rrs>;
#[doc = "Register `MACL3A21R` writer"]
pub type W = crate::W<MACL3A21Rrs>;
#[doc = "Field `L3A21` reader - Layer 3 Address 2 Field"]
pub type L3A21_R = crate::FieldReader<u32>;
#[doc = "Field `L3A21` writer - Layer 3 Address 2 Field"]
pub type L3A21_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    pub fn l3a21(&self) -> L3A21_R {
        L3A21_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    #[must_use]
    pub fn l3a21(&mut self) -> L3A21_W<MACL3A21Rrs> {
        L3A21_W::new(self, 0)
    }
}
#[doc = "Layer3 address 2 filter 1 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a21r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a21r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A21Rrs;
impl crate::RegisterSpec for MACL3A21Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a21r::R`](R) reader structure"]
impl crate::Readable for MACL3A21Rrs {}
#[doc = "`write(|w| ..)` method takes [`macl3a21r::W`](W) writer structure"]
impl crate::Writable for MACL3A21Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3A21R to value 0"]
impl crate::Resettable for MACL3A21Rrs {
    const RESET_VALUE: u32 = 0;
}
