#[doc = "Register `WIFCR` writer"]
pub type W = crate::W<WIFCRrs>;
#[doc = "Field `CTEIF` writer - CTEIF"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERIF` writer - CERIF"]
pub type CERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLLIF` writer - CPLLLIF"]
pub type CPLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLUIF` writer - CPLLUIF"]
pub type CPLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRRIF` writer - CRRIF"]
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<WIFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - CERIF"]
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<WIFCRrs> {
        CERIF_W::new(self, 1)
    }
    #[doc = "Bit 9 - CPLLLIF"]
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<WIFCRrs> {
        CPLLLIF_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPLLUIF"]
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<WIFCRrs> {
        CPLLUIF_W::new(self, 10)
    }
    #[doc = "Bit 13 - CRRIF"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<WIFCRrs> {
        CRRIF_W::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFCRrs;
impl crate::RegisterSpec for WIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wifcr::W`](W) writer structure"]
impl crate::Writable for WIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFCR to value 0"]
impl crate::Resettable for WIFCRrs {
    const RESET_VALUE: u32 = 0;
}
