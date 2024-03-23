#[doc = "Register `GICD_SPISR1` reader"]
pub type R = crate::R<GICD_SPISR1rs>;
#[doc = "Field `SPISR1` reader - SPISR1"]
pub type SPISR1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SPISR1"]
    #[inline(always)]
    pub fn spisr1(&self) -> SPISR1_R {
        SPISR1_R::new(self.bits)
    }
}
#[doc = "For interrupts ID = SPI number+32, from SPI \\[x*32+31\\]
to SPI \\[x*32\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR1rs;
impl crate::RegisterSpec for GICD_SPISR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr1::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR1rs {}
#[doc = "`reset()` method sets GICD_SPISR1 to value 0"]
impl crate::Resettable for GICD_SPISR1rs {
    const RESET_VALUE: u32 = 0;
}
