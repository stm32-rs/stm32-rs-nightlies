///Register `C2APB2ENR` reader
pub type R = crate::R<C2APB2ENRrs>;
///Register `C2APB2ENR` writer
pub type W = crate::W<C2APB2ENRrs>;
///Field `TIM1EN` reader - CPU2 TIM1 timer clock enable
pub type TIM1EN_R = crate::BitReader;
///Field `TIM1EN` writer - CPU2 TIM1 timer clock enable
pub type TIM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1EN` reader - CPU2 SPI1 clock enable
pub type SPI1EN_R = crate::BitReader;
///Field `SPI1EN` writer - CPU2 SPI1 clock enable
pub type SPI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1EN` reader - CPU2 USART1clock enable
pub type USART1EN_R = crate::BitReader;
///Field `USART1EN` writer - CPU2 USART1clock enable
pub type USART1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16EN` reader - CPU2 TIM16 timer clock enable
pub type TIM16EN_R = crate::BitReader;
///Field `TIM16EN` writer - CPU2 TIM16 timer clock enable
pub type TIM16EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17EN` reader - CPU2 TIM17 timer clock enable
pub type TIM17EN_R = crate::BitReader;
///Field `TIM17EN` writer - CPU2 TIM17 timer clock enable
pub type TIM17EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1EN` reader - CPU2 SAI1 clock enable
pub type SAI1EN_R = crate::BitReader;
///Field `SAI1EN` writer - CPU2 SAI1 clock enable
pub type SAI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU2 USART1clock enable
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - CPU2 SAI1 clock enable
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB2ENR")
            .field("sai1en", &self.sai1en())
            .field("tim17en", &self.tim17en())
            .field("tim16en", &self.tim16en())
            .field("usart1en", &self.usart1en())
            .field("spi1en", &self.spi1en())
            .field("tim1en", &self.tim1en())
            .finish()
    }
}
impl W {
    ///Bit 11 - CPU2 TIM1 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim1en(&mut self) -> TIM1EN_W<C2APB2ENRrs> {
        TIM1EN_W::new(self, 11)
    }
    ///Bit 12 - CPU2 SPI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn spi1en(&mut self) -> SPI1EN_W<C2APB2ENRrs> {
        SPI1EN_W::new(self, 12)
    }
    ///Bit 14 - CPU2 USART1clock enable
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<C2APB2ENRrs> {
        USART1EN_W::new(self, 14)
    }
    ///Bit 17 - CPU2 TIM16 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim16en(&mut self) -> TIM16EN_W<C2APB2ENRrs> {
        TIM16EN_W::new(self, 17)
    }
    ///Bit 18 - CPU2 TIM17 timer clock enable
    #[inline(always)]
    #[must_use]
    pub fn tim17en(&mut self) -> TIM17EN_W<C2APB2ENRrs> {
        TIM17EN_W::new(self, 18)
    }
    ///Bit 21 - CPU2 SAI1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn sai1en(&mut self) -> SAI1EN_W<C2APB2ENRrs> {
        SAI1EN_W::new(self, 21)
    }
}
/**CPU2 APB2ENR

You can [`read`](crate::Reg::read) this register and get [`c2apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:C2APB2ENR)*/
pub struct C2APB2ENRrs;
impl crate::RegisterSpec for C2APB2ENRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb2enr::R`](R) reader structure
impl crate::Readable for C2APB2ENRrs {}
///`write(|w| ..)` method takes [`c2apb2enr::W`](W) writer structure
impl crate::Writable for C2APB2ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets C2APB2ENR to value 0
impl crate::Resettable for C2APB2ENRrs {
    const RESET_VALUE: u32 = 0;
}
