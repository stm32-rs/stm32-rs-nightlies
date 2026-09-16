///Register `BUSLPENR` reader
pub type R = crate::R<BUSLPENRrs>;
///Register `BUSLPENR` writer
pub type W = crate::W<BUSLPENRrs>;
///Field `ACLKNLPEN` reader - ACLKN sleep enable
pub type ACLKNLPEN_R = crate::BitReader;
///Field `ACLKNLPEN` writer - ACLKN sleep enable
pub type ACLKNLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCLPEN` reader - ACLKNC sleep enable
pub type ACLKNCLPEN_R = crate::BitReader;
///Field `ACLKNCLPEN` writer - ACLKNC sleep enable
pub type ACLKNCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMLPEN` reader - AHBM sleep enable
pub type AHBMLPEN_R = crate::BitReader;
///Field `AHBMLPEN` writer - AHBM sleep enable
pub type AHBMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1LPEN` reader - AHB1 sleep enable
pub type AHB1LPEN_R = crate::BitReader;
///Field `AHB1LPEN` writer - AHB1 sleep enable
pub type AHB1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2LPEN` reader - AHB2 sleep enable
pub type AHB2LPEN_R = crate::BitReader;
///Field `AHB2LPEN` writer - AHB2 sleep enable
pub type AHB2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3LPEN` reader - AHB3 sleep enable
pub type AHB3LPEN_R = crate::BitReader;
///Field `AHB3LPEN` writer - AHB3 sleep enable
pub type AHB3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4LPEN` reader - AHB4 sleep enable
pub type AHB4LPEN_R = crate::BitReader;
///Field `AHB4LPEN` writer - AHB4 sleep enable
pub type AHB4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5LPEN` reader - AHB5 sleep enable
pub type AHB5LPEN_R = crate::BitReader;
///Field `AHB5LPEN` writer - AHB5 sleep enable
pub type AHB5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1LPEN` reader - APB1 sleep enable
pub type APB1LPEN_R = crate::BitReader;
///Field `APB1LPEN` writer - APB1 sleep enable
pub type APB1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2LPEN` reader - APB2 sleep enable
pub type APB2LPEN_R = crate::BitReader;
///Field `APB2LPEN` writer - APB2 sleep enable
pub type APB2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3LPEN` reader - APB3 sleep enable
pub type APB3LPEN_R = crate::BitReader;
///Field `APB3LPEN` writer - APB3 sleep enable
pub type APB3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4LPEN` reader - APB4 sleep enable
pub type APB4LPEN_R = crate::BitReader;
///Field `APB4LPEN` writer - APB4 sleep enable
pub type APB4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5LPEN` reader - APB5 sleep enable
pub type APB5LPEN_R = crate::BitReader;
///Field `APB5LPEN` writer - APB5 sleep enable
pub type APB5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ACLKN sleep enable
    #[inline(always)]
    pub fn aclknlpen(&self) -> ACLKNLPEN_R {
        ACLKNLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACLKNC sleep enable
    #[inline(always)]
    pub fn aclknclpen(&self) -> ACLKNCLPEN_R {
        ACLKNCLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHBM sleep enable
    #[inline(always)]
    pub fn ahbmlpen(&self) -> AHBMLPEN_R {
        AHBMLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AHB1 sleep enable
    #[inline(always)]
    pub fn ahb1lpen(&self) -> AHB1LPEN_R {
        AHB1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AHB2 sleep enable
    #[inline(always)]
    pub fn ahb2lpen(&self) -> AHB2LPEN_R {
        AHB2LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AHB3 sleep enable
    #[inline(always)]
    pub fn ahb3lpen(&self) -> AHB3LPEN_R {
        AHB3LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - AHB4 sleep enable
    #[inline(always)]
    pub fn ahb4lpen(&self) -> AHB4LPEN_R {
        AHB4LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - AHB5 sleep enable
    #[inline(always)]
    pub fn ahb5lpen(&self) -> AHB5LPEN_R {
        AHB5LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - APB1 sleep enable
    #[inline(always)]
    pub fn apb1lpen(&self) -> APB1LPEN_R {
        APB1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - APB2 sleep enable
    #[inline(always)]
    pub fn apb2lpen(&self) -> APB2LPEN_R {
        APB2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - APB3 sleep enable
    #[inline(always)]
    pub fn apb3lpen(&self) -> APB3LPEN_R {
        APB3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - APB4 sleep enable
    #[inline(always)]
    pub fn apb4lpen(&self) -> APB4LPEN_R {
        APB4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - APB5 sleep enable
    #[inline(always)]
    pub fn apb5lpen(&self) -> APB5LPEN_R {
        APB5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSLPENR")
            .field("aclknlpen", &self.aclknlpen())
            .field("aclknclpen", &self.aclknclpen())
            .field("ahbmlpen", &self.ahbmlpen())
            .field("ahb1lpen", &self.ahb1lpen())
            .field("ahb2lpen", &self.ahb2lpen())
            .field("ahb3lpen", &self.ahb3lpen())
            .field("ahb4lpen", &self.ahb4lpen())
            .field("ahb5lpen", &self.ahb5lpen())
            .field("apb1lpen", &self.apb1lpen())
            .field("apb2lpen", &self.apb2lpen())
            .field("apb3lpen", &self.apb3lpen())
            .field("apb4lpen", &self.apb4lpen())
            .field("apb5lpen", &self.apb5lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - ACLKN sleep enable
    #[inline(always)]
    pub fn aclknlpen(&mut self) -> ACLKNLPEN_W<'_, BUSLPENRrs> {
        ACLKNLPEN_W::new(self, 0)
    }
    ///Bit 1 - ACLKNC sleep enable
    #[inline(always)]
    pub fn aclknclpen(&mut self) -> ACLKNCLPEN_W<'_, BUSLPENRrs> {
        ACLKNCLPEN_W::new(self, 1)
    }
    ///Bit 2 - AHBM sleep enable
    #[inline(always)]
    pub fn ahbmlpen(&mut self) -> AHBMLPEN_W<'_, BUSLPENRrs> {
        AHBMLPEN_W::new(self, 2)
    }
    ///Bit 3 - AHB1 sleep enable
    #[inline(always)]
    pub fn ahb1lpen(&mut self) -> AHB1LPEN_W<'_, BUSLPENRrs> {
        AHB1LPEN_W::new(self, 3)
    }
    ///Bit 4 - AHB2 sleep enable
    #[inline(always)]
    pub fn ahb2lpen(&mut self) -> AHB2LPEN_W<'_, BUSLPENRrs> {
        AHB2LPEN_W::new(self, 4)
    }
    ///Bit 5 - AHB3 sleep enable
    #[inline(always)]
    pub fn ahb3lpen(&mut self) -> AHB3LPEN_W<'_, BUSLPENRrs> {
        AHB3LPEN_W::new(self, 5)
    }
    ///Bit 6 - AHB4 sleep enable
    #[inline(always)]
    pub fn ahb4lpen(&mut self) -> AHB4LPEN_W<'_, BUSLPENRrs> {
        AHB4LPEN_W::new(self, 6)
    }
    ///Bit 7 - AHB5 sleep enable
    #[inline(always)]
    pub fn ahb5lpen(&mut self) -> AHB5LPEN_W<'_, BUSLPENRrs> {
        AHB5LPEN_W::new(self, 7)
    }
    ///Bit 8 - APB1 sleep enable
    #[inline(always)]
    pub fn apb1lpen(&mut self) -> APB1LPEN_W<'_, BUSLPENRrs> {
        APB1LPEN_W::new(self, 8)
    }
    ///Bit 9 - APB2 sleep enable
    #[inline(always)]
    pub fn apb2lpen(&mut self) -> APB2LPEN_W<'_, BUSLPENRrs> {
        APB2LPEN_W::new(self, 9)
    }
    ///Bit 10 - APB3 sleep enable
    #[inline(always)]
    pub fn apb3lpen(&mut self) -> APB3LPEN_W<'_, BUSLPENRrs> {
        APB3LPEN_W::new(self, 10)
    }
    ///Bit 11 - APB4 sleep enable
    #[inline(always)]
    pub fn apb4lpen(&mut self) -> APB4LPEN_W<'_, BUSLPENRrs> {
        APB4LPEN_W::new(self, 11)
    }
    ///Bit 12 - APB5 sleep enable
    #[inline(always)]
    pub fn apb5lpen(&mut self) -> APB5LPEN_W<'_, BUSLPENRrs> {
        APB5LPEN_W::new(self, 12)
    }
}
/**RCC SoC buses Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`buslpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buslpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:BUSLPENR)*/
pub struct BUSLPENRrs;
impl crate::RegisterSpec for BUSLPENRrs {
    type Ux = u32;
}
///`read()` method returns [`buslpenr::R`](R) reader structure
impl crate::Readable for BUSLPENRrs {}
///`write(|w| ..)` method takes [`buslpenr::W`](W) writer structure
impl crate::Writable for BUSLPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BUSLPENR to value 0x03
impl crate::Resettable for BUSLPENRrs {
    const RESET_VALUE: u32 = 0x03;
}
