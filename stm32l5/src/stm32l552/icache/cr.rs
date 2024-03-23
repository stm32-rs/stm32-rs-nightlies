#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEINV` reader - CACHEINV"]
pub type CACHEINV_R = crate::BitReader;
#[doc = "Field `CACHEINV` writer - CACHEINV"]
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAYSEL` reader - WAYSEL"]
pub type WAYSEL_R = crate::BitReader;
#[doc = "Field `WAYSEL` writer - WAYSEL"]
pub type WAYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HITMEN` reader - HITMEN"]
pub type HITMEN_R = crate::BitReader;
#[doc = "Field `HITMEN` writer - HITMEN"]
pub type HITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSMEN` reader - MISSMEN"]
pub type MISSMEN_R = crate::BitReader;
#[doc = "Field `MISSMEN` writer - MISSMEN"]
pub type MISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HITMRST` reader - HITMRST"]
pub type HITMRST_R = crate::BitReader;
#[doc = "Field `HITMRST` writer - HITMRST"]
pub type HITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSMRST` reader - MISSMRST"]
pub type MISSMRST_R = crate::BitReader;
#[doc = "Field `MISSMRST` writer - MISSMRST"]
pub type MISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    pub fn cacheinv(&self) -> CACHEINV_R {
        CACHEINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MISSMRST"]
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<CRrs> {
        CACHEINV_W::new(self, 1)
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    #[must_use]
    pub fn waysel(&mut self) -> WAYSEL_W<CRrs> {
        WAYSEL_W::new(self, 2)
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    #[must_use]
    pub fn hitmen(&mut self) -> HITMEN_W<CRrs> {
        HITMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn missmen(&mut self) -> MISSMEN_W<CRrs> {
        MISSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    #[must_use]
    pub fn hitmrst(&mut self) -> HITMRST_W<CRrs> {
        HITMRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - MISSMRST"]
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
