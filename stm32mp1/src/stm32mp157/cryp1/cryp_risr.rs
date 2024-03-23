#[doc = "Register `CRYP_RISR` reader"]
pub type R = crate::R<CRYP_RISRrs>;
#[doc = "Field `INRIS` reader - INRIS"]
pub type INRIS_R = crate::BitReader;
#[doc = "Field `OUTRIS` reader - OUTRIS"]
pub type OUTRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INRIS"]
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUTRIS"]
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "The CRYP_RISR register is the raw interrupt status register. It is a read-only register. When a read operation is performed, this register gives the current raw status of the corresponding interrupt, i.e. the interrupt information without taking CRYP_IMSCR mask into account. Write operations have no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_risr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_RISRrs;
impl crate::RegisterSpec for CRYP_RISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_risr::R`](R) reader structure"]
impl crate::Readable for CRYP_RISRrs {}
#[doc = "`reset()` method sets CRYP_RISR to value 0x01"]
impl crate::Resettable for CRYP_RISRrs {
    const RESET_VALUE: u32 = 0x01;
}
