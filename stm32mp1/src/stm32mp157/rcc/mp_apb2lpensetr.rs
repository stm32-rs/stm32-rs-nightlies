///Register `MP_APB2LPENSETR` reader
pub type R = crate::R<MP_APB2LPENSETRrs>;
///Register `MP_APB2LPENSETR` writer
pub type W = crate::W<MP_APB2LPENSETRrs>;
///Field `TIM1LPEN` reader - TIM1LPEN
pub type TIM1LPEN_R = crate::BitReader;
///Field `TIM1LPEN` writer - TIM1LPEN
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8LPEN` reader - TIM8LPEN
pub type TIM8LPEN_R = crate::BitReader;
///Field `TIM8LPEN` writer - TIM8LPEN
pub type TIM8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15LPEN` reader - TIM15LPEN
pub type TIM15LPEN_R = crate::BitReader;
///Field `TIM15LPEN` writer - TIM15LPEN
pub type TIM15LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16LPEN` reader - TIM16LPEN
pub type TIM16LPEN_R = crate::BitReader;
///Field `TIM16LPEN` writer - TIM16LPEN
pub type TIM16LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17LPEN` reader - TIM17LPEN
pub type TIM17LPEN_R = crate::BitReader;
///Field `TIM17LPEN` writer - TIM17LPEN
pub type TIM17LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPEN` reader - SPI1LPEN
pub type SPI1LPEN_R = crate::BitReader;
///Field `SPI1LPEN` writer - SPI1LPEN
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4LPEN` reader - SPI4LPEN
pub type SPI4LPEN_R = crate::BitReader;
///Field `SPI4LPEN` writer - SPI4LPEN
pub type SPI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5LPEN` reader - SPI5LPEN
pub type SPI5LPEN_R = crate::BitReader;
///Field `SPI5LPEN` writer - SPI5LPEN
pub type SPI5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6LPEN` reader - USART6LPEN
pub type USART6LPEN_R = crate::BitReader;
///Field `USART6LPEN` writer - USART6LPEN
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1LPEN` reader - SAI1LPEN
pub type SAI1LPEN_R = crate::BitReader;
///Field `SAI1LPEN` writer - SAI1LPEN
pub type SAI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2LPEN` reader - SAI2LPEN
pub type SAI2LPEN_R = crate::BitReader;
///Field `SAI2LPEN` writer - SAI2LPEN
pub type SAI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI3LPEN` reader - SAI3LPEN
pub type SAI3LPEN_R = crate::BitReader;
///Field `SAI3LPEN` writer - SAI3LPEN
pub type SAI3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDMLPEN` reader - DFSDMLPEN
pub type DFSDMLPEN_R = crate::BitReader;
///Field `DFSDMLPEN` writer - DFSDMLPEN
pub type DFSDMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADFSDMLPEN` reader - ADFSDMLPEN
pub type ADFSDMLPEN_R = crate::BitReader;
///Field `ADFSDMLPEN` writer - ADFSDMLPEN
pub type ADFSDMLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANLPEN` reader - FDCANLPEN
pub type FDCANLPEN_R = crate::BitReader;
///Field `FDCANLPEN` writer - FDCANLPEN
pub type FDCANLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1LPEN
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8LPEN
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM15LPEN
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM16LPEN
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM17LPEN
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SPI1LPEN
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPI4LPEN
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPI5LPEN
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - USART6LPEN
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - SAI1LPEN
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI2LPEN
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SAI3LPEN
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - DFSDMLPEN
    #[inline(always)]
    pub fn dfsdmlpen(&self) -> DFSDMLPEN_R {
        DFSDMLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADFSDMLPEN
    #[inline(always)]
    pub fn adfsdmlpen(&self) -> ADFSDMLPEN_R {
        ADFSDMLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - FDCANLPEN
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_APB2LPENSETR")
            .field("tim1lpen", &self.tim1lpen())
            .field("tim8lpen", &self.tim8lpen())
            .field("tim15lpen", &self.tim15lpen())
            .field("tim16lpen", &self.tim16lpen())
            .field("tim17lpen", &self.tim17lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("spi5lpen", &self.spi5lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("sai1lpen", &self.sai1lpen())
            .field("sai2lpen", &self.sai2lpen())
            .field("sai3lpen", &self.sai3lpen())
            .field("dfsdmlpen", &self.dfsdmlpen())
            .field("adfsdmlpen", &self.adfsdmlpen())
            .field("fdcanlpen", &self.fdcanlpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1LPEN
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, MP_APB2LPENSETRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM8LPEN
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<'_, MP_APB2LPENSETRrs> {
        TIM8LPEN_W::new(self, 1)
    }
    ///Bit 2 - TIM15LPEN
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<'_, MP_APB2LPENSETRrs> {
        TIM15LPEN_W::new(self, 2)
    }
    ///Bit 3 - TIM16LPEN
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<'_, MP_APB2LPENSETRrs> {
        TIM16LPEN_W::new(self, 3)
    }
    ///Bit 4 - TIM17LPEN
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<'_, MP_APB2LPENSETRrs> {
        TIM17LPEN_W::new(self, 4)
    }
    ///Bit 8 - SPI1LPEN
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, MP_APB2LPENSETRrs> {
        SPI1LPEN_W::new(self, 8)
    }
    ///Bit 9 - SPI4LPEN
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<'_, MP_APB2LPENSETRrs> {
        SPI4LPEN_W::new(self, 9)
    }
    ///Bit 10 - SPI5LPEN
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<'_, MP_APB2LPENSETRrs> {
        SPI5LPEN_W::new(self, 10)
    }
    ///Bit 13 - USART6LPEN
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<'_, MP_APB2LPENSETRrs> {
        USART6LPEN_W::new(self, 13)
    }
    ///Bit 16 - SAI1LPEN
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<'_, MP_APB2LPENSETRrs> {
        SAI1LPEN_W::new(self, 16)
    }
    ///Bit 17 - SAI2LPEN
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<'_, MP_APB2LPENSETRrs> {
        SAI2LPEN_W::new(self, 17)
    }
    ///Bit 18 - SAI3LPEN
    #[inline(always)]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W<'_, MP_APB2LPENSETRrs> {
        SAI3LPEN_W::new(self, 18)
    }
    ///Bit 20 - DFSDMLPEN
    #[inline(always)]
    pub fn dfsdmlpen(&mut self) -> DFSDMLPEN_W<'_, MP_APB2LPENSETRrs> {
        DFSDMLPEN_W::new(self, 20)
    }
    ///Bit 21 - ADFSDMLPEN
    #[inline(always)]
    pub fn adfsdmlpen(&mut self) -> ADFSDMLPEN_W<'_, MP_APB2LPENSETRrs> {
        ADFSDMLPEN_W::new(self, 21)
    }
    ///Bit 24 - FDCANLPEN
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W<'_, MP_APB2LPENSETRrs> {
        FDCANLPEN_W::new(self, 24)
    }
}
/**This register is used by the MCU in order to clear the PERxLPEN bits

You can [`read`](crate::Reg::read) this register and get [`mp_apb2lpensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_apb2lpensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_APB2LPENSETR)*/
pub struct MP_APB2LPENSETRrs;
impl crate::RegisterSpec for MP_APB2LPENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_apb2lpensetr::R`](R) reader structure
impl crate::Readable for MP_APB2LPENSETRrs {}
///`write(|w| ..)` method takes [`mp_apb2lpensetr::W`](W) writer structure
impl crate::Writable for MP_APB2LPENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_APB2LPENSETR to value 0x0137_271f
impl crate::Resettable for MP_APB2LPENSETRrs {
    const RESET_VALUE: u32 = 0x0137_271f;
}
