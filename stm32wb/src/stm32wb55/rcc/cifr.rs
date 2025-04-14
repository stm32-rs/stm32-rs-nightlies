///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI1 ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSI1RDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
}
impl From<LSI1RDYF> for bool {
    #[inline(always)]
    fn from(variant: LSI1RDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSI1RDYF` reader - LSI1 ready interrupt flag
pub type LSI1RDYF_R = crate::BitReader<LSI1RDYF>;
impl LSI1RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSI1RDYF {
        match self.bits {
            false => LSI1RDYF::NotInterrupted,
            true => LSI1RDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSI1RDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSI1RDYF::Interrupted
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub use LSI1RDYF_R as LSERDYF_R;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub use LSI1RDYF_R as MSIRDYF_R;
///Field `HSIRDYF` reader - HSI ready interrupt flag
pub use LSI1RDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE ready interrupt flag
pub use LSI1RDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub use LSI1RDYF_R as PLLRDYF_R;
///Field `PLLSAI1RDYF` reader - PLLSAI1 ready interrupt flag
pub use LSI1RDYF_R as PLLSAI1RDYF_R;
///Field `CSSF` reader - HSE Clock security system interrupt flag
pub use LSI1RDYF_R as CSSF_R;
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub use LSI1RDYF_R as LSECSSF_R;
///Field `HSI48RDYF` reader - HSI48 ready interrupt flag
pub use LSI1RDYF_R as HSI48RDYF_R;
///Field `LSI2RDYF` reader - LSI2 ready interrupt flag
pub use LSI1RDYF_R as LSI2RDYF_R;
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
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("lsi1rdyf", &self.lsi1rdyf())
            .field("lsi2rdyf", &self.lsi2rdyf())
            .field("hsi48rdyf", &self.hsi48rdyf())
            .field("lsecssf", &self.lsecssf())
            .field("cssf", &self.cssf())
            .field("pllsai1rdyf", &self.pllsai1rdyf())
            .field("pllrdyf", &self.pllrdyf())
            .field("hserdyf", &self.hserdyf())
            .field("hsirdyf", &self.hsirdyf())
            .field("msirdyf", &self.msirdyf())
            .field("lserdyf", &self.lserdyf())
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
impl crate::Resettable for CIFRrs {}
