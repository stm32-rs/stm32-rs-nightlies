#[doc = "Register `DAC_SWTRGR` writer"]
pub type W = crate::W<DAC_SWTRGRrs>;
#[doc = "Field `SWTRIG1` writer - SWTRIG1"]
pub type SWTRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` writer - SWTRIG2"]
pub type SWTRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SWTRIG1"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<DAC_SWTRGRrs> {
        SWTRIG1_W::new(self, 0)
    }
    #[doc = "Bit 1 - SWTRIG2"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<DAC_SWTRGRrs> {
        SWTRIG2_W::new(self, 1)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_SWTRGRrs;
impl crate::RegisterSpec for DAC_SWTRGRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dac_swtrgr::W`](W) writer structure"]
impl crate::Writable for DAC_SWTRGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_SWTRGR to value 0"]
impl crate::Resettable for DAC_SWTRGRrs {
    const RESET_VALUE: u32 = 0;
}
