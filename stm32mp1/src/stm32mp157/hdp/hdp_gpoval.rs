#[doc = "Register `HDP_GPOVAL` reader"]
pub type R = crate::R<HDP_GPOVALrs>;
#[doc = "Register `HDP_GPOVAL` writer"]
pub type W = crate::W<HDP_GPOVALrs>;
#[doc = "Field `HDPGPOVAL` reader - HDPGPOVAL"]
pub type HDPGPOVAL_R = crate::FieldReader;
#[doc = "Field `HDPGPOVAL` writer - HDPGPOVAL"]
pub type HDPGPOVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    pub fn hdpgpoval(&self) -> HDPGPOVAL_R {
        HDPGPOVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    #[must_use]
    pub fn hdpgpoval(&mut self) -> HDPGPOVAL_W<HDP_GPOVALrs> {
        HDPGPOVAL_W::new(self, 0)
    }
}
#[doc = "HDP GPO value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp_gpoval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp_gpoval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP_GPOVALrs;
impl crate::RegisterSpec for HDP_GPOVALrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp_gpoval::R`](R) reader structure"]
impl crate::Readable for HDP_GPOVALrs {}
#[doc = "`write(|w| ..)` method takes [`hdp_gpoval::W`](W) writer structure"]
impl crate::Writable for HDP_GPOVALrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDP_GPOVAL to value 0"]
impl crate::Resettable for HDP_GPOVALrs {
    const RESET_VALUE: u32 = 0;
}
