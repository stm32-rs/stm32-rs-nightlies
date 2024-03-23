#[doc = "Register `APB2SECSR` reader"]
pub type R = crate::R<APB2SECSRrs>;
#[doc = "Field `SYSCFGSECF` reader - SYSCFGSECF"]
pub type SYSCFGSECF_R = crate::BitReader;
#[doc = "Field `TIM1SECF` reader - TIM1SECF"]
pub type TIM1SECF_R = crate::BitReader;
#[doc = "Field `SPI1SECF` reader - SPI1SECF"]
pub type SPI1SECF_R = crate::BitReader;
#[doc = "Field `TIM8SECF` reader - TIM8SECF"]
pub type TIM8SECF_R = crate::BitReader;
#[doc = "Field `USART1SECF` reader - USART1SECF"]
pub type USART1SECF_R = crate::BitReader;
#[doc = "Field `TIM15SECF` reader - TIM15SECF"]
pub type TIM15SECF_R = crate::BitReader;
#[doc = "Field `TIM16SECF` reader - TIM16SECF"]
pub type TIM16SECF_R = crate::BitReader;
#[doc = "Field `TIM17SECF` reader - TIM17SECF"]
pub type TIM17SECF_R = crate::BitReader;
#[doc = "Field `SAI1SECF` reader - SAI1SECF"]
pub type SAI1SECF_R = crate::BitReader;
#[doc = "Field `SAI2SECF` reader - SAI2SECF"]
pub type SAI2SECF_R = crate::BitReader;
#[doc = "Field `DFSDM1SECF` reader - DFSDM1SECF"]
pub type DFSDM1SECF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SYSCFGSECF"]
    #[inline(always)]
    pub fn syscfgsecf(&self) -> SYSCFGSECF_R {
        SYSCFGSECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1SECF"]
    #[inline(always)]
    pub fn tim1secf(&self) -> TIM1SECF_R {
        TIM1SECF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1SECF"]
    #[inline(always)]
    pub fn spi1secf(&self) -> SPI1SECF_R {
        SPI1SECF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8SECF"]
    #[inline(always)]
    pub fn tim8secf(&self) -> TIM8SECF_R {
        TIM8SECF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1SECF"]
    #[inline(always)]
    pub fn usart1secf(&self) -> USART1SECF_R {
        USART1SECF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15SECF"]
    #[inline(always)]
    pub fn tim15secf(&self) -> TIM15SECF_R {
        TIM15SECF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16SECF"]
    #[inline(always)]
    pub fn tim16secf(&self) -> TIM16SECF_R {
        TIM16SECF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17SECF"]
    #[inline(always)]
    pub fn tim17secf(&self) -> TIM17SECF_R {
        TIM17SECF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI1SECF"]
    #[inline(always)]
    pub fn sai1secf(&self) -> SAI1SECF_R {
        SAI1SECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI2SECF"]
    #[inline(always)]
    pub fn sai2secf(&self) -> SAI2SECF_R {
        SAI2SECF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - DFSDM1SECF"]
    #[inline(always)]
    pub fn dfsdm1secf(&self) -> DFSDM1SECF_R {
        DFSDM1SECF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "RCC APB2 security status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb2secsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB2SECSRrs;
impl crate::RegisterSpec for APB2SECSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2secsr::R`](R) reader structure"]
impl crate::Readable for APB2SECSRrs {}
#[doc = "`reset()` method sets APB2SECSR to value 0"]
impl crate::Resettable for APB2SECSRrs {
    const RESET_VALUE: u32 = 0;
}
