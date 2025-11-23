///Register `P2FCTCR` reader
pub type R = crate::R<P2FCTCRrs>;
///Register `P2FCTCR` writer
pub type W = crate::W<P2FCTCRrs>;
///Field `FRATE` reader - Frame capture rate control
pub type FRATE_R = crate::FieldReader;
///Field `FRATE` writer - Frame capture rate control
pub type FRATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CPTMODE` reader - Capture mode
pub type CPTMODE_R = crate::BitReader;
///Field `CPTMODE` writer - Capture mode
pub type CPTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPTREQ` reader - Capture requested
pub type CPTREQ_R = crate::BitReader;
///Field `CPTREQ` writer - Capture requested
pub type CPTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("P2FCTCR")
            .field("frate", &self.frate())
            .field("cptmode", &self.cptmode())
            .field("cptreq", &self.cptreq())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Frame capture rate control
    #[inline(always)]
    pub fn frate(&mut self) -> FRATE_W<'_, P2FCTCRrs> {
        FRATE_W::new(self, 0)
    }
    ///Bit 2 - Capture mode
    #[inline(always)]
    pub fn cptmode(&mut self) -> CPTMODE_W<'_, P2FCTCRrs> {
        CPTMODE_W::new(self, 2)
    }
    ///Bit 3 - Capture requested
    #[inline(always)]
    pub fn cptreq(&mut self) -> CPTREQ_W<'_, P2FCTCRrs> {
        CPTREQ_W::new(self, 3)
    }
}
/**DCMIPP Pipex flow control configuration register

You can [`read`](crate::Reg::read) this register and get [`p2fctcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2fctcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P2FCTCR)*/
pub struct P2FCTCRrs;
impl crate::RegisterSpec for P2FCTCRrs {
    type Ux = u32;
}
///`read()` method returns [`p2fctcr::R`](R) reader structure
impl crate::Readable for P2FCTCRrs {}
///`write(|w| ..)` method takes [`p2fctcr::W`](W) writer structure
impl crate::Writable for P2FCTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P2FCTCR to value 0
impl crate::Resettable for P2FCTCRrs {}
