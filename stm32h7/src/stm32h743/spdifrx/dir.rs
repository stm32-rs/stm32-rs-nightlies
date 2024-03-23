#[doc = "Register `DIR` reader"]
pub type R = crate::R<DIRrs>;
#[doc = "Field `THI` reader - Threshold HIGH"]
pub type THI_R = crate::FieldReader<u16>;
#[doc = "Field `TLO` reader - Threshold LOW"]
pub type TLO_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Threshold HIGH"]
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Threshold LOW"]
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Debug Information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dir::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIRrs;
impl crate::RegisterSpec for DIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DIRrs {}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIRrs {
    const RESET_VALUE: u32 = 0;
}
