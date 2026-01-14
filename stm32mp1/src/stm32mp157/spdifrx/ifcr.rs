///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
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
        f.debug_struct("IFCR").finish()
    }
}
impl W {
    ///Bit 2 - PERRCF
    #[inline(always)]
    pub fn perrcf(&mut self) -> PERRCF_W<'_, IFCRrs> {
        PERRCF_W::new(self, 2)
    }
    ///Bit 3 - OVRCF
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<'_, IFCRrs> {
        OVRCF_W::new(self, 3)
    }
    ///Bit 4 - SBDCF
    #[inline(always)]
    pub fn sbdcf(&mut self) -> SBDCF_W<'_, IFCRrs> {
        SBDCF_W::new(self, 4)
    }
    ///Bit 5 - SYNCDCF
    #[inline(always)]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<'_, IFCRrs> {
        SYNCDCF_W::new(self, 5)
    }
}
/**Interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SPDIFRX:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
