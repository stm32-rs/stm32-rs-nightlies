///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
///Field `LSI1RDYF` reader - LSI1 ready interrupt flag
pub type LSI1RDYF_R = crate::BitReader;
///Field `LSERDYF` reader - LSE ready interrupt flag
pub type LSERDYF_R = crate::BitReader;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub type MSIRDYF_R = crate::BitReader;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub type HSIRDYF_R = crate::BitReader;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub type HSERDYF_R = crate::BitReader;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub type PLLRDYF_R = crate::BitReader;
///Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag
pub type PLLSAI1RDYF_R = crate::BitReader;
///Field `HSECSSF` reader - HSE Clock security system interrupt flag
pub type HSECSSF_R = crate::BitReader;
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub type LSECSSF_R = crate::BitReader;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub type HSI48RDYF_R = crate::BitReader;
///Field `LSI2RDYF` reader - LSI2 ready interrupt flag
pub type LSI2RDYF_R = crate::BitReader;
impl R {
    ///Bit 0 - LSI1 ready interrupt flag
    #[inline(always)]
    pub fn lsi1rdyf(&self) -> LSI1RDYF_R {
        LSI1RDYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyf(&self) -> MSIRDYF_R {
        MSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt flag
    #[inline(always)]
    pub fn pllsai1rdyf(&self) -> PLLSAI1RDYF_R {
        PLLSAI1RDYF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - HSE Clock security system interrupt flag
    #[inline(always)]
    pub fn hsecssf(&self) -> HSECSSF_R {
        HSECSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt flag
    #[inline(always)]
    pub fn hsi48rdyf(&self) -> HSI48RDYF_R {
        HSI48RDYF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LSI2 ready interrupt flag
    #[inline(always)]
    pub fn lsi2rdyf(&self) -> LSI2RDYF_R {
        LSI2RDYF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsi2rdyf", &self.lsi2rdyf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("lsecssf", &self.lsecssf())
            .field("hsecssf", &self.hsecssf())
            .field("pllsai1rdyf", &self.pllsai1rdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("msirdyf", &self.msirdyf())
            .field("lserdyf", &self.lserdyf())
            .field("lsi1rdyf", &self.lsi1rdyf())
            .finish()
    }
}
/**Clock interrupt flag register

You can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {
    const RESET_VALUE: u32 = 0;
}
