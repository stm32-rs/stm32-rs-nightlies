#[doc = "Register `FLT1ICR` reader"]
pub type R = crate::R<FLT1ICRrs>;
#[doc = "Register `FLT1ICR` writer"]
pub type W = crate::W<FLT1ICRrs>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag"]
pub type CLRCKABF_R = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag"]
pub type CLRCKABF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag"]
pub type CLRSCDF_R = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag"]
pub type CLRSCDF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<FLT1ICRrs> {
        CLRJOVRF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<FLT1ICRrs> {
        CLRROVRF_W::new(self, 3)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrckabf(&mut self) -> CLRCKABF_W<FLT1ICRrs> {
        CLRCKABF_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrscdf(&mut self) -> CLRSCDF_W<FLT1ICRrs> {
        CLRSCDF_W::new(self, 24)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt1icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flt1icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT1ICRrs;
impl crate::RegisterSpec for FLT1ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt1icr::R`](R) reader structure"]
impl crate::Readable for FLT1ICRrs {}
#[doc = "`write(|w| ..)` method takes [`flt1icr::W`](W) writer structure"]
impl crate::Writable for FLT1ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLT1ICR to value 0"]
impl crate::Resettable for FLT1ICRrs {
    const RESET_VALUE: u32 = 0;
}
