#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CCF` writer - Computation complete flag clear"]
pub type CCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWEIF` writer - Read or write error interrupt flag clear"]
pub type RWEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIF` writer - Key error interrupt flag clear"]
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEIF` writer - RNGEIF"]
pub type RNGEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Computation complete flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<ICRrs> {
        CCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Read or write error interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn rweif(&mut self) -> RWEIF_W<ICRrs> {
        RWEIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn keif(&mut self) -> KEIF_W<ICRrs> {
        KEIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - RNGEIF"]
    #[inline(always)]
    #[must_use]
    pub fn rngeif(&mut self) -> RNGEIF_W<ICRrs> {
        RNGEIF_W::new(self, 3)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
