#[doc = "Reader of register CLIDR"]
pub type R = crate::R<u32, super::CLIDR>;
#[doc = "Reader of field `CL1`"]
pub type CL1_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL2`"]
pub type CL2_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL3`"]
pub type CL3_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL4`"]
pub type CL4_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL5`"]
pub type CL5_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL6`"]
pub type CL6_R = crate::R<u8, u8>;
#[doc = "Reader of field `CL7`"]
pub type CL7_R = crate::R<u8, u8>;
#[doc = "Reader of field `LoUIS`"]
pub type LOUIS_R = crate::R<u8, u8>;
#[doc = "Reader of field `LoC`"]
pub type LOC_R = crate::R<u8, u8>;
#[doc = "Reader of field `LoU`"]
pub type LOU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - CL1"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - CL2"]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - CL3"]
    #[inline(always)]
    pub fn cl3(&self) -> CL3_R {
        CL3_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - CL4"]
    #[inline(always)]
    pub fn cl4(&self) -> CL4_R {
        CL4_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - CL5"]
    #[inline(always)]
    pub fn cl5(&self) -> CL5_R {
        CL5_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - CL6"]
    #[inline(always)]
    pub fn cl6(&self) -> CL6_R {
        CL6_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - CL7"]
    #[inline(always)]
    pub fn cl7(&self) -> CL7_R {
        CL7_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - LoUIS"]
    #[inline(always)]
    pub fn lo_uis(&self) -> LOUIS_R {
        LOUIS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - LoC"]
    #[inline(always)]
    pub fn lo_c(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - LoU"]
    #[inline(always)]
    pub fn lo_u(&self) -> LOU_R {
        LOU_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
