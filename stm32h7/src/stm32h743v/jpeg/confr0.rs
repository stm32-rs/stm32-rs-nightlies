#[doc = "Register `CONFR0` writer"]
pub type W = crate::W<CONFR0rs>;
#[doc = "Field `START` writer - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CONFR0rs> {
        START_W::new(self, 0)
    }
}
#[doc = "JPEG codec control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFR0rs;
impl crate::RegisterSpec for CONFR0rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`confr0::W`](W) writer structure"]
impl crate::Writable for CONFR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFR0 to value 0"]
impl crate::Resettable for CONFR0rs {
    const RESET_VALUE: u32 = 0;
}
