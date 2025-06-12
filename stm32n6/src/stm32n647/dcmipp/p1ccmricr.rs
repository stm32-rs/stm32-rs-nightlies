///Register `P1CCMRICR` reader
pub type R = crate::R<P1CCMRICRrs>;
///Field `ROILSZ` reader - Current region of interest line size width
pub type ROILSZ_R = crate::FieldReader;
///Field `ROI1EN` reader - Current region of interest 1 enable
pub type ROI1EN_R = crate::BitReader;
///Field `ROI2EN` reader - Current region of interest 2 enable
pub type ROI2EN_R = crate::BitReader;
///Field `ROI3EN` reader - Current region of interest 3 enable
pub type ROI3EN_R = crate::BitReader;
///Field `ROI4EN` reader - Current region of interest 4 enable
pub type ROI4EN_R = crate::BitReader;
///Field `ROI5EN` reader - Current region of interest 5 enable
pub type ROI5EN_R = crate::BitReader;
///Field `ROI6EN` reader - Current region of interest 6 enable
pub type ROI6EN_R = crate::BitReader;
///Field `ROI7EN` reader - Current region of interest 7 enable
pub type ROI7EN_R = crate::BitReader;
///Field `ROI8EN` reader - Current region of interest 8 enable
pub type ROI8EN_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Current region of interest line size width
    #[inline(always)]
    pub fn roilsz(&self) -> ROILSZ_R {
        ROILSZ_R::new((self.bits & 3) as u8)
    }
    ///Bit 16 - Current region of interest 1 enable
    #[inline(always)]
    pub fn roi1en(&self) -> ROI1EN_R {
        ROI1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Current region of interest 2 enable
    #[inline(always)]
    pub fn roi2en(&self) -> ROI2EN_R {
        ROI2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Current region of interest 3 enable
    #[inline(always)]
    pub fn roi3en(&self) -> ROI3EN_R {
        ROI3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Current region of interest 4 enable
    #[inline(always)]
    pub fn roi4en(&self) -> ROI4EN_R {
        ROI4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Current region of interest 5 enable
    #[inline(always)]
    pub fn roi5en(&self) -> ROI5EN_R {
        ROI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Current region of interest 6 enable
    #[inline(always)]
    pub fn roi6en(&self) -> ROI6EN_R {
        ROI6EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Current region of interest 7 enable
    #[inline(always)]
    pub fn roi7en(&self) -> ROI7EN_R {
        ROI7EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Current region of interest 8 enable
    #[inline(always)]
    pub fn roi8en(&self) -> ROI8EN_R {
        ROI8EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CCMRICR")
            .field("roilsz", &self.roilsz())
            .field("roi1en", &self.roi1en())
            .field("roi2en", &self.roi2en())
            .field("roi3en", &self.roi3en())
            .field("roi4en", &self.roi4en())
            .field("roi5en", &self.roi5en())
            .field("roi6en", &self.roi6en())
            .field("roi7en", &self.roi7en())
            .field("roi8en", &self.roi8en())
            .finish()
    }
}
/**DCMIPP Pipex current common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p1ccmricr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:P1CCMRICR)*/
pub struct P1CCMRICRrs;
impl crate::RegisterSpec for P1CCMRICRrs {
    type Ux = u32;
}
///`read()` method returns [`p1ccmricr::R`](R) reader structure
impl crate::Readable for P1CCMRICRrs {}
///`reset()` method sets P1CCMRICR to value 0
impl crate::Resettable for P1CCMRICRrs {}
