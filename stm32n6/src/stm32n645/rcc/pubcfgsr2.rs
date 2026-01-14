///Register `PUBCFGSR2` writer
pub type W = crate::W<PUBCFGSR2rs>;
///Field `IC1PUBS` writer - Defines the public protection of the IC1 configuration bits (enable, ready, divider).
pub type IC1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2PUBS` writer - Defines the public protection of the IC2 configuration bits (enable, ready, divider).
pub type IC2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3PUBS` writer - Defines the public protection of the IC3 configuration bits (enable, ready, divider).
pub type IC3PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4PUBS` writer - Defines the public protection of the IC4 configuration bits (enable, ready, divider).
pub type IC4PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5PUBS` writer - Defines the public protection of the IC5 configuration bits (enable, ready, divider).
pub type IC5PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6PUBS` writer - Defines the public protection of the IC6 configuration bits (enable, ready, divider).
pub type IC6PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7PUBS` writer - Defines the public protection of the IC7 configuration bits (enable, ready, divider).
pub type IC7PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8PUBS` writer - Defines the public protection of the IC8 configuration bits (enable, ready, divider).
pub type IC8PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9PUBS` writer - Defines the public protection of the IC9 configuration bits (enable, ready, divider).
pub type IC9PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10PUBS` writer - Defines the public protection of the IC10 configuration bits (enable, ready, divider).
pub type IC10PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11PUBS` writer - Defines the public protection of the IC11 configuration bits (enable, ready, divider).
pub type IC11PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12PUBS` writer - Defines the public protection of the IC12 configuration bits (enable, ready, divider).
pub type IC12PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13PUBS` writer - Defines the public protection of the IC13 configuration bits (enable, ready, divider).
pub type IC13PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14PUBS` writer - Defines the public protection of the IC14 configuration bits (enable, ready, divider).
pub type IC14PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15PUBS` writer - Defines the public protection of the IC15 configuration bits (enable, ready, divider).
pub type IC15PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16PUBS` writer - Defines the public protection of the IC16 configuration bits (enable, ready, divider).
pub type IC16PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17PUBS` writer - Defines the public protection of the IC17 configuration bits (enable, ready, divider).
pub type IC17PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18PUBS` writer - Defines the public protection of the IC18 configuration bits (enable, ready, divider).
pub type IC18PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19PUBS` writer - Defines the public protection of the IC19 configuration bits (enable, ready, divider).
pub type IC19PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20PUBS` writer - Defines the public protection of the IC20 configuration bits (enable, ready, divider).
pub type IC20PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the IC1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic1pubs(&mut self) -> IC1PUBS_W<'_, PUBCFGSR2rs> {
        IC1PUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the IC2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic2pubs(&mut self) -> IC2PUBS_W<'_, PUBCFGSR2rs> {
        IC2PUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the IC3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic3pubs(&mut self) -> IC3PUBS_W<'_, PUBCFGSR2rs> {
        IC3PUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the IC4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic4pubs(&mut self) -> IC4PUBS_W<'_, PUBCFGSR2rs> {
        IC4PUBS_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the IC5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic5pubs(&mut self) -> IC5PUBS_W<'_, PUBCFGSR2rs> {
        IC5PUBS_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the IC6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic6pubs(&mut self) -> IC6PUBS_W<'_, PUBCFGSR2rs> {
        IC6PUBS_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the IC7 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic7pubs(&mut self) -> IC7PUBS_W<'_, PUBCFGSR2rs> {
        IC7PUBS_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the IC8 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic8pubs(&mut self) -> IC8PUBS_W<'_, PUBCFGSR2rs> {
        IC8PUBS_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the IC9 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic9pubs(&mut self) -> IC9PUBS_W<'_, PUBCFGSR2rs> {
        IC9PUBS_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the IC10 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic10pubs(&mut self) -> IC10PUBS_W<'_, PUBCFGSR2rs> {
        IC10PUBS_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the IC11 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic11pubs(&mut self) -> IC11PUBS_W<'_, PUBCFGSR2rs> {
        IC11PUBS_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the IC12 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic12pubs(&mut self) -> IC12PUBS_W<'_, PUBCFGSR2rs> {
        IC12PUBS_W::new(self, 11)
    }
    ///Bit 12 - Defines the public protection of the IC13 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic13pubs(&mut self) -> IC13PUBS_W<'_, PUBCFGSR2rs> {
        IC13PUBS_W::new(self, 12)
    }
    ///Bit 13 - Defines the public protection of the IC14 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic14pubs(&mut self) -> IC14PUBS_W<'_, PUBCFGSR2rs> {
        IC14PUBS_W::new(self, 13)
    }
    ///Bit 14 - Defines the public protection of the IC15 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic15pubs(&mut self) -> IC15PUBS_W<'_, PUBCFGSR2rs> {
        IC15PUBS_W::new(self, 14)
    }
    ///Bit 15 - Defines the public protection of the IC16 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic16pubs(&mut self) -> IC16PUBS_W<'_, PUBCFGSR2rs> {
        IC16PUBS_W::new(self, 15)
    }
    ///Bit 16 - Defines the public protection of the IC17 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic17pubs(&mut self) -> IC17PUBS_W<'_, PUBCFGSR2rs> {
        IC17PUBS_W::new(self, 16)
    }
    ///Bit 17 - Defines the public protection of the IC18 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic18pubs(&mut self) -> IC18PUBS_W<'_, PUBCFGSR2rs> {
        IC18PUBS_W::new(self, 17)
    }
    ///Bit 18 - Defines the public protection of the IC19 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic19pubs(&mut self) -> IC19PUBS_W<'_, PUBCFGSR2rs> {
        IC19PUBS_W::new(self, 18)
    }
    ///Bit 19 - Defines the public protection of the IC20 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic20pubs(&mut self) -> IC20PUBS_W<'_, PUBCFGSR2rs> {
        IC20PUBS_W::new(self, 19)
    }
}
/**RCC divider public configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PUBCFGSR2)*/
pub struct PUBCFGSR2rs;
impl crate::RegisterSpec for PUBCFGSR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr2::W`](W) writer structure
impl crate::Writable for PUBCFGSR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR2 to value 0
impl crate::Resettable for PUBCFGSR2rs {}
