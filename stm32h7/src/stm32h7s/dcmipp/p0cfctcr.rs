///Register `P0CFCTCR` reader
pub type R = crate::R<P0CFCTCRrs>;
///Field `FRATE` reader - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode, ignored in Snapshot mode.
pub type FRATE_R = crate::FieldReader;
///Field `CPTMODE` reader - Capture mode
pub type CPTMODE_R = crate::BitReader;
///Field `CPTREQ` reader - Capture requested When PIPEN = 1, and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first frame received. In continuous grab mode the capture remains active and CPTREQ = 1, until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
pub type CPTREQ_R = crate::BitReader;
impl R {
    ///Bits 0:1 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode, ignored in Snapshot mode.
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Capture mode
    #[inline(always)]
    pub fn cptmode(&self) -> CPTMODE_R {
        CPTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture requested When PIPEN = 1, and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first frame received. In continuous grab mode the capture remains active and CPTREQ = 1, until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
    #[inline(always)]
    pub fn cptreq(&self) -> CPTREQ_R {
        CPTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0CFCTCR")
            .field("frate", &self.frate())
            .field("cptmode", &self.cptmode())
            .field("cptreq", &self.cptreq())
            .finish()
    }
}
/**DCMIPP Pipe0 current flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0cfctcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:P0CFCTCR)*/
pub struct P0CFCTCRrs;
impl crate::RegisterSpec for P0CFCTCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0cfctcr::R`](R) reader structure
impl crate::Readable for P0CFCTCRrs {}
///`reset()` method sets P0CFCTCR to value 0
impl crate::Resettable for P0CFCTCRrs {}
