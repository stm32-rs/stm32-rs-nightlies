#[doc = "Register `MACL3A30` reader"]
pub type R = crate::R<MACL3A30rs>;
#[doc = "Register `MACL3A30` writer"]
pub type W = crate::W<MACL3A30rs>;
#[doc = "Field `L3A30` reader - Layer 3 Address 3 Field"]
pub type L3A30_R = crate::FieldReader<u32>;
#[doc = "Field `L3A30` writer - Layer 3 Address 3 Field"]
pub type L3A30_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field"]
    #[inline(always)]
    pub fn l3a30(&self) -> L3A30_R {
        L3A30_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 3 Field"]
    #[inline(always)]
    #[must_use]
    pub fn l3a30(&mut self) -> L3A30_W<MACL3A30rs> {
        L3A30_W::new(self, 0)
    }
}
#[doc = "Layer3 Address 3 filter 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macl3a30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macl3a30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACL3A30rs;
impl crate::RegisterSpec for MACL3A30rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a30::R`](R) reader structure"]
impl crate::Readable for MACL3A30rs {}
#[doc = "`write(|w| ..)` method takes [`macl3a30::W`](W) writer structure"]
impl crate::Writable for MACL3A30rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACL3A30 to value 0"]
impl crate::Resettable for MACL3A30rs {
    const RESET_VALUE: u32 = 0;
}
