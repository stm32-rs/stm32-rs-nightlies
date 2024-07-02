///Register `DFSDM_CH3DLYR` reader
pub type R = crate::R<DFSDM_CH3DLYRrs>;
///Register `DFSDM_CH3DLYR` writer
pub type W = crate::W<DFSDM_CH3DLYRrs>;
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
        f.debug_struct("DFSDM_CH3DLYR")
            .field("plsskp", &self.plsskp())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - PLSSKP
    #[inline(always)]
    #[must_use]
    pub fn plsskp(&mut self) -> PLSSKP_W<DFSDM_CH3DLYRrs> {
        PLSSKP_W::new(self, 0)
    }
}
/**DFSDM channel 3 delay register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_ch3dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_ch3dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:DFSDM_CH3DLYR)*/
pub struct DFSDM_CH3DLYRrs;
impl crate::RegisterSpec for DFSDM_CH3DLYRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_ch3dlyr::R`](R) reader structure
impl crate::Readable for DFSDM_CH3DLYRrs {}
///`write(|w| ..)` method takes [`dfsdm_ch3dlyr::W`](W) writer structure
impl crate::Writable for DFSDM_CH3DLYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DFSDM_CH3DLYR to value 0
impl crate::Resettable for DFSDM_CH3DLYRrs {
    const RESET_VALUE: u32 = 0;
}
