#[doc = "Reader of register HWCFGR1"]
pub type R = crate::R<u32, super::HWCFGR1>;
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG2`"]
pub type CFG2_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG3`"]
pub type CFG3_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG4`"]
pub type CFG4_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG5`"]
pub type CFG5_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG6`"]
pub type CFG6_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG7`"]
pub type CFG7_R = crate::R<u8, u8>;
#[doc = "Reader of field `CFG8`"]
pub type CFG8_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CFG3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - CFG4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CFG5"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CFG6"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CFG7"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CFG8"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
