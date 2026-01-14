///Register `BUSLPENCR` writer
pub type W = crate::W<BUSLPENCRrs>;
///Field `ACLKNLPENC` writer - ACLKN sleep enable
pub type ACLKNLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCLPENC` writer - ACLKNC sleep enable
pub type ACLKNCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMLPENC` writer - AHBM sleep enable
pub type AHBMLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1LPENC` writer - AHB1 sleep enable
pub type AHB1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2LPENC` writer - AHB2 sleep enable
pub type AHB2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3LPENC` writer - AHB3 sleep enable
pub type AHB3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4LPENC` writer - AHB4 sleep enable
pub type AHB4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5LPENC` writer - AHB5 sleep enable
pub type AHB5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1LPENC` writer - APB1 sleep enable
pub type APB1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2LPENC` writer - APB2 sleep enable
pub type APB2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3LPENC` writer - APB3 sleep enable
pub type APB3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4LPENC` writer - APB4 sleep enable
pub type APB4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5LPENC` writer - APB5 sleep enable
pub type APB5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<BUSLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - ACLKN sleep enable
    #[inline(always)]
    pub fn aclknlpenc(&mut self) -> ACLKNLPENC_W<'_, BUSLPENCRrs> {
        ACLKNLPENC_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC sleep enable
    #[inline(always)]
    pub fn aclknclpenc(&mut self) -> ACLKNCLPENC_W<'_, BUSLPENCRrs> {
        ACLKNCLPENC_W::new(self, 1)
    }
    ///Bit 2 - AHBM sleep enable
    #[inline(always)]
    pub fn ahbmlpenc(&mut self) -> AHBMLPENC_W<'_, BUSLPENCRrs> {
        AHBMLPENC_W::new(self, 2)
    }
    ///Bit 3 - AHB1 sleep enable
    #[inline(always)]
    pub fn ahb1lpenc(&mut self) -> AHB1LPENC_W<'_, BUSLPENCRrs> {
        AHB1LPENC_W::new(self, 3)
    }
    ///Bit 4 - AHB2 sleep enable
    #[inline(always)]
    pub fn ahb2lpenc(&mut self) -> AHB2LPENC_W<'_, BUSLPENCRrs> {
        AHB2LPENC_W::new(self, 4)
    }
    ///Bit 5 - AHB3 sleep enable
    #[inline(always)]
    pub fn ahb3lpenc(&mut self) -> AHB3LPENC_W<'_, BUSLPENCRrs> {
        AHB3LPENC_W::new(self, 5)
    }
    ///Bit 6 - AHB4 sleep enable
    #[inline(always)]
    pub fn ahb4lpenc(&mut self) -> AHB4LPENC_W<'_, BUSLPENCRrs> {
        AHB4LPENC_W::new(self, 6)
    }
    ///Bit 7 - AHB5 sleep enable
    #[inline(always)]
    pub fn ahb5lpenc(&mut self) -> AHB5LPENC_W<'_, BUSLPENCRrs> {
        AHB5LPENC_W::new(self, 7)
    }
    ///Bit 8 - APB1 sleep enable
    #[inline(always)]
    pub fn apb1lpenc(&mut self) -> APB1LPENC_W<'_, BUSLPENCRrs> {
        APB1LPENC_W::new(self, 8)
    }
    ///Bit 9 - APB2 sleep enable
    #[inline(always)]
    pub fn apb2lpenc(&mut self) -> APB2LPENC_W<'_, BUSLPENCRrs> {
        APB2LPENC_W::new(self, 9)
    }
    ///Bit 10 - APB3 sleep enable
    #[inline(always)]
    pub fn apb3lpenc(&mut self) -> APB3LPENC_W<'_, BUSLPENCRrs> {
        APB3LPENC_W::new(self, 10)
    }
    ///Bit 11 - APB4 sleep enable
    #[inline(always)]
    pub fn apb4lpenc(&mut self) -> APB4LPENC_W<'_, BUSLPENCRrs> {
        APB4LPENC_W::new(self, 11)
    }
    ///Bit 12 - APB5 sleep enable
    #[inline(always)]
    pub fn apb5lpenc(&mut self) -> APB5LPENC_W<'_, BUSLPENCRrs> {
        APB5LPENC_W::new(self, 12)
    }
}
/**RCC bus Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:BUSLPENCR)*/
pub struct BUSLPENCRrs;
impl crate::RegisterSpec for BUSLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`buslpencr::W`](W) writer structure
impl crate::Writable for BUSLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSLPENCR to value 0
impl crate::Resettable for BUSLPENCRrs {}
