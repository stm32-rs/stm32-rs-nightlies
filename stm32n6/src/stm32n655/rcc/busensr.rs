///Register `BUSENSR` writer
pub type W = crate::W<BUSENSRrs>;
///Field `ACLKNENS` writer - ACLKN enable
pub type ACLKNENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCENS` writer - ACLKNC enable
pub type ACLKNCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMENS` writer - AHBM enable
pub type AHBMENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1ENS` writer - AHB1 enable
pub type AHB1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2ENS` writer - AHB2 enable
pub type AHB2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3ENS` writer - AHB3 enable
pub type AHB3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4ENS` writer - AHB4 enable
pub type AHB4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5ENS` writer - AHB5 enable
pub type AHB5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1ENS` writer - APB1 enable
pub type APB1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2ENS` writer - APB2 enable
pub type APB2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3ENS` writer - APB3 enable
pub type APB3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4ENS` writer - APB4 enable
pub type APB4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5ENS` writer - APB5 enable
pub type APB5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN enable
    #[inline(always)]
    pub fn aclknens(&mut self) -> ACLKNENS_W<'_, BUSENSRrs> {
        ACLKNENS_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC enable
    #[inline(always)]
    pub fn aclkncens(&mut self) -> ACLKNCENS_W<'_, BUSENSRrs> {
        ACLKNCENS_W::new(self, 1)
    }
    ///Bit 2 - AHBM enable
    #[inline(always)]
    pub fn ahbmens(&mut self) -> AHBMENS_W<'_, BUSENSRrs> {
        AHBMENS_W::new(self, 2)
    }
    ///Bit 3 - AHB1 enable
    #[inline(always)]
    pub fn ahb1ens(&mut self) -> AHB1ENS_W<'_, BUSENSRrs> {
        AHB1ENS_W::new(self, 3)
    }
    ///Bit 4 - AHB2 enable
    #[inline(always)]
    pub fn ahb2ens(&mut self) -> AHB2ENS_W<'_, BUSENSRrs> {
        AHB2ENS_W::new(self, 4)
    }
    ///Bit 5 - AHB3 enable
    #[inline(always)]
    pub fn ahb3ens(&mut self) -> AHB3ENS_W<'_, BUSENSRrs> {
        AHB3ENS_W::new(self, 5)
    }
    ///Bit 6 - AHB4 enable
    #[inline(always)]
    pub fn ahb4ens(&mut self) -> AHB4ENS_W<'_, BUSENSRrs> {
        AHB4ENS_W::new(self, 6)
    }
    ///Bit 7 - AHB5 enable
    #[inline(always)]
    pub fn ahb5ens(&mut self) -> AHB5ENS_W<'_, BUSENSRrs> {
        AHB5ENS_W::new(self, 7)
    }
    ///Bit 8 - APB1 enable
    #[inline(always)]
    pub fn apb1ens(&mut self) -> APB1ENS_W<'_, BUSENSRrs> {
        APB1ENS_W::new(self, 8)
    }
    ///Bit 9 - APB2 enable
    #[inline(always)]
    pub fn apb2ens(&mut self) -> APB2ENS_W<'_, BUSENSRrs> {
        APB2ENS_W::new(self, 9)
    }
    ///Bit 10 - APB3 enable
    #[inline(always)]
    pub fn apb3ens(&mut self) -> APB3ENS_W<'_, BUSENSRrs> {
        APB3ENS_W::new(self, 10)
    }
    ///Bit 11 - APB4 enable
    #[inline(always)]
    pub fn apb4ens(&mut self) -> APB4ENS_W<'_, BUSENSRrs> {
        APB4ENS_W::new(self, 11)
    }
    ///Bit 12 - APB5 enable
    #[inline(always)]
    pub fn apb5ens(&mut self) -> APB5ENS_W<'_, BUSENSRrs> {
        APB5ENS_W::new(self, 12)
    }
}
/**RCC bus enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:BUSENSR)*/
pub struct BUSENSRrs;
impl crate::RegisterSpec for BUSENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`busensr::W`](W) writer structure
impl crate::Writable for BUSENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSENSR to value 0
impl crate::Resettable for BUSENSRrs {}
