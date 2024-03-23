#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Clear transfer error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEF {
    #[doc = "1: Writing 1 clears the TEF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTEF> for bool {
    #[inline(always)]
    fn from(variant: CTEF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEF` writer - Clear transfer error flag"]
pub type CTEF_W<'a, REG> = crate::BitWriter<'a, REG, CTEF>;
impl<'a, REG> CTEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 clears the TEF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEF::Clear)
    }
}
#[doc = "Clear transfer complete flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCF {
    #[doc = "1: Writing 1 clears the TCF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTCF> for bool {
    #[inline(always)]
    fn from(variant: CTCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCF` writer - Clear transfer complete flag"]
pub type CTCF_W<'a, REG> = crate::BitWriter<'a, REG, CTCF>;
impl<'a, REG> CTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 clears the TCF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCF::Clear)
    }
}
#[doc = "Clear status match flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSMF {
    #[doc = "1: Writing 1 clears the SMF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CSMF> for bool {
    #[inline(always)]
    fn from(variant: CSMF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSMF` writer - Clear status match flag"]
pub type CSMF_W<'a, REG> = crate::BitWriter<'a, REG, CSMF>;
impl<'a, REG> CSMF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 clears the SMF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSMF::Clear)
    }
}
#[doc = "Clear timeout flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOF {
    #[doc = "1: Writing 1 clears the TOF flag in the OCTOSPI_SR register"]
    Clear = 1,
}
impl From<CTOF> for bool {
    #[inline(always)]
    fn from(variant: CTOF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOF` writer - Clear timeout flag"]
pub type CTOF_W<'a, REG> = crate::BitWriter<'a, REG, CTOF>;
impl<'a, REG> CTOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 clears the TOF flag in the OCTOSPI_SR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTOF::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear transfer error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctef(&mut self) -> CTEF_W<FCRrs> {
        CTEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctcf(&mut self) -> CTCF_W<FCRrs> {
        CTCF_W::new(self, 1)
    }
    #[doc = "Bit 3 - Clear status match flag"]
    #[inline(always)]
    #[must_use]
    pub fn csmf(&mut self) -> CSMF_W<FCRrs> {
        CSMF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctof(&mut self) -> CTOF_W<FCRrs> {
        CTOF_W::new(self, 4)
    }
}
#[doc = "flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCRrs {
    const RESET_VALUE: u32 = 0;
}
