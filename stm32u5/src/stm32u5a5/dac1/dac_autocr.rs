#[doc = "Register `DAC_AUTOCR` reader"]
pub type R = crate::R<DAC_AUTOCRrs>;
#[doc = "Register `DAC_AUTOCR` writer"]
pub type W = crate::W<DAC_AUTOCRrs>;
#[doc = "Field `AUTOMODE` reader - DAC Autonomous mode"]
pub type AUTOMODE_R = crate::BitReader;
#[doc = "Field `AUTOMODE` writer - DAC Autonomous mode"]
pub type AUTOMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 22 - DAC Autonomous mode"]
    #[inline(always)]
    pub fn automode(&self) -> AUTOMODE_R {
        AUTOMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - DAC Autonomous mode"]
    #[inline(always)]
    #[must_use]
    pub fn automode(&mut self) -> AUTOMODE_W<DAC_AUTOCRrs> {
        AUTOMODE_W::new(self, 22)
    }
}
#[doc = "Autonomous mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_autocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_autocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAC_AUTOCRrs;
impl crate::RegisterSpec for DAC_AUTOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_autocr::R`](R) reader structure"]
impl crate::Readable for DAC_AUTOCRrs {}
#[doc = "`write(|w| ..)` method takes [`dac_autocr::W`](W) writer structure"]
impl crate::Writable for DAC_AUTOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_AUTOCR to value 0"]
impl crate::Resettable for DAC_AUTOCRrs {
    const RESET_VALUE: u32 = 0;
}
