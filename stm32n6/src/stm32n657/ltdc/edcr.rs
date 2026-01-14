///Register `EDCR` reader
pub type R = crate::R<EDCRrs>;
///Register `EDCR` writer
pub type W = crate::W<EDCRrs>;
///Field `OCYEN` reader - output conversion to YCbCr 422 enable
pub type OCYEN_R = crate::BitReader;
///Field `OCYEN` writer - output conversion to YCbCr 422 enable
pub type OCYEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCYSEL` reader - output conversion to YCbCr 422
pub type OCYSEL_R = crate::BitReader;
///Field `OCYSEL` writer - output conversion to YCbCr 422
pub type OCYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCYCO` reader - output conversion to YCbCr 422
pub type OCYCO_R = crate::BitReader;
///Field `OCYCO` writer - output conversion to YCbCr 422
pub type OCYCO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 25 - output conversion to YCbCr 422 enable
    #[inline(always)]
    pub fn ocyen(&self) -> OCYEN_R {
        OCYEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - output conversion to YCbCr 422
    #[inline(always)]
    pub fn ocysel(&self) -> OCYSEL_R {
        OCYSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - output conversion to YCbCr 422
    #[inline(always)]
    pub fn ocyco(&self) -> OCYCO_R {
        OCYCO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDCR")
            .field("ocyen", &self.ocyen())
            .field("ocysel", &self.ocysel())
            .field("ocyco", &self.ocyco())
            .finish()
    }
}
impl W {
    ///Bit 25 - output conversion to YCbCr 422 enable
    #[inline(always)]
    pub fn ocyen(&mut self) -> OCYEN_W<'_, EDCRrs> {
        OCYEN_W::new(self, 25)
    }
    ///Bit 26 - output conversion to YCbCr 422
    #[inline(always)]
    pub fn ocysel(&mut self) -> OCYSEL_W<'_, EDCRrs> {
        OCYSEL_W::new(self, 26)
    }
    ///Bit 27 - output conversion to YCbCr 422
    #[inline(always)]
    pub fn ocyco(&mut self) -> OCYCO_W<'_, EDCRrs> {
        OCYCO_W::new(self, 27)
    }
}
/**LTDC external display control register

You can [`read`](crate::Reg::read) this register and get [`edcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:EDCR)*/
pub struct EDCRrs;
impl crate::RegisterSpec for EDCRrs {
    type Ux = u32;
}
///`read()` method returns [`edcr::R`](R) reader structure
impl crate::Readable for EDCRrs {}
///`write(|w| ..)` method takes [`edcr::W`](W) writer structure
impl crate::Writable for EDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EDCR to value 0
impl crate::Resettable for EDCRrs {}
