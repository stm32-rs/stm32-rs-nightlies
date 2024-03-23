#[doc = "Register `DDRPHYC_DX1GSR1` reader"]
pub type R = crate::R<DDRPHYC_DX1GSR1rs>;
#[doc = "Field `DFTERR` reader - DFTERR"]
pub type DFTERR_R = crate::BitReader;
#[doc = "Field `DQSDFT` reader - DQSDFT"]
pub type DQSDFT_R = crate::FieldReader;
#[doc = "Field `RVERR` reader - RVERR"]
pub type RVERR_R = crate::BitReader;
#[doc = "Field `RVIERR` reader - RVIERR"]
pub type RVIERR_R = crate::BitReader;
#[doc = "Field `RVPASS` reader - RVPASS"]
pub type RVPASS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DFTERR"]
    #[inline(always)]
    pub fn dfterr(&self) -> DFTERR_R {
        DFTERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - DQSDFT"]
    #[inline(always)]
    pub fn dqsdft(&self) -> DQSDFT_R {
        DQSDFT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 12 - RVERR"]
    #[inline(always)]
    pub fn rverr(&self) -> RVERR_R {
        RVERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - RVIERR"]
    #[inline(always)]
    pub fn rvierr(&self) -> RVIERR_R {
        RVIERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - RVPASS"]
    #[inline(always)]
    pub fn rvpass(&self) -> RVPASS_R {
        RVPASS_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "DDRPHYC byte lane 1 GS register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx1gsr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX1GSR1rs;
impl crate::RegisterSpec for DDRPHYC_DX1GSR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dx1gsr1::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GSR1rs {}
#[doc = "`reset()` method sets DDRPHYC_DX1GSR1 to value 0"]
impl crate::Resettable for DDRPHYC_DX1GSR1rs {
    const RESET_VALUE: u32 = 0;
}
