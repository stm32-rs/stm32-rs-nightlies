///Register `MISR` reader
pub type R = crate::R<MISRrs>;
/**TAMP1MF:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MF {
    ///0: No tamper detected - Masked
    Idle = 0,
    ///1: Tamper detected - Masked
    Tamper = 1,
}
impl From<TAMP1MF> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MF) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1MF` reader - TAMP1MF:
pub type TAMP1MF_R = crate::BitReader<TAMP1MF>;
impl TAMP1MF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1MF {
        match self.bits {
            false => TAMP1MF::Idle,
            true => TAMP1MF::Tamper,
        }
    }
    ///No tamper detected - Masked
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == TAMP1MF::Idle
    }
    ///Tamper detected - Masked
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == TAMP1MF::Tamper
    }
}
///Field `TAMP2MF` reader - TAMP2MF
pub use TAMP1MF_R as TAMP2MF_R;
///Field `TAMP3MF` reader - TAMP3MF
pub use TAMP1MF_R as TAMP3MF_R;
/**ITAMP3MF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3MF {
    ///0: No tamper detected - Masked
    Idle = 0,
    ///1: Internal tamper detected - Masked
    Tamper = 1,
}
impl From<ITAMP3MF> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3MF) -> Self {
        variant as u8 != 0
    }
}
///Field `ITAMP3MF` reader - ITAMP3MF
pub type ITAMP3MF_R = crate::BitReader<ITAMP3MF>;
impl ITAMP3MF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3MF {
        match self.bits {
            false => ITAMP3MF::Idle,
            true => ITAMP3MF::Tamper,
        }
    }
    ///No tamper detected - Masked
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ITAMP3MF::Idle
    }
    ///Internal tamper detected - Masked
    #[inline(always)]
    pub fn is_tamper(&self) -> bool {
        *self == ITAMP3MF::Tamper
    }
}
///Field `ITAMP5MF` reader - ITAMP5MF
pub use ITAMP3MF_R as ITAMP5MF_R;
///Field `ITAMP6MF` reader - ITAMP6MF
pub use ITAMP3MF_R as ITAMP6MF_R;
///Field `ITAMP8MF` reader - ITAMP8MF
pub use ITAMP3MF_R as ITAMP8MF_R;
impl R {
    ///Bit 0 - TAMP1MF:
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TAMP2MF
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TAMP3MF
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 18 - ITAMP3MF
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - ITAMP5MF
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ITAMP6MF
    #[inline(always)]
    pub fn itamp6mf(&self) -> ITAMP6MF_R {
        ITAMP6MF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - ITAMP8MF
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISR")
            .field("tamp1mf", &self.tamp1mf())
            .field("tamp2mf", &self.tamp2mf())
            .field("tamp3mf", &self.tamp3mf())
            .field("itamp3mf", &self.itamp3mf())
            .field("itamp5mf", &self.itamp5mf())
            .field("itamp6mf", &self.itamp6mf())
            .field("itamp8mf", &self.itamp8mf())
            .finish()
    }
}
/**TAMP masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TAMP:MISR)*/
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
///`read()` method returns [`misr::R`](R) reader structure
impl crate::Readable for MISRrs {}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISRrs {}
