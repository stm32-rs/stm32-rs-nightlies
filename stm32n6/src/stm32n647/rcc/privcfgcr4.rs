///Register `PRIVCFGCR4` writer
pub type W = crate::W<PRIVCFGCR4rs>;
///Field `ACLKNPVC` writer - Defines the privilege protection of the ACLKN configuration bits (enable, ready, divider).
pub type ACLKNPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPVC` writer - Defines the privilege protection of the ACLKNC configuration bits (enable, ready, divider).
pub type ACLKNCPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPVC` writer - Defines the privilege protection of the AHBM configuration bits (enable, ready, divider).
pub type AHBMPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PVC` writer - Defines the privilege protection of the AHB1 configuration bits (enable, ready, divider).
pub type AHB1PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PVC` writer - Defines the privilege protection of the AHB2 configuration bits (enable, ready, divider).
pub type AHB2PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PVC` writer - Defines the privilege protection of the AHB3 configuration bits (enable, ready, divider).
pub type AHB3PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PVC` writer - Defines the privilege protection of the AHB4 configuration bits (enable, ready, divider).
pub type AHB4PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PVC` writer - Defines the privilege protection of the AHB5 configuration bits (enable, ready, divider).
pub type AHB5PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PVC` writer - Defines the privilege protection of the APB1 configuration bits (enable, ready, divider).
pub type APB1PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PVC` writer - Defines the privilege protection of the APB2 configuration bits (enable, ready, divider).
pub type APB2PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PVC` writer - Defines the privilege protection of the APB3 configuration bits (enable, ready, divider).
pub type APB3PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PVC` writer - Defines the privilege protection of the APB4 configuration bits (enable, ready, divider).
pub type APB4PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PVC` writer - Defines the privilege protection of the APB5 configuration bits (enable, ready, divider).
pub type APB5PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPVC` writer - Defines the privilege protection of the NOC configuration bits (enable, ready, divider).
pub type NOCPVC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGCR4rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the ACLKN configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclknpvc(&mut self) -> ACLKNPVC_W<'_, PRIVCFGCR4rs> {
        ACLKNPVC_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the ACLKNC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn aclkncpvc(&mut self) -> ACLKNCPVC_W<'_, PRIVCFGCR4rs> {
        ACLKNCPVC_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the AHBM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbmpvc(&mut self) -> AHBMPVC_W<'_, PRIVCFGCR4rs> {
        AHBMPVC_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the AHB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb1pvc(&mut self) -> AHB1PVC_W<'_, PRIVCFGCR4rs> {
        AHB1PVC_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the AHB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb2pvc(&mut self) -> AHB2PVC_W<'_, PRIVCFGCR4rs> {
        AHB2PVC_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the AHB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb3pvc(&mut self) -> AHB3PVC_W<'_, PRIVCFGCR4rs> {
        AHB3PVC_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the AHB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb4pvc(&mut self) -> AHB4PVC_W<'_, PRIVCFGCR4rs> {
        AHB4PVC_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the AHB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahb5pvc(&mut self) -> AHB5PVC_W<'_, PRIVCFGCR4rs> {
        AHB5PVC_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the APB1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb1pvc(&mut self) -> APB1PVC_W<'_, PRIVCFGCR4rs> {
        APB1PVC_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the APB2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb2pvc(&mut self) -> APB2PVC_W<'_, PRIVCFGCR4rs> {
        APB2PVC_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the APB3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb3pvc(&mut self) -> APB3PVC_W<'_, PRIVCFGCR4rs> {
        APB3PVC_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the APB4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb4pvc(&mut self) -> APB4PVC_W<'_, PRIVCFGCR4rs> {
        APB4PVC_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the APB5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn apb5pvc(&mut self) -> APB5PVC_W<'_, PRIVCFGCR4rs> {
        APB5PVC_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the NOC configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn nocpvc(&mut self) -> NOCPVC_W<'_, PRIVCFGCR4rs> {
        NOCPVC_W::new(self, 13)
    }
}
/**RCC privilege configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGCR4)*/
pub struct PRIVCFGCR4rs;
impl crate::RegisterSpec for PRIVCFGCR4rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgcr4::W`](W) writer structure
impl crate::Writable for PRIVCFGCR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGCR4 to value 0
impl crate::Resettable for PRIVCFGCR4rs {}
