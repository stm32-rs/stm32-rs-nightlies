///Register `RCC_CIFR` reader
pub type R = crate::R<RCC_CIFRrs>;
///Field `LSIRDYF` reader - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.
pub type LSIRDYF_R = crate::BitReader;
///Field `LSERDYF` reader - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.
pub type LSERDYF_R = crate::BitReader;
///Field `MSIRDYF` reader - MSI ready interrupt flag Set by hardware when the MSI clock becomes stable and MSIRDYDIE is set. Cleared by software setting the MSIRDYC bit.
pub type MSIRDYF_R = crate::BitReader;
///Field `HSIRDYF` reader - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
pub type HSIRDYF_R = crate::BitReader;
///Field `HSERDYF` reader - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
pub type HSERDYF_R = crate::BitReader;
///Field `PLLRDYF` reader - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYIE is set. Cleared by software setting the PLLRDYC bit.
pub type PLLRDYF_R = crate::BitReader;
///Field `CSSF` reader - HSE clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
pub type CSSF_R = crate::BitReader;
///Field `LSECSSF` reader - LSE clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.
pub type LSECSSF_R = crate::BitReader;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to RCC clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit.
pub type HSI48RDYF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYDIE is set. Cleared by software setting the LSIRDYC bit.
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYDIE is set. Cleared by software setting the LSERDYC bit.
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt flag Set by hardware when the MSI clock becomes stable and MSIRDYDIE is set. Cleared by software setting the MSIRDYC bit.
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16 ready interrupt flag Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in a response to setting the HSION (refer to Clock control register (RCC_CR)). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit.
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit.
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag Set by hardware when the PLL locks and PLLRDYIE is set. Cleared by software setting the PLLRDYC bit.
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - HSE clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit.
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt flag Set by hardware when a failure is detected in the LSE oscillator. Cleared by software by setting the LSECSSC bit.
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set in a response to setting the HSI48ON (refer to RCC clock recovery RC register (RCC_CRRCR)). Cleared by software setting the HSI48RDYC bit.
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("msirdyf", &self.msirdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("hserdyf", &self.hserdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("cssf", &self.cssf())
            .field("lsecssf", &self.lsecssf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .finish()
    }
}
/**Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`rcc_cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#RCC:RCC_CIFR)*/
pub struct RCC_CIFRrs;
impl crate::RegisterSpec for RCC_CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cifr::R`](R) reader structure
impl crate::Readable for RCC_CIFRrs {}
///`reset()` method sets RCC_CIFR to value 0
impl crate::Resettable for RCC_CIFRrs {
    const RESET_VALUE: u32 = 0;
}