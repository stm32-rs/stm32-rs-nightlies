#[doc = "Register `AWSCDR` reader"]
pub type R = crate::R<AWSCDRrs>;
#[doc = "Register `AWSCDR` writer"]
pub type W = crate::W<AWSCDRrs>;
#[doc = "Field `SCDT` reader - SCDT"]
pub type SCDT_R = crate::FieldReader;
#[doc = "Field `SCDT` writer - SCDT"]
pub type SCDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `BKSCD` reader - BKSCD"]
pub type BKSCD_R = crate::FieldReader;
#[doc = "Field `BKSCD` writer - BKSCD"]
pub type BKSCD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `AWFOSR` reader - AWFOSR"]
pub type AWFOSR_R = crate::FieldReader;
#[doc = "Field `AWFOSR` writer - AWFOSR"]
pub type AWFOSR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "AWFORD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWFORD {
    #[doc = "0: FastSinc filter type"]
    FastSinc = 0,
    #[doc = "1: Sinc1 filter type"]
    Sinc1 = 1,
    #[doc = "2: Sinc2 filter type"]
    Sinc2 = 2,
    #[doc = "3: Sinc3 filter type"]
    Sinc3 = 3,
}
impl From<AWFORD> for u8 {
    #[inline(always)]
    fn from(variant: AWFORD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AWFORD {
    type Ux = u8;
}
#[doc = "Field `AWFORD` reader - AWFORD"]
pub type AWFORD_R = crate::FieldReader<AWFORD>;
impl AWFORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWFORD {
        match self.bits {
            0 => AWFORD::FastSinc,
            1 => AWFORD::Sinc1,
            2 => AWFORD::Sinc2,
            3 => AWFORD::Sinc3,
            _ => unreachable!(),
        }
    }
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn is_fast_sinc(&self) -> bool {
        *self == AWFORD::FastSinc
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn is_sinc1(&self) -> bool {
        *self == AWFORD::Sinc1
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn is_sinc2(&self) -> bool {
        *self == AWFORD::Sinc2
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn is_sinc3(&self) -> bool {
        *self == AWFORD::Sinc3
    }
}
#[doc = "Field `AWFORD` writer - AWFORD"]
pub type AWFORD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, AWFORD>;
impl<'a, REG> AWFORD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn fast_sinc(self) -> &'a mut crate::W<REG> {
        self.variant(AWFORD::FastSinc)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn sinc1(self) -> &'a mut crate::W<REG> {
        self.variant(AWFORD::Sinc1)
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn sinc2(self) -> &'a mut crate::W<REG> {
        self.variant(AWFORD::Sinc2)
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn sinc3(self) -> &'a mut crate::W<REG> {
        self.variant(AWFORD::Sinc3)
    }
}
impl R {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<AWSCDRrs> {
        SCDT_W::new(self, 0)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<AWSCDRrs> {
        BKSCD_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<AWSCDRrs> {
        AWFOSR_W::new(self, 16)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<AWSCDRrs> {
        AWFORD_W::new(self, 22)
    }
}
#[doc = "analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awscdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awscdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWSCDRrs;
impl crate::RegisterSpec for AWSCDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awscdr::R`](R) reader structure"]
impl crate::Readable for AWSCDRrs {}
#[doc = "`write(|w| ..)` method takes [`awscdr::W`](W) writer structure"]
impl crate::Writable for AWSCDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWSCDR to value 0"]
impl crate::Resettable for AWSCDRrs {
    const RESET_VALUE: u32 = 0;
}
