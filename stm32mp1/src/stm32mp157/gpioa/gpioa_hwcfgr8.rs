#[doc = "Reader of register GPIOA_HWCFGR8"]
pub type R = crate::R<u32, super::GPIOA_HWCFGR8>;
#[doc = "Reader of field `AF_PRIO8`"]
pub type AF_PRIO8_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO9`"]
pub type AF_PRIO9_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO10`"]
pub type AF_PRIO10_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO11`"]
pub type AF_PRIO11_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO12`"]
pub type AF_PRIO12_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO13`"]
pub type AF_PRIO13_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO14`"]
pub type AF_PRIO14_R = crate::R<u8, u8>;
#[doc = "Reader of field `AF_PRIO15`"]
pub type AF_PRIO15_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - AF_PRIO8"]
    #[inline(always)]
    pub fn af_prio8(&self) -> AF_PRIO8_R {
        AF_PRIO8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_PRIO9"]
    #[inline(always)]
    pub fn af_prio9(&self) -> AF_PRIO9_R {
        AF_PRIO9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AF_PRIO10"]
    #[inline(always)]
    pub fn af_prio10(&self) -> AF_PRIO10_R {
        AF_PRIO10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AF_PRIO11"]
    #[inline(always)]
    pub fn af_prio11(&self) -> AF_PRIO11_R {
        AF_PRIO11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AF_PRIO12"]
    #[inline(always)]
    pub fn af_prio12(&self) -> AF_PRIO12_R {
        AF_PRIO12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AF_PRIO13"]
    #[inline(always)]
    pub fn af_prio13(&self) -> AF_PRIO13_R {
        AF_PRIO13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AF_PRIO14"]
    #[inline(always)]
    pub fn af_prio14(&self) -> AF_PRIO14_R {
        AF_PRIO14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AF_PRIO15"]
    #[inline(always)]
    pub fn af_prio15(&self) -> AF_PRIO15_R {
        AF_PRIO15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
