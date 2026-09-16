///Register `CMCR` reader
pub type R = crate::R<CMCRrs>;
///Register `CMCR` writer
pub type W = crate::W<CMCRrs>;
///Field `INSEL` reader - input selection
pub type INSEL_R = crate::BitReader;
///Field `INSEL` writer - input selection
pub type INSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSFC` reader - Pipe selection for the frame counter
pub type PSFC_R = crate::FieldReader;
///Field `PSFC` writer - Pipe selection for the frame counter
pub type PSFC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CFC` writer - Clear frame counter
pub type CFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWAPRB` reader - Swap R/U and B/V
pub type SWAPRB_R = crate::BitReader;
///Field `SWAPRB` writer - Swap R/U and B/V
pub type SWAPRB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - input selection
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Pipe selection for the frame counter
    #[inline(always)]
    pub fn psfc(&self) -> PSFC_R {
        PSFC_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 7 - Swap R/U and B/V
    #[inline(always)]
    pub fn swaprb(&self) -> SWAPRB_R {
        SWAPRB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMCR")
            .field("insel", &self.insel())
            .field("psfc", &self.psfc())
            .field("swaprb", &self.swaprb())
            .finish()
    }
}
impl W {
    ///Bit 0 - input selection
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W<'_, CMCRrs> {
        INSEL_W::new(self, 0)
    }
    ///Bits 1:2 - Pipe selection for the frame counter
    #[inline(always)]
    pub fn psfc(&mut self) -> PSFC_W<'_, CMCRrs> {
        PSFC_W::new(self, 1)
    }
    ///Bit 4 - Clear frame counter
    #[inline(always)]
    pub fn cfc(&mut self) -> CFC_W<'_, CMCRrs> {
        CFC_W::new(self, 4)
    }
    ///Bit 7 - Swap R/U and B/V
    #[inline(always)]
    pub fn swaprb(&mut self) -> SWAPRB_W<'_, CMCRrs> {
        SWAPRB_W::new(self, 7)
    }
}
/**DCMIPP common configuration register

You can [`read`](crate::Reg::read) this register and get [`cmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:CMCR)*/
pub struct CMCRrs;
impl crate::RegisterSpec for CMCRrs {
    type Ux = u32;
}
///`read()` method returns [`cmcr::R`](R) reader structure
impl crate::Readable for CMCRrs {}
///`write(|w| ..)` method takes [`cmcr::W`](W) writer structure
impl crate::Writable for CMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMCR to value 0
impl crate::Resettable for CMCRrs {}
