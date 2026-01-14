///Register `PRESCR` reader
pub type R = crate::R<PRESCRrs>;
///Register `PRESCR` writer
pub type W = crate::W<PRESCRrs>;
///Field `FSC` reader - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected, but the first occurrence of LSC after an FEC code is interpreted as the start of frame delimiter.
pub type FSC_R = crate::FieldReader;
///Field `FSC` writer - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected, but the first occurrence of LSC after an FEC code is interpreted as the start of frame delimiter.
pub type FSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LSC` reader - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LSC.
pub type LSC_R = crate::FieldReader;
///Field `LSC` writer - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LSC.
pub type LSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `LEC` reader - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LEC.
pub type LEC_R = crate::FieldReader;
///Field `LEC` writer - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LEC.
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `FEC` reader - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF00 00XY) are interpreted as frame end delimiters.
pub type FEC_R = crate::FieldReader;
///Field `FEC` writer - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF00 00XY) are interpreted as frame end delimiters.
pub type FEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected, but the first occurrence of LSC after an FEC code is interpreted as the start of frame delimiter.
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LSC.
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LEC.
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF00 00XY) are interpreted as frame end delimiters.
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESCR")
            .field("fsc", &self.fsc())
            .field("lsc", &self.lsc())
            .field("lec", &self.lec())
            .field("fec", &self.fec())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Frame start delimiter code This byte specifies the code of the frame start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FSC. If FSC is programmed to 0xFF, no frame start delimiter is detected, but the first occurrence of LSC after an FEC code is interpreted as the start of frame delimiter.
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W<'_, PRESCRrs> {
        FSC_W::new(self, 0)
    }
    ///Bits 8:15 - Line start delimiter code This byte specifies the code of the line start delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LSC.
    #[inline(always)]
    pub fn lsc(&mut self) -> LSC_W<'_, PRESCRrs> {
        LSC_W::new(self, 8)
    }
    ///Bits 16:23 - Line end delimiter code This byte specifies the code of the line end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, LEC.
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<'_, PRESCRrs> {
        LEC_W::new(self, 16)
    }
    ///Bits 24:31 - Frame end delimiter code This byte specifies the code of the frame end delimiter. The code consists of four bytes in the form of 0xFF, 0x00, 0x00, FEC. If FEC is programmed to 0xFF, all the unused codes (0xFF00 00XY) are interpreted as frame end delimiters.
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<'_, PRESCRrs> {
        FEC_W::new(self, 24)
    }
}
/**DCMIPP parallel interface embedded synchronization code register

You can [`read`](crate::Reg::read) this register and get [`prescr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:PRESCR)*/
pub struct PRESCRrs;
impl crate::RegisterSpec for PRESCRrs {
    type Ux = u32;
}
///`read()` method returns [`prescr::R`](R) reader structure
impl crate::Readable for PRESCRrs {}
///`write(|w| ..)` method takes [`prescr::W`](W) writer structure
impl crate::Writable for PRESCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRESCR to value 0
impl crate::Resettable for PRESCRrs {}
