#[doc = "Register `RISR` reader"]
pub type R = crate::R<RISRrs>;
#[doc = "Field `INRIS` reader - Input FIFO service raw interrupt status"]
pub type INRIS_R = crate::BitReader;
#[doc = "Field `OUTRIS` reader - Output FIFO service raw interrupt status"]
pub type OUTRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input FIFO service raw interrupt status"]
    #[inline(always)]
    pub fn inris(&self) -> INRIS_R {
        INRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output FIFO service raw interrupt status"]
    #[inline(always)]
    pub fn outris(&self) -> OUTRIS_R {
        OUTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "raw interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`risr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RISRrs;
impl crate::RegisterSpec for RISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`risr::R`](R) reader structure"]
impl crate::Readable for RISRrs {}
#[doc = "`reset()` method sets RISR to value 0x01"]
impl crate::Resettable for RISRrs {
    const RESET_VALUE: u32 = 0x01;
}
