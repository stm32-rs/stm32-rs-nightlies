#[doc = "Reader of register APB2SECSR"]
pub type R = crate::R<u32, super::APB2SECSR>;
#[doc = "Reader of field `DFSDM1SECF`"]
pub type DFSDM1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAI2SECF`"]
pub type SAI2SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SAI1SECF`"]
pub type SAI1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM17SECF`"]
pub type TIM17SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM16SECF`"]
pub type TIM16SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM15SECF`"]
pub type TIM15SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `USART1SECF`"]
pub type USART1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM8SECF`"]
pub type TIM8SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPI1SECF`"]
pub type SPI1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIM1SECF`"]
pub type TIM1SECF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSCFGSECF`"]
pub type SYSCFGSECF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 24 - DFSDM1SECF"]
    #[inline(always)]
    pub fn dfsdm1secf(&self) -> DFSDM1SECF_R {
        DFSDM1SECF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SAI2SECF"]
    #[inline(always)]
    pub fn sai2secf(&self) -> SAI2SECF_R {
        SAI2SECF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SAI1SECF"]
    #[inline(always)]
    pub fn sai1secf(&self) -> SAI1SECF_R {
        SAI1SECF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17SECF"]
    #[inline(always)]
    pub fn tim17secf(&self) -> TIM17SECF_R {
        TIM17SECF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16SECF"]
    #[inline(always)]
    pub fn tim16secf(&self) -> TIM16SECF_R {
        TIM16SECF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15SECF"]
    #[inline(always)]
    pub fn tim15secf(&self) -> TIM15SECF_R {
        TIM15SECF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - USART1SECF"]
    #[inline(always)]
    pub fn usart1secf(&self) -> USART1SECF_R {
        USART1SECF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TIM8SECF"]
    #[inline(always)]
    pub fn tim8secf(&self) -> TIM8SECF_R {
        TIM8SECF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1SECF"]
    #[inline(always)]
    pub fn spi1secf(&self) -> SPI1SECF_R {
        SPI1SECF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TIM1SECF"]
    #[inline(always)]
    pub fn tim1secf(&self) -> TIM1SECF_R {
        TIM1SECF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SYSCFGSECF"]
    #[inline(always)]
    pub fn syscfgsecf(&self) -> SYSCFGSECF_R {
        SYSCFGSECF_R::new((self.bits & 0x01) != 0)
    }
}
