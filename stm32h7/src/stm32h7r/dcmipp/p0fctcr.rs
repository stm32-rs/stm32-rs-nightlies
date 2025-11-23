///Register `P0FCTCR` reader
pub type R = crate::R<P0FCTCRrs>;
///Register `P0FCTCR` writer
pub type W = crate::W<P0FCTCRrs>;
///Field `FRATE` reader - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode, ignored in Snapshot mode.
pub type FRATE_R = crate::FieldReader;
///Field `FRATE` writer - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode, ignored in Snapshot mode.
pub type FRATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPTMODE` reader - Capture mode
pub type CPTMODE_R = crate::BitReader;
///Field `CPTMODE` writer - Capture mode
pub type CPTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPTREQ` reader - Capture requested When PIPEN = 1 and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first received frame. In Continuous grab mode, the capture remains active and CPTREQ = 1 until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
pub type CPTREQ_R = crate::BitReader;
///Field `CPTREQ` writer - Capture requested When PIPEN = 1 and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first received frame. In Continuous grab mode, the capture remains active and CPTREQ = 1 until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
pub type CPTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 3 - Capture requested When PIPEN = 1 and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first received frame. In Continuous grab mode, the capture remains active and CPTREQ = 1 until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
    #[inline(always)]
    pub fn cptreq(&self) -> CPTREQ_R {
        CPTREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0FCTCR")
            .field("frate", &self.frate())
            .field("cptmode", &self.cptmode())
            .field("cptreq", &self.cptreq())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Frame capture rate control These bits define the frequency of frame capture. They are meaningful only in Continuous grab mode, ignored in Snapshot mode.
    #[inline(always)]
    pub fn frate(&mut self) -> FRATE_W<'_, P0FCTCRrs> {
        FRATE_W::new(self, 0)
    }
    ///Bit 2 - Capture mode
    #[inline(always)]
    pub fn cptmode(&mut self) -> CPTMODE_W<'_, P0FCTCRrs> {
        CPTMODE_W::new(self, 2)
    }
    ///Bit 3 - Capture requested When PIPEN = 1 and when the CPTREQ is set to 1 the pipe waits for the first VSync, and automatically starts a capture and sets CPTACT = 1 to mention it. In Snapshot mode the CPTREQ bit is automatically cleared at the start of the first received frame. In Continuous grab mode, the capture remains active and CPTREQ = 1 until the software clears CPTREQ: the capture stops and CPTACT is reset at the end of the ongoing frame. The DCMI and pipe configuration registers must be correctly programmed before enabling this bit.
    #[inline(always)]
    pub fn cptreq(&mut self) -> CPTREQ_W<'_, P0FCTCRrs> {
        CPTREQ_W::new(self, 3)
    }
}
/**DCMIPP Pipe0 flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p0fctcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0fctcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:P0FCTCR)*/
pub struct P0FCTCRrs;
impl crate::RegisterSpec for P0FCTCRrs {
    type Ux = u32;
}
///`read()` method returns [`p0fctcr::R`](R) reader structure
impl crate::Readable for P0FCTCRrs {}
///`write(|w| ..)` method takes [`p0fctcr::W`](W) writer structure
impl crate::Writable for P0FCTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P0FCTCR to value 0
impl crate::Resettable for P0FCTCRrs {}
