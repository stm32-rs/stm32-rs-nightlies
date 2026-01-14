///Register `PUBCFGCR2` writer
pub type W = crate::W<PUBCFGCR2rs>;
///Field `IC1PUBC` writer - Defines the public protection of the IC1 configuration bits (enable, ready, divider).
pub type IC1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC2PUBC` writer - Defines the public protection of the IC2 configuration bits (enable, ready, divider).
pub type IC2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC3PUBC` writer - Defines the public protection of the IC3 configuration bits (enable, ready, divider).
pub type IC3PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC4PUBC` writer - Defines the public protection of the IC4 configuration bits (enable, ready, divider).
pub type IC4PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC5PUBC` writer - Defines the public protection of the IC5 configuration bits (enable, ready, divider).
pub type IC5PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC6PUBC` writer - Defines the public protection of the IC6 configuration bits (enable, ready, divider).
pub type IC6PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC7PUBC` writer - Defines the public protection of the IC7 configuration bits (enable, ready, divider).
pub type IC7PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC8PUBC` writer - Defines the public protection of the IC8 configuration bits (enable, ready, divider).
pub type IC8PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC9PUBC` writer - Defines the public protection of the IC9 configuration bits (enable, ready, divider).
pub type IC9PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC10PUBC` writer - Defines the public protection of the IC10 configuration bits (enable, ready, divider).
pub type IC10PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC11PUBC` writer - Defines the public protection of the IC11 configuration bits (enable, ready, divider).
pub type IC11PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC12PUBC` writer - Defines the public protection of the IC12 configuration bits (enable, ready, divider).
pub type IC12PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC13PUBC` writer - Defines the public protection of the IC13 configuration bits (enable, ready, divider).
pub type IC13PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC14PUBC` writer - Defines the public protection of the IC14 configuration bits (enable, ready, divider).
pub type IC14PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC15PUBC` writer - Defines the public protection of the IC15 configuration bits (enable, ready, divider).
pub type IC15PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC16PUBC` writer - Defines the public protection of the IC16 configuration bits (enable, ready, divider).
pub type IC16PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC17PUBC` writer - Defines the public protection of the IC17 configuration bits (enable, ready, divider).
pub type IC17PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC18PUBC` writer - Defines the public protection of the IC18 configuration bits (enable, ready, divider).
pub type IC18PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC19PUBC` writer - Defines the public protection of the IC19 configuration bits (enable, ready, divider).
pub type IC19PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IC20PUBC` writer - Defines the public protection of the IC20 configuration bits (enable, ready, divider).
pub type IC20PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the IC1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic1pubc(&mut self) -> IC1PUBC_W<'_, PUBCFGCR2rs> {
        IC1PUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the IC2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic2pubc(&mut self) -> IC2PUBC_W<'_, PUBCFGCR2rs> {
        IC2PUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the IC3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic3pubc(&mut self) -> IC3PUBC_W<'_, PUBCFGCR2rs> {
        IC3PUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the IC4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic4pubc(&mut self) -> IC4PUBC_W<'_, PUBCFGCR2rs> {
        IC4PUBC_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the IC5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic5pubc(&mut self) -> IC5PUBC_W<'_, PUBCFGCR2rs> {
        IC5PUBC_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the IC6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic6pubc(&mut self) -> IC6PUBC_W<'_, PUBCFGCR2rs> {
        IC6PUBC_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the IC7 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic7pubc(&mut self) -> IC7PUBC_W<'_, PUBCFGCR2rs> {
        IC7PUBC_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the IC8 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic8pubc(&mut self) -> IC8PUBC_W<'_, PUBCFGCR2rs> {
        IC8PUBC_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the IC9 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic9pubc(&mut self) -> IC9PUBC_W<'_, PUBCFGCR2rs> {
        IC9PUBC_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the IC10 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic10pubc(&mut self) -> IC10PUBC_W<'_, PUBCFGCR2rs> {
        IC10PUBC_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the IC11 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic11pubc(&mut self) -> IC11PUBC_W<'_, PUBCFGCR2rs> {
        IC11PUBC_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the IC12 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic12pubc(&mut self) -> IC12PUBC_W<'_, PUBCFGCR2rs> {
        IC12PUBC_W::new(self, 11)
    }
    ///Bit 12 - Defines the public protection of the IC13 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic13pubc(&mut self) -> IC13PUBC_W<'_, PUBCFGCR2rs> {
        IC13PUBC_W::new(self, 12)
    }
    ///Bit 13 - Defines the public protection of the IC14 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic14pubc(&mut self) -> IC14PUBC_W<'_, PUBCFGCR2rs> {
        IC14PUBC_W::new(self, 13)
    }
    ///Bit 14 - Defines the public protection of the IC15 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic15pubc(&mut self) -> IC15PUBC_W<'_, PUBCFGCR2rs> {
        IC15PUBC_W::new(self, 14)
    }
    ///Bit 15 - Defines the public protection of the IC16 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic16pubc(&mut self) -> IC16PUBC_W<'_, PUBCFGCR2rs> {
        IC16PUBC_W::new(self, 15)
    }
    ///Bit 16 - Defines the public protection of the IC17 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic17pubc(&mut self) -> IC17PUBC_W<'_, PUBCFGCR2rs> {
        IC17PUBC_W::new(self, 16)
    }
    ///Bit 17 - Defines the public protection of the IC18 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic18pubc(&mut self) -> IC18PUBC_W<'_, PUBCFGCR2rs> {
        IC18PUBC_W::new(self, 17)
    }
    ///Bit 18 - Defines the public protection of the IC19 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic19pubc(&mut self) -> IC19PUBC_W<'_, PUBCFGCR2rs> {
        IC19PUBC_W::new(self, 18)
    }
    ///Bit 19 - Defines the public protection of the IC20 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ic20pubc(&mut self) -> IC20PUBC_W<'_, PUBCFGCR2rs> {
        IC20PUBC_W::new(self, 19)
    }
}
/**RCC divider public configuration register2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PUBCFGCR2)*/
pub struct PUBCFGCR2rs;
impl crate::RegisterSpec for PUBCFGCR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr2::W`](W) writer structure
impl crate::Writable for PUBCFGCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR2 to value 0
impl crate::Resettable for PUBCFGCR2rs {}
