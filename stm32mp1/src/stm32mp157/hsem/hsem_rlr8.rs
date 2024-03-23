#[doc = "Register `HSEM_RLR8` reader"]
pub type R = crate::R<HSEM_RLR8rs>;
#[doc = "Field `PROCID` reader - PROCID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `COREID` reader - COREID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - PROCID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Accesses the same physical bits as HSEM_Rx. The HSEM_RLRx shall be used to perform a 1-step Read lock. Only Read accesses with authorized AHB bus master IDs are granted. Read accesses with unauthorized AHB bus master IDs are discarded.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_rlr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_RLR8rs;
impl crate::RegisterSpec for HSEM_RLR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_rlr8::R`](R) reader structure"]
impl crate::Readable for HSEM_RLR8rs {}
#[doc = "`reset()` method sets HSEM_RLR8 to value 0"]
impl crate::Resettable for HSEM_RLR8rs {
    const RESET_VALUE: u32 = 0;
}
