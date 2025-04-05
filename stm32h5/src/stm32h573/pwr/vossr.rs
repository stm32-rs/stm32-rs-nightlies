///Register `VOSSR` reader
pub type R = crate::R<VOSSRrs>;
/**Ready bit for V sub CORE /sub voltage scaling output selection.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOSRDYR {
    ///0: Not ready, voltage level below VOS selected level
    NotReady = 0,
    ///1: Ready, voltage level at or above VOS selected level
    Ready = 1,
}
impl From<VOSRDYR> for bool {
    #[inline(always)]
    fn from(variant: VOSRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `VOSRDY` reader - Ready bit for V sub CORE /sub voltage scaling output selection.
pub type VOSRDY_R = crate::BitReader<VOSRDYR>;
impl VOSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VOSRDYR {
        match self.bits {
            false => VOSRDYR::NotReady,
            true => VOSRDYR::Ready,
        }
    }
    ///Not ready, voltage level below VOS selected level
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VOSRDYR::NotReady
    }
    ///Ready, voltage level at or above VOS selected level
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VOSRDYR::Ready
    }
}
/**Voltage level ready for currently used VOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACTVOSRDYR {
    ///0: VCORE is above or below the current voltage scaling provided by ACTVOS\[1:0\]
    NotReady = 0,
    ///1: VCORE is equal to the current voltage scaling provided by ACTVOS\[1:0\]
    Ready = 1,
}
impl From<ACTVOSRDYR> for bool {
    #[inline(always)]
    fn from(variant: ACTVOSRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `ACTVOSRDY` reader - Voltage level ready for currently used VOS
pub type ACTVOSRDY_R = crate::BitReader<ACTVOSRDYR>;
impl ACTVOSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOSRDYR {
        match self.bits {
            false => ACTVOSRDYR::NotReady,
            true => ACTVOSRDYR::Ready,
        }
    }
    ///VCORE is above or below the current voltage scaling provided by ACTVOS\[1:0\]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ACTVOSRDYR::NotReady
    }
    ///VCORE is equal to the current voltage scaling provided by ACTVOS\[1:0\]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ACTVOSRDYR::Ready
    }
}
/**voltage output scaling currently applied to V sub CORE /sub This field provides the last VOS value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACTVOSR {
    ///0: VOS3 (lowest power)
    Vos3 = 0,
    ///1: VOS2
    Vos2 = 1,
    ///2: VOS1
    Vos1 = 2,
    ///3: VOS0 (highest frequency)
    Vos0 = 3,
}
impl From<ACTVOSR> for u8 {
    #[inline(always)]
    fn from(variant: ACTVOSR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ACTVOSR {
    type Ux = u8;
}
impl crate::IsEnum for ACTVOSR {}
///Field `ACTVOS` reader - voltage output scaling currently applied to V sub CORE /sub This field provides the last VOS value.
pub type ACTVOS_R = crate::FieldReader<ACTVOSR>;
impl ACTVOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACTVOSR {
        match self.bits {
            0 => ACTVOSR::Vos3,
            1 => ACTVOSR::Vos2,
            2 => ACTVOSR::Vos1,
            3 => ACTVOSR::Vos0,
            _ => unreachable!(),
        }
    }
    ///VOS3 (lowest power)
    #[inline(always)]
    pub fn is_vos3(&self) -> bool {
        *self == ACTVOSR::Vos3
    }
    ///VOS2
    #[inline(always)]
    pub fn is_vos2(&self) -> bool {
        *self == ACTVOSR::Vos2
    }
    ///VOS1
    #[inline(always)]
    pub fn is_vos1(&self) -> bool {
        *self == ACTVOSR::Vos1
    }
    ///VOS0 (highest frequency)
    #[inline(always)]
    pub fn is_vos0(&self) -> bool {
        *self == ACTVOSR::Vos0
    }
}
impl R {
    ///Bit 3 - Ready bit for V sub CORE /sub voltage scaling output selection.
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - Voltage level ready for currently used VOS
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - voltage output scaling currently applied to V sub CORE /sub This field provides the last VOS value.
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VOSSR")
            .field("vosrdy", &self.vosrdy())
            .field("actvosrdy", &self.actvosrdy())
            .field("actvos", &self.actvos())
            .finish()
    }
}
/**PWR voltage scaling status register

You can [`read`](crate::Reg::read) this register and get [`vossr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#PWR:VOSSR)*/
pub struct VOSSRrs;
impl crate::RegisterSpec for VOSSRrs {
    type Ux = u32;
}
///`read()` method returns [`vossr::R`](R) reader structure
impl crate::Readable for VOSSRrs {}
///`reset()` method sets VOSSR to value 0x08
impl crate::Resettable for VOSSRrs {
    const RESET_VALUE: u32 = 0x08;
}
