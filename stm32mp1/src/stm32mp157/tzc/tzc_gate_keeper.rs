#[doc = "Register `TZC_GATE_KEEPER` reader"]
pub type R = crate::R<TZC_GATE_KEEPERrs>;
#[doc = "Register `TZC_GATE_KEEPER` writer"]
pub type W = crate::W<TZC_GATE_KEEPERrs>;
#[doc = "Field `OPENREQ` reader - OPENREQ"]
pub type OPENREQ_R = crate::FieldReader;
#[doc = "Field `OPENREQ` writer - OPENREQ"]
pub type OPENREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPENSTAT` reader - OPENSTAT"]
pub type OPENSTAT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - OPENREQ"]
    #[inline(always)]
    pub fn openreq(&self) -> OPENREQ_R {
        OPENREQ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - OPENSTAT"]
    #[inline(always)]
    pub fn openstat(&self) -> OPENSTAT_R {
        OPENSTAT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OPENREQ"]
    #[inline(always)]
    #[must_use]
    pub fn openreq(&mut self) -> OPENREQ_W<TZC_GATE_KEEPERrs> {
        OPENREQ_W::new(self, 0)
    }
}
#[doc = "Provides control and status for the gate keeper in each filter unit implemented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_gate_keeper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_gate_keeper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_GATE_KEEPERrs;
impl crate::RegisterSpec for TZC_GATE_KEEPERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_gate_keeper::R`](R) reader structure"]
impl crate::Readable for TZC_GATE_KEEPERrs {}
#[doc = "`write(|w| ..)` method takes [`tzc_gate_keeper::W`](W) writer structure"]
impl crate::Writable for TZC_GATE_KEEPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_GATE_KEEPER to value 0"]
impl crate::Resettable for TZC_GATE_KEEPERrs {
    const RESET_VALUE: u32 = 0;
}
