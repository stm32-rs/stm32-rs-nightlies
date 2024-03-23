#[doc = "Register `PCROP2AER` reader"]
pub type R = crate::R<PCROP2AERrs>;
#[doc = "Register `PCROP2AER` writer"]
pub type W = crate::W<PCROP2AERrs>;
#[doc = "Field `PCROP2A_END` reader - PCROP2A area end offset, bank2"]
pub type PCROP2A_END_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP2A_END` writer - PCROP2A area end offset, bank2"]
pub type PCROP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - PCROP2A area end offset, bank2"]
    #[inline(always)]
    pub fn pcrop2a_end(&self) -> PCROP2A_END_R {
        PCROP2A_END_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - PCROP2A area end offset, bank2"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2a_end(&mut self) -> PCROP2A_END_W<PCROP2AERrs> {
        PCROP2A_END_W::new(self, 0)
    }
}
#[doc = "Flash PCROP2 area A end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2aer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2aer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2AERrs;
impl crate::RegisterSpec for PCROP2AERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2aer::R`](R) reader structure"]
impl crate::Readable for PCROP2AERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2aer::W`](W) writer structure"]
impl crate::Writable for PCROP2AERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2AER to value 0"]
impl crate::Resettable for PCROP2AERrs {
    const RESET_VALUE: u32 = 0;
}
