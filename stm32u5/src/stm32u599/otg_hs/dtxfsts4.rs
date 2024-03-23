#[doc = "Register `DTXFSTS4` reader"]
pub type R = crate::R<DTXFSTS4rs>;
#[doc = "Field `INEPTFSAV` reader - INEPTFSAV"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS4rs;
impl crate::RegisterSpec for DTXFSTS4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts4::R`](R) reader structure"]
impl crate::Readable for DTXFSTS4rs {}
#[doc = "`reset()` method sets DTXFSTS4 to value 0x0200"]
impl crate::Resettable for DTXFSTS4rs {
    const RESET_VALUE: u32 = 0x0200;
}
