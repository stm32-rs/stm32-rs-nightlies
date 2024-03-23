#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `EN` reader - enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHEINV` writer - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHECMD` reader - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
pub type CACHECMD_R = crate::FieldReader;
#[doc = "Field `CACHECMD` writer - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
pub type CACHECMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STARTCMD` writer - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
pub type STARTCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHITMEN` reader - read-hit monitor enable"]
pub type RHITMEN_R = crate::BitReader;
#[doc = "Field `RHITMEN` writer - read-hit monitor enable"]
pub type RHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMISSMEN` reader - read-miss monitor enable"]
pub type RMISSMEN_R = crate::BitReader;
#[doc = "Field `RMISSMEN` writer - read-miss monitor enable"]
pub type RMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RHITMRST` reader - read-hit monitor reset"]
pub type RHITMRST_R = crate::BitReader;
#[doc = "Field `RHITMRST` writer - read-hit monitor reset"]
pub type RHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMISSMRST` reader - read-miss monitor reset"]
pub type RMISSMRST_R = crate::BitReader;
#[doc = "Field `RMISSMRST` writer - read-miss monitor reset"]
pub type RMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHITMEN` reader - write-hit monitor enable"]
pub type WHITMEN_R = crate::BitReader;
#[doc = "Field `WHITMEN` writer - write-hit monitor enable"]
pub type WHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMISSMEN` reader - write-miss monitor enable"]
pub type WMISSMEN_R = crate::BitReader;
#[doc = "Field `WMISSMEN` writer - write-miss monitor enable"]
pub type WMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WHITMRST` reader - write-hit monitor reset"]
pub type WHITMRST_R = crate::BitReader;
#[doc = "Field `WHITMRST` writer - write-hit monitor reset"]
pub type WHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WMISSMRST` reader - write-miss monitor reset"]
pub type WMISSMRST_R = crate::BitReader;
#[doc = "Field `WMISSMRST` writer - write-miss monitor reset"]
pub type WMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HBURST` reader - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
pub type HBURST_R = crate::BitReader;
#[doc = "Field `HBURST` writer - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - read-hit monitor enable"]
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - read-miss monitor enable"]
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - read-hit monitor reset"]
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - read-miss monitor reset"]
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - write-hit monitor enable"]
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - write-miss monitor enable"]
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - write-hit monitor reset"]
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - write-miss monitor reset"]
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<CRrs> {
        CACHEINV_W::new(self, 1)
    }
    #[doc = "Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cachecmd(&mut self) -> CACHECMD_W<CRrs> {
        CACHECMD_W::new(self, 8)
    }
    #[doc = "Bit 11 - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn startcmd(&mut self) -> STARTCMD_W<CRrs> {
        STARTCMD_W::new(self, 11)
    }
    #[doc = "Bit 16 - read-hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmen(&mut self) -> RHITMEN_W<CRrs> {
        RHITMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - read-miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<CRrs> {
        RMISSMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - read-hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<CRrs> {
        RHITMRST_W::new(self, 18)
    }
    #[doc = "Bit 19 - read-miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<CRrs> {
        RMISSMRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - write-hit monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn whitmen(&mut self) -> WHITMEN_W<CRrs> {
        WHITMEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - write-miss monitor enable"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<CRrs> {
        WMISSMEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - write-hit monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn whitmrst(&mut self) -> WHITMRST_W<CRrs> {
        WHITMRST_W::new(self, 22)
    }
    #[doc = "Bit 23 - write-miss monitor reset"]
    #[inline(always)]
    #[must_use]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<CRrs> {
        WMISSMRST_W::new(self, 23)
    }
    #[doc = "Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type."]
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<CRrs> {
        HBURST_W::new(self, 31)
    }
}
#[doc = "DCACHE control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
