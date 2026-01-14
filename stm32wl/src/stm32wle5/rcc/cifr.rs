///Register `CIFR` reader
pub type R = crate::R<CIFRrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYF {
    ///0: Not interrupted
    NotInterrupted = 0,
    ///1: Interrupted
    Interrupted = 1,
}
impl From<LSIRDYF> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYF` reader - LSI ready interrupt flag
pub type LSIRDYF_R = crate::BitReader<LSIRDYF>;
impl LSIRDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYF {
        match self.bits {
            false => LSIRDYF::NotInterrupted,
            true => LSIRDYF::Interrupted,
        }
    }
    ///Not interrupted
    #[inline(always)]
    pub fn is_not_interrupted(&self) -> bool {
        *self == LSIRDYF::NotInterrupted
    }
    ///Interrupted
    #[inline(always)]
    pub fn is_interrupted(&self) -> bool {
        *self == LSIRDYF::Interrupted
    }
}
///Field `LSERDYF` reader - LSE ready interrupt flag
pub use LSIRDYF_R as LSERDYF_R;
///Field `MSIRDYF` reader - MSI ready interrupt flag
pub use LSIRDYF_R as MSIRDYF_R;
///Field `HSIRDYF` reader - HSI16 ready interrupt flag
pub use LSIRDYF_R as HSIRDYF_R;
///Field `HSERDYF` reader - HSE32 ready interrupt flag
pub use LSIRDYF_R as HSERDYF_R;
///Field `PLLRDYF` reader - PLL ready interrupt flag
pub use LSIRDYF_R as PLLRDYF_R;
///Field `CSSF` reader - HSE32 Clock security system interrupt flag
pub use LSIRDYF_R as CSSF_R;
///Field `LSECSSF` reader - LSE Clock security system interrupt flag
pub use LSIRDYF_R as LSECSSF_R;
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
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
    ///Bit 3 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE32 ready interrupt flag
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - HSE32 Clock security system interrupt flag
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSE Clock security system interrupt flag
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIFR")
            .field("lsirdyf", &self.lsirdyf())
            .field("lsecssf", &self.lsecssf())
            .field("cssf", &self.cssf())
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#RCC:CIFR)*/
pub struct CIFRrs;
impl crate::RegisterSpec for CIFRrs {
    type Ux = u32;
}
///`read()` method returns [`cifr::R`](R) reader structure
impl crate::Readable for CIFRrs {}
///`reset()` method sets CIFR to value 0
impl crate::Resettable for CIFRrs {}
