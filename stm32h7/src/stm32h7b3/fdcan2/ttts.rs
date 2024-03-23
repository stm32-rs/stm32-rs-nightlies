#[doc = "Register `TTTS` reader"]
pub type R = crate::R<TTTSrs>;
#[doc = "Register `TTTS` writer"]
pub type W = crate::W<TTTSrs>;
#[doc = "Field `SWTDEL` reader - Stop watch trigger input selection"]
pub type SWTDEL_R = crate::FieldReader;
#[doc = "Field `SWTDEL` writer - Stop watch trigger input selection"]
pub type SWTDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVTSEL` reader - Event trigger input selection"]
pub type EVTSEL_R = crate::FieldReader;
#[doc = "Field `EVTSEL` writer - Event trigger input selection"]
pub type EVTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn swtdel(&mut self) -> SWTDEL_W<TTTSrs> {
        SWTDEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    #[must_use]
    pub fn evtsel(&mut self) -> EVTSEL_W<TTTSrs> {
        EVTSEL_W::new(self, 4)
    }
}
#[doc = "FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTTSrs;
impl crate::RegisterSpec for TTTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttts::R`](R) reader structure"]
impl crate::Readable for TTTSrs {}
#[doc = "`write(|w| ..)` method takes [`ttts::W`](W) writer structure"]
impl crate::Writable for TTTSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TTTS to value 0"]
impl crate::Resettable for TTTSrs {
    const RESET_VALUE: u32 = 0;
}
