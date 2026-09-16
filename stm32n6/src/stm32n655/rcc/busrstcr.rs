///Register `BUSRSTCR` writer
pub type W = crate::W<BUSRSTCRrs>;
///Field `ACLKNRSTC` writer - ACLKN reset
pub type ACLKNRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMRSTC` writer - AHBM reset
pub type AHBMRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1RSTC` writer - AHB1 reset
pub type AHB1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2RSTC` writer - AHB2 reset
pub type AHB2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3RSTC` writer - AHB3 reset
pub type AHB3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4RSTC` writer - AHB4 reset
pub type AHB4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5RSTC` writer - AHB5 reset
pub type AHB5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1RSTC` writer - APB1 reset
pub type APB1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2RSTC` writer - APB2 reset
pub type APB2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3RSTC` writer - APB3 reset
pub type APB3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4RSTC` writer - APB4 reset
pub type APB4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5RSTC` writer - APB5 reset
pub type APB5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCRSTC` writer - NOC reset
pub type NOCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN reset
    #[inline(always)]
    pub fn aclknrstc(&mut self) -> ACLKNRSTC_W<'_, BUSRSTCRrs> {
        ACLKNRSTC_W::new(self, 0)
    }
    ///Bit 2 - AHBM reset
    #[inline(always)]
    pub fn ahbmrstc(&mut self) -> AHBMRSTC_W<'_, BUSRSTCRrs> {
        AHBMRSTC_W::new(self, 2)
    }
    ///Bit 3 - AHB1 reset
    #[inline(always)]
    pub fn ahb1rstc(&mut self) -> AHB1RSTC_W<'_, BUSRSTCRrs> {
        AHB1RSTC_W::new(self, 3)
    }
    ///Bit 4 - AHB2 reset
    #[inline(always)]
    pub fn ahb2rstc(&mut self) -> AHB2RSTC_W<'_, BUSRSTCRrs> {
        AHB2RSTC_W::new(self, 4)
    }
    ///Bit 5 - AHB3 reset
    #[inline(always)]
    pub fn ahb3rstc(&mut self) -> AHB3RSTC_W<'_, BUSRSTCRrs> {
        AHB3RSTC_W::new(self, 5)
    }
    ///Bit 6 - AHB4 reset
    #[inline(always)]
    pub fn ahb4rstc(&mut self) -> AHB4RSTC_W<'_, BUSRSTCRrs> {
        AHB4RSTC_W::new(self, 6)
    }
    ///Bit 7 - AHB5 reset
    #[inline(always)]
    pub fn ahb5rstc(&mut self) -> AHB5RSTC_W<'_, BUSRSTCRrs> {
        AHB5RSTC_W::new(self, 7)
    }
    ///Bit 8 - APB1 reset
    #[inline(always)]
    pub fn apb1rstc(&mut self) -> APB1RSTC_W<'_, BUSRSTCRrs> {
        APB1RSTC_W::new(self, 8)
    }
    ///Bit 9 - APB2 reset
    #[inline(always)]
    pub fn apb2rstc(&mut self) -> APB2RSTC_W<'_, BUSRSTCRrs> {
        APB2RSTC_W::new(self, 9)
    }
    ///Bit 10 - APB3 reset
    #[inline(always)]
    pub fn apb3rstc(&mut self) -> APB3RSTC_W<'_, BUSRSTCRrs> {
        APB3RSTC_W::new(self, 10)
    }
    ///Bit 11 - APB4 reset
    #[inline(always)]
    pub fn apb4rstc(&mut self) -> APB4RSTC_W<'_, BUSRSTCRrs> {
        APB4RSTC_W::new(self, 11)
    }
    ///Bit 12 - APB5 reset
    #[inline(always)]
    pub fn apb5rstc(&mut self) -> APB5RSTC_W<'_, BUSRSTCRrs> {
        APB5RSTC_W::new(self, 12)
    }
    ///Bit 13 - NOC reset
    #[inline(always)]
    pub fn nocrstc(&mut self) -> NOCRSTC_W<'_, BUSRSTCRrs> {
        NOCRSTC_W::new(self, 13)
    }
}
/**RCC bus reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:BUSRSTCR)*/
pub struct BUSRSTCRrs;
impl crate::RegisterSpec for BUSRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`busrstcr::W`](W) writer structure
impl crate::Writable for BUSRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSRSTCR to value 0
impl crate::Resettable for BUSRSTCRrs {}
