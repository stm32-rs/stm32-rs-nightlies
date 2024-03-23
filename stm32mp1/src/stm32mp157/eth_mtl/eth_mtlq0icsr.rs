#[doc = "Register `ETH_MTLQ0ICSR` reader"]
pub type R = crate::R<ETH_MTLQ0ICSRrs>;
#[doc = "Register `ETH_MTLQ0ICSR` writer"]
pub type W = crate::W<ETH_MTLQ0ICSRrs>;
#[doc = "Field `TXUNFIS` reader - TXUNFIS"]
pub type TXUNFIS_R = crate::BitReader;
#[doc = "Field `ABPSIS` reader - ABPSIS"]
pub type ABPSIS_R = crate::BitReader;
#[doc = "Field `ABPSIS` writer - ABPSIS"]
pub type ABPSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUIE` reader - TXUIE"]
pub type TXUIE_R = crate::BitReader;
#[doc = "Field `TXUIE` writer - TXUIE"]
pub type TXUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABPSIE` reader - ABPSIE"]
pub type ABPSIE_R = crate::BitReader;
#[doc = "Field `ABPSIE` writer - ABPSIE"]
pub type ABPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVFIS` reader - RXOVFIS"]
pub type RXOVFIS_R = crate::BitReader;
#[doc = "Field `RXOVFIS` writer - RXOVFIS"]
pub type RXOVFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader;
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ABPSIS"]
    #[inline(always)]
    #[must_use]
    pub fn abpsis(&mut self) -> ABPSIS_W<ETH_MTLQ0ICSRrs> {
        ABPSIS_W::new(self, 1)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<ETH_MTLQ0ICSRrs> {
        TXUIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - ABPSIE"]
    #[inline(always)]
    #[must_use]
    pub fn abpsie(&mut self) -> ABPSIE_W<ETH_MTLQ0ICSRrs> {
        ABPSIE_W::new(self, 9)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<ETH_MTLQ0ICSRrs> {
        RXOVFIS_W::new(self, 16)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<ETH_MTLQ0ICSRrs> {
        RXOIE_W::new(self, 24)
    }
}
#[doc = "Queue 0 interrupt control status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtlq0icsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mtlq0icsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLQ0ICSRrs;
impl crate::RegisterSpec for ETH_MTLQ0ICSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtlq0icsr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLQ0ICSRrs {}
#[doc = "`write(|w| ..)` method takes [`eth_mtlq0icsr::W`](W) writer structure"]
impl crate::Writable for ETH_MTLQ0ICSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MTLQ0ICSR to value 0"]
impl crate::Resettable for ETH_MTLQ0ICSRrs {
    const RESET_VALUE: u32 = 0;
}
