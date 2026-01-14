///Register `APB2ENR` reader
pub type R = crate::R<APB2ENRrs>;
///Register `APB2ENR` writer
pub type W = crate::W<APB2ENRrs>;
///Field `TIM1EN` reader - TIM1 enable
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - TIM1 enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8EN` reader - TIM8 enable
pub type TIM8EN_R = crate::BitReader;
///Field `TIM8EN` writer - TIM8 enable
pub type TIM8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - USART1 enable
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - USART1 enable
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6EN` reader - USART6 enable
pub type USART6EN_R = crate::BitReader;
///Field `USART6EN` writer - USART6 enable
pub type USART6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9EN` reader - UART9 enable
pub type UART9EN_R = crate::BitReader;
///Field `UART9EN` writer - UART9 enable
pub type UART9EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10EN` reader - USART10 enable
pub type USART10EN_R = crate::BitReader;
///Field `USART10EN` writer - USART10 enable
pub type USART10EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1EN` reader - SPI1 enable
pub type SPI1EN_R = crate::BitReader;
///Field `SPI1EN` writer - SPI1 enable
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4EN` reader - SPI4 enable
pub type SPI4EN_R = crate::BitReader;
///Field `SPI4EN` writer - SPI4 enable
pub type SPI4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18EN` reader - TIM18 enable
pub type TIM18EN_R = crate::BitReader;
///Field `TIM18EN` writer - TIM18 enable
pub type TIM18EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15EN` reader - TIM15 enable
pub type TIM15EN_R = crate::BitReader;
///Field `TIM15EN` writer - TIM15 enable
pub type TIM15EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - TIM16 enable
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - TIM16 enable
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17EN` reader - TIM17 enable
pub type TIM17EN_R = crate::BitReader;
///Field `TIM17EN` writer - TIM17 enable
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9EN` reader - TIM9 enable
pub type TIM9EN_R = crate::BitReader;
///Field `TIM9EN` writer - TIM9 enable
pub type TIM9EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5EN` reader - SPI5 enable
pub type SPI5EN_R = crate::BitReader;
///Field `SPI5EN` writer - SPI5 enable
pub type SPI5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1EN` reader - SAI1 enable
pub type SAI1EN_R = crate::BitReader;
///Field `SAI1EN` writer - SAI1 enable
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2EN` reader - SAI2 enable
pub type SAI2EN_R = crate::BitReader;
///Field `SAI2EN` writer - SAI2 enable
pub type SAI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 enable
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 enable
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - UART9 enable
    #[inline(always)]
    pub fn uart9en(&self) -> UART9EN_R {
        UART9EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USART10 enable
    #[inline(always)]
    pub fn usart10en(&self) -> USART10EN_R {
        USART10EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - SPI1 enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 enable
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - TIM18 enable
    #[inline(always)]
    pub fn tim18en(&self) -> TIM18EN_R {
        TIM18EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 enable
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM9 enable
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPI5 enable
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SAI1 enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 enable
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2ENR")
            .field("tim1en", &self.tim1en())
            .field("tim8en", &self.tim8en())
            .field("usart1en", &self.usart1en())
            .field("usart6en", &self.usart6en())
            .field("uart9en", &self.uart9en())
            .field("usart10en", &self.usart10en())
            .field("spi1en", &self.spi1en())
            .field("spi4en", &self.spi4en())
            .field("tim18en", &self.tim18en())
            .field("tim15en", &self.tim15en())
            .field("tim16en", &self.tim16en())
            .field("tim17en", &self.tim17en())
            .field("tim9en", &self.tim9en())
            .field("spi5en", &self.spi5en())
            .field("sai1en", &self.sai1en())
            .field("sai2en", &self.sai2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<'_, APB2ENRrs> {
        TIM1EN_W::new(self, 0)
    }
    ///Bit 1 - TIM8 enable
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<'_, APB2ENRrs> {
        TIM8EN_W::new(self, 1)
    }
    ///Bit 4 - USART1 enable
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<'_, APB2ENRrs> {
        USART1EN_W::new(self, 4)
    }
    ///Bit 5 - USART6 enable
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W<'_, APB2ENRrs> {
        USART6EN_W::new(self, 5)
    }
    ///Bit 6 - UART9 enable
    #[inline(always)]
    pub fn uart9en(&mut self) -> UART9EN_W<'_, APB2ENRrs> {
        UART9EN_W::new(self, 6)
    }
    ///Bit 7 - USART10 enable
    #[inline(always)]
    pub fn usart10en(&mut self) -> USART10EN_W<'_, APB2ENRrs> {
        USART10EN_W::new(self, 7)
    }
    ///Bit 12 - SPI1 enable
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<'_, APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 13 - SPI4 enable
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W<'_, APB2ENRrs> {
        SPI4EN_W::new(self, 13)
    }
    ///Bit 15 - TIM18 enable
    #[inline(always)]
    pub fn tim18en(&mut self) -> TIM18EN_W<'_, APB2ENRrs> {
        TIM18EN_W::new(self, 15)
    }
    ///Bit 16 - TIM15 enable
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W<'_, APB2ENRrs> {
        TIM15EN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 enable
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<'_, APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 enable
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<'_, APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 19 - TIM9 enable
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<'_, APB2ENRrs> {
        TIM9EN_W::new(self, 19)
    }
    ///Bit 20 - SPI5 enable
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W<'_, APB2ENRrs> {
        SPI5EN_W::new(self, 20)
    }
    ///Bit 21 - SAI1 enable
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<'_, APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 enable
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W<'_, APB2ENRrs> {
        SAI2EN_W::new(self, 22)
    }
}
/**RCC APB2 enable register

You can [`read`](crate::Reg::read) this register and get [`apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB2ENR)*/
pub struct APB2ENRrs;
impl crate::RegisterSpec for APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2enr::R`](R) reader structure
impl crate::Readable for APB2ENRrs {}
///`write(|w| ..)` method takes [`apb2enr::W`](W) writer structure
impl crate::Writable for APB2ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENR to value 0
impl crate::Resettable for APB2ENRrs {}
