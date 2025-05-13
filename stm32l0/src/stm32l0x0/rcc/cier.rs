///Register `CIER` reader
pub type R = crate::R<CIERrs>;
/**LSI ready interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYIE {
    ///0: Ready interrupt disabled
    Disabled = 0,
    ///1: Ready interrupt enabled
    Enabled = 1,
}
impl From<LSIRDYIE> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LSIRDYIE` reader - LSI ready interrupt flag
pub type LSIRDYIE_R = crate::BitReader<LSIRDYIE>;
impl LSIRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYIE {
        match self.bits {
            false => LSIRDYIE::Disabled,
            true => LSIRDYIE::Enabled,
        }
    }
    ///Ready interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSIRDYIE::Disabled
    }
    ///Ready interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSIRDYIE::Enabled
    }
}
///Field `LSERDYIE` reader - LSE ready interrupt flag
pub use LSIRDYIE_R as LSERDYIE_R;
///Field `HSI16RDYIE` reader - HSI16 ready interrupt flag
pub use LSIRDYIE_R as HSI16RDYIE_R;
///Field `HSERDYIE` reader - HSE ready interrupt flag
pub use LSIRDYIE_R as HSERDYIE_R;
///Field `PLLRDYIE` reader - PLL ready interrupt flag
pub use LSIRDYIE_R as PLLRDYIE_R;
///Field `MSIRDYIE` reader - MSI ready interrupt flag
pub use LSIRDYIE_R as MSIRDYIE_R;
/**LSE CSS interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSE {
    ///0: LSE CSS interrupt disabled
    Disabled = 0,
    ///1: LSE CSS interrupt enabled
    Enabled = 1,
}
impl From<CSSLSE> for bool {
    #[inline(always)]
    fn from(variant: CSSLSE) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSLSE` reader - LSE CSS interrupt flag
pub type CSSLSE_R = crate::BitReader<CSSLSE>;
impl CSSLSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSE {
        match self.bits {
            false => CSSLSE::Disabled,
            true => CSSLSE::Enabled,
        }
    }
    ///LSE CSS interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSLSE::Disabled
    }
    ///LSE CSS interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSLSE::Enabled
    }
}
impl R {
    ///Bit 0 - LSI ready interrupt flag
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt flag
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSI16 ready interrupt flag
    #[inline(always)]
    pub fn hsi16rdyie(&self) -> HSI16RDYIE_R {
        HSI16RDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSE ready interrupt flag
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL ready interrupt flag
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MSI ready interrupt flag
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LSE CSS interrupt flag
    #[inline(always)]
    pub fn csslse(&self) -> CSSLSE_R {
        CSSLSE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("csslse", &self.csslse())
            .field("lsirdyie", &self.lsirdyie())
            .field("msirdyie", &self.msirdyie())
            .field("pllrdyie", &self.pllrdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsi16rdyie", &self.hsi16rdyie())
            .field("lserdyie", &self.lserdyie())
            .finish()
    }
}
/**Clock interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
