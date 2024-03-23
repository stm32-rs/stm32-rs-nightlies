#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEINV` writer - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAYSEL` reader - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_R = crate::BitReader;
#[doc = "Field `WAYSEL` writer - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
pub type WAYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HITMEN` reader - hit monitor enable"]
pub type HITMEN_R = crate::BitReader;
#[doc = "Field `HITMEN` writer - hit monitor enable"]
pub type HITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSMEN` reader - miss monitor enable"]
pub type MISSMEN_R = crate::BitReader;
#[doc = "Field `MISSMEN` writer - miss monitor enable"]
pub type MISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HITMRST` reader - hit monitor reset"]
pub type HITMRST_R = crate::BitReader;
#[doc = "Field `HITMRST` writer - hit monitor reset"]
pub type HITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSMRST` reader - miss monitor reset"]
pub type MISSMRST_R = crate::BitReader;
#[doc = "Field `MISSMRST` writer - miss monitor reset"]
pub type MISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - cache invalidation Set by software and cleared by hardware when the BUSYF flag is set (during cache maintenance operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<CRrs> {
        CACHEINV_W::new(self, 1)
    }
    #[doc = "Bit 2 - cache associativity mode selection This bit allows user to choose ICACHE set-associativity. It can be written by software only when cache is disabled (EN = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn waysel(&mut self) -> WAYSEL_W<CRrs> {
        WAYSEL_W::new(self, 2)
    }
    #[doc = "Bit 16 - hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn hitmen(&mut self) -> HITMEN_W<CRrs> {
        HITMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn missmen(&mut self) -> MISSMEN_W<CRrs> {
        MISSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn hitmrst(&mut self) -> HITMRST_W<CRrs> {
        HITMRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn missmrst(&mut self) -> MISSMRST_W<CRrs> {
        MISSMRST_W::new(self, 19)
    }
}
#[doc = "ICACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0x04"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x04;
}
