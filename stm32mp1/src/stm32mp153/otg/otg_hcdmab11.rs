#[doc = "Register `OTG_HCDMAB11` reader"]
pub type R = crate::R<OTG_HCDMAB11rs>;
#[doc = "Field `HCDMAB` reader - HCDMAB"]
pub type HCDMAB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HCDMAB"]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
#[doc = "OTG host channel-n DMA address buffer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_hcdmab11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_HCDMAB11rs;
impl crate::RegisterSpec for OTG_HCDMAB11rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hcdmab11::R`](R) reader structure"]
impl crate::Readable for OTG_HCDMAB11rs {}
#[doc = "`reset()` method sets OTG_HCDMAB11 to value 0"]
impl crate::Resettable for OTG_HCDMAB11rs {
    const RESET_VALUE: u32 = 0;
}
