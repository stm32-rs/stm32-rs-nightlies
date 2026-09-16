///Register `HDPLSR` reader
pub type R = crate::R<HDPLSRrs>;
/**temporal isolation level This bitfield returns the current temporal isolation level.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HDPLR {
    ///81: Protection level to be used to execute and protect immutable Root of Trust (IROT) stage
    Hdpl1 = 81,
    ///111: Protection level to be used to execute the application
    Hdpl3 = 111,
    ///138: Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage
    Hdpl2 = 138,
    ///180: Protection level reserved for ST code and data
    Hdpl0 = 180,
}
impl From<HDPLR> for u8 {
    #[inline(always)]
    fn from(variant: HDPLR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HDPLR {
    type Ux = u8;
}
impl crate::IsEnum for HDPLR {}
///Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level.
pub type HDPL_R = crate::FieldReader<HDPLR>;
impl HDPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HDPLR> {
        match self.bits {
            81 => Some(HDPLR::Hdpl1),
            111 => Some(HDPLR::Hdpl3),
            138 => Some(HDPLR::Hdpl2),
            180 => Some(HDPLR::Hdpl0),
            _ => None,
        }
    }
    ///Protection level to be used to execute and protect immutable Root of Trust (IROT) stage
    #[inline(always)]
    pub fn is_hdpl1(&self) -> bool {
        *self == HDPLR::Hdpl1
    }
    ///Protection level to be used to execute the application
    #[inline(always)]
    pub fn is_hdpl3(&self) -> bool {
        *self == HDPLR::Hdpl3
    }
    ///Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage
    #[inline(always)]
    pub fn is_hdpl2(&self) -> bool {
        *self == HDPLR::Hdpl2
    }
    ///Protection level reserved for ST code and data
    #[inline(always)]
    pub fn is_hdpl0(&self) -> bool {
        *self == HDPLR::Hdpl0
    }
}
impl R {
    ///Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level.
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDPLSR")
            .field("hdpl", &self.hdpl())
            .finish()
    }
}
/**SBS temporal isolation status register

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:HDPLSR)*/
pub struct HDPLSRrs;
impl crate::RegisterSpec for HDPLSRrs {
    type Ux = u32;
}
///`read()` method returns [`hdplsr::R`](R) reader structure
impl crate::Readable for HDPLSRrs {}
///`reset()` method sets HDPLSR to value 0
impl crate::Resettable for HDPLSRrs {}
