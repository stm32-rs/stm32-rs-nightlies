///Register `P2CFCTCR` reader
pub type R = crate::R<P2CFCTCRrs>;
///Field `FRATE` reader - Frame capture rate control
pub type FRATE_R = crate::FieldReader;
///Field `CPTMODE` reader - Capture mode
pub type CPTMODE_R = crate::BitReader;
///Field `CPTREQ` reader - Capture requested
pub type CPTREQ_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Frame capture rate control
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Capture mode
    #[inline(always)]
    pub fn cptmode(&self) -> CPTMODE_R {
        CPTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture requested
    #[inline(always)]
    pub fn cptreq(&self) -> CPTREQ_R {
        CPTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2CFCTCR")
            .field("frate", &self.frate())
            .field("cptmode", &self.cptmode())
            .field("cptreq", &self.cptreq())
            .finish()
    }
}
/**DCMIPP Pipex current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p2cfctcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMIPP:P2CFCTCR)*/
pub struct P2CFCTCRrs;
impl crate::RegisterSpec for P2CFCTCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2cfctcr::R`](R) reader structure
impl crate::Readable for P2CFCTCRrs {}
///`reset()` method sets P2CFCTCR to value 0
impl crate::Resettable for P2CFCTCRrs {}
