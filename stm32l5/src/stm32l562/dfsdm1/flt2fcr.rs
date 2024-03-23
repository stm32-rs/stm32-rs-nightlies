#[doc = "Register `FLT2FCR` reader"]
pub type R = crate::R<FLT2FCRrs>;
#[doc = "Register `FLT2FCR` writer"]
pub type W = crate::W<FLT2FCRrs>;
#[doc = "Field `IOSR` reader - Integrator oversampling ratio (averaging length)"]
pub type IOSR_R = crate::FieldReader;
#[doc = "Field `IOSR` writer - Integrator oversampling ratio (averaging length)"]
pub type IOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_R = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `FORD` reader - Sinc filter order"]
pub type FORD_R = crate::FieldReader;
#[doc = "Field `FORD` writer - Sinc filter order"]
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<FLT2FCRrs> {
        IOSR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<FLT2FCRrs> {
        FOSR_W::new(self, 16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<FLT2FCRrs> {
        FORD_W::new(self, 29)
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flt2fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flt2fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLT2FCRrs;
impl crate::RegisterSpec for FLT2FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flt2fcr::R`](R) reader structure"]
impl crate::Readable for FLT2FCRrs {}
#[doc = "`write(|w| ..)` method takes [`flt2fcr::W`](W) writer structure"]
impl crate::Writable for FLT2FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLT2FCR to value 0"]
impl crate::Resettable for FLT2FCRrs {
    const RESET_VALUE: u32 = 0;
}
