#[doc = "Register `CALFACT` reader"]
pub type R = crate::R<CALFACTrs>;
#[doc = "Register `CALFACT` writer"]
pub type W = crate::W<CALFACTrs>;
#[doc = "Field `CALFACT` reader - ADC calibration factor in single-ended mode"]
pub type CALFACT_R = crate::FieldReader;
#[doc = "Field `CALFACT` writer - ADC calibration factor in single-ended mode"]
pub type CALFACT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CALFACT_W<CALFACTrs> {
        CALFACT_W::new(self, 0)
    }
}
#[doc = "ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALFACTrs;
impl crate::RegisterSpec for CALFACTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact::R`](R) reader structure"]
impl crate::Readable for CALFACTrs {}
#[doc = "`write(|w| ..)` method takes [`calfact::W`](W) writer structure"]
impl crate::Writable for CALFACTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CALFACTrs {
    const RESET_VALUE: u32 = 0;
}
