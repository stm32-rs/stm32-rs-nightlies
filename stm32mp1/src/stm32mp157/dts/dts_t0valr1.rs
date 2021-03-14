#[doc = "Reader of register DTS_T0VALR1"]
pub type R = crate::R<u32, super::DTS_T0VALR1>;
#[doc = "Reader of field `TS1_FMT0`"]
pub type TS1_FMT0_R = crate::R<u16, u16>;
#[doc = "Reader of field `TS1_T0`"]
pub type TS1_T0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - TS1_FMT0"]
    #[inline(always)]
    pub fn ts1_fmt0(&self) -> TS1_FMT0_R {
        TS1_FMT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - TS1_T0"]
    #[inline(always)]
    pub fn ts1_t0(&self) -> TS1_T0_R {
        TS1_T0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
