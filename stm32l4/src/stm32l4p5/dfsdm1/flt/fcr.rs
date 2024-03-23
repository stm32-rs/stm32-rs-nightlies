#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCRrs>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCRrs>;
#[doc = "Field `IOSR` reader - Integrator oversampling ratio (averaging length)"]
pub type IOSR_R = crate::FieldReader;
#[doc = "Field `IOSR` writer - Integrator oversampling ratio (averaging length)"]
pub type IOSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_R = crate::FieldReader<u16>;
#[doc = "Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)"]
pub type FOSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
#[doc = "Sinc filter order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORD {
    #[doc = "0: FastSinc filter type"]
    FastSinc = 0,
    #[doc = "1: Sinc1 filter type"]
    Sinc1 = 1,
    #[doc = "2: Sinc2 filter type"]
    Sinc2 = 2,
    #[doc = "3: Sinc3 filter type"]
    Sinc3 = 3,
    #[doc = "4: Sinc4 filter type"]
    Sinc4 = 4,
    #[doc = "5: Sinc5 filter type"]
    Sinc5 = 5,
}
impl From<FORD> for u8 {
    #[inline(always)]
    fn from(variant: FORD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORD {
    type Ux = u8;
}
#[doc = "Field `FORD` reader - Sinc filter order"]
pub type FORD_R = crate::FieldReader<FORD>;
impl FORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FORD> {
        match self.bits {
            0 => Some(FORD::FastSinc),
            1 => Some(FORD::Sinc1),
            2 => Some(FORD::Sinc2),
            3 => Some(FORD::Sinc3),
            4 => Some(FORD::Sinc4),
            5 => Some(FORD::Sinc5),
            _ => None,
        }
    }
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn is_fast_sinc(&self) -> bool {
        *self == FORD::FastSinc
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn is_sinc1(&self) -> bool {
        *self == FORD::Sinc1
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn is_sinc2(&self) -> bool {
        *self == FORD::Sinc2
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn is_sinc3(&self) -> bool {
        *self == FORD::Sinc3
    }
    #[doc = "Sinc4 filter type"]
    #[inline(always)]
    pub fn is_sinc4(&self) -> bool {
        *self == FORD::Sinc4
    }
    #[doc = "Sinc5 filter type"]
    #[inline(always)]
    pub fn is_sinc5(&self) -> bool {
        *self == FORD::Sinc5
    }
}
#[doc = "Field `FORD` writer - Sinc filter order"]
pub type FORD_W<'a, REG> = crate::FieldWriter<'a, REG, 3, FORD>;
impl<'a, REG> FORD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn fast_sinc(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::FastSinc)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn sinc1(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc1)
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn sinc2(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc2)
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn sinc3(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc3)
    }
    #[doc = "Sinc4 filter type"]
    #[inline(always)]
    pub fn sinc4(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc4)
    }
    #[doc = "Sinc5 filter type"]
    #[inline(always)]
    pub fn sinc5(self) -> &'a mut crate::W<REG> {
        self.variant(FORD::Sinc5)
    }
}
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
    pub fn iosr(&mut self) -> IOSR_W<FCRrs> {
        IOSR_W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<FCRrs> {
        FOSR_W::new(self, 16)
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<FCRrs> {
        FORD_W::new(self, 29)
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCRrs;
impl crate::RegisterSpec for FCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCRrs {}
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
