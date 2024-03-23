#[doc = "Register `PCROP1SR` reader"]
pub type R = crate::R<PCROP1SRrs>;
#[doc = "Register `PCROP1SR` writer"]
pub type W = crate::W<PCROP1SRrs>;
#[doc = "Field `PCROP1_STRT` reader - Bank 1 PCROP area start offset"]
pub type PCROP1_STRT_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP1_STRT` writer - Bank 1 PCROP area start offset"]
pub type PCROP1_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop1_strt(&self) -> PCROP1_STRT_R {
        PCROP1_STRT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Bank 1 PCROP area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_strt(&mut self) -> PCROP1_STRT_W<PCROP1SRrs> {
        PCROP1_STRT_W::new(self, 0)
    }
}
#[doc = "Flash Bank 1 PCROP Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop1sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop1sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP1SRrs;
impl crate::RegisterSpec for PCROP1SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop1sr::R`](R) reader structure"]
impl crate::Readable for PCROP1SRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop1sr::W`](W) writer structure"]
impl crate::Writable for PCROP1SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP1SR to value 0xffff_0000"]
impl crate::Resettable for PCROP1SRrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
