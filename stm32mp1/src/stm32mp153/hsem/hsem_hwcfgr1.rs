#[doc = "Register `HSEM_HWCFGR1` reader"]
pub type R = crate::R<HSEM_HWCFGR1rs>;
#[doc = "Field `NBSEM` reader - NBSEM"]
pub type NBSEM_R = crate::FieldReader;
#[doc = "Field `NBINT` reader - NBINT"]
pub type NBINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - NBSEM"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - NBINT"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "HSEM hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_HWCFGR1rs;
impl crate::RegisterSpec for HSEM_HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HSEM_HWCFGR1rs {}
#[doc = "`reset()` method sets HSEM_HWCFGR1 to value 0x0220"]
impl crate::Resettable for HSEM_HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0220;
}
