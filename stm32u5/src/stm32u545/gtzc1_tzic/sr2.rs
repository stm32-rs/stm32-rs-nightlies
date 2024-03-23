#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2rs>;
#[doc = "Field `TIM1F` reader - illegal access flag for TIM1"]
pub type TIM1F_R = crate::BitReader;
#[doc = "Field `SPI1F` reader - illegal access flag for SPI1"]
pub type SPI1F_R = crate::BitReader;
#[doc = "Field `TIM8F` reader - illegal access flag for TIM8"]
pub type TIM8F_R = crate::BitReader;
#[doc = "Field `USART1F` reader - illegal access flag for USART1"]
pub type USART1F_R = crate::BitReader;
#[doc = "Field `TIM15F` reader - illegal access flag for TIM5"]
pub type TIM15F_R = crate::BitReader;
#[doc = "Field `TIM16F` reader - illegal access flag for TIM6"]
pub type TIM16F_R = crate::BitReader;
#[doc = "Field `TIM17F` reader - illegal access flag for TIM7"]
pub type TIM17F_R = crate::BitReader;
#[doc = "Field `SAI1F` reader - illegal access flag for SAI1"]
pub type SAI1F_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - illegal access flag for TIM1"]
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - illegal access flag for SPI1"]
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - illegal access flag for TIM8"]
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - illegal access flag for USART1"]
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - illegal access flag for TIM5"]
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - illegal access flag for TIM6"]
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - illegal access flag for TIM7"]
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - illegal access flag for SAI1"]
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "TZIC status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2rs {}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2rs {
    const RESET_VALUE: u32 = 0;
}
