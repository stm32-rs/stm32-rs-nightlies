#[doc = "Reader of register DDRPHYC_DX0GSR1"]
pub type R = crate::R<u32, super::DDRPHYC_DX0GSR1>;
#[doc = "Reader of field `DFTERR`"]
pub type DFTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DQSDFT`"]
pub type DQSDFT_R = crate::R<u8, u8>;
#[doc = "Reader of field `RVERR`"]
pub type RVERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVIERR`"]
pub type RVIERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVPASS`"]
pub type RVPASS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - DQSDFT"]
    #[inline(always)]
    pub fn dqsdft(&self) -> DQSDFT_R {
        DQSDFT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 12 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RVIERR"]
    #[inline(always)]
    pub fn rvierr(&self) -> RVIERR_R {
        RVIERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - RVPASS"]
    #[inline(always)]
    pub fn rvpass(&self) -> RVPASS_R {
        RVPASS_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
