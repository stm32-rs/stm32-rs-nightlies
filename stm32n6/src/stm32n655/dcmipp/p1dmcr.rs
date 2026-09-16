///Register `P1DMCR` reader
pub type R = crate::R<P1DMCRrs>;
///Register `P1DMCR` writer
pub type W = crate::W<P1DMCRrs>;
///Field `ENABLE` reader - None
pub type ENABLE_R = crate::BitReader;
///Field `ENABLE` writer - None
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TYPE` reader - Raw Bayer type
pub type TYPE_R = crate::FieldReader;
///Field `TYPE` writer - Raw Bayer type
pub type TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PEAK` reader - Strength of the peak detection
pub type PEAK_R = crate::FieldReader;
///Field `PEAK` writer - Strength of the peak detection
pub type PEAK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LINEV` reader - Strength of the vertical line detection
pub type LINEV_R = crate::FieldReader;
///Field `LINEV` writer - Strength of the vertical line detection
pub type LINEV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LINEH` reader - Strength of the horizontal line detection
pub type LINEH_R = crate::FieldReader;
///Field `LINEH` writer - Strength of the horizontal line detection
pub type LINEH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `EDGE` reader - Strength of the edge detection
pub type EDGE_R = crate::FieldReader;
///Field `EDGE` writer - Strength of the edge detection
pub type EDGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Raw Bayer type
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 16:18 - Strength of the peak detection
    #[inline(always)]
    pub fn peak(&self) -> PEAK_R {
        PEAK_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Strength of the vertical line detection
    #[inline(always)]
    pub fn linev(&self) -> LINEV_R {
        LINEV_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 24:26 - Strength of the horizontal line detection
    #[inline(always)]
    pub fn lineh(&self) -> LINEH_R {
        LINEH_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 28:30 - Strength of the edge detection
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1DMCR")
            .field("enable", &self.enable())
            .field("type_", &self.type_())
            .field("peak", &self.peak())
            .field("linev", &self.linev())
            .field("lineh", &self.lineh())
            .field("edge", &self.edge())
            .finish()
    }
}
impl W {
    ///Bit 0 - None
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, P1DMCRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bits 1:2 - Raw Bayer type
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<'_, P1DMCRrs> {
        TYPE_W::new(self, 1)
    }
    ///Bits 16:18 - Strength of the peak detection
    #[inline(always)]
    pub fn peak(&mut self) -> PEAK_W<'_, P1DMCRrs> {
        PEAK_W::new(self, 16)
    }
    ///Bits 20:22 - Strength of the vertical line detection
    #[inline(always)]
    pub fn linev(&mut self) -> LINEV_W<'_, P1DMCRrs> {
        LINEV_W::new(self, 20)
    }
    ///Bits 24:26 - Strength of the horizontal line detection
    #[inline(always)]
    pub fn lineh(&mut self) -> LINEH_W<'_, P1DMCRrs> {
        LINEH_W::new(self, 24)
    }
    ///Bits 28:30 - Strength of the edge detection
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W<'_, P1DMCRrs> {
        EDGE_W::new(self, 28)
    }
}
/**DCMIPP Pipe1 demosaicing configuration register

You can [`read`](crate::Reg::read) this register and get [`p1dmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1dmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1DMCR)*/
pub struct P1DMCRrs;
impl crate::RegisterSpec for P1DMCRrs {
    type Ux = u32;
}
///`read()` method returns [`p1dmcr::R`](R) reader structure
impl crate::Readable for P1DMCRrs {}
///`write(|w| ..)` method takes [`p1dmcr::W`](W) writer structure
impl crate::Writable for P1DMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1DMCR to value 0
impl crate::Resettable for P1DMCRrs {}
