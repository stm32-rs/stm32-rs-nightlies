#[doc = "Register `HAINT` reader"]
pub type R = crate::R<HAINTrs>;
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub type HAINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAINTrs;
impl crate::RegisterSpec for HAINTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haint::R`](R) reader structure"]
impl crate::Readable for HAINTrs {}
#[doc = "`reset()` method sets HAINT to value 0"]
impl crate::Resettable for HAINTrs {
    const RESET_VALUE: u32 = 0;
}
