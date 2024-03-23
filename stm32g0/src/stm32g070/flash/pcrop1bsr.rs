#[doc = "Register `PCROP1BSR` reader"]
pub type R = crate::R<PCROP1BSRrs>;
#[doc = "Register `PCROP1BSR` writer"]
pub type W = crate::W<PCROP1BSRrs>;
#[doc = "Field `PCROP1B_STRT` reader - PCROP1B area start offset"]
pub type PCROP1B_STRT_R = crate::FieldReader;
#[doc = "Field `PCROP1B_STRT` writer - PCROP1B area start offset"]
pub type PCROP1B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PCROP1B area start offset"]
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PCROP1B area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1b_strt(&mut self) -> PCROP1B_STRT_W<PCROP1BSRrs> {
        PCROP1B_STRT_W::new(self, 0)
    }
}
#[doc = "Flash PCROP zone B Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1bsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1bsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1BSRrs;
impl crate::RegisterSpec for PCROP1BSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1bsr::R`](R) reader structure"]
impl crate::Readable for PCROP1BSRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1bsr::W`](W) writer structure"]
impl crate::Writable for PCROP1BSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1BSR to value 0xffff_ffff"]
impl crate::Resettable for PCROP1BSRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
