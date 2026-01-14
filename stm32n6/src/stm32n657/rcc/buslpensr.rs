///Register `BUSLPENSR` writer
pub type W = crate::W<BUSLPENSRrs>;
///Field `ACLKNLPENS` writer - ACLKN sleep enable
pub type ACLKNLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCLPENS` writer - ACLKNC sleep enable
pub type ACLKNCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMLPENS` writer - AHBM sleep enable
pub type AHBMLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1LPENS` writer - AHB1 sleep enable
pub type AHB1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2LPENS` writer - AHB2 sleep enable
pub type AHB2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3LPENS` writer - AHB3 sleep enable
pub type AHB3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4LPENS` writer - AHB4 sleep enable
pub type AHB4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5LPENS` writer - AHB5 sleep enable
pub type AHB5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1LPENS` writer - APB1 sleep enable
pub type APB1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2LPENS` writer - APB2 sleep enable
pub type APB2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3LPENS` writer - APB3 sleep enable
pub type APB3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4LPENS` writer - APB4 sleep enable
pub type APB4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5LPENS` writer - APB5 sleep enable
pub type APB5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN sleep enable
    #[inline(always)]
    pub fn aclknlpens(&mut self) -> ACLKNLPENS_W<'_, BUSLPENSRrs> {
        ACLKNLPENS_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC sleep enable
    #[inline(always)]
    pub fn aclknclpens(&mut self) -> ACLKNCLPENS_W<'_, BUSLPENSRrs> {
        ACLKNCLPENS_W::new(self, 1)
    }
    ///Bit 2 - AHBM sleep enable
    #[inline(always)]
    pub fn ahbmlpens(&mut self) -> AHBMLPENS_W<'_, BUSLPENSRrs> {
        AHBMLPENS_W::new(self, 2)
    }
    ///Bit 3 - AHB1 sleep enable
    #[inline(always)]
    pub fn ahb1lpens(&mut self) -> AHB1LPENS_W<'_, BUSLPENSRrs> {
        AHB1LPENS_W::new(self, 3)
    }
    ///Bit 4 - AHB2 sleep enable
    #[inline(always)]
    pub fn ahb2lpens(&mut self) -> AHB2LPENS_W<'_, BUSLPENSRrs> {
        AHB2LPENS_W::new(self, 4)
    }
    ///Bit 5 - AHB3 sleep enable
    #[inline(always)]
    pub fn ahb3lpens(&mut self) -> AHB3LPENS_W<'_, BUSLPENSRrs> {
        AHB3LPENS_W::new(self, 5)
    }
    ///Bit 6 - AHB4 sleep enable
    #[inline(always)]
    pub fn ahb4lpens(&mut self) -> AHB4LPENS_W<'_, BUSLPENSRrs> {
        AHB4LPENS_W::new(self, 6)
    }
    ///Bit 7 - AHB5 sleep enable
    #[inline(always)]
    pub fn ahb5lpens(&mut self) -> AHB5LPENS_W<'_, BUSLPENSRrs> {
        AHB5LPENS_W::new(self, 7)
    }
    ///Bit 8 - APB1 sleep enable
    #[inline(always)]
    pub fn apb1lpens(&mut self) -> APB1LPENS_W<'_, BUSLPENSRrs> {
        APB1LPENS_W::new(self, 8)
    }
    ///Bit 9 - APB2 sleep enable
    #[inline(always)]
    pub fn apb2lpens(&mut self) -> APB2LPENS_W<'_, BUSLPENSRrs> {
        APB2LPENS_W::new(self, 9)
    }
    ///Bit 10 - APB3 sleep enable
    #[inline(always)]
    pub fn apb3lpens(&mut self) -> APB3LPENS_W<'_, BUSLPENSRrs> {
        APB3LPENS_W::new(self, 10)
    }
    ///Bit 11 - APB4 sleep enable
    #[inline(always)]
    pub fn apb4lpens(&mut self) -> APB4LPENS_W<'_, BUSLPENSRrs> {
        APB4LPENS_W::new(self, 11)
    }
    ///Bit 12 - APB5 sleep enable
    #[inline(always)]
    pub fn apb5lpens(&mut self) -> APB5LPENS_W<'_, BUSLPENSRrs> {
        APB5LPENS_W::new(self, 12)
    }
}
/**RCC bus Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:BUSLPENSR)*/
pub struct BUSLPENSRrs;
impl crate::RegisterSpec for BUSLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`buslpensr::W`](W) writer structure
impl crate::Writable for BUSLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSLPENSR to value 0
impl crate::Resettable for BUSLPENSRrs {}
