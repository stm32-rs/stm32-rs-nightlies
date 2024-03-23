#[doc = "Register `DFSDM_CHDATIN5R` reader"]
pub type R = crate::R<DFSDM_CHDATIN5Rrs>;
#[doc = "Register `DFSDM_CHDATIN5R` writer"]
pub type W = crate::W<DFSDM_CHDATIN5Rrs>;
#[doc = "Field `INDAT0` reader - Input data for channel 5"]
pub type INDAT0_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT0` writer - Input data for channel 5"]
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INDAT1` reader - Input data for channel 6"]
pub type INDAT1_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT1` writer - Input data for channel 6"]
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data for channel 5"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Input data for channel 6"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input data for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<DFSDM_CHDATIN5Rrs> {
        INDAT0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Input data for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<DFSDM_CHDATIN5Rrs> {
        INDAT1_W::new(self, 16)
    }
}
#[doc = "DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin5r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin5r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CHDATIN5Rrs;
impl crate::RegisterSpec for DFSDM_CHDATIN5Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_chdatin5r::R`](R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN5Rrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_chdatin5r::W`](W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN5Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_CHDATIN5R to value 0"]
impl crate::Resettable for DFSDM_CHDATIN5Rrs {
    const RESET_VALUE: u32 = 0;
}
