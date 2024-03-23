#[doc = "Register `PCROP2BSR` reader"]
pub type R = crate::R<PCROP2BSRrs>;
#[doc = "Register `PCROP2BSR` writer"]
pub type W = crate::W<PCROP2BSRrs>;
#[doc = "Field `PCROP2B_STRT` reader - PCROP2B area start offset, Bank 2"]
pub type PCROP2B_STRT_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP2B_STRT` writer - PCROP2B area start offset, Bank 2"]
pub type PCROP2B_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - PCROP2B area start offset, Bank 2"]
    #[inline(always)]
    pub fn pcrop2b_strt(&self) -> PCROP2B_STRT_R {
        PCROP2B_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2B area start offset, Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2b_strt(&mut self) -> PCROP2B_STRT_W<PCROP2BSRrs> {
        PCROP2B_STRT_W::new(self, 0)
    }
}
#[doc = "FLASH PCROP2 area B start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2bsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2bsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2BSRrs;
impl crate::RegisterSpec for PCROP2BSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2bsr::R`](R) reader structure"]
impl crate::Readable for PCROP2BSRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2bsr::W`](W) writer structure"]
impl crate::Writable for PCROP2BSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2BSR to value 0xffff_ffff"]
impl crate::Resettable for PCROP2BSRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
