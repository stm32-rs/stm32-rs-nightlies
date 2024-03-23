#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `SEIF` writer - Security error interrupt flag clear This bit is written by application, and always read as 0."]
pub type SEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONEIF` writer - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0."]
pub type XONEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIF` writer - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again."]
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Security error interrupt flag clear This bit is written by application, and always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn seif(&mut self) -> SEIF_W<ICRrs> {
        SEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn xoneif(&mut self) -> XONEIF_W<ICRrs> {
        XONEIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again."]
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
