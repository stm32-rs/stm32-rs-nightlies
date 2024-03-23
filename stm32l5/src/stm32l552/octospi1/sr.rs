#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `CTEF` writer - Clear transfer error flag"]
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMF` writer - Clear status match flag"]
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTOF` writer - Clear timeout flag"]
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<SRrs> {
        CTEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<SRrs> {
        CTCF_W::new(self, 1)
    }
    #[doc = "Bit 3 - Clear status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<SRrs> {
        CSMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<SRrs> {
        CTOF_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
