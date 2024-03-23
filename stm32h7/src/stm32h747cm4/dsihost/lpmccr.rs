#[doc = "Register `LPMCCR` reader"]
pub type R = crate::R<LPMCCRrs>;
#[doc = "Register `LPMCCR` writer"]
pub type W = crate::W<LPMCCRrs>;
#[doc = "Field `VLPSIZE` reader - VACT largest packet size"]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `VLPSIZE` writer - VACT largest packet size"]
pub type VLPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPSIZE` reader - Largest packet size"]
pub type LPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` writer - Largest packet size"]
pub type LPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VACT largest packet size"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Largest packet size"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VACT largest packet size"]
    #[inline(always)]
    #[must_use]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<LPMCCRrs> {
        VLPSIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Largest packet size"]
    #[inline(always)]
    #[must_use]
    pub fn lpsize(&mut self) -> LPSIZE_W<LPMCCRrs> {
        LPSIZE_W::new(self, 16)
    }
}
#[doc = "DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCCRrs;
impl crate::RegisterSpec for LPMCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmccr::R`](R) reader structure"]
impl crate::Readable for LPMCCRrs {}
#[doc = "`write(|w| ..)` method takes [`lpmccr::W`](W) writer structure"]
impl crate::Writable for LPMCCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPMCCR to value 0"]
impl crate::Resettable for LPMCCRrs {
    const RESET_VALUE: u32 = 0;
}
