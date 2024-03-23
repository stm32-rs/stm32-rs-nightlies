#[doc = "Register `HTR3` reader"]
pub type R = crate::R<HTR3rs>;
#[doc = "Register `HTR3` writer"]
pub type W = crate::W<HTR3rs>;
#[doc = "Field `HTR3` reader - Analog watchdog 3 higher threshold"]
pub type HTR3_R = crate::FieldReader<u32>;
#[doc = "Field `HTR3` writer - Analog watchdog 3 higher threshold"]
pub type HTR3_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn htr3(&mut self) -> HTR3_W<HTR3rs> {
        HTR3_W::new(self, 0)
    }
}
#[doc = "ADC watchdog higher threshold register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTR3rs;
impl crate::RegisterSpec for HTR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr3::R`](R) reader structure"]
impl crate::Readable for HTR3rs {}
#[doc = "`write(|w| ..)` method takes [`htr3::W`](W) writer structure"]
impl crate::Writable for HTR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HTR3 to value 0x03ff_ffff"]
impl crate::Resettable for HTR3rs {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
