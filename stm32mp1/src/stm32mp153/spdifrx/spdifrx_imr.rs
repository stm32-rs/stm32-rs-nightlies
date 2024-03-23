#[doc = "Register `SPDIFRX_IMR` reader"]
pub type R = crate::R<SPDIFRX_IMRrs>;
#[doc = "Register `SPDIFRX_IMR` writer"]
pub type W = crate::W<SPDIFRX_IMRrs>;
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub type RXNEIE_R = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRNEIE` reader - CSRNEIE"]
pub type CSRNEIE_R = crate::BitReader;
#[doc = "Field `CSRNEIE` writer - CSRNEIE"]
pub type CSRNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIE` reader - PERRIE"]
pub type PERRIE_R = crate::BitReader;
#[doc = "Field `PERRIE` writer - PERRIE"]
pub type PERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - OVRIE"]
pub type OVRIE_R = crate::BitReader;
#[doc = "Field `OVRIE` writer - OVRIE"]
pub type OVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBLKIE` reader - SBLKIE"]
pub type SBLKIE_R = crate::BitReader;
#[doc = "Field `SBLKIE` writer - SBLKIE"]
pub type SBLKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIE` reader - SYNCDIE"]
pub type SYNCDIE_R = crate::BitReader;
#[doc = "Field `SYNCDIE` writer - SYNCDIE"]
pub type SYNCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEIE` reader - IFEIE"]
pub type IFEIE_R = crate::BitReader;
#[doc = "Field `IFEIE` writer - IFEIE"]
pub type IFEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<SPDIFRX_IMRrs> {
        RXNEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CSRNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn csrneie(&mut self) -> CSRNEIE_W<SPDIFRX_IMRrs> {
        CSRNEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - PERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn perrie(&mut self) -> PERRIE_W<SPDIFRX_IMRrs> {
        PERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<SPDIFRX_IMRrs> {
        OVRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SBLKIE"]
    #[inline(always)]
    #[must_use]
    pub fn sblkie(&mut self) -> SBLKIE_W<SPDIFRX_IMRrs> {
        SBLKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - SYNCDIE"]
    #[inline(always)]
    #[must_use]
    pub fn syncdie(&mut self) -> SYNCDIE_W<SPDIFRX_IMRrs> {
        SYNCDIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - IFEIE"]
    #[inline(always)]
    #[must_use]
    pub fn ifeie(&mut self) -> IFEIE_W<SPDIFRX_IMRrs> {
        IFEIE_W::new(self, 6)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdifrx_imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdifrx_imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPDIFRX_IMRrs;
impl crate::RegisterSpec for SPDIFRX_IMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spdifrx_imr::R`](R) reader structure"]
impl crate::Readable for SPDIFRX_IMRrs {}
#[doc = "`write(|w| ..)` method takes [`spdifrx_imr::W`](W) writer structure"]
impl crate::Writable for SPDIFRX_IMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPDIFRX_IMR to value 0"]
impl crate::Resettable for SPDIFRX_IMRrs {
    const RESET_VALUE: u32 = 0;
}
