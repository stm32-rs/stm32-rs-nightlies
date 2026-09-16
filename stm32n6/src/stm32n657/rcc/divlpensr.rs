///Register `DIVLPENSR` writer
pub type W = crate::W<DIVLPENSRrs>;
///Field `IC1LPENS` writer - IC1 sleep enable
pub type IC1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2LPENS` writer - IC2 sleep enable
pub type IC2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3LPENS` writer - IC3 sleep enable
pub type IC3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4LPENS` writer - IC4 sleep enable
pub type IC4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5LPENS` writer - IC5 sleep enable
pub type IC5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6LPENS` writer - IC6 sleep enable
pub type IC6LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7LPENS` writer - IC7 sleep enable
pub type IC7LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8LPENS` writer - IC8 sleep enable
pub type IC8LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9LPENS` writer - IC9 sleep enable
pub type IC9LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10LPENS` writer - IC10 sleep enable
pub type IC10LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11LPENS` writer - IC11 sleep enable
pub type IC11LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12LPENS` writer - IC12 sleep enable
pub type IC12LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13LPENS` writer - IC13 sleep enable
pub type IC13LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14LPENS` writer - IC14 sleep enable
pub type IC14LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15LPENS` writer - IC15 sleep enable
pub type IC15LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16LPENS` writer - IC16 sleep enable
pub type IC16LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17LPENS` writer - IC17 sleep enable
pub type IC17LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18LPENS` writer - IC18 sleep enable
pub type IC18LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19LPENS` writer - IC19 sleep enable
pub type IC19LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20LPENS` writer - IC20 sleep enable
pub type IC20LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DIVLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - IC1 sleep enable
    #[inline(always)]
    pub fn ic1lpens(&mut self) -> IC1LPENS_W<'_, DIVLPENSRrs> {
        IC1LPENS_W::new(self, 0)
    }
    ///Bit 1 - IC2 sleep enable
    #[inline(always)]
    pub fn ic2lpens(&mut self) -> IC2LPENS_W<'_, DIVLPENSRrs> {
        IC2LPENS_W::new(self, 1)
    }
    ///Bit 2 - IC3 sleep enable
    #[inline(always)]
    pub fn ic3lpens(&mut self) -> IC3LPENS_W<'_, DIVLPENSRrs> {
        IC3LPENS_W::new(self, 2)
    }
    ///Bit 3 - IC4 sleep enable
    #[inline(always)]
    pub fn ic4lpens(&mut self) -> IC4LPENS_W<'_, DIVLPENSRrs> {
        IC4LPENS_W::new(self, 3)
    }
    ///Bit 4 - IC5 sleep enable
    #[inline(always)]
    pub fn ic5lpens(&mut self) -> IC5LPENS_W<'_, DIVLPENSRrs> {
        IC5LPENS_W::new(self, 4)
    }
    ///Bit 5 - IC6 sleep enable
    #[inline(always)]
    pub fn ic6lpens(&mut self) -> IC6LPENS_W<'_, DIVLPENSRrs> {
        IC6LPENS_W::new(self, 5)
    }
    ///Bit 6 - IC7 sleep enable
    #[inline(always)]
    pub fn ic7lpens(&mut self) -> IC7LPENS_W<'_, DIVLPENSRrs> {
        IC7LPENS_W::new(self, 6)
    }
    ///Bit 7 - IC8 sleep enable
    #[inline(always)]
    pub fn ic8lpens(&mut self) -> IC8LPENS_W<'_, DIVLPENSRrs> {
        IC8LPENS_W::new(self, 7)
    }
    ///Bit 8 - IC9 sleep enable
    #[inline(always)]
    pub fn ic9lpens(&mut self) -> IC9LPENS_W<'_, DIVLPENSRrs> {
        IC9LPENS_W::new(self, 8)
    }
    ///Bit 9 - IC10 sleep enable
    #[inline(always)]
    pub fn ic10lpens(&mut self) -> IC10LPENS_W<'_, DIVLPENSRrs> {
        IC10LPENS_W::new(self, 9)
    }
    ///Bit 10 - IC11 sleep enable
    #[inline(always)]
    pub fn ic11lpens(&mut self) -> IC11LPENS_W<'_, DIVLPENSRrs> {
        IC11LPENS_W::new(self, 10)
    }
    ///Bit 11 - IC12 sleep enable
    #[inline(always)]
    pub fn ic12lpens(&mut self) -> IC12LPENS_W<'_, DIVLPENSRrs> {
        IC12LPENS_W::new(self, 11)
    }
    ///Bit 12 - IC13 sleep enable
    #[inline(always)]
    pub fn ic13lpens(&mut self) -> IC13LPENS_W<'_, DIVLPENSRrs> {
        IC13LPENS_W::new(self, 12)
    }
    ///Bit 13 - IC14 sleep enable
    #[inline(always)]
    pub fn ic14lpens(&mut self) -> IC14LPENS_W<'_, DIVLPENSRrs> {
        IC14LPENS_W::new(self, 13)
    }
    ///Bit 14 - IC15 sleep enable
    #[inline(always)]
    pub fn ic15lpens(&mut self) -> IC15LPENS_W<'_, DIVLPENSRrs> {
        IC15LPENS_W::new(self, 14)
    }
    ///Bit 15 - IC16 sleep enable
    #[inline(always)]
    pub fn ic16lpens(&mut self) -> IC16LPENS_W<'_, DIVLPENSRrs> {
        IC16LPENS_W::new(self, 15)
    }
    ///Bit 16 - IC17 sleep enable
    #[inline(always)]
    pub fn ic17lpens(&mut self) -> IC17LPENS_W<'_, DIVLPENSRrs> {
        IC17LPENS_W::new(self, 16)
    }
    ///Bit 17 - IC18 sleep enable
    #[inline(always)]
    pub fn ic18lpens(&mut self) -> IC18LPENS_W<'_, DIVLPENSRrs> {
        IC18LPENS_W::new(self, 17)
    }
    ///Bit 18 - IC19 sleep enable
    #[inline(always)]
    pub fn ic19lpens(&mut self) -> IC19LPENS_W<'_, DIVLPENSRrs> {
        IC19LPENS_W::new(self, 18)
    }
    ///Bit 19 - IC20 sleep enable
    #[inline(always)]
    pub fn ic20lpens(&mut self) -> IC20LPENS_W<'_, DIVLPENSRrs> {
        IC20LPENS_W::new(self, 19)
    }
}
/**RCC divider Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divlpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:DIVLPENSR)*/
pub struct DIVLPENSRrs;
impl crate::RegisterSpec for DIVLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`divlpensr::W`](W) writer structure
impl crate::Writable for DIVLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVLPENSR to value 0
impl crate::Resettable for DIVLPENSRrs {}
