#[doc = "Register `DTXFSTS1` reader"]
pub type R = crate::R<DTXFSTS1rs>;
#[doc = "Field `INEPTFSAV` reader - INEPTFSAV"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS1rs;
impl crate::RegisterSpec for DTXFSTS1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts1::R`](R) reader structure"]
impl crate::Readable for DTXFSTS1rs {}
#[doc = "`reset()` method sets DTXFSTS1 to value 0x0200"]
impl crate::Resettable for DTXFSTS1rs {
    const RESET_VALUE: u32 = 0x0200;
}
