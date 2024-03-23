#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CLIF` writer - Clears the Line Interrupt Flag"]
pub type CLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag"]
pub type CFUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag"]
pub type CTERRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRRIF` writer - Clears Register Reload Interrupt Flag"]
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clears the Line Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<ICRrs> {
        CLIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<ICRrs> {
        CFUIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the Transfer Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clears Register Reload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
#[doc = "LTDC Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
