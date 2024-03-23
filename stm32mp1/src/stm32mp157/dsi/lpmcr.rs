#[doc = "Register `LPMCR` reader"]
pub type R = crate::R<LPMCRrs>;
#[doc = "Register `LPMCR` writer"]
pub type W = crate::W<LPMCRrs>;
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VLPSIZE_R = crate::FieldReader;
#[doc = "Field `VLPSIZE` writer - VLPSIZE"]
pub type VLPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LPSIZE_R = crate::FieldReader;
#[doc = "Field `LPSIZE` writer - LPSIZE"]
pub type LPSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn vlpsize(&mut self) -> VLPSIZE_W<LPMCRrs> {
        VLPSIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn lpsize(&mut self) -> LPSIZE_W<LPMCRrs> {
        LPSIZE_W::new(self, 16)
    }
}
#[doc = "DSI Host low-power mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPMCRrs;
impl crate::RegisterSpec for LPMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmcr::R`](R) reader structure"]
impl crate::Readable for LPMCRrs {}
#[doc = "`write(|w| ..)` method takes [`lpmcr::W`](W) writer structure"]
impl crate::Writable for LPMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPMCR to value 0"]
impl crate::Resettable for LPMCRrs {
    const RESET_VALUE: u32 = 0;
}
