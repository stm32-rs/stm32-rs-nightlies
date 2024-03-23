#[doc = "Register `PCROP2SR` reader"]
pub type R = crate::R<PCROP2SRrs>;
#[doc = "Register `PCROP2SR` writer"]
pub type W = crate::W<PCROP2SRrs>;
#[doc = "Field `PCROP2_STRT` reader - Bank 2 PCROP area start offset"]
pub type PCROP2_STRT_R = crate::FieldReader<u32>;
#[doc = "Field `PCROP2_STRT` writer - Bank 2 PCROP area start offset"]
pub type PCROP2_STRT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    pub fn pcrop2_strt(&self) -> PCROP2_STRT_R {
        PCROP2_STRT_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Bank 2 PCROP area start offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2_strt(&mut self) -> PCROP2_STRT_W<PCROP2SRrs> {
        PCROP2_STRT_W::new(self, 0)
    }
}
#[doc = "Flash Bank 2 PCROP Start address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2SRrs;
impl crate::RegisterSpec for PCROP2SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2sr::R`](R) reader structure"]
impl crate::Readable for PCROP2SRrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2sr::W`](W) writer structure"]
impl crate::Writable for PCROP2SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2SR to value 0xffff_0000"]
impl crate::Resettable for PCROP2SRrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
