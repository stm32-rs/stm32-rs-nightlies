#[doc = "Register `MISR` reader"]
pub type R = crate::R<MISRrs>;
#[doc = "Field `INMIS` reader - Input FIFO service masked interrupt status"]
pub type INMIS_R = crate::BitReader;
#[doc = "Field `OUTMIS` reader - Output FIFO service masked interrupt status"]
pub type OUTMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISRrs;
impl crate::RegisterSpec for MISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MISRrs {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISRrs {
    const RESET_VALUE: u32 = 0;
}
