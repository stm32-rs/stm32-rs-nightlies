#[doc = "Register `DFSDM_CHDATIN1R` reader"]
pub type R = crate::R<DFSDM_CHDATIN1Rrs>;
#[doc = "Register `DFSDM_CHDATIN1R` writer"]
pub type W = crate::W<DFSDM_CHDATIN1Rrs>;
#[doc = "Field `INDAT0` reader - Input data for channel 1"]
pub type INDAT0_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT0` writer - Input data for channel 1"]
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INDAT1` reader - Input data for channel 2"]
pub type INDAT1_R = crate::FieldReader<u16>;
#[doc = "Field `INDAT1` writer - Input data for channel 2"]
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data for channel 1"]
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Input data for channel 2"]
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input data for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn indat0(&mut self) -> INDAT0_W<DFSDM_CHDATIN1Rrs> {
        INDAT0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Input data for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn indat1(&mut self) -> INDAT1_W<DFSDM_CHDATIN1Rrs> {
        INDAT1_W::new(self, 16)
    }
}
#[doc = "DFSDM channel data input register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_chdatin1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_chdatin1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_CHDATIN1Rrs;
impl crate::RegisterSpec for DFSDM_CHDATIN1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_chdatin1r::R`](R) reader structure"]
impl crate::Readable for DFSDM_CHDATIN1Rrs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_chdatin1r::W`](W) writer structure"]
impl crate::Writable for DFSDM_CHDATIN1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_CHDATIN1R to value 0"]
impl crate::Resettable for DFSDM_CHDATIN1Rrs {
    const RESET_VALUE: u32 = 0;
}
