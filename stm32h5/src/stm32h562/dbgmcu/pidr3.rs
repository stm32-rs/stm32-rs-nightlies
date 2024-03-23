#[doc = "Register `PIDR3` reader"]
pub type R = crate::R<PIDR3rs>;
#[doc = "Field `CMOD` reader - customer modified"]
pub type CMOD_R = crate::FieldReader;
#[doc = "Field `REVAND` reader - metal fix version"]
pub type REVAND_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - customer modified"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - metal fix version"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DBGMCU CoreSight peripheral identity register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIDR3rs;
impl crate::RegisterSpec for PIDR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr3::R`](R) reader structure"]
impl crate::Readable for PIDR3rs {}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3rs {
    const RESET_VALUE: u32 = 0;
}
