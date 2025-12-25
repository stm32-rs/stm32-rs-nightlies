///Register `P0SR` reader
pub type R = crate::R<P0SRrs>;
///Field `LINEF` reader - Multi-line capture completed raw interrupt status
pub type LINEF_R = crate::BitReader;
///Field `FRAMEF` reader - Frame capture completed raw interrupt status
pub type FRAMEF_R = crate::BitReader;
///Field `VSYNCF` reader - VSYNC raw interrupt status
pub type VSYNCF_R = crate::BitReader;
///Field `LIMITF` reader - Limit raw interrupt status
pub type LIMITF_R = crate::BitReader;
///Field `OVRF` reader - Overrun raw interrupt status
pub type OVRF_R = crate::BitReader;
///Field `LSTLINE` reader - Last line LSB bit, sampled at frame capture complete event.
pub type LSTLINE_R = crate::BitReader;
///Field `LSTFRM` reader - Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number that can be delivered by the camera through the CSI2 interface.
pub type LSTFRM_R = crate::BitReader;
///Field `CPTACT` reader - Capture immediate status
pub type CPTACT_R = crate::BitReader;
impl R {
    ///Bit 0 - Multi-line capture completed raw interrupt status
    #[inline(always)]
    pub fn linef(&self) -> LINEF_R {
        LINEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame capture completed raw interrupt status
    #[inline(always)]
    pub fn framef(&self) -> FRAMEF_R {
        FRAMEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VSYNC raw interrupt status
    #[inline(always)]
    pub fn vsyncf(&self) -> VSYNCF_R {
        VSYNCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Limit raw interrupt status
    #[inline(always)]
    pub fn limitf(&self) -> LIMITF_R {
        LIMITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Overrun raw interrupt status
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Last line LSB bit, sampled at frame capture complete event.
    #[inline(always)]
    pub fn lstline(&self) -> LSTLINE_R {
        LSTLINE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Last frame LSB bit, sampled at frame capture complete event. The information is extracted from the frame data number that can be delivered by the camera through the CSI2 interface.
    #[inline(always)]
    pub fn lstfrm(&self) -> LSTFRM_R {
        LSTFRM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 23 - Capture immediate status
    #[inline(always)]
    pub fn cptact(&self) -> CPTACT_R {
        CPTACT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0SR")
            .field("linef", &self.linef())
            .field("framef", &self.framef())
            .field("vsyncf", &self.vsyncf())
            .field("limitf", &self.limitf())
            .field("ovrf", &self.ovrf())
            .field("lstline", &self.lstline())
            .field("lstfrm", &self.lstfrm())
            .field("cptact", &self.cptact())
            .finish()
    }
}
/**DCMIPP Pipe0 status register

You can [`read`](crate::Reg::read) this register and get [`p0sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P0SR)*/
pub struct P0SRrs;
impl crate::RegisterSpec for P0SRrs {
    type Ux = u32;
}
///`read()` method returns [`p0sr::R`](R) reader structure
impl crate::Readable for P0SRrs {}
///`reset()` method sets P0SR to value 0
impl crate::Resettable for P0SRrs {}
