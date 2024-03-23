#[doc = "Register `AWSCDR` reader"]
pub type R = crate::R<AWSCDRrs>;
#[doc = "Register `AWSCDR` writer"]
pub type W = crate::W<AWSCDRrs>;
#[doc = "Field `SCDT` reader - short-circuit detector threshold for channel 0"]
pub type SCDT_R = crate::FieldReader;
#[doc = "Field `SCDT` writer - short-circuit detector threshold for channel 0"]
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel 0"]
pub type BKSCD_R = crate::FieldReader;
#[doc = "Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel 0"]
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel 0"]
pub type AWFOSR_R = crate::FieldReader;
#[doc = "Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel 0"]
pub type AWFOSR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AWFORD` reader - Analog watchdog Sinc filter order on channel 0"]
pub type AWFORD_R = crate::FieldReader;
#[doc = "Field `AWFORD` writer - Analog watchdog Sinc filter order on channel 0"]
pub type AWFORD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel 0"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel 0"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 0"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel 0"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - short-circuit detector threshold for channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<AWSCDRrs> {
        SCDT_W::new(self, 0)
    }
    #[doc = "Bits 12:15 - Break signal assignment for short-circuit detector on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<AWSCDRrs> {
        BKSCD_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<AWSCDRrs> {
        AWFOSR_W::new(self, 16)
    }
    #[doc = "Bits 22:23 - Analog watchdog Sinc filter order on channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<AWSCDRrs> {
        AWFORD_W::new(self, 22)
    }
}
#[doc = "DFSDM analog watchdog and short-circuit detector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awscdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awscdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
