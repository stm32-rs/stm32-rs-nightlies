///Register `DIVENCR` writer
pub type W = crate::W<DIVENCRrs>;
///Field `IC1ENC` writer - IC1 enable
pub type IC1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2ENC` writer - IC2 enable
pub type IC2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3ENC` writer - IC3 enable
pub type IC3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4ENC` writer - IC4 enable
pub type IC4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5ENC` writer - IC5 enable
pub type IC5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6ENC` writer - IC6 enable
pub type IC6ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7ENC` writer - IC7 enable
pub type IC7ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8ENC` writer - IC8 enable
pub type IC8ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9ENC` writer - IC9 enable
pub type IC9ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10ENC` writer - IC10 enable
pub type IC10ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11ENC` writer - IC11 enable
pub type IC11ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12ENC` writer - IC12 enable
pub type IC12ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13ENC` writer - IC13 enable
pub type IC13ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14ENC` writer - IC14 enable
pub type IC14ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15ENC` writer - IC15 enable
pub type IC15ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16ENC` writer - IC16 enable
pub type IC16ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17ENC` writer - IC17 enable
pub type IC17ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18ENC` writer - IC18 enable
pub type IC18ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19ENC` writer - IC19 enable
pub type IC19ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20ENC` writer - IC20 enable
pub type IC20ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DIVENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - IC1 enable
    #[inline(always)]
    pub fn ic1enc(&mut self) -> IC1ENC_W<'_, DIVENCRrs> {
        IC1ENC_W::new(self, 0)
    }
    ///Bit 1 - IC2 enable
    #[inline(always)]
    pub fn ic2enc(&mut self) -> IC2ENC_W<'_, DIVENCRrs> {
        IC2ENC_W::new(self, 1)
    }
    ///Bit 2 - IC3 enable
    #[inline(always)]
    pub fn ic3enc(&mut self) -> IC3ENC_W<'_, DIVENCRrs> {
        IC3ENC_W::new(self, 2)
    }
    ///Bit 3 - IC4 enable
    #[inline(always)]
    pub fn ic4enc(&mut self) -> IC4ENC_W<'_, DIVENCRrs> {
        IC4ENC_W::new(self, 3)
    }
    ///Bit 4 - IC5 enable
    #[inline(always)]
    pub fn ic5enc(&mut self) -> IC5ENC_W<'_, DIVENCRrs> {
        IC5ENC_W::new(self, 4)
    }
    ///Bit 5 - IC6 enable
    #[inline(always)]
    pub fn ic6enc(&mut self) -> IC6ENC_W<'_, DIVENCRrs> {
        IC6ENC_W::new(self, 5)
    }
    ///Bit 6 - IC7 enable
    #[inline(always)]
    pub fn ic7enc(&mut self) -> IC7ENC_W<'_, DIVENCRrs> {
        IC7ENC_W::new(self, 6)
    }
    ///Bit 7 - IC8 enable
    #[inline(always)]
    pub fn ic8enc(&mut self) -> IC8ENC_W<'_, DIVENCRrs> {
        IC8ENC_W::new(self, 7)
    }
    ///Bit 8 - IC9 enable
    #[inline(always)]
    pub fn ic9enc(&mut self) -> IC9ENC_W<'_, DIVENCRrs> {
        IC9ENC_W::new(self, 8)
    }
    ///Bit 9 - IC10 enable
    #[inline(always)]
    pub fn ic10enc(&mut self) -> IC10ENC_W<'_, DIVENCRrs> {
        IC10ENC_W::new(self, 9)
    }
    ///Bit 10 - IC11 enable
    #[inline(always)]
    pub fn ic11enc(&mut self) -> IC11ENC_W<'_, DIVENCRrs> {
        IC11ENC_W::new(self, 10)
    }
    ///Bit 11 - IC12 enable
    #[inline(always)]
    pub fn ic12enc(&mut self) -> IC12ENC_W<'_, DIVENCRrs> {
        IC12ENC_W::new(self, 11)
    }
    ///Bit 12 - IC13 enable
    #[inline(always)]
    pub fn ic13enc(&mut self) -> IC13ENC_W<'_, DIVENCRrs> {
        IC13ENC_W::new(self, 12)
    }
    ///Bit 13 - IC14 enable
    #[inline(always)]
    pub fn ic14enc(&mut self) -> IC14ENC_W<'_, DIVENCRrs> {
        IC14ENC_W::new(self, 13)
    }
    ///Bit 14 - IC15 enable
    #[inline(always)]
    pub fn ic15enc(&mut self) -> IC15ENC_W<'_, DIVENCRrs> {
        IC15ENC_W::new(self, 14)
    }
    ///Bit 15 - IC16 enable
    #[inline(always)]
    pub fn ic16enc(&mut self) -> IC16ENC_W<'_, DIVENCRrs> {
        IC16ENC_W::new(self, 15)
    }
    ///Bit 16 - IC17 enable
    #[inline(always)]
    pub fn ic17enc(&mut self) -> IC17ENC_W<'_, DIVENCRrs> {
        IC17ENC_W::new(self, 16)
    }
    ///Bit 17 - IC18 enable
    #[inline(always)]
    pub fn ic18enc(&mut self) -> IC18ENC_W<'_, DIVENCRrs> {
        IC18ENC_W::new(self, 17)
    }
    ///Bit 18 - IC19 enable
    #[inline(always)]
    pub fn ic19enc(&mut self) -> IC19ENC_W<'_, DIVENCRrs> {
        IC19ENC_W::new(self, 18)
    }
    ///Bit 19 - IC20 enable
    #[inline(always)]
    pub fn ic20enc(&mut self) -> IC20ENC_W<'_, DIVENCRrs> {
        IC20ENC_W::new(self, 19)
    }
}
/**RCC divider enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:DIVENCR)*/
pub struct DIVENCRrs;
impl crate::RegisterSpec for DIVENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`divencr::W`](W) writer structure
impl crate::Writable for DIVENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIVENCR to value 0
impl crate::Resettable for DIVENCRrs {}
