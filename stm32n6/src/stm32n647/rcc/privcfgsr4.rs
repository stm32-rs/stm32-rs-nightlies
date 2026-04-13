///Register `PRIVCFGSR4` writer
pub type W = crate::W<PRIVCFGSR4rs>;
///Field `ACLKNPVS` writer - Defines the privilege protection of the ACLKN configuration bits (enable, ready, divider).
pub type ACLKNPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPVS` writer - Defines the privilege protection of the ACLKNC configuration bits (enable, ready, divider).
pub type ACLKNCPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPVS` writer - Defines the privilege protection of the AHBM configuration bits (enable, ready, divider).
pub type AHBMPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PVS` writer - Defines the privilege protection of the AHB1 configuration bits (enable, ready, divider).
pub type AHB1PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PVS` writer - Defines the privilege protection of the AHB2 configuration bits (enable, ready, divider).
pub type AHB2PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PVS` writer - Defines the privilege protection of the AHB3 configuration bits (enable, ready, divider).
pub type AHB3PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PVS` writer - Defines the privilege protection of the AHB4 configuration bits (enable, ready, divider).
pub type AHB4PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PVS` writer - Defines the privilege protection of the AHB5 configuration bits (enable, ready, divider).
pub type AHB5PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PVS` writer - Defines the privilege protection of the APB1 configuration bits (enable, ready, divider).
pub type APB1PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PVS` writer - Defines the privilege protection of the APB2 configuration bits (enable, ready, divider).
pub type APB2PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PVS` writer - Defines the privilege protection of the APB3 configuration bits (enable, ready, divider).
pub type APB3PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PVS` writer - Defines the privilege protection of the APB4 configuration bits (enable, ready, divider).
pub type APB4PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PVS` writer - Defines the privilege protection of the APB5 configuration bits (enable, ready, divider).
pub type APB5PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPVS` writer - Defines the privilege protection of the NOC configuration bits (enable, ready, divider).
pub type NOCPVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGSR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the ACLKN configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclknpvs(&mut self) -> ACLKNPVS_W<'_, PRIVCFGSR4rs> {
        ACLKNPVS_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the ACLKNC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclkncpvs(&mut self) -> ACLKNCPVS_W<'_, PRIVCFGSR4rs> {
        ACLKNCPVS_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the AHBM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbmpvs(&mut self) -> AHBMPVS_W<'_, PRIVCFGSR4rs> {
        AHBMPVS_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the AHB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb1pvs(&mut self) -> AHB1PVS_W<'_, PRIVCFGSR4rs> {
        AHB1PVS_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the AHB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb2pvs(&mut self) -> AHB2PVS_W<'_, PRIVCFGSR4rs> {
        AHB2PVS_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the AHB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb3pvs(&mut self) -> AHB3PVS_W<'_, PRIVCFGSR4rs> {
        AHB3PVS_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the AHB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb4pvs(&mut self) -> AHB4PVS_W<'_, PRIVCFGSR4rs> {
        AHB4PVS_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the AHB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb5pvs(&mut self) -> AHB5PVS_W<'_, PRIVCFGSR4rs> {
        AHB5PVS_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the APB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb1pvs(&mut self) -> APB1PVS_W<'_, PRIVCFGSR4rs> {
        APB1PVS_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the APB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb2pvs(&mut self) -> APB2PVS_W<'_, PRIVCFGSR4rs> {
        APB2PVS_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the APB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb3pvs(&mut self) -> APB3PVS_W<'_, PRIVCFGSR4rs> {
        APB3PVS_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the APB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb4pvs(&mut self) -> APB4PVS_W<'_, PRIVCFGSR4rs> {
        APB4PVS_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the APB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb5pvs(&mut self) -> APB5PVS_W<'_, PRIVCFGSR4rs> {
        APB5PVS_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the NOC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn nocpvs(&mut self) -> NOCPVS_W<'_, PRIVCFGSR4rs> {
        NOCPVS_W::new(self, 13)
    }
}
/**RCC privilege configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGSR4)*/
pub struct PRIVCFGSR4rs;
impl crate::RegisterSpec for PRIVCFGSR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgsr4::W`](W) writer structure
impl crate::Writable for PRIVCFGSR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGSR4 to value 0
impl crate::Resettable for PRIVCFGSR4rs {}
