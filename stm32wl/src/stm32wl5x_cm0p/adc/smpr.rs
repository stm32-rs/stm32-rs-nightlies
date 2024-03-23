#[doc = "Register `SMPR` reader"]
pub type R = crate::R<SMPRrs>;
#[doc = "Register `SMPR` writer"]
pub type W = crate::W<SMPRrs>;
#[doc = "Field `SMP1` reader - SMP1"]
pub type SMP1_R = crate::FieldReader;
#[doc = "Field `SMP1` writer - SMP1"]
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP2` reader - SMP2"]
pub type SMP2_R = crate::FieldReader;
#[doc = "Field `SMP2` writer - SMP2"]
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMPSEL` reader - SMPSEL"]
pub type SMPSEL_R = crate::FieldReader<u32>;
#[doc = "Field `SMPSEL` writer - SMPSEL"]
pub type SMPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new((self.bits >> 8) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPRrs> {
        SMP1_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPRrs> {
        SMP2_W::new(self, 4)
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn smpsel(&mut self) -> SMPSEL_W<SMPRrs> {
        SMPSEL_W::new(self, 8)
    }
}
#[doc = "ADC sampling time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPRrs;
impl crate::RegisterSpec for SMPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr::R`](R) reader structure"]
impl crate::Readable for SMPRrs {}
#[doc = "`write(|w| ..)` method takes [`smpr::W`](W) writer structure"]
impl crate::Writable for SMPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPRrs {
    const RESET_VALUE: u32 = 0;
}
