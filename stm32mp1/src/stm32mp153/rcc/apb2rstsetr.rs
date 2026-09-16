///Register `APB2RSTSETR` reader
pub type R = crate::R<APB2RSTSETRrs>;
///Register `APB2RSTSETR` writer
pub type W = crate::W<APB2RSTSETRrs>;
///Field `TIM1RST` reader - TIM1RST
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1RST
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8RST` reader - TIM8RST
pub type TIM8RST_R = crate::BitReader;
///Field `TIM8RST` writer - TIM8RST
pub type TIM8RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15RST` reader - TIM15RST
pub type TIM15RST_R = crate::BitReader;
///Field `TIM15RST` writer - TIM15RST
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16RST
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16RST
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RST` reader - TIM17RST
pub type TIM17RST_R = crate::BitReader;
///Field `TIM17RST` writer - TIM17RST
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI1RST
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI1RST
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4RST` reader - SPI4RST
pub type SPI4RST_R = crate::BitReader;
///Field `SPI4RST` writer - SPI4RST
pub type SPI4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5RST` reader - SPI5RST
pub type SPI5RST_R = crate::BitReader;
///Field `SPI5RST` writer - SPI5RST
pub type SPI5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6RST` reader - USART6RST
pub type USART6RST_R = crate::BitReader;
///Field `USART6RST` writer - USART6RST
pub type USART6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1RST` reader - SAI1RST
pub type SAI1RST_R = crate::BitReader;
///Field `SAI1RST` writer - SAI1RST
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2RST` reader - SAI2RST
pub type SAI2RST_R = crate::BitReader;
///Field `SAI2RST` writer - SAI2RST
pub type SAI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI3RST` reader - SAI3RST
pub type SAI3RST_R = crate::BitReader;
///Field `SAI3RST` writer - SAI3RST
pub type SAI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DFSDMRST` reader - DFSDMRST
pub type DFSDMRST_R = crate::BitReader;
///Field `DFSDMRST` writer - DFSDMRST
pub type DFSDMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FDCANRST` reader - FDCANRST
pub type FDCANRST_R = crate::BitReader;
///Field `FDCANRST` writer - FDCANRST
pub type FDCANRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1RST
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8RST
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM15RST
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM16RST
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM17RST
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - SPI1RST
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPI4RST
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPI5RST
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - USART6RST
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - SAI1RST
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAI2RST
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SAI3RST
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - DFSDMRST
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - FDCANRST
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTSETR")
            .field("tim1rst", &self.tim1rst())
            .field("tim8rst", &self.tim8rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("spi1rst", &self.spi1rst())
            .field("spi4rst", &self.spi4rst())
            .field("spi5rst", &self.spi5rst())
            .field("usart6rst", &self.usart6rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .field("sai3rst", &self.sai3rst())
            .field("dfsdmrst", &self.dfsdmrst())
            .field("fdcanrst", &self.fdcanrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1RST
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTSETRrs> {
        TIM1RST_W::new(self, 0)
    }
    ///Bit 1 - TIM8RST
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<'_, APB2RSTSETRrs> {
        TIM8RST_W::new(self, 1)
    }
    ///Bit 2 - TIM15RST
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTSETRrs> {
        TIM15RST_W::new(self, 2)
    }
    ///Bit 3 - TIM16RST
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTSETRrs> {
        TIM16RST_W::new(self, 3)
    }
    ///Bit 4 - TIM17RST
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTSETRrs> {
        TIM17RST_W::new(self, 4)
    }
    ///Bit 8 - SPI1RST
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTSETRrs> {
        SPI1RST_W::new(self, 8)
    }
    ///Bit 9 - SPI4RST
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W<'_, APB2RSTSETRrs> {
        SPI4RST_W::new(self, 9)
    }
    ///Bit 10 - SPI5RST
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W<'_, APB2RSTSETRrs> {
        SPI5RST_W::new(self, 10)
    }
    ///Bit 13 - USART6RST
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W<'_, APB2RSTSETRrs> {
        USART6RST_W::new(self, 13)
    }
    ///Bit 16 - SAI1RST
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<'_, APB2RSTSETRrs> {
        SAI1RST_W::new(self, 16)
    }
    ///Bit 17 - SAI2RST
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<'_, APB2RSTSETRrs> {
        SAI2RST_W::new(self, 17)
    }
    ///Bit 18 - SAI3RST
    #[inline(always)]
    pub fn sai3rst(&mut self) -> SAI3RST_W<'_, APB2RSTSETRrs> {
        SAI3RST_W::new(self, 18)
    }
    ///Bit 20 - DFSDMRST
    #[inline(always)]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W<'_, APB2RSTSETRrs> {
        DFSDMRST_W::new(self, 20)
    }
    ///Bit 24 - FDCANRST
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<'_, APB2RSTSETRrs> {
        FDCANRST_W::new(self, 24)
    }
}
/**This register is used to activate the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`apb2rstsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB2RSTSETR)*/
pub struct APB2RSTSETRrs;
impl crate::RegisterSpec for APB2RSTSETRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstsetr::R`](R) reader structure
impl crate::Readable for APB2RSTSETRrs {}
///`write(|w| ..)` method takes [`apb2rstsetr::W`](W) writer structure
impl crate::Writable for APB2RSTSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTSETR to value 0
impl crate::Resettable for APB2RSTSETRrs {}
