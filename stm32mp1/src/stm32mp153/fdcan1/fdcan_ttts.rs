#[doc = "Register `FDCAN_TTTS` reader"]
pub type R = crate::R<FDCAN_TTTSrs>;
#[doc = "Register `FDCAN_TTTS` writer"]
pub type W = crate::W<FDCAN_TTTSrs>;
#[doc = "Field `SWTDEL` reader - SWTDEL"]
pub type SWTDEL_R = crate::FieldReader;
#[doc = "Field `SWTDEL` writer - SWTDEL"]
pub type SWTDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVTSEL` reader - EVTSEL"]
pub type EVTSEL_R = crate::FieldReader;
#[doc = "Field `EVTSEL` writer - EVTSEL"]
pub type EVTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - SWTDEL"]
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - EVTSEL"]
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SWTDEL"]
    #[inline(always)]
    #[must_use]
    pub fn swtdel(&mut self) -> SWTDEL_W<FDCAN_TTTSrs> {
        SWTDEL_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - EVTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn evtsel(&mut self) -> EVTSEL_W<FDCAN_TTTSrs> {
        EVTSEL_W::new(self, 4)
    }
}
#[doc = "The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_ttts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_ttts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TTTSrs;
impl crate::RegisterSpec for FDCAN_TTTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttts::R`](R) reader structure"]
impl crate::Readable for FDCAN_TTTSrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttts::W`](W) writer structure"]
impl crate::Writable for FDCAN_TTTSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_TTTS to value 0"]
impl crate::Resettable for FDCAN_TTTSrs {
    const RESET_VALUE: u32 = 0;
}
