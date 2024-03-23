#[doc = "Register `LPTIM_CNT` reader"]
pub type R = crate::R<LPTIM_CNTrs>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lptim_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPTIM_CNTrs;
impl crate::RegisterSpec for LPTIM_CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lptim_cnt::R`](R) reader structure"]
impl crate::Readable for LPTIM_CNTrs {}
#[doc = "`reset()` method sets LPTIM_CNT to value 0"]
impl crate::Resettable for LPTIM_CNTrs {
    const RESET_VALUE: u32 = 0;
}
