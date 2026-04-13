///Register `BUSRSTSR` writer
pub type W = crate::W<BUSRSTSRrs>;
///Field `ACLKNRSTS` writer - ACLKN reset
pub type ACLKNRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMRSTS` writer - AHBM reset
pub type AHBMRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1RSTS` writer - AHB1 reset
pub type AHB1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2RSTS` writer - AHB2 reset
pub type AHB2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3RSTS` writer - AHB3 reset
pub type AHB3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4RSTS` writer - AHB4 reset
pub type AHB4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5RSTS` writer - AHB5 reset
pub type AHB5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1RSTS` writer - APB1 reset
pub type APB1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2RSTS` writer - APB2 reset
pub type APB2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3RSTS` writer - APB3 reset
pub type APB3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4RSTS` writer - APB4 reset
pub type APB4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5RSTS` writer - APB5 reset
pub type APB5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCRSTS` writer - NOC reset
pub type NOCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN reset
    #[inline(always)]
    pub fn aclknrsts(&mut self) -> ACLKNRSTS_W<'_, BUSRSTSRrs> {
        ACLKNRSTS_W::new(self, 0)
    }
    ///Bit 2 - AHBM reset
    #[inline(always)]
    pub fn ahbmrsts(&mut self) -> AHBMRSTS_W<'_, BUSRSTSRrs> {
        AHBMRSTS_W::new(self, 2)
    }
    ///Bit 3 - AHB1 reset
    #[inline(always)]
    pub fn ahb1rsts(&mut self) -> AHB1RSTS_W<'_, BUSRSTSRrs> {
        AHB1RSTS_W::new(self, 3)
    }
    ///Bit 4 - AHB2 reset
    #[inline(always)]
    pub fn ahb2rsts(&mut self) -> AHB2RSTS_W<'_, BUSRSTSRrs> {
        AHB2RSTS_W::new(self, 4)
    }
    ///Bit 5 - AHB3 reset
    #[inline(always)]
    pub fn ahb3rsts(&mut self) -> AHB3RSTS_W<'_, BUSRSTSRrs> {
        AHB3RSTS_W::new(self, 5)
    }
    ///Bit 6 - AHB4 reset
    #[inline(always)]
    pub fn ahb4rsts(&mut self) -> AHB4RSTS_W<'_, BUSRSTSRrs> {
        AHB4RSTS_W::new(self, 6)
    }
    ///Bit 7 - AHB5 reset
    #[inline(always)]
    pub fn ahb5rsts(&mut self) -> AHB5RSTS_W<'_, BUSRSTSRrs> {
        AHB5RSTS_W::new(self, 7)
    }
    ///Bit 8 - APB1 reset
    #[inline(always)]
    pub fn apb1rsts(&mut self) -> APB1RSTS_W<'_, BUSRSTSRrs> {
        APB1RSTS_W::new(self, 8)
    }
    ///Bit 9 - APB2 reset
    #[inline(always)]
    pub fn apb2rsts(&mut self) -> APB2RSTS_W<'_, BUSRSTSRrs> {
        APB2RSTS_W::new(self, 9)
    }
    ///Bit 10 - APB3 reset
    #[inline(always)]
    pub fn apb3rsts(&mut self) -> APB3RSTS_W<'_, BUSRSTSRrs> {
        APB3RSTS_W::new(self, 10)
    }
    ///Bit 11 - APB4 reset
    #[inline(always)]
    pub fn apb4rsts(&mut self) -> APB4RSTS_W<'_, BUSRSTSRrs> {
        APB4RSTS_W::new(self, 11)
    }
    ///Bit 12 - APB5 reset
    #[inline(always)]
    pub fn apb5rsts(&mut self) -> APB5RSTS_W<'_, BUSRSTSRrs> {
        APB5RSTS_W::new(self, 12)
    }
    ///Bit 13 - NOC reset
    #[inline(always)]
    pub fn nocrsts(&mut self) -> NOCRSTS_W<'_, BUSRSTSRrs> {
        NOCRSTS_W::new(self, 13)
    }
}
/**RCC bus reset set register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSRSTSR)*/
pub struct BUSRSTSRrs;
impl crate::RegisterSpec for BUSRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`busrstsr::W`](W) writer structure
impl crate::Writable for BUSRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSRSTSR to value 0
impl crate::Resettable for BUSRSTSRrs {}
