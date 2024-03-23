#[doc = "Register `DR` reader"]
pub type R = crate::R<DRrs>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DRrs>;
#[doc = "Field `TS1_MFREQ` reader - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_R = crate::FieldReader<u16>;
#[doc = "Field `TS1_MFREQ` writer - Value of the counter output value for temperature sensor 1"]
pub type TS1_MFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    pub fn ts1_mfreq(&self) -> TS1_MFREQ_R {
        TS1_MFREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of the counter output value for temperature sensor 1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_mfreq(&mut self) -> TS1_MFREQ_W<DRrs> {
        TS1_MFREQ_W::new(self, 0)
    }
}
#[doc = "Temperature sensor data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRrs;
impl crate::RegisterSpec for DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DRrs {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DRrs {
    const RESET_VALUE: u32 = 0;
}
