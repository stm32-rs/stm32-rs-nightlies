///Register `SPDIFRX_IFCR` reader
pub type R = crate::R<SPDIFRX_IFCRrs>;
///Register `SPDIFRX_IFCR` writer
pub type W = crate::W<SPDIFRX_IFCRrs>;
///Field `PERRCF` writer - PERRCF
pub type PERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRCF` writer - OVRCF
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBDCF` writer - SBDCF
pub type SBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCDCF` writer - SYNCDCF
pub type SYNCDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPDIFRX_IFCR").finish()
    }
}
impl W {
    ///Bit 2 - PERRCF
    #[inline(always)]
    #[must_use]
    pub fn perrcf(&mut self) -> PERRCF_W<SPDIFRX_IFCRrs> {
        PERRCF_W::new(self, 2)
    }
    ///Bit 3 - OVRCF
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<SPDIFRX_IFCRrs> {
        OVRCF_W::new(self, 3)
    }
    ///Bit 4 - SBDCF
    #[inline(always)]
    #[must_use]
    pub fn sbdcf(&mut self) -> SBDCF_W<SPDIFRX_IFCRrs> {
        SBDCF_W::new(self, 4)
    }
    ///Bit 5 - SYNCDCF
    #[inline(always)]
    #[must_use]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<SPDIFRX_IFCRrs> {
        SYNCDCF_W::new(self, 5)
    }
}
/**Interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`spdifrx_ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdifrx_ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPDIFRX:SPDIFRX_IFCR)*/
pub struct SPDIFRX_IFCRrs;
impl crate::RegisterSpec for SPDIFRX_IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`spdifrx_ifcr::R`](R) reader structure
impl crate::Readable for SPDIFRX_IFCRrs {}
///`write(|w| ..)` method takes [`spdifrx_ifcr::W`](W) writer structure
impl crate::Writable for SPDIFRX_IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPDIFRX_IFCR to value 0
impl crate::Resettable for SPDIFRX_IFCRrs {
    const RESET_VALUE: u32 = 0;
}
