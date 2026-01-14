///Register `PRIVCFGR3` reader
pub type R = crate::R<PRIVCFGR3rs>;
///Register `PRIVCFGR3` writer
pub type W = crate::W<PRIVCFGR3rs>;
///Field `PRIV64` reader - Privilege enable on event input x
pub type PRIV64_R = crate::BitReader;
///Field `PRIV64` writer - Privilege enable on event input x
pub type PRIV64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV65` reader - Privilege enable on event input x
pub type PRIV65_R = crate::BitReader;
///Field `PRIV65` writer - Privilege enable on event input x
pub type PRIV65_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV66` reader - Privilege enable on event input x
pub type PRIV66_R = crate::BitReader;
///Field `PRIV66` writer - Privilege enable on event input x
pub type PRIV66_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV68` reader - Privilege enable on event input x
pub type PRIV68_R = crate::BitReader;
///Field `PRIV68` writer - Privilege enable on event input x
pub type PRIV68_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV69` reader - Privilege enable on event input x
pub type PRIV69_R = crate::BitReader;
///Field `PRIV69` writer - Privilege enable on event input x
pub type PRIV69_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV70` reader - Privilege enable on event input x
pub type PRIV70_R = crate::BitReader;
///Field `PRIV70` writer - Privilege enable on event input x
pub type PRIV70_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV71` reader - Privilege enable on event input x
pub type PRIV71_R = crate::BitReader;
///Field `PRIV71` writer - Privilege enable on event input x
pub type PRIV71_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV72` reader - Privilege enable on event input x
pub type PRIV72_R = crate::BitReader;
///Field `PRIV72` writer - Privilege enable on event input x
pub type PRIV72_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV73` reader - Privilege enable on event input x
pub type PRIV73_R = crate::BitReader;
///Field `PRIV73` writer - Privilege enable on event input x
pub type PRIV73_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV74` reader - Privilege enable on event input x
pub type PRIV74_R = crate::BitReader;
///Field `PRIV74` writer - Privilege enable on event input x
pub type PRIV74_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRIV77` reader - Privilege enable on event input 77
pub type PRIV77_R = crate::BitReader;
///Field `PRIV77` writer - Privilege enable on event input 77
pub type PRIV77_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv64(&self) -> PRIV64_R {
        PRIV64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv65(&self) -> PRIV65_R {
        PRIV65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv66(&self) -> PRIV66_R {
        PRIV66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv68(&self) -> PRIV68_R {
        PRIV68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv69(&self) -> PRIV69_R {
        PRIV69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv70(&self) -> PRIV70_R {
        PRIV70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv71(&self) -> PRIV71_R {
        PRIV71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv72(&self) -> PRIV72_R {
        PRIV72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv73(&self) -> PRIV73_R {
        PRIV73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv74(&self) -> PRIV74_R {
        PRIV74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Privilege enable on event input 77
    #[inline(always)]
    pub fn priv77(&self) -> PRIV77_R {
        PRIV77_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR3")
            .field("priv64", &self.priv64())
            .field("priv65", &self.priv65())
            .field("priv66", &self.priv66())
            .field("priv68", &self.priv68())
            .field("priv69", &self.priv69())
            .field("priv70", &self.priv70())
            .field("priv71", &self.priv71())
            .field("priv72", &self.priv72())
            .field("priv73", &self.priv73())
            .field("priv74", &self.priv74())
            .field("priv77", &self.priv77())
            .finish()
    }
}
impl W {
    ///Bit 0 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv64(&mut self) -> PRIV64_W<'_, PRIVCFGR3rs> {
        PRIV64_W::new(self, 0)
    }
    ///Bit 1 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv65(&mut self) -> PRIV65_W<'_, PRIVCFGR3rs> {
        PRIV65_W::new(self, 1)
    }
    ///Bit 2 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv66(&mut self) -> PRIV66_W<'_, PRIVCFGR3rs> {
        PRIV66_W::new(self, 2)
    }
    ///Bit 4 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv68(&mut self) -> PRIV68_W<'_, PRIVCFGR3rs> {
        PRIV68_W::new(self, 4)
    }
    ///Bit 5 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv69(&mut self) -> PRIV69_W<'_, PRIVCFGR3rs> {
        PRIV69_W::new(self, 5)
    }
    ///Bit 6 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv70(&mut self) -> PRIV70_W<'_, PRIVCFGR3rs> {
        PRIV70_W::new(self, 6)
    }
    ///Bit 7 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv71(&mut self) -> PRIV71_W<'_, PRIVCFGR3rs> {
        PRIV71_W::new(self, 7)
    }
    ///Bit 8 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv72(&mut self) -> PRIV72_W<'_, PRIVCFGR3rs> {
        PRIV72_W::new(self, 8)
    }
    ///Bit 9 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv73(&mut self) -> PRIV73_W<'_, PRIVCFGR3rs> {
        PRIV73_W::new(self, 9)
    }
    ///Bit 10 - Privilege enable on event input x
    #[inline(always)]
    pub fn priv74(&mut self) -> PRIV74_W<'_, PRIVCFGR3rs> {
        PRIV74_W::new(self, 10)
    }
    ///Bit 13 - Privilege enable on event input 77
    #[inline(always)]
    pub fn priv77(&mut self) -> PRIV77_W<'_, PRIVCFGR3rs> {
        PRIV77_W::new(self, 13)
    }
}
/**EXTI privilege enable register

You can [`read`](crate::Reg::read) this register and get [`privcfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#EXTI:PRIVCFGR3)*/
pub struct PRIVCFGR3rs;
impl crate::RegisterSpec for PRIVCFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr3::R`](R) reader structure
impl crate::Readable for PRIVCFGR3rs {}
///`write(|w| ..)` method takes [`privcfgr3::W`](W) writer structure
impl crate::Writable for PRIVCFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR3 to value 0
impl crate::Resettable for PRIVCFGR3rs {}
