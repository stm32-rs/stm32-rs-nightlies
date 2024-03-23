#[doc = "Register `HDP2R_PRG` reader"]
pub type R = crate::R<HDP2R_PRGrs>;
#[doc = "Register `HDP2R_PRG` writer"]
pub type W = crate::W<HDP2R_PRGrs>;
#[doc = "Field `HDP2_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors"]
pub type HDP2_STRT_R = crate::FieldReader;
#[doc = "Field `HDP2_STRT` writer - HDPL barrier start set in number of 8-Kbyte sectors"]
pub type HDP2_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HDP2_END` reader - HDPL barrier end set in number of 8-Kbyte sectors"]
pub type HDP2_END_R = crate::FieldReader;
#[doc = "Field `HDP2_END` writer - HDPL barrier end set in number of 8-Kbyte sectors"]
pub type HDP2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp2_strt(&self) -> HDP2_STRT_R {
        HDP2_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors"]
    #[inline(always)]
    pub fn hdp2_end(&self) -> HDP2_END_R {
        HDP2_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_strt(&mut self) -> HDP2_STRT_W<HDP2R_PRGrs> {
        HDP2_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors"]
    #[inline(always)]
    #[must_use]
    pub fn hdp2_end(&mut self) -> HDP2_END_W<HDP2R_PRGrs> {
        HDP2_END_W::new(self, 16)
    }
}
#[doc = "FLASH HDP Bank 2 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdp2r_prg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdp2r_prg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDP2R_PRGrs;
impl crate::RegisterSpec for HDP2R_PRGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdp2r_prg::R`](R) reader structure"]
impl crate::Readable for HDP2R_PRGrs {}
#[doc = "`write(|w| ..)` method takes [`hdp2r_prg::W`](W) writer structure"]
impl crate::Writable for HDP2R_PRGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDP2R_PRG to value 0"]
impl crate::Resettable for HDP2R_PRGrs {
    const RESET_VALUE: u32 = 0;
}
