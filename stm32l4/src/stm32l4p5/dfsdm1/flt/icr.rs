#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Clear the injected conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRJOVRFW {
    #[doc = "1: Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register"]
    Clear = 1,
}
impl From<CLRJOVRFW> for bool {
    #[inline(always)]
    fn from(variant: CLRJOVRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader<CLRJOVRFW>;
impl CLRJOVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRJOVRFW> {
        match self.bits {
            true => Some(CLRJOVRFW::Clear),
            _ => None,
        }
    }
    #[doc = "Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRJOVRFW::Clear
    }
}
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG, CLRJOVRFW>;
impl<'a, REG> CLRJOVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing ‘1’ clears the JOVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRJOVRFW::Clear)
    }
}
#[doc = "Clear the regular conversion overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRROVRFW {
    #[doc = "1: Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register"]
    Clear = 1,
}
impl From<CLRROVRFW> for bool {
    #[inline(always)]
    fn from(variant: CLRROVRFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader<CLRROVRFW>;
impl CLRROVRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLRROVRFW> {
        match self.bits {
            true => Some(CLRROVRFW::Clear),
            _ => None,
        }
    }
    #[doc = "Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLRROVRFW::Clear
    }
}
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG, CLRROVRFW>;
impl<'a, REG> CLRROVRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing ‘1’ clears the ROVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLRROVRFW::Clear)
    }
}
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag"]
pub type CLRCKABF_R = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag"]
pub type CLRCKABF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag"]
pub type CLRSCDF_R = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag"]
pub type CLRSCDF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
