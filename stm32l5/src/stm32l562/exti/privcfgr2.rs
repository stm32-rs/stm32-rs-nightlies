///Register `PRIVCFGR2` reader
pub type R = crate::R<PRIVCFGR2rs>;
///Register `PRIVCFGR2` writer
pub type W = crate::W<PRIVCFGR2rs>;
///Field `PRIV32` reader - PRIV32
pub type PRIV32_R = crate::BitReader;
///Field `PRIV32` writer - PRIV32
pub type PRIV32_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV33` reader - PRIV33
pub type PRIV33_R = crate::BitReader;
///Field `PRIV33` writer - PRIV33
pub type PRIV33_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV34` reader - PRIV34
pub type PRIV34_R = crate::BitReader;
///Field `PRIV34` writer - PRIV34
pub type PRIV34_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV35` reader - PRIV35
pub type PRIV35_R = crate::BitReader;
///Field `PRIV35` writer - PRIV35
pub type PRIV35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV36` reader - PRIV36
pub type PRIV36_R = crate::BitReader;
///Field `PRIV36` writer - PRIV36
pub type PRIV36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV37` reader - PRIV37
pub type PRIV37_R = crate::BitReader;
///Field `PRIV37` writer - PRIV37
pub type PRIV37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV38` reader - PRIV38
pub type PRIV38_R = crate::BitReader;
///Field `PRIV38` writer - PRIV38
pub type PRIV38_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV39` reader - PRIV39
pub type PRIV39_R = crate::BitReader;
///Field `PRIV39` writer - PRIV39
pub type PRIV39_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV40` reader - PRIV40
pub type PRIV40_R = crate::BitReader;
///Field `PRIV40` writer - PRIV40
pub type PRIV40_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV41` reader - PRIV41
pub type PRIV41_R = crate::BitReader;
///Field `PRIV41` writer - PRIV41
pub type PRIV41_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV42` reader - PRIV42
pub type PRIV42_R = crate::BitReader;
///Field `PRIV42` writer - PRIV42
pub type PRIV42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PRIV32
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PRIV33
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PRIV34
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PRIV35
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PRIV36
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PRIV37
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRIV38
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PRIV39
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PRIV40
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PRIV41
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PRIV42
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR2")
            .field("priv32", &self.priv32())
            .field("priv33", &self.priv33())
            .field("priv34", &self.priv34())
            .field("priv35", &self.priv35())
            .field("priv36", &self.priv36())
            .field("priv37", &self.priv37())
            .field("priv38", &self.priv38())
            .field("priv39", &self.priv39())
            .field("priv40", &self.priv40())
            .field("priv41", &self.priv41())
            .field("priv42", &self.priv42())
            .finish()
    }
}
impl W {
    ///Bit 0 - PRIV32
    #[inline(always)]
    pub fn priv32(&mut self) -> PRIV32_W<'_, PRIVCFGR2rs> {
        PRIV32_W::new(self, 0)
    }
    ///Bit 1 - PRIV33
    #[inline(always)]
    pub fn priv33(&mut self) -> PRIV33_W<'_, PRIVCFGR2rs> {
        PRIV33_W::new(self, 1)
    }
    ///Bit 2 - PRIV34
    #[inline(always)]
    pub fn priv34(&mut self) -> PRIV34_W<'_, PRIVCFGR2rs> {
        PRIV34_W::new(self, 2)
    }
    ///Bit 3 - PRIV35
    #[inline(always)]
    pub fn priv35(&mut self) -> PRIV35_W<'_, PRIVCFGR2rs> {
        PRIV35_W::new(self, 3)
    }
    ///Bit 4 - PRIV36
    #[inline(always)]
    pub fn priv36(&mut self) -> PRIV36_W<'_, PRIVCFGR2rs> {
        PRIV36_W::new(self, 4)
    }
    ///Bit 5 - PRIV37
    #[inline(always)]
    pub fn priv37(&mut self) -> PRIV37_W<'_, PRIVCFGR2rs> {
        PRIV37_W::new(self, 5)
    }
    ///Bit 6 - PRIV38
    #[inline(always)]
    pub fn priv38(&mut self) -> PRIV38_W<'_, PRIVCFGR2rs> {
        PRIV38_W::new(self, 6)
    }
    ///Bit 7 - PRIV39
    #[inline(always)]
    pub fn priv39(&mut self) -> PRIV39_W<'_, PRIVCFGR2rs> {
        PRIV39_W::new(self, 7)
    }
    ///Bit 8 - PRIV40
    #[inline(always)]
    pub fn priv40(&mut self) -> PRIV40_W<'_, PRIVCFGR2rs> {
        PRIV40_W::new(self, 8)
    }
    ///Bit 9 - PRIV41
    #[inline(always)]
    pub fn priv41(&mut self) -> PRIV41_W<'_, PRIVCFGR2rs> {
        PRIV41_W::new(self, 9)
    }
    ///Bit 10 - PRIV42
    #[inline(always)]
    pub fn priv42(&mut self) -> PRIV42_W<'_, PRIVCFGR2rs> {
        PRIV42_W::new(self, 10)
    }
}
/**EXTI security enable register

You can [`read`](crate::Reg::read) this register and get [`privcfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#EXTI:PRIVCFGR2)*/
pub struct PRIVCFGR2rs;
impl crate::RegisterSpec for PRIVCFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr2::R`](R) reader structure
impl crate::Readable for PRIVCFGR2rs {}
///`write(|w| ..)` method takes [`privcfgr2::W`](W) writer structure
impl crate::Writable for PRIVCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR2 to value 0
impl crate::Resettable for PRIVCFGR2rs {}
