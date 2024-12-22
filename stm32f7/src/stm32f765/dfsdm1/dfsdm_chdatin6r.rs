///Register `DFSDM_CHDATIN6R` reader
pub type R = crate::R<DFSDM_CHDATIN6Rrs>;
///Register `DFSDM_CHDATIN6R` writer
pub type W = crate::W<DFSDM_CHDATIN6Rrs>;
///Field `INDAT0` reader - Input data for channel 6
pub type INDAT0_R = crate::FieldReader<u16>;
///Field `INDAT0` writer - Input data for channel 6
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INDAT1` reader - Input data for channel 7
pub type INDAT1_R = crate::FieldReader<u16>;
///Field `INDAT1` writer - Input data for channel 7
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Input data for channel 6
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Input data for channel 7
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_CHDATIN6R")
            .field("indat0", &self.indat0())
            .field("indat1", &self.indat1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Input data for channel 6
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W<DFSDM_CHDATIN6Rrs> {
        INDAT0_W::new(self, 0)
    }
    ///Bits 16:31 - Input data for channel 7
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W<DFSDM_CHDATIN6Rrs> {
        INDAT1_W::new(self, 16)
    }
}
/**DFSDM channel data input register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_chdatin6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_chdatin6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#DFSDM1:DFSDM_CHDATIN6R)*/
pub struct DFSDM_CHDATIN6Rrs;
impl crate::RegisterSpec for DFSDM_CHDATIN6Rrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_chdatin6r::R`](R) reader structure
impl crate::Readable for DFSDM_CHDATIN6Rrs {}
///`write(|w| ..)` method takes [`dfsdm_chdatin6r::W`](W) writer structure
impl crate::Writable for DFSDM_CHDATIN6Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_CHDATIN6R to value 0
impl crate::Resettable for DFSDM_CHDATIN6Rrs {
    const RESET_VALUE: u32 = 0;
}
