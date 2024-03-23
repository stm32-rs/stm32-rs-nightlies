#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFR {
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    Finished = 0,
    #[doc = "1: The EWI Interrupt Service Routine has been triggered"]
    Pending = 1,
}
impl From<EWIFR> for bool {
    #[inline(always)]
    fn from(variant: EWIFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` reader - Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled."]
pub type EWIF_R = crate::BitReader<EWIFR>;
impl EWIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EWIFR {
        match self.bits {
            false => EWIFR::Finished,
            true => EWIFR::Pending,
        }
    }
    #[doc = "The EWI Interrupt Service Routine has been serviced"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == EWIFR::Finished
    }
    #[doc = "The EWI Interrupt Service Routine has been triggered"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == EWIFR::Pending
    }
}
#[doc = "Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIFW {
    #[doc = "0: The EWI Interrupt Service Routine has been serviced"]
    Finished = 0,
}
impl From<EWIFW> for bool {
    #[inline(always)]
    fn from(variant: EWIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EWIF` writer - Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled."]
pub type EWIF_W<'a, REG> = crate::BitWriter0C<'a, REG, EWIFW>;
impl<'a, REG> EWIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EWI Interrupt Service Routine has been serviced"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut crate::W<REG> {
        self.variant(EWIFW::Finished)
    }
}
impl R {
    #[doc = "Bit 0 - Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled."]
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Early wakeup interrupt flag This bit is set by hardware when the counter has reached the value 0x40. It must be cleared by software by writing 0. A write of 1 has no effect. This bit is also set if the interrupt is not enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<SRrs> {
        EWIF_W::new(self, 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
