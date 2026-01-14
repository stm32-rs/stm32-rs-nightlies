///Register `DIVENSR` writer
pub type W = crate::W<DIVENSRrs>;
///Field `IC1ENS` writer - IC1 enable
pub type IC1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2ENS` writer - IC2 enable
pub type IC2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3ENS` writer - IC3 enable
pub type IC3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4ENS` writer - IC4 enable
pub type IC4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5ENS` writer - IC5 enable
pub type IC5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6ENS` writer - IC6 enable
pub type IC6ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7ENS` writer - IC7 enable
pub type IC7ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8ENS` writer - IC8 enable
pub type IC8ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9ENS` writer - IC9 enable
pub type IC9ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10ENS` writer - IC10 enable
pub type IC10ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11ENS` writer - IC11 enable
pub type IC11ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12ENS` writer - IC12 enable
pub type IC12ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13ENS` writer - IC13 enable
pub type IC13ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14ENS` writer - IC14 enable
pub type IC14ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15ENS` writer - IC15 enable
pub type IC15ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16ENS` writer - IC16 enable
pub type IC16ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17ENS` writer - IC17 enable
pub type IC17ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18ENS` writer - IC18 enable
pub type IC18ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19ENS` writer - IC19 enable
pub type IC19ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20ENS` writer - IC20 enable
pub type IC20ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DIVENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - IC1 enable
    #[inline(always)]
    pub fn ic1ens(&mut self) -> IC1ENS_W<'_, DIVENSRrs> {
        IC1ENS_W::new(self, 0)
    }
    ///Bit 1 - IC2 enable
    #[inline(always)]
    pub fn ic2ens(&mut self) -> IC2ENS_W<'_, DIVENSRrs> {
        IC2ENS_W::new(self, 1)
    }
    ///Bit 2 - IC3 enable
    #[inline(always)]
    pub fn ic3ens(&mut self) -> IC3ENS_W<'_, DIVENSRrs> {
        IC3ENS_W::new(self, 2)
    }
    ///Bit 3 - IC4 enable
    #[inline(always)]
    pub fn ic4ens(&mut self) -> IC4ENS_W<'_, DIVENSRrs> {
        IC4ENS_W::new(self, 3)
    }
    ///Bit 4 - IC5 enable
    #[inline(always)]
    pub fn ic5ens(&mut self) -> IC5ENS_W<'_, DIVENSRrs> {
        IC5ENS_W::new(self, 4)
    }
    ///Bit 5 - IC6 enable
    #[inline(always)]
    pub fn ic6ens(&mut self) -> IC6ENS_W<'_, DIVENSRrs> {
        IC6ENS_W::new(self, 5)
    }
    ///Bit 6 - IC7 enable
    #[inline(always)]
    pub fn ic7ens(&mut self) -> IC7ENS_W<'_, DIVENSRrs> {
        IC7ENS_W::new(self, 6)
    }
    ///Bit 7 - IC8 enable
    #[inline(always)]
    pub fn ic8ens(&mut self) -> IC8ENS_W<'_, DIVENSRrs> {
        IC8ENS_W::new(self, 7)
    }
    ///Bit 8 - IC9 enable
    #[inline(always)]
    pub fn ic9ens(&mut self) -> IC9ENS_W<'_, DIVENSRrs> {
        IC9ENS_W::new(self, 8)
    }
    ///Bit 9 - IC10 enable
    #[inline(always)]
    pub fn ic10ens(&mut self) -> IC10ENS_W<'_, DIVENSRrs> {
        IC10ENS_W::new(self, 9)
    }
    ///Bit 10 - IC11 enable
    #[inline(always)]
    pub fn ic11ens(&mut self) -> IC11ENS_W<'_, DIVENSRrs> {
        IC11ENS_W::new(self, 10)
    }
    ///Bit 11 - IC12 enable
    #[inline(always)]
    pub fn ic12ens(&mut self) -> IC12ENS_W<'_, DIVENSRrs> {
        IC12ENS_W::new(self, 11)
    }
    ///Bit 12 - IC13 enable
    #[inline(always)]
    pub fn ic13ens(&mut self) -> IC13ENS_W<'_, DIVENSRrs> {
        IC13ENS_W::new(self, 12)
    }
    ///Bit 13 - IC14 enable
    #[inline(always)]
    pub fn ic14ens(&mut self) -> IC14ENS_W<'_, DIVENSRrs> {
        IC14ENS_W::new(self, 13)
    }
    ///Bit 14 - IC15 enable
    #[inline(always)]
    pub fn ic15ens(&mut self) -> IC15ENS_W<'_, DIVENSRrs> {
        IC15ENS_W::new(self, 14)
    }
    ///Bit 15 - IC16 enable
    #[inline(always)]
    pub fn ic16ens(&mut self) -> IC16ENS_W<'_, DIVENSRrs> {
        IC16ENS_W::new(self, 15)
    }
    ///Bit 16 - IC17 enable
    #[inline(always)]
    pub fn ic17ens(&mut self) -> IC17ENS_W<'_, DIVENSRrs> {
        IC17ENS_W::new(self, 16)
    }
    ///Bit 17 - IC18 enable
    #[inline(always)]
    pub fn ic18ens(&mut self) -> IC18ENS_W<'_, DIVENSRrs> {
        IC18ENS_W::new(self, 17)
    }
    ///Bit 18 - IC19 enable
    #[inline(always)]
    pub fn ic19ens(&mut self) -> IC19ENS_W<'_, DIVENSRrs> {
        IC19ENS_W::new(self, 18)
    }
    ///Bit 19 - IC20 enable
    #[inline(always)]
    pub fn ic20ens(&mut self) -> IC20ENS_W<'_, DIVENSRrs> {
        IC20ENS_W::new(self, 19)
    }
}
/**RCC Divider enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:DIVENSR)*/
pub struct DIVENSRrs;
impl crate::RegisterSpec for DIVENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`divensr::W`](W) writer structure
impl crate::Writable for DIVENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVENSR to value 0
impl crate::Resettable for DIVENSRrs {}
