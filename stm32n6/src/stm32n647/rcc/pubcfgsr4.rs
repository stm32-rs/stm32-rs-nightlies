///Register `PUBCFGSR4` writer
pub type W = crate::W<PUBCFGSR4rs>;
///Field `ACLKNPUBS` writer - Defines the public protection of the ACLKN configuration bits (enable, ready, divider).
pub type ACLKNPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPUBS` writer - Defines the public protection of the ACLKNC configuration bits (enable, ready, divider).
pub type ACLKNCPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPUBS` writer - Defines the public protection of the AHBM configuration bits (enable, ready, divider).
pub type AHBMPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PUBS` writer - Defines the public protection of the AHB1 configuration bits (enable, ready, divider).
pub type AHB1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PUBS` writer - Defines the public protection of the AHB2 configuration bits (enable, ready, divider).
pub type AHB2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PUBS` writer - Defines the public protection of the AHB3 configuration bits (enable, ready, divider).
pub type AHB3PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PUBS` writer - Defines the public protection of the AHB4 configuration bits (enable, ready, divider).
pub type AHB4PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PUBS` writer - Defines the public protection of the AHB5 configuration bits (enable, ready, divider).
pub type AHB5PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PUBS` writer - Defines the public protection of the APB1 configuration bits (enable, ready, divider).
pub type APB1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PUBS` writer - Defines the public protection of the APB2 configuration bits (enable, ready, divider).
pub type APB2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PUBS` writer - Defines the public protection of the APB3 configuration bits (enable, ready, divider).
pub type APB3PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PUBS` writer - Defines the public protection of the APB4 configuration bits (enable, ready, divider).
pub type APB4PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PUBS` writer - Defines the public protection of the APB5 configuration bits (enable, ready, divider).
pub type APB5PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPUBS` writer - Defines the public protection of the NOC configuration bits (enable, ready, divider).
pub type NOCPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the ACLKN configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclknpubs(&mut self) -> ACLKNPUBS_W<'_, PUBCFGSR4rs> {
        ACLKNPUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the ACLKNC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclkncpubs(&mut self) -> ACLKNCPUBS_W<'_, PUBCFGSR4rs> {
        ACLKNCPUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AHBM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbmpubs(&mut self) -> AHBMPUBS_W<'_, PUBCFGSR4rs> {
        AHBMPUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AHB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb1pubs(&mut self) -> AHB1PUBS_W<'_, PUBCFGSR4rs> {
        AHB1PUBS_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb2pubs(&mut self) -> AHB2PUBS_W<'_, PUBCFGSR4rs> {
        AHB2PUBS_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb3pubs(&mut self) -> AHB3PUBS_W<'_, PUBCFGSR4rs> {
        AHB3PUBS_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the AHB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb4pubs(&mut self) -> AHB4PUBS_W<'_, PUBCFGSR4rs> {
        AHB4PUBS_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AHB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb5pubs(&mut self) -> AHB5PUBS_W<'_, PUBCFGSR4rs> {
        AHB5PUBS_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the APB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb1pubs(&mut self) -> APB1PUBS_W<'_, PUBCFGSR4rs> {
        APB1PUBS_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the APB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb2pubs(&mut self) -> APB2PUBS_W<'_, PUBCFGSR4rs> {
        APB2PUBS_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the APB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb3pubs(&mut self) -> APB3PUBS_W<'_, PUBCFGSR4rs> {
        APB3PUBS_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the APB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb4pubs(&mut self) -> APB4PUBS_W<'_, PUBCFGSR4rs> {
        APB4PUBS_W::new(self, 11)
    }
    ///Bit 12 - Defines the public protection of the APB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb5pubs(&mut self) -> APB5PUBS_W<'_, PUBCFGSR4rs> {
        APB5PUBS_W::new(self, 12)
    }
    ///Bit 13 - Defines the public protection of the NOC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn nocpubs(&mut self) -> NOCPUBS_W<'_, PUBCFGSR4rs> {
        NOCPUBS_W::new(self, 13)
    }
}
/**RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PUBCFGSR4)*/
pub struct PUBCFGSR4rs;
impl crate::RegisterSpec for PUBCFGSR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr4::W`](W) writer structure
impl crate::Writable for PUBCFGSR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR4 to value 0
impl crate::Resettable for PUBCFGSR4rs {}
