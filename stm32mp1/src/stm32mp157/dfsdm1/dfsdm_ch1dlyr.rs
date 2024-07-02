///Register `DFSDM_CH1DLYR` reader
pub type R = crate::R<DFSDM_CH1DLYRrs>;
///Register `DFSDM_CH1DLYR` writer
pub type W = crate::W<DFSDM_CH1DLYRrs>;
///Field `PLSSKP` reader - PLSSKP
pub type PLSSKP_R = crate::FieldReader;
///Field `PLSSKP` writer - PLSSKP
pub type PLSSKP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_CH1DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<DFSDM_CH1DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 1 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch1dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch1dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:DFSDM_CH1DLYR)*/
pub struct DFSDM_CH1DLYRrs;
impl crate::RegisterSpec for DFSDM_CH1DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ch1dlyr::R`](R) reader structure
impl crate::Readable for DFSDM_CH1DLYRrs {}
///`write(|w| ..)` method takes [`dfsdm_ch1dlyr::W`](W) writer structure
impl crate::Writable for DFSDM_CH1DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_CH1DLYR to value 0
impl crate::Resettable for DFSDM_CH1DLYRrs {
    const RESET_VALUE: u32 = 0;
}
