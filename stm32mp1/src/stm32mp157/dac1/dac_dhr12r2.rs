#[doc = "Register `DAC_DHR12R2` reader"]
pub type R = crate::R<DAC_DHR12R2rs>;
#[doc = "Register `DAC_DHR12R2` writer"]
pub type W = crate::W<DAC_DHR12R2rs>;
#[doc = "Field `DACC2DHR` reader - DACC2DHR"]
pub type DACC2DHR_R = crate::FieldReader<u16>;
#[doc = "Field `DACC2DHR` writer - DACC2DHR"]
pub type DACC2DHR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DACC2DHR"]
    #[inline(always)]
    #[must_use]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W<DAC_DHR12R2rs> {
        DACC2DHR_W::new(self, 0)
    }
}
#[doc = "This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_dhr12r2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_dhr12r2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_DHR12R2rs;
impl crate::RegisterSpec for DAC_DHR12R2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_dhr12r2::R`](R) reader structure"]
impl crate::Readable for DAC_DHR12R2rs {}
#[doc = "`write(|w| ..)` method takes [`dac_dhr12r2::W`](W) writer structure"]
impl crate::Writable for DAC_DHR12R2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_DHR12R2 to value 0"]
impl crate::Resettable for DAC_DHR12R2rs {
    const RESET_VALUE: u32 = 0;
}
