#[doc = "Register `DCACHE_CR` reader"]
pub type R = crate::R<DCACHE_CRrs>;
#[doc = "Register `DCACHE_CR` writer"]
pub type W = crate::W<DCACHE_CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEINV` writer - CACHEINV"]
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHECMD` reader - CACHECMD"]
pub type CACHECMD_R = crate::FieldReader;
#[doc = "Field `CACHECMD` writer - CACHECMD"]
pub type CACHECMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STARTCMD` writer - STARTCMD"]
pub type STARTCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHITMEN` reader - RHITMEN"]
pub type RHITMEN_R = crate::BitReader;
#[doc = "Field `RHITMEN` writer - RHITMEN"]
pub type RHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMISSMEN` reader - RMISSMEN"]
pub type RMISSMEN_R = crate::BitReader;
#[doc = "Field `RMISSMEN` writer - RMISSMEN"]
pub type RMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHITMRST` reader - RHITMRST"]
pub type RHITMRST_R = crate::BitReader;
#[doc = "Field `RHITMRST` writer - RHITMRST"]
pub type RHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMISSMRST` reader - RMISSMRST"]
pub type RMISSMRST_R = crate::BitReader;
#[doc = "Field `RMISSMRST` writer - RMISSMRST"]
pub type RMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHITMEN` reader - WHITMEN"]
pub type WHITMEN_R = crate::BitReader;
#[doc = "Field `WHITMEN` writer - WHITMEN"]
pub type WHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMISSMEN` reader - WMISSMEN"]
pub type WMISSMEN_R = crate::BitReader;
#[doc = "Field `WMISSMEN` writer - WMISSMEN"]
pub type WMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHITMRST` reader - WHITMRST"]
pub type WHITMRST_R = crate::BitReader;
#[doc = "Field `WHITMRST` writer - WHITMRST"]
pub type WHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMISSMRST` reader - WMISSMRST"]
pub type WMISSMRST_R = crate::BitReader;
#[doc = "Field `WMISSMRST` writer - WMISSMRST"]
pub type WMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBURST` reader - HBURST"]
pub type HBURST_R = crate::BitReader;
#[doc = "Field `HBURST` writer - HBURST"]
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - CACHECMD"]
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - RHITMEN"]
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RMISSMEN"]
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RHITMRST"]
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RMISSMRST"]
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WHITMEN"]
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WMISSMEN"]
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WHITMRST"]
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WMISSMRST"]
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DCACHE_CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<DCACHE_CRrs> {
        CACHEINV_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - CACHECMD"]
    #[inline(always)]
    #[must_use]
    pub fn cachecmd(&mut self) -> CACHECMD_W<DCACHE_CRrs> {
        CACHECMD_W::new(self, 8)
    }
    #[doc = "Bit 11 - STARTCMD"]
    #[inline(always)]
    #[must_use]
    pub fn startcmd(&mut self) -> STARTCMD_W<DCACHE_CRrs> {
        STARTCMD_W::new(self, 11)
    }
    #[doc = "Bit 16 - RHITMEN"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmen(&mut self) -> RHITMEN_W<DCACHE_CRrs> {
        RHITMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - RMISSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<DCACHE_CRrs> {
        RMISSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - RHITMRST"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<DCACHE_CRrs> {
        RHITMRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - RMISSMRST"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<DCACHE_CRrs> {
        RMISSMRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - WHITMEN"]
    #[inline(always)]
    #[must_use]
    pub fn whitmen(&mut self) -> WHITMEN_W<DCACHE_CRrs> {
        WHITMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - WMISSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<DCACHE_CRrs> {
        WMISSMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - WHITMRST"]
    #[inline(always)]
    #[must_use]
    pub fn whitmrst(&mut self) -> WHITMRST_W<DCACHE_CRrs> {
        WHITMRST_W::new(self, 22)
    }
    #[doc = "Bit 23 - WMISSMRST"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<DCACHE_CRrs> {
        WMISSMRST_W::new(self, 23)
    }
    #[doc = "Bit 31 - HBURST"]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<DCACHE_CRrs> {
        HBURST_W::new(self, 31)
    }
}
#[doc = "DCACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_CRrs;
impl crate::RegisterSpec for DCACHE_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_cr::R`](R) reader structure"]
impl crate::Readable for DCACHE_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dcache_cr::W`](W) writer structure"]
impl crate::Writable for DCACHE_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_CR to value 0"]
impl crate::Resettable for DCACHE_CRrs {
    const RESET_VALUE: u32 = 0;
}
