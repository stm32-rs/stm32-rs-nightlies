#[doc = "Register `WIFCR` reader"]
pub type R = crate::R<WIFCRrs>;
#[doc = "Register `WIFCR` writer"]
pub type W = crate::W<WIFCRrs>;
#[doc = "Field `CTEIF` reader - Clear tearing effect interrupt flag"]
pub type CTEIF_R = crate::BitReader;
#[doc = "Field `CTEIF` writer - Clear tearing effect interrupt flag"]
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERIF` reader - Clear end of refresh interrupt flag"]
pub type CERIF_R = crate::BitReader;
#[doc = "Field `CERIF` writer - Clear end of refresh interrupt flag"]
pub type CERIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLLIF` reader - Clear PLL lock interrupt flag"]
pub type CPLLLIF_R = crate::BitReader;
#[doc = "Field `CPLLLIF` writer - Clear PLL lock interrupt flag"]
pub type CPLLLIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLUIF` reader - Clear PLL unlock interrupt flag"]
pub type CPLLUIF_R = crate::BitReader;
#[doc = "Field `CPLLUIF` writer - Clear PLL unlock interrupt flag"]
pub type CPLLUIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRRIF` reader - Clear regulator ready interrupt flag"]
pub type CRRIF_R = crate::BitReader;
#[doc = "Field `CRRIF` writer - Clear regulator ready interrupt flag"]
pub type CRRIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear tearing effect interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear end of refresh interrupt flag"]
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear PLL lock interrupt flag"]
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear PLL unlock interrupt flag"]
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Clear regulator ready interrupt flag"]
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear tearing effect interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<WIFCRrs> {
        CTEIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear end of refresh interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cerif(&mut self) -> CERIF_W<WIFCRrs> {
        CERIF_W::new(self, 1)
    }
    #[doc = "Bit 9 - Clear PLL lock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cplllif(&mut self) -> CPLLLIF_W<WIFCRrs> {
        CPLLLIF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear PLL unlock interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cplluif(&mut self) -> CPLLUIF_W<WIFCRrs> {
        CPLLUIF_W::new(self, 10)
    }
    #[doc = "Bit 13 - Clear regulator ready interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<WIFCRrs> {
        CRRIF_W::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFCRrs;
impl crate::RegisterSpec for WIFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifcr::R`](R) reader structure"]
impl crate::Readable for WIFCRrs {}
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
