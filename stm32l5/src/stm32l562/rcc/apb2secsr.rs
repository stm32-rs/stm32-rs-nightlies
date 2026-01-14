///Register `APB2SECSR` reader
pub type R = crate::R<APB2SECSRrs>;
///Field `SYSCFGSECF` reader - SYSCFGSECF
pub type SYSCFGSECF_R = crate::BitReader;
///Field `TIM1SECF` reader - TIM1SECF
pub type TIM1SECF_R = crate::BitReader;
///Field `SPI1SECF` reader - SPI1SECF
pub type SPI1SECF_R = crate::BitReader;
///Field `TIM8SECF` reader - TIM8SECF
pub type TIM8SECF_R = crate::BitReader;
///Field `USART1SECF` reader - USART1SECF
pub type USART1SECF_R = crate::BitReader;
///Field `TIM15SECF` reader - TIM15SECF
pub type TIM15SECF_R = crate::BitReader;
///Field `TIM16SECF` reader - TIM16SECF
pub type TIM16SECF_R = crate::BitReader;
///Field `TIM17SECF` reader - TIM17SECF
pub type TIM17SECF_R = crate::BitReader;
///Field `SAI1SECF` reader - SAI1SECF
pub type SAI1SECF_R = crate::BitReader;
///Field `SAI2SECF` reader - SAI2SECF
pub type SAI2SECF_R = crate::BitReader;
///Field `DFSDM1SECF` reader - DFSDM1SECF
pub type DFSDM1SECF_R = crate::BitReader;
impl R {
    ///Bit 0 - SYSCFGSECF
    #[inline(always)]
    pub fn syscfgsecf(&self) -> SYSCFGSECF_R {
        SYSCFGSECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1SECF
    #[inline(always)]
    pub fn tim1secf(&self) -> TIM1SECF_R {
        TIM1SECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1SECF
    #[inline(always)]
    pub fn spi1secf(&self) -> SPI1SECF_R {
        SPI1SECF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM8SECF
    #[inline(always)]
    pub fn tim8secf(&self) -> TIM8SECF_R {
        TIM8SECF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USART1SECF
    #[inline(always)]
    pub fn usart1secf(&self) -> USART1SECF_R {
        USART1SECF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15SECF
    #[inline(always)]
    pub fn tim15secf(&self) -> TIM15SECF_R {
        TIM15SECF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16SECF
    #[inline(always)]
    pub fn tim16secf(&self) -> TIM16SECF_R {
        TIM16SECF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17SECF
    #[inline(always)]
    pub fn tim17secf(&self) -> TIM17SECF_R {
        TIM17SECF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - SAI1SECF
    #[inline(always)]
    pub fn sai1secf(&self) -> SAI1SECF_R {
        SAI1SECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2SECF
    #[inline(always)]
    pub fn sai2secf(&self) -> SAI2SECF_R {
        SAI2SECF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - DFSDM1SECF
    #[inline(always)]
    pub fn dfsdm1secf(&self) -> DFSDM1SECF_R {
        DFSDM1SECF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2SECSR")
            .field("dfsdm1secf", &self.dfsdm1secf())
            .field("sai2secf", &self.sai2secf())
            .field("sai1secf", &self.sai1secf())
            .field("tim17secf", &self.tim17secf())
            .field("tim16secf", &self.tim16secf())
            .field("tim15secf", &self.tim15secf())
            .field("usart1secf", &self.usart1secf())
            .field("tim8secf", &self.tim8secf())
            .field("spi1secf", &self.spi1secf())
            .field("tim1secf", &self.tim1secf())
            .field("syscfgsecf", &self.syscfgsecf())
            .finish()
    }
}
/**RCC APB2 security status register

You can [`read`](crate::Reg::read) this register and get [`apb2secsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:APB2SECSR)*/
pub struct APB2SECSRrs;
impl crate::RegisterSpec for APB2SECSRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2secsr::R`](R) reader structure
impl crate::Readable for APB2SECSRrs {}
///`reset()` method sets APB2SECSR to value 0
impl crate::Resettable for APB2SECSRrs {}
