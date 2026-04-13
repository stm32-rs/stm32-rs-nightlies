///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `EN` reader - enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHEINV` writer - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect.
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHECMD` reader - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
pub type CACHECMD_R = crate::FieldReader;
///Field `CACHECMD` writer - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
pub type CACHECMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STARTCMD` writer - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect.
pub type STARTCMD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RHITMEN` reader - read-hit monitor enable
pub type RHITMEN_R = crate::BitReader;
///Field `RHITMEN` writer - read-hit monitor enable
pub type RHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMISSMEN` reader - read-miss monitor enable
pub type RMISSMEN_R = crate::BitReader;
///Field `RMISSMEN` writer - read-miss monitor enable
pub type RMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RHITMRST` reader - read-hit monitor reset
pub type RHITMRST_R = crate::BitReader;
///Field `RHITMRST` writer - read-hit monitor reset
pub type RHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMISSMRST` reader - read-miss monitor reset
pub type RMISSMRST_R = crate::BitReader;
///Field `RMISSMRST` writer - read-miss monitor reset
pub type RMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHITMEN` reader - write-hit monitor enable
pub type WHITMEN_R = crate::BitReader;
///Field `WHITMEN` writer - write-hit monitor enable
pub type WHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMISSMEN` reader - write-miss monitor enable
pub type WMISSMEN_R = crate::BitReader;
///Field `WMISSMEN` writer - write-miss monitor enable
pub type WMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHITMRST` reader - write-hit monitor reset
pub type WHITMRST_R = crate::BitReader;
///Field `WHITMRST` writer - write-hit monitor reset
pub type WHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMISSMRST` reader - write-miss monitor reset
pub type WMISSMRST_R = crate::BitReader;
///Field `WMISSMRST` writer - write-miss monitor reset
pub type WMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HBURST` reader - output burst type for cache master port read accesses Write access is always done in INCR burst type.
pub type HBURST_R = crate::BitReader;
///Field `HBURST` writer - output burst type for cache master port read accesses Write access is always done in INCR burst type.
pub type HBURST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - read-hit monitor reset
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - read-miss monitor reset
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - write-hit monitor reset
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - write-miss monitor reset
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type.
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("cachecmd", &self.cachecmd())
            .field("rhitmen", &self.rhitmen())
            .field("rmissmen", &self.rmissmen())
            .field("rhitmrst", &self.rhitmrst())
            .field("rmissmrst", &self.rmissmrst())
            .field("whitmen", &self.whitmen())
            .field("wmissmen", &self.wmissmen())
            .field("whitmrst", &self.whitmrst())
            .field("wmissmrst", &self.wmissmrst())
            .field("hburst", &self.hburst())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - full cache invalidation Can be set by software, only when EN = 1. Cleared by hardware when the BUSYF flag is set (during full cache invalidation operation). Writing 0 has no effect.
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W<'_, CRrs> {
        CACHEINV_W::new(self, 1)
    }
    ///Bits 8:10 - cache command maintenance operation (cleans and/or invalidates an address range) Can be set and cleared by software, only when no maintenance command is ongoing (BUSYCMDF = 0). others: reserved
    #[inline(always)]
    pub fn cachecmd(&mut self) -> CACHECMD_W<'_, CRrs> {
        CACHECMD_W::new(self, 8)
    }
    ///Bit 11 - starts maintenance command (maintenance operation defined in CACHECMD). Can be set by software, only when EN = 1, BUSYCMDF = 0, BUSYF = 0 and CACHECMD = 0b001, 0b010 or 0b011. Cleared by hardware when the BUSYCMDF flag is set (during cache maintenance operation). Writing 0 has no effect.
    #[inline(always)]
    pub fn startcmd(&mut self) -> STARTCMD_W<'_, CRrs> {
        STARTCMD_W::new(self, 11)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    pub fn rhitmen(&mut self) -> RHITMEN_W<'_, CRrs> {
        RHITMEN_W::new(self, 16)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<'_, CRrs> {
        RMISSMEN_W::new(self, 17)
    }
    ///Bit 18 - read-hit monitor reset
    #[inline(always)]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<'_, CRrs> {
        RHITMRST_W::new(self, 18)
    }
    ///Bit 19 - read-miss monitor reset
    #[inline(always)]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<'_, CRrs> {
        RMISSMRST_W::new(self, 19)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    pub fn whitmen(&mut self) -> WHITMEN_W<'_, CRrs> {
        WHITMEN_W::new(self, 20)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<'_, CRrs> {
        WMISSMEN_W::new(self, 21)
    }
    ///Bit 22 - write-hit monitor reset
    #[inline(always)]
    pub fn whitmrst(&mut self) -> WHITMRST_W<'_, CRrs> {
        WHITMRST_W::new(self, 22)
    }
    ///Bit 23 - write-miss monitor reset
    #[inline(always)]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<'_, CRrs> {
        WMISSMRST_W::new(self, 23)
    }
    ///Bit 31 - output burst type for cache master port read accesses Write access is always done in INCR burst type.
    #[inline(always)]
    pub fn hburst(&mut self) -> HBURST_W<'_, CRrs> {
        HBURST_W::new(self, 31)
    }
}
/**DCACHE control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#DCACHE:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
