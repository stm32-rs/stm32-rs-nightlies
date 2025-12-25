///Register `PUBCFGCR4` writer
pub type W = crate::W<PUBCFGCR4rs>;
///Field `ACLKNPUBC` writer - Defines the public protection of the ACLKN configuration bits (enable, ready, divider).
pub type ACLKNPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPUBC` writer - Defines the public protection of the ACLKNC configuration bits (enable, ready, divider).
pub type ACLKNCPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPUBC` writer - Defines the public protection of the AHBM configuration bits (enable, ready, divider).
pub type AHBMPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PUBC` writer - Defines the public protection of the AHB1 configuration bits (enable, ready, divider).
pub type AHB1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PUBC` writer - Defines the public protection of the AHB2 configuration bits (enable, ready, divider).
pub type AHB2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PUBC` writer - Defines the public protection of the AHB3 configuration bits (enable, ready, divider).
pub type AHB3PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PUBC` writer - Defines the public protection of the AHB4 configuration bits (enable, ready, divider).
pub type AHB4PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PUBC` writer - Defines the public protection of the AHB5 configuration bits (enable, ready, divider).
pub type AHB5PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PUBC` writer - Defines the public protection of the APB1 configuration bits (enable, ready, divider).
pub type APB1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PUBC` writer - Defines the public protection of the APB2 configuration bits (enable, ready, divider).
pub type APB2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PUBC` writer - Defines the public protection of the APB3 configuration bits (enable, ready, divider).
pub type APB3PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PUBC` writer - Defines the public protection of the APB4 configuration bits (enable, ready, divider).
pub type APB4PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PUBC` writer - Defines the public protection of the APB5 configuration bits (enable, ready, divider).
pub type APB5PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPUBC` writer - Defines the public protection of the NOC configuration bits (enable, ready, divider).
pub type NOCPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the ACLKN configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclknpubc(&mut self) -> ACLKNPUBC_W<'_, PUBCFGCR4rs> {
        ACLKNPUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the ACLKNC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclkncpubc(&mut self) -> ACLKNCPUBC_W<'_, PUBCFGCR4rs> {
        ACLKNCPUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AHBM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbmpubc(&mut self) -> AHBMPUBC_W<'_, PUBCFGCR4rs> {
        AHBMPUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AHB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb1pubc(&mut self) -> AHB1PUBC_W<'_, PUBCFGCR4rs> {
        AHB1PUBC_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb2pubc(&mut self) -> AHB2PUBC_W<'_, PUBCFGCR4rs> {
        AHB2PUBC_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb3pubc(&mut self) -> AHB3PUBC_W<'_, PUBCFGCR4rs> {
        AHB3PUBC_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the AHB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb4pubc(&mut self) -> AHB4PUBC_W<'_, PUBCFGCR4rs> {
        AHB4PUBC_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AHB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb5pubc(&mut self) -> AHB5PUBC_W<'_, PUBCFGCR4rs> {
        AHB5PUBC_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the APB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb1pubc(&mut self) -> APB1PUBC_W<'_, PUBCFGCR4rs> {
        APB1PUBC_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the APB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb2pubc(&mut self) -> APB2PUBC_W<'_, PUBCFGCR4rs> {
        APB2PUBC_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the APB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb3pubc(&mut self) -> APB3PUBC_W<'_, PUBCFGCR4rs> {
        APB3PUBC_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the APB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb4pubc(&mut self) -> APB4PUBC_W<'_, PUBCFGCR4rs> {
        APB4PUBC_W::new(self, 11)
    }
    ///Bit 12 - Defines the public protection of the APB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb5pubc(&mut self) -> APB5PUBC_W<'_, PUBCFGCR4rs> {
        APB5PUBC_W::new(self, 12)
    }
    ///Bit 13 - Defines the public protection of the NOC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn nocpubc(&mut self) -> NOCPUBC_W<'_, PUBCFGCR4rs> {
        NOCPUBC_W::new(self, 13)
    }
}
/**RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGCR4)*/
pub struct PUBCFGCR4rs;
impl crate::RegisterSpec for PUBCFGCR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr4::W`](W) writer structure
impl crate::Writable for PUBCFGCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR4 to value 0
impl crate::Resettable for PUBCFGCR4rs {}
