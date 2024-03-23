#[doc = "Register `RESPI1` reader"]
pub type R = crate::R<RESPI1rs>;
#[doc = "Field `CARDSTATUS1` reader - CARDSTATUS1"]
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respi1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPI1rs;
impl crate::RegisterSpec for RESPI1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respi1::R`](R) reader structure"]
impl crate::Readable for RESPI1rs {}
#[doc = "`reset()` method sets RESPI1 to value 0"]
impl crate::Resettable for RESPI1rs {
    const RESET_VALUE: u32 = 0;
}
