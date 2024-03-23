#[doc = "Register `PCROP2BER` reader"]
pub type R = crate::R<PCROP2BERrs>;
#[doc = "Register `PCROP2BER` writer"]
pub type W = crate::W<PCROP2BERrs>;
#[doc = "Field `PCROP2B_END` reader - PCROP2B area end offset, Bank 2"]
pub type PCROP2B_END_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP2B_END` writer - PCROP2B area end offset, Bank 2"]
pub type PCROP2B_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - PCROP2B area end offset, Bank 2"]
    #[inline(always)]
    pub fn pcrop2b_end(&self) -> PCROP2B_END_R {
        PCROP2B_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2B area end offset, Bank 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2b_end(&mut self) -> PCROP2B_END_W<PCROP2BERrs> {
        PCROP2B_END_W::new(self, 0)
    }
}
#[doc = "FLASH PCROP2 area B end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2ber::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2ber::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2BERrs;
impl crate::RegisterSpec for PCROP2BERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2ber::R`](R) reader structure"]
impl crate::Readable for PCROP2BERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2ber::W`](W) writer structure"]
impl crate::Writable for PCROP2BERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2BER to value 0"]
impl crate::Resettable for PCROP2BERrs {
    const RESET_VALUE: u32 = 0;
}
