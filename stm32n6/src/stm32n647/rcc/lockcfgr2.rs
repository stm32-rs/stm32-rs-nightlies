///Register `LOCKCFGR2` writer
pub type W = crate::W<LOCKCFGR2rs>;
///Field `IC1LOCK` writer - Defines the lock protection of the IC1 divider configuration bits.
pub type IC1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2LOCK` writer - Defines the lock protection of the IC2 divider configuration bits.
pub type IC2LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3LOCK` writer - Defines the lock protection of the IC3 divider configuration bits.
pub type IC3LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4LOCK` writer - Defines the lock protection of the IC4 divider configuration bits.
pub type IC4LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5LOCK` writer - Defines the lock protection of the IC5 divider configuration bits.
pub type IC5LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6LOCK` writer - Defines the lock protection of the IC6 divider configuration bits.
pub type IC6LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7LOCK` writer - Defines the lock protection of the IC7 divider configuration bits.
pub type IC7LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8LOCK` writer - Defines the lock protection of the IC8 divider configuration bits.
pub type IC8LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9LOCK` writer - Defines the lock protection of the IC9 divider configuration bits.
pub type IC9LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10LOCK` writer - Defines the lock protection of the IC10 divider configuration bits.
pub type IC10LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11LOCK` writer - Defines the lock protection of the IC11 divider configuration bits.
pub type IC11LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12LOCK` writer - Defines the lock protection of the IC12 divider configuration bits.
pub type IC12LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13LOCK` writer - Defines the lock protection of the IC13 divider configuration bits.
pub type IC13LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14LOCK` writer - Defines the lock protection of the IC14 divider configuration bits.
pub type IC14LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15LOCK` writer - Defines the lock protection of the IC15 divider configuration bits.
pub type IC15LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16LOCK` writer - Defines the lock protection of the IC16 divider configuration bits.
pub type IC16LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17LOCK` writer - Defines the lock protection of the IC17 divider configuration bits.
pub type IC17LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18LOCK` writer - Defines the lock protection of the IC18 divider configuration bits.
pub type IC18LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19LOCK` writer - Defines the lock protection of the IC19 divider configuration bits.
pub type IC19LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20LOCK` writer - Defines the lock protection of the IC20 divider configuration bits.
pub type IC20LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the IC1 divider configuration bits.
    #[inline(always)]
    pub fn ic1lock(&mut self) -> IC1LOCK_W<'_, LOCKCFGR2rs> {
        IC1LOCK_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the IC2 divider configuration bits.
    #[inline(always)]
    pub fn ic2lock(&mut self) -> IC2LOCK_W<'_, LOCKCFGR2rs> {
        IC2LOCK_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the IC3 divider configuration bits.
    #[inline(always)]
    pub fn ic3lock(&mut self) -> IC3LOCK_W<'_, LOCKCFGR2rs> {
        IC3LOCK_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the IC4 divider configuration bits.
    #[inline(always)]
    pub fn ic4lock(&mut self) -> IC4LOCK_W<'_, LOCKCFGR2rs> {
        IC4LOCK_W::new(self, 3)
    }
    ///Bit 4 - Defines the lock protection of the IC5 divider configuration bits.
    #[inline(always)]
    pub fn ic5lock(&mut self) -> IC5LOCK_W<'_, LOCKCFGR2rs> {
        IC5LOCK_W::new(self, 4)
    }
    ///Bit 5 - Defines the lock protection of the IC6 divider configuration bits.
    #[inline(always)]
    pub fn ic6lock(&mut self) -> IC6LOCK_W<'_, LOCKCFGR2rs> {
        IC6LOCK_W::new(self, 5)
    }
    ///Bit 6 - Defines the lock protection of the IC7 divider configuration bits.
    #[inline(always)]
    pub fn ic7lock(&mut self) -> IC7LOCK_W<'_, LOCKCFGR2rs> {
        IC7LOCK_W::new(self, 6)
    }
    ///Bit 7 - Defines the lock protection of the IC8 divider configuration bits.
    #[inline(always)]
    pub fn ic8lock(&mut self) -> IC8LOCK_W<'_, LOCKCFGR2rs> {
        IC8LOCK_W::new(self, 7)
    }
    ///Bit 8 - Defines the lock protection of the IC9 divider configuration bits.
    #[inline(always)]
    pub fn ic9lock(&mut self) -> IC9LOCK_W<'_, LOCKCFGR2rs> {
        IC9LOCK_W::new(self, 8)
    }
    ///Bit 9 - Defines the lock protection of the IC10 divider configuration bits.
    #[inline(always)]
    pub fn ic10lock(&mut self) -> IC10LOCK_W<'_, LOCKCFGR2rs> {
        IC10LOCK_W::new(self, 9)
    }
    ///Bit 10 - Defines the lock protection of the IC11 divider configuration bits.
    #[inline(always)]
    pub fn ic11lock(&mut self) -> IC11LOCK_W<'_, LOCKCFGR2rs> {
        IC11LOCK_W::new(self, 10)
    }
    ///Bit 11 - Defines the lock protection of the IC12 divider configuration bits.
    #[inline(always)]
    pub fn ic12lock(&mut self) -> IC12LOCK_W<'_, LOCKCFGR2rs> {
        IC12LOCK_W::new(self, 11)
    }
    ///Bit 12 - Defines the lock protection of the IC13 divider configuration bits.
    #[inline(always)]
    pub fn ic13lock(&mut self) -> IC13LOCK_W<'_, LOCKCFGR2rs> {
        IC13LOCK_W::new(self, 12)
    }
    ///Bit 13 - Defines the lock protection of the IC14 divider configuration bits.
    #[inline(always)]
    pub fn ic14lock(&mut self) -> IC14LOCK_W<'_, LOCKCFGR2rs> {
        IC14LOCK_W::new(self, 13)
    }
    ///Bit 14 - Defines the lock protection of the IC15 divider configuration bits.
    #[inline(always)]
    pub fn ic15lock(&mut self) -> IC15LOCK_W<'_, LOCKCFGR2rs> {
        IC15LOCK_W::new(self, 14)
    }
    ///Bit 15 - Defines the lock protection of the IC16 divider configuration bits.
    #[inline(always)]
    pub fn ic16lock(&mut self) -> IC16LOCK_W<'_, LOCKCFGR2rs> {
        IC16LOCK_W::new(self, 15)
    }
    ///Bit 16 - Defines the lock protection of the IC17 divider configuration bits.
    #[inline(always)]
    pub fn ic17lock(&mut self) -> IC17LOCK_W<'_, LOCKCFGR2rs> {
        IC17LOCK_W::new(self, 16)
    }
    ///Bit 17 - Defines the lock protection of the IC18 divider configuration bits.
    #[inline(always)]
    pub fn ic18lock(&mut self) -> IC18LOCK_W<'_, LOCKCFGR2rs> {
        IC18LOCK_W::new(self, 17)
    }
    ///Bit 18 - Defines the lock protection of the IC19 divider configuration bits.
    #[inline(always)]
    pub fn ic19lock(&mut self) -> IC19LOCK_W<'_, LOCKCFGR2rs> {
        IC19LOCK_W::new(self, 18)
    }
    ///Bit 19 - Defines the lock protection of the IC20 divider configuration bits.
    #[inline(always)]
    pub fn ic20lock(&mut self) -> IC20LOCK_W<'_, LOCKCFGR2rs> {
        IC20LOCK_W::new(self, 19)
    }
}
/**RCC divider lock configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR2)*/
pub struct LOCKCFGR2rs;
impl crate::RegisterSpec for LOCKCFGR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgr2::W`](W) writer structure
impl crate::Writable for LOCKCFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGR2 to value 0
impl crate::Resettable for LOCKCFGR2rs {}
