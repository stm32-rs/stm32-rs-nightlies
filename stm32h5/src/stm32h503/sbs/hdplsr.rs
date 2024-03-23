#[doc = "Register `HDPLSR` reader"]
pub type R = crate::R<HDPLSRrs>;
#[doc = "temporal isolation level This bitfield returns the current temporal isolation level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HDPLR {
    #[doc = "81: Protection level to be used to execute and protect immutable Root of Trust (IROT) stage"]
    Hdpl1 = 81,
    #[doc = "111: Protection level to be used to execute the application"]
    Hdpl3 = 111,
    #[doc = "138: Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage"]
    Hdpl2 = 138,
    #[doc = "180: Protection level reserved for ST code and data"]
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
#[doc = "Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level."]
pub type HDPL_R = crate::FieldReader<HDPLR>;
impl HDPL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Protection level to be used to execute and protect immutable Root of Trust (IROT) stage"]
    #[inline(always)]
    pub fn is_hdpl1(&self) -> bool {
        *self == HDPLR::Hdpl1
    }
    #[doc = "Protection level to be used to execute the application"]
    #[inline(always)]
    pub fn is_hdpl3(&self) -> bool {
        *self == HDPLR::Hdpl3
    }
    #[doc = "Protection level to be used to execute and protect an updatable Root of Trust (UROT) stage"]
    #[inline(always)]
    pub fn is_hdpl2(&self) -> bool {
        *self == HDPLR::Hdpl2
    }
    #[doc = "Protection level reserved for ST code and data"]
    #[inline(always)]
    pub fn is_hdpl0(&self) -> bool {
        *self == HDPLR::Hdpl0
    }
}
impl R {
    #[doc = "Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level."]
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "SBS temporal isolation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdplsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDPLSRrs;
impl crate::RegisterSpec for HDPLSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdplsr::R`](R) reader structure"]
impl crate::Readable for HDPLSRrs {}
#[doc = "`reset()` method sets HDPLSR to value 0"]
impl crate::Resettable for HDPLSRrs {
    const RESET_VALUE: u32 = 0;
}
