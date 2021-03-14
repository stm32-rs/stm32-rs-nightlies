#[doc = "Reader of register GPIOI_HWCFGR7"]
pub type R = crate::R<u32, super::GPIOI_HWCFGR7>;
#[doc = "Reader of field `AF_PRIO0`"]
pub type AF_PRIO0_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO1`"]
pub type AF_PRIO1_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO2`"]
pub type AF_PRIO2_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO3`"]
pub type AF_PRIO3_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO4`"]
pub type AF_PRIO4_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO5`"]
pub type AF_PRIO5_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO6`"]
pub type AF_PRIO6_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO7`"]
pub type AF_PRIO7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - AF_PRIO0"]
    #[inline(always)]
    pub fn af_prio0(&self) -> AF_PRIO0_R {
        AF_PRIO0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO1"]
    #[inline(always)]
    pub fn af_prio1(&self) -> AF_PRIO1_R {
        AF_PRIO1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO2"]
    #[inline(always)]
    pub fn af_prio2(&self) -> AF_PRIO2_R {
        AF_PRIO2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO3"]
    #[inline(always)]
    pub fn af_prio3(&self) -> AF_PRIO3_R {
        AF_PRIO3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO4"]
    #[inline(always)]
    pub fn af_prio4(&self) -> AF_PRIO4_R {
        AF_PRIO4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO5"]
    #[inline(always)]
    pub fn af_prio5(&self) -> AF_PRIO5_R {
        AF_PRIO5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO6"]
    #[inline(always)]
    pub fn af_prio6(&self) -> AF_PRIO6_R {
        AF_PRIO6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO7"]
    #[inline(always)]
    pub fn af_prio7(&self) -> AF_PRIO7_R {
        AF_PRIO7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
