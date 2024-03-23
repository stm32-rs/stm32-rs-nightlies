#[doc = "Register `MACATSSR` reader"]
pub type R = crate::R<MACATSSRrs>;
#[doc = "Field `AUXTSHI` reader - Auxiliary Timestamp"]
pub type AUXTSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Auxiliary Timestamp"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AUXTSHI_R {
        AUXTSHI_R::new(self.bits)
    }
}
#[doc = "Auxiliary timestamp seconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macatssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACATSSRrs;
impl crate::RegisterSpec for MACATSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatssr::R`](R) reader structure"]
impl crate::Readable for MACATSSRrs {}
#[doc = "`reset()` method sets MACATSSR to value 0"]
impl crate::Resettable for MACATSSRrs {
    const RESET_VALUE: u32 = 0;
}
