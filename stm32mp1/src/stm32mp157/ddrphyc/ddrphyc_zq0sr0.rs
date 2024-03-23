#[doc = "Register `DDRPHYC_ZQ0SR0` reader"]
pub type R = crate::R<DDRPHYC_ZQ0SR0rs>;
#[doc = "Field `ZCTRL` reader - ZCTRL"]
pub type ZCTRL_R = crate::FieldReader<u32>;
#[doc = "Field `ZERR` reader - ZERR"]
pub type ZERR_R = crate::BitReader;
#[doc = "Field `ZDONE` reader - ZDONE"]
pub type ZDONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - ZCTRL"]
    #[inline(always)]
    pub fn zctrl(&self) -> ZCTRL_R {
        ZCTRL_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30 - ZERR"]
    #[inline(always)]
    pub fn zerr(&self) -> ZERR_R {
        ZERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ZDONE"]
    #[inline(always)]
    pub fn zdone(&self) -> ZDONE_R {
        ZDONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DDRPHYC ZQ0S register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_zq0sr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_ZQ0SR0rs;
impl crate::RegisterSpec for DDRPHYC_ZQ0SR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_zq0sr0::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR0rs {}
#[doc = "`reset()` method sets DDRPHYC_ZQ0SR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0SR0rs {
    const RESET_VALUE: u32 = 0x014a;
}
