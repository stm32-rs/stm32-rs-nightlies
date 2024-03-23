#[doc = "Register `HYSCR1` reader"]
pub type R = crate::R<HYSCR1rs>;
#[doc = "Register `HYSCR1` writer"]
pub type W = crate::W<HYSCR1rs>;
#[doc = "Field `PA` reader - Port A hysteresis control on/off"]
pub type PA_R = crate::FieldReader<u16>;
#[doc = "Field `PA` writer - Port A hysteresis control on/off"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PB` reader - Port B hysteresis control on/off"]
pub type PB_R = crate::FieldReader<u16>;
#[doc = "Field `PB` writer - Port B hysteresis control on/off"]
pub type PB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port A hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<HYSCR1rs> {
        PA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Port B hysteresis control on/off"]
    #[inline(always)]
    #[must_use]
    pub fn pb(&mut self) -> PB_W<HYSCR1rs> {
        PB_W::new(self, 16)
    }
}
#[doc = "RI hysteresis control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hyscr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hyscr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HYSCR1rs;
impl crate::RegisterSpec for HYSCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hyscr1::R`](R) reader structure"]
impl crate::Readable for HYSCR1rs {}
#[doc = "`write(|w| ..)` method takes [`hyscr1::W`](W) writer structure"]
impl crate::Writable for HYSCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HYSCR1 to value 0"]
impl crate::Resettable for HYSCR1rs {
    const RESET_VALUE: u32 = 0;
}
