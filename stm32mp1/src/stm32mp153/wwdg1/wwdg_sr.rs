///Register `WWDG_SR` reader
pub type R = crate::R<WWDG_SRrs>;
///Register `WWDG_SR` writer
pub type W = crate::W<WWDG_SRrs>;
///Field `EWIF` reader - EWIF
pub type EWIF_R = crate::BitReader;
///Field `EWIF` writer - EWIF
pub type EWIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EWIF
    #[inline(always)]
    pub fn ewif(&self) -> EWIF_R {
        EWIF_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG_SR")
            .field("ewif", &self.ewif())
            .finish()
    }
}
impl W {
    ///Bit 0 - EWIF
    #[inline(always)]
    #[must_use]
    pub fn ewif(&mut self) -> EWIF_W<WWDG_SRrs> {
        EWIF_W::new(self, 0)
    }
}
/**Status register

You can [`read`](crate::Reg::read) this register and get [`wwdg_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#WWDG1:WWDG_SR)*/
pub struct WWDG_SRrs;
impl crate::RegisterSpec for WWDG_SRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_sr::R`](R) reader structure
impl crate::Readable for WWDG_SRrs {}
///`write(|w| ..)` method takes [`wwdg_sr::W`](W) writer structure
impl crate::Writable for WWDG_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WWDG_SR to value 0
impl crate::Resettable for WWDG_SRrs {
    const RESET_VALUE: u32 = 0;
}
