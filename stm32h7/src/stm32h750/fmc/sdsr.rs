///Register `SDSR` reader
pub type R = crate::R<SDSRrs>;
/**Refresh error flag An interrupt is generated if REIE = 1 and RE = 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    ///0: No refresh error has been detected
    NoError = 0,
    ///1: A refresh error has been detected
    Error = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::NoError,
            true => RE::Error,
        }
    }
    ///No refresh error has been detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RE::NoError
    }
    ///A refresh error has been detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RE::Error
    }
}
/**Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODES1 {
    ///0: Normal Mode
    Normal = 0,
    ///1: Self-refresh mode
    SelfRefresh = 1,
    ///2: Power-down mode
    PowerDown = 2,
}
impl From<MODES1> for u8 {
    #[inline(always)]
    fn from(variant: MODES1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODES1 {
    type Ux = u8;
}
impl crate::IsEnum for MODES1 {}
///Field `MODES1` reader - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1.
pub type MODES1_R = crate::FieldReader<MODES1>;
impl MODES1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODES1> {
        match self.bits {
            0 => Some(MODES1::Normal),
            1 => Some(MODES1::SelfRefresh),
            2 => Some(MODES1::PowerDown),
            _ => None,
        }
    }
    ///Normal Mode
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODES1::Normal
    }
    ///Self-refresh mode
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == MODES1::SelfRefresh
    }
    ///Power-down mode
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == MODES1::PowerDown
    }
}
///Field `MODES2` reader - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2.
pub use MODES1_R as MODES2_R;
impl R {
    ///Bit 0 - Refresh error flag An interrupt is generated if REIE = 1 and RE = 1
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Status Mode for Bank 1 These bits define the Status Mode of SDRAM Bank 1.
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Status Mode for Bank 2 These bits define the Status Mode of SDRAM Bank 2.
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDSR")
            .field("re", &self.re())
            .field("modes1", &self.modes1())
            .field("modes2", &self.modes2())
            .finish()
    }
}
/**SDRAM Status register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#FMC:SDSR)*/
pub struct SDSRrs;
impl crate::RegisterSpec for SDSRrs {
    type Ux = u32;
}
///`read()` method returns [`sdsr::R`](R) reader structure
impl crate::Readable for SDSRrs {}
///`reset()` method sets SDSR to value 0
impl crate::Resettable for SDSRrs {}
