///Register `PRIVCFGCR2` writer
pub type W = crate::W<PRIVCFGCR2rs>;
///Field `IC1PVC` writer - Defines the privilege protection of the IC1 configuration bits (enable, ready, divider).
pub type IC1PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2PVC` writer - Defines the privilege protection of the IC2 configuration bits (enable, ready, divider).
pub type IC2PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3PVC` writer - Defines the privilege protection of the IC3 configuration bits (enable, ready, divider).
pub type IC3PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4PVC` writer - Defines the privilege protection of the IC4 configuration bits (enable, ready, divider).
pub type IC4PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5PVC` writer - Defines the privilege protection of the IC5 configuration bits (enable, ready, divider).
pub type IC5PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6PVC` writer - Defines the privilege protection of the IC6 configuration bits (enable, ready, divider).
pub type IC6PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7PVC` writer - Defines the privilege protection of the IC7 configuration bits (enable, ready, divider).
pub type IC7PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8PVC` writer - Defines the privilege protection of the IC8 configuration bits (enable, ready, divider).
pub type IC8PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9PVC` writer - Defines the privilege protection of the IC9 configuration bits (enable, ready, divider).
pub type IC9PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10PVC` writer - Defines the privilege protection of the IC10 configuration bits (enable, ready, divider).
pub type IC10PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11PVC` writer - Defines the privilege protection of the IC11 configuration bits (enable, ready, divider).
pub type IC11PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12PVC` writer - Defines the privilege protection of the IC12 configuration bits (enable, ready, divider).
pub type IC12PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13PVC` writer - Defines the privilege protection of the IC13 configuration bits (enable, ready, divider).
pub type IC13PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14PVC` writer - Defines the privilege protection of the IC14 configuration bits (enable, ready, divider).
pub type IC14PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15PVC` writer - Defines the privilege protection of the IC15 configuration bits (enable, ready, divider).
pub type IC15PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16PVC` writer - Defines the privilege protection of the IC16 configuration bits (enable, ready, divider).
pub type IC16PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17PVC` writer - Defines the privilege protection of the IC17 configuration bits (enable, ready, divider).
pub type IC17PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18PVC` writer - Defines the privilege protection of the IC18 configuration bits (enable, ready, divider).
pub type IC18PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19PVC` writer - Defines the privilege protection of the IC19 configuration bits (enable, ready, divider).
pub type IC19PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20PVC` writer - Defines the privilege protection of the IC20 configuration bits (enable, ready, divider).
pub type IC20PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the IC1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic1pvc(&mut self) -> IC1PVC_W<'_, PRIVCFGCR2rs> {
        IC1PVC_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the IC2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic2pvc(&mut self) -> IC2PVC_W<'_, PRIVCFGCR2rs> {
        IC2PVC_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the IC3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic3pvc(&mut self) -> IC3PVC_W<'_, PRIVCFGCR2rs> {
        IC3PVC_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the IC4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic4pvc(&mut self) -> IC4PVC_W<'_, PRIVCFGCR2rs> {
        IC4PVC_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the IC5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic5pvc(&mut self) -> IC5PVC_W<'_, PRIVCFGCR2rs> {
        IC5PVC_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the IC6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic6pvc(&mut self) -> IC6PVC_W<'_, PRIVCFGCR2rs> {
        IC6PVC_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the IC7 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic7pvc(&mut self) -> IC7PVC_W<'_, PRIVCFGCR2rs> {
        IC7PVC_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the IC8 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic8pvc(&mut self) -> IC8PVC_W<'_, PRIVCFGCR2rs> {
        IC8PVC_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the IC9 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic9pvc(&mut self) -> IC9PVC_W<'_, PRIVCFGCR2rs> {
        IC9PVC_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the IC10 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic10pvc(&mut self) -> IC10PVC_W<'_, PRIVCFGCR2rs> {
        IC10PVC_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the IC11 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic11pvc(&mut self) -> IC11PVC_W<'_, PRIVCFGCR2rs> {
        IC11PVC_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the IC12 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic12pvc(&mut self) -> IC12PVC_W<'_, PRIVCFGCR2rs> {
        IC12PVC_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the IC13 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic13pvc(&mut self) -> IC13PVC_W<'_, PRIVCFGCR2rs> {
        IC13PVC_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the IC14 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic14pvc(&mut self) -> IC14PVC_W<'_, PRIVCFGCR2rs> {
        IC14PVC_W::new(self, 13)
    }
    ///Bit 14 - Defines the privilege protection of the IC15 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic15pvc(&mut self) -> IC15PVC_W<'_, PRIVCFGCR2rs> {
        IC15PVC_W::new(self, 14)
    }
    ///Bit 15 - Defines the privilege protection of the IC16 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic16pvc(&mut self) -> IC16PVC_W<'_, PRIVCFGCR2rs> {
        IC16PVC_W::new(self, 15)
    }
    ///Bit 16 - Defines the privilege protection of the IC17 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic17pvc(&mut self) -> IC17PVC_W<'_, PRIVCFGCR2rs> {
        IC17PVC_W::new(self, 16)
    }
    ///Bit 17 - Defines the privilege protection of the IC18 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic18pvc(&mut self) -> IC18PVC_W<'_, PRIVCFGCR2rs> {
        IC18PVC_W::new(self, 17)
    }
    ///Bit 18 - Defines the privilege protection of the IC19 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic19pvc(&mut self) -> IC19PVC_W<'_, PRIVCFGCR2rs> {
        IC19PVC_W::new(self, 18)
    }
    ///Bit 19 - Defines the privilege protection of the IC20 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic20pvc(&mut self) -> IC20PVC_W<'_, PRIVCFGCR2rs> {
        IC20PVC_W::new(self, 19)
    }
}
/**RCC divider privilege configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PRIVCFGCR2)*/
pub struct PRIVCFGCR2rs;
impl crate::RegisterSpec for PRIVCFGCR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgcr2::W`](W) writer structure
impl crate::Writable for PRIVCFGCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGCR2 to value 0
impl crate::Resettable for PRIVCFGCR2rs {}
