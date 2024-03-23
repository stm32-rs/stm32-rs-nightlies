#[doc = "Register `DDRPHYC_DX3GSR0` reader"]
pub type R = crate::R<DDRPHYC_DX3GSR0rs>;
#[doc = "Field `DTDONE` reader - DTDONE"]
pub type DTDONE_R = crate::BitReader;
#[doc = "Field `DTERR` reader - DTERR"]
pub type DTERR_R = crate::BitReader;
#[doc = "Field `DTIERR` reader - DTIERR"]
pub type DTIERR_R = crate::BitReader;
#[doc = "Field `DTPASS` reader - DTPASS"]
pub type DTPASS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DTDONE"]
    #[inline(always)]
    pub fn dtdone(&self) -> DTDONE_R {
        DTDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DTERR_R {
        DTERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DTIERR"]
    #[inline(always)]
    pub fn dtierr(&self) -> DTIERR_R {
        DTIERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 13:15 - DTPASS"]
    #[inline(always)]
    pub fn dtpass(&self) -> DTPASS_R {
        DTPASS_R::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "DDRPHYC byte lane 3 GS register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx3gsr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX3GSR0rs;
impl crate::RegisterSpec for DDRPHYC_DX3GSR0rs {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ddrphyc_dx3gsr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR0rs {}
#[doc = "`reset()` method sets DDRPHYC_DX3GSR0 to value 0"]
impl crate::Resettable for DDRPHYC_DX3GSR0rs {
    const RESET_VALUE: u16 = 0;
}
