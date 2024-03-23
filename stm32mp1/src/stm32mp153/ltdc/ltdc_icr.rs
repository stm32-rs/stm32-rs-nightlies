#[doc = "Register `LTDC_ICR` writer"]
pub type W = crate::W<LTDC_ICRrs>;
#[doc = "Field `CLIF` writer - CLIF"]
pub type CLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFUIF` writer - CFUIF"]
pub type CFUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTERRIF` writer - CTERRIF"]
pub type CTERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRRIF` writer - CRRIF"]
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CLIF"]
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<LTDC_ICRrs> {
        CLIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CFUIF"]
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<LTDC_ICRrs> {
        CFUIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTERRIF"]
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<LTDC_ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CRRIF"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<LTDC_ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
#[doc = "LTDC Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_ICRrs;
impl crate::RegisterSpec for LTDC_ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ltdc_icr::W`](W) writer structure"]
impl crate::Writable for LTDC_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_ICR to value 0"]
impl crate::Resettable for LTDC_ICRrs {
    const RESET_VALUE: u32 = 0;
}
