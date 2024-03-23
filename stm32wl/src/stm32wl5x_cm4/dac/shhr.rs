#[doc = "Register `SHHR` reader"]
pub type R = crate::R<SHHRrs>;
#[doc = "Register `SHHR` writer"]
pub type W = crate::W<SHHRrs>;
#[doc = "Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
pub type THOLD1_R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
pub type THOLD1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in Sample and Hold mode)"]
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> THOLD1_W<SHHRrs> {
        THOLD1_W::new(self, 0)
    }
}
#[doc = "Sample and Hold hold time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHHRrs;
impl crate::RegisterSpec for SHHRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shhr::R`](R) reader structure"]
impl crate::Readable for SHHRrs {}
#[doc = "`write(|w| ..)` method takes [`shhr::W`](W) writer structure"]
impl crate::Writable for SHHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHHR to value 0x0001_0001"]
impl crate::Resettable for SHHRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
