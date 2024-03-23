#[doc = "Register `DR_10` reader"]
pub type R = crate::R<DR_10rs>;
#[doc = "Field `DRNL1` reader - Data value"]
pub type DRNL1_R = crate::FieldReader<u16>;
#[doc = "Field `DRNL2` reader - Data value"]
pub type DRNL2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data value"]
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data value"]
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr_10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_10rs;
impl crate::RegisterSpec for DR_10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr_10::R`](R) reader structure"]
impl crate::Readable for DR_10rs {}
#[doc = "`reset()` method sets DR_10 to value 0"]
impl crate::Resettable for DR_10rs {
    const RESET_VALUE: u32 = 0;
}
