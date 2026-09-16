///Register `DATINR` reader
pub type R = crate::R<DATINRrs>;
///Register `DATINR` writer
pub type W = crate::W<DATINRrs>;
///Field `INDAT0` reader - Input data for channel 0
pub type INDAT0_R = crate::FieldReader<u16>;
///Field `INDAT0` writer - Input data for channel 0
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `INDAT1` reader - Input data for channel 1
pub type INDAT1_R = crate::FieldReader<u16>;
///Field `INDAT1` writer - Input data for channel 1
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Input data for channel 0
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Input data for channel 1
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATINR")
            .field("indat0", &self.indat0())
            .field("indat1", &self.indat1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Input data for channel 0
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W<'_, DATINRrs> {
        INDAT0_W::new(self, 0)
    }
    ///Bits 16:31 - Input data for channel 1
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W<'_, DATINRrs> {
        INDAT1_W::new(self, 16)
    }
}
/**DFSDM channel data input register

You can [`read`](crate::Reg::read) this register and get [`datinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATINRrs;
impl crate::RegisterSpec for DATINRrs {
    type Ux = u32;
}
///`read()` method returns [`datinr::R`](R) reader structure
impl crate::Readable for DATINRrs {}
///`write(|w| ..)` method takes [`datinr::W`](W) writer structure
impl crate::Writable for DATINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DATINR to value 0
impl crate::Resettable for DATINRrs {}
