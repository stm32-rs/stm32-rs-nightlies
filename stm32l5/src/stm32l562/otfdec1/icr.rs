#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `SEIF` writer - SEIF"]
pub type SEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONEIF` writer - Execute-only execute-Never Error Interrupt Flag clear"]
pub type XONEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIF` writer - KEIF"]
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SEIF"]
    #[inline(always)]
    #[must_use]
    pub fn seif(&mut self) -> SEIF_W<ICRrs> {
        SEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Execute-only execute-Never Error Interrupt Flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn xoneif(&mut self) -> XONEIF_W<ICRrs> {
        XONEIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - KEIF"]
    #[inline(always)]
    #[must_use]
    pub fn keif(&mut self) -> KEIF_W<ICRrs> {
        KEIF_W::new(self, 2)
    }
}
#[doc = "OTFDEC interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
