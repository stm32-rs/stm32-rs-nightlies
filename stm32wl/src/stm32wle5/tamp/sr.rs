///Register `SR` reader
pub type R = crate::R<SRrs>;
/**TAMP1F

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1F {
    ///0: No tamper detected
    Idle = 0,
    ///1: Tamper detected
    Tamper = 1,
}
impl From<TAMP1F> for bool {
    #[inline(always)]
    fn from(variant: TAMP1F) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1F` reader - TAMP1F
pub type TAMP1F_R = crate::BitReader<TAMP1F>;
impl TAMP1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1F {
        match self.bits {
            false => TAMP1F::Idle,
            true => TAMP1F::Tamper,
        }
    }
    ///No tamper detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TAMP1F::Idle
    }
    ///Tamper detected
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == TAMP1F::Tamper
    }
}
///Field `TAMP2F` reader - TAMP2F
pub use TAMP1F_R as TAMP2F_R;
///Field `TAMP3F` reader - TAMP3F
pub use TAMP1F_R as TAMP3F_R;
/**ITAMP3F

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3F {
    ///0: No tamper detected
    Idle = 0,
    ///1: Internal tamper detected
    Tamper = 1,
}
impl From<ITAMP3F> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3F) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3F` reader - ITAMP3F
pub type ITAMP3F_R = crate::BitReader<ITAMP3F>;
impl ITAMP3F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3F {
        match self.bits {
            false => ITAMP3F::Idle,
            true => ITAMP3F::Tamper,
        }
    }
    ///No tamper detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ITAMP3F::Idle
    }
    ///Internal tamper detected
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == ITAMP3F::Tamper
    }
}
///Field `ITAMP5F` reader - ITAMP5F
pub use ITAMP3F_R as ITAMP5F_R;
///Field `ITAMP6F` reader - ITAMP6F
pub use ITAMP3F_R as ITAMP6F_R;
///Field `ITAMP8F` reader - ITAMP8F
pub use ITAMP3F_R as ITAMP8F_R;
impl R {
    ///Bit 0 - TAMP1F
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2F
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3F
    #[inline(always)]
    pub fn tamp3f(&self) -> TAMP3F_R {
        TAMP3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 18 - ITAMP3F
    #[inline(always)]
    pub fn itamp3f(&self) -> ITAMP3F_R {
        ITAMP3F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5F
    #[inline(always)]
    pub fn itamp5f(&self) -> ITAMP5F_R {
        ITAMP5F_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6F
    #[inline(always)]
    pub fn itamp6f(&self) -> ITAMP6F_R {
        ITAMP6F_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - ITAMP8F
    #[inline(always)]
    pub fn itamp8f(&self) -> ITAMP8F_R {
        ITAMP8F_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("tamp1f", &self.tamp1f())
            .field("tamp2f", &self.tamp2f())
            .field("tamp3f", &self.tamp3f())
            .field("itamp3f", &self.itamp3f())
            .field("itamp5f", &self.itamp5f())
            .field("itamp6f", &self.itamp6f())
            .field("itamp8f", &self.itamp8f())
            .finish()
    }
}
/**TAMP status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TAMP:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
