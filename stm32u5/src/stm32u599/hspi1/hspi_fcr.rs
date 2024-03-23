#[doc = "Register `HSPI_FCR` writer"]
pub type W = crate::W<HSPI_FCRrs>;
#[doc = "Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the HSPI_SR register."]
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the HSPI_SR register."]
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the HSPI_SR register."]
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the HSPI_SR register."]
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the HSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<HSPI_FCRrs> {
        CTEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the HSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<HSPI_FCRrs> {
        CTCF_W::new(self, 1)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the HSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<HSPI_FCRrs> {
        CSMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the HSPI_SR register."]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<HSPI_FCRrs> {
        CTOF_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_FCRrs;
impl crate::RegisterSpec for HSPI_FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hspi_fcr::W`](W) writer structure"]
impl crate::Writable for HSPI_FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_FCR to value 0"]
impl crate::Resettable for HSPI_FCRrs {
    const RESET_VALUE: u32 = 0;
}
