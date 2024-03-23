#[doc = "Register `CLRFR` writer"]
pub type W = crate::W<CLRFRrs>;
#[doc = "Clear PKA End of Operation flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROCENDFC {
    #[doc = "1: Clear PROCENDF flag"]
    Clear = 1,
}
impl From<PROCENDFC> for bool {
    #[inline(always)]
    fn from(variant: PROCENDFC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROCENDFC` writer - Clear PKA End of Operation flag"]
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG, PROCENDFC>;
impl<'a, REG> PROCENDFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear PROCENDF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PROCENDFC::Clear)
    }
}
#[doc = "Clear PKA RAM error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMERRFC {
    #[doc = "1: Clear RAMERRF flag"]
    Clear = 1,
}
impl From<RAMERRFC> for bool {
    #[inline(always)]
    fn from(variant: RAMERRFC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAMERRFC` writer - Clear PKA RAM error flag"]
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG, RAMERRFC>;
impl<'a, REG> RAMERRFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear RAMERRF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RAMERRFC::Clear)
    }
}
#[doc = "Clear Address error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRERRFC {
    #[doc = "1: Clear ADDRERRF flag"]
    Clear = 1,
}
impl From<ADDRERRFC> for bool {
    #[inline(always)]
    fn from(variant: ADDRERRFC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRERRFC` writer - Clear Address error flag"]
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG, ADDRERRFC>;
impl<'a, REG> ADDRERRFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear ADDRERRF flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRERRFC::Clear)
    }
}
impl W {
    #[doc = "Bit 17 - Clear PKA End of Operation flag"]
    #[inline(always)]
    #[must_use]
    pub fn procendfc(&mut self) -> PROCENDFC_W<CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    #[doc = "Bit 19 - Clear PKA RAM error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear Address error flag"]
    #[inline(always)]
    #[must_use]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
}
#[doc = "clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clrfr::W`](W) writer structure"]
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFRrs {
    const RESET_VALUE: u32 = 0;
}
