///Register `PRIVCFGSR2` writer
pub type W = crate::W<PRIVCFGSR2rs>;
///Field `IC1PVS` writer - Defines the privilege protection of the IC1 configuration bits (enable, ready, divider).
pub type IC1PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2PVS` writer - Defines the privilege protection of the IC2 configuration bits (enable, ready, divider).
pub type IC2PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3PVS` writer - Defines the privilege protection of the IC3 configuration bits (enable, ready, divider).
pub type IC3PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4PVS` writer - Defines the privilege protection of the IC4 configuration bits (enable, ready, divider).
pub type IC4PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5PVS` writer - Defines the privilege protection of the IC5 configuration bits (enable, ready, divider).
pub type IC5PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6PVS` writer - Defines the privilege protection of the IC6 configuration bits (enable, ready, divider).
pub type IC6PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7PVS` writer - Defines the privilege protection of the IC7 configuration bits (enable, ready, divider).
pub type IC7PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8PVS` writer - Defines the privilege protection of the IC8 configuration bits (enable, ready, divider).
pub type IC8PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9PVS` writer - Defines the privilege protection of the IC9 configuration bits (enable, ready, divider).
pub type IC9PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10PVS` writer - Defines the privilege protection of the IC10 configuration bits (enable, ready, divider).
pub type IC10PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11PVS` writer - Defines the privilege protection of the IC11 configuration bits (enable, ready, divider).
pub type IC11PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12PVS` writer - Defines the privilege protection of the IC12 configuration bits (enable, ready, divider).
pub type IC12PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13PVS` writer - Defines the privilege protection of the IC13 configuration bits (enable, ready, divider).
pub type IC13PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14PVS` writer - Defines the privilege protection of the IC14 configuration bits (enable, ready, divider).
pub type IC14PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15PVS` writer - Defines the privilege protection of the IC15 configuration bits (enable, ready, divider).
pub type IC15PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16PVS` writer - Defines the privilege protection of the IC16 configuration bits (enable, ready, divider).
pub type IC16PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17PVS` writer - Defines the privilege protection of the IC17 configuration bits (enable, ready, divider).
pub type IC17PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18PVS` writer - Defines the privilege protection of the IC18 configuration bits (enable, ready, divider).
pub type IC18PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19PVS` writer - Defines the privilege protection of the IC19 configuration bits (enable, ready, divider).
pub type IC19PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20PVS` writer - Defines the privilege protection of the IC20 configuration bits (enable, ready, divider).
pub type IC20PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGSR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the IC1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic1pvs(&mut self) -> IC1PVS_W<'_, PRIVCFGSR2rs> {
        IC1PVS_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the IC2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic2pvs(&mut self) -> IC2PVS_W<'_, PRIVCFGSR2rs> {
        IC2PVS_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the IC3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic3pvs(&mut self) -> IC3PVS_W<'_, PRIVCFGSR2rs> {
        IC3PVS_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the IC4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic4pvs(&mut self) -> IC4PVS_W<'_, PRIVCFGSR2rs> {
        IC4PVS_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the IC5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic5pvs(&mut self) -> IC5PVS_W<'_, PRIVCFGSR2rs> {
        IC5PVS_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the IC6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic6pvs(&mut self) -> IC6PVS_W<'_, PRIVCFGSR2rs> {
        IC6PVS_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the IC7 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic7pvs(&mut self) -> IC7PVS_W<'_, PRIVCFGSR2rs> {
        IC7PVS_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the IC8 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic8pvs(&mut self) -> IC8PVS_W<'_, PRIVCFGSR2rs> {
        IC8PVS_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the IC9 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic9pvs(&mut self) -> IC9PVS_W<'_, PRIVCFGSR2rs> {
        IC9PVS_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the IC10 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic10pvs(&mut self) -> IC10PVS_W<'_, PRIVCFGSR2rs> {
        IC10PVS_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the IC11 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic11pvs(&mut self) -> IC11PVS_W<'_, PRIVCFGSR2rs> {
        IC11PVS_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the IC12 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic12pvs(&mut self) -> IC12PVS_W<'_, PRIVCFGSR2rs> {
        IC12PVS_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the IC13 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic13pvs(&mut self) -> IC13PVS_W<'_, PRIVCFGSR2rs> {
        IC13PVS_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the IC14 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic14pvs(&mut self) -> IC14PVS_W<'_, PRIVCFGSR2rs> {
        IC14PVS_W::new(self, 13)
    }
    ///Bit 14 - Defines the privilege protection of the IC15 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic15pvs(&mut self) -> IC15PVS_W<'_, PRIVCFGSR2rs> {
        IC15PVS_W::new(self, 14)
    }
    ///Bit 15 - Defines the privilege protection of the IC16 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic16pvs(&mut self) -> IC16PVS_W<'_, PRIVCFGSR2rs> {
        IC16PVS_W::new(self, 15)
    }
    ///Bit 16 - Defines the privilege protection of the IC17 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic17pvs(&mut self) -> IC17PVS_W<'_, PRIVCFGSR2rs> {
        IC17PVS_W::new(self, 16)
    }
    ///Bit 17 - Defines the privilege protection of the IC18 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic18pvs(&mut self) -> IC18PVS_W<'_, PRIVCFGSR2rs> {
        IC18PVS_W::new(self, 17)
    }
    ///Bit 18 - Defines the privilege protection of the IC19 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic19pvs(&mut self) -> IC19PVS_W<'_, PRIVCFGSR2rs> {
        IC19PVS_W::new(self, 18)
    }
    ///Bit 19 - Defines the privilege protection of the IC20 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic20pvs(&mut self) -> IC20PVS_W<'_, PRIVCFGSR2rs> {
        IC20PVS_W::new(self, 19)
    }
}
/**RCC divider privilege configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PRIVCFGSR2)*/
pub struct PRIVCFGSR2rs;
impl crate::RegisterSpec for PRIVCFGSR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgsr2::W`](W) writer structure
impl crate::Writable for PRIVCFGSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGSR2 to value 0
impl crate::Resettable for PRIVCFGSR2rs {}
