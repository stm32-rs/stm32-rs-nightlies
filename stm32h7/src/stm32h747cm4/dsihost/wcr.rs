#[doc = "Register `WCR` reader"]
pub type R = crate::R<WCRrs>;
#[doc = "Register `WCR` writer"]
pub type W = crate::W<WCRrs>;
#[doc = "Field `COLM` reader - Color mode"]
pub type COLM_R = crate::BitReader;
#[doc = "Field `COLM` writer - Color mode"]
pub type COLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHTDN` reader - Shutdown"]
pub type SHTDN_R = crate::BitReader;
#[doc = "Field `SHTDN` writer - Shutdown"]
pub type SHTDN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCEN` reader - LTDC enable"]
pub type LTDCEN_R = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDC enable"]
pub type LTDCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIEN` reader - DSI enable"]
pub type DSIEN_R = crate::BitReader;
#[doc = "Field `DSIEN` writer - DSI enable"]
pub type DSIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Color mode"]
    #[inline(always)]
    pub fn colm(&self) -> COLM_R {
        COLM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    pub fn shtdn(&self) -> SHTDN_R {
        SHTDN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LTDC enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSI enable"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Color mode"]
    #[inline(always)]
    #[must_use]
    pub fn colm(&mut self) -> COLM_W<WCRrs> {
        COLM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Shutdown"]
    #[inline(always)]
    #[must_use]
    pub fn shtdn(&mut self) -> SHTDN_W<WCRrs> {
        SHTDN_W::new(self, 1)
    }
    #[doc = "Bit 2 - LTDC enable"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcen(&mut self) -> LTDCEN_W<WCRrs> {
        LTDCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - DSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsien(&mut self) -> DSIEN_W<WCRrs> {
        DSIEN_W::new(self, 3)
    }
}
#[doc = "DSI wrapper control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCRrs;
impl crate::RegisterSpec for WCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wcr::R`](R) reader structure"]
impl crate::Readable for WCRrs {}
#[doc = "`write(|w| ..)` method takes [`wcr::W`](W) writer structure"]
impl crate::Writable for WCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WCR to value 0"]
impl crate::Resettable for WCRrs {
    const RESET_VALUE: u32 = 0;
}
