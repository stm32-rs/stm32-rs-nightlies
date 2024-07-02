///Register `DFSDM_CH0DLYR` reader
pub type R = crate::R<DFSDM_CH0DLYRrs>;
///Register `DFSDM_CH0DLYR` writer
pub type W = crate::W<DFSDM_CH0DLYRrs>;
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
        f.debug_struct("DFSDM_CH0DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<DFSDM_CH0DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 0 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch0dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch0dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:DFSDM_CH0DLYR)*/
pub struct DFSDM_CH0DLYRrs;
impl crate::RegisterSpec for DFSDM_CH0DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ch0dlyr::R`](R) reader structure
impl crate::Readable for DFSDM_CH0DLYRrs {}
///`write(|w| ..)` method takes [`dfsdm_ch0dlyr::W`](W) writer structure
impl crate::Writable for DFSDM_CH0DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_CH0DLYR to value 0
impl crate::Resettable for DFSDM_CH0DLYRrs {
    const RESET_VALUE: u32 = 0;
}
