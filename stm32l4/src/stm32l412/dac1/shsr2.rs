#[doc = "Register `SHSR2` reader"]
pub type R = crate::R<SHSR2rs>;
#[doc = "Register `SHSR2` writer"]
pub type W = crate::W<SHSR2rs>;
#[doc = "Field `TSAMPLE2` reader - DAC Channel 2 sample Time"]
pub type TSAMPLE2_R = crate::FieldReader<u16>;
#[doc = "Field `TSAMPLE2` writer - DAC Channel 2 sample Time"]
pub type TSAMPLE2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    pub fn tsample2(&self) -> TSAMPLE2_R {
        TSAMPLE2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 2 sample Time"]
    #[inline(always)]
    #[must_use]
    pub fn tsample2(&mut self) -> TSAMPLE2_W<SHSR2rs> {
        TSAMPLE2_W::new(self, 0)
    }
}
#[doc = "Sample and Hold sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHSR2rs;
impl crate::RegisterSpec for SHSR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shsr2::R`](R) reader structure"]
impl crate::Readable for SHSR2rs {}
#[doc = "`write(|w| ..)` method takes [`shsr2::W`](W) writer structure"]
impl crate::Writable for SHSR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHSR2 to value 0"]
impl crate::Resettable for SHSR2rs {
    const RESET_VALUE: u32 = 0;
}
