#[doc = "Register `HWCFGR1` reader"]
pub type R = crate::R<HWCFGR1rs>;
#[doc = "Field `NBSEM` reader - Hardware Configuration number of semaphores"]
pub type NBSEM_R = crate::FieldReader;
#[doc = "Field `NBINT` reader - Hardware Configuration number of interrupts supported number of master IDs"]
pub type NBINT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Hardware Configuration number of semaphores"]
    #[inline(always)]
    pub fn nbsem(&self) -> NBSEM_R {
        NBSEM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Hardware Configuration number of interrupts supported number of master IDs"]
    #[inline(always)]
    pub fn nbint(&self) -> NBINT_R {
        NBINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Semaphore hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HWCFGR1rs;
impl crate::RegisterSpec for HWCFGR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfgr1::R`](R) reader structure"]
impl crate::Readable for HWCFGR1rs {}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0220"]
impl crate::Resettable for HWCFGR1rs {
    const RESET_VALUE: u32 = 0x0220;
}
