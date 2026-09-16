///Register `CH4DATINR` reader
pub type R = crate::R<CH4DATINRrs>;
///Register `CH4DATINR` writer
pub type W = crate::W<CH4DATINRrs>;
///Field `INDAT0` reader - INDAT0
pub type INDAT0_R = crate::FieldReader<u16>;
///Field `INDAT0` writer - INDAT0
pub type INDAT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `INDAT1` reader - INDAT1
pub type INDAT1_R = crate::FieldReader<u16>;
///Field `INDAT1` writer - INDAT1
pub type INDAT1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - INDAT0
    #[inline(always)]
    pub fn indat0(&self) -> INDAT0_R {
        INDAT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - INDAT1
    #[inline(always)]
    pub fn indat1(&self) -> INDAT1_R {
        INDAT1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH4DATINR")
            .field("indat0", &self.indat0())
            .field("indat1", &self.indat1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - INDAT0
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W<'_, CH4DATINRrs> {
        INDAT0_W::new(self, 0)
    }
    ///Bits 16:31 - INDAT1
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W<'_, CH4DATINRrs> {
        INDAT1_W::new(self, 16)
    }
}
/**This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch4datinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4datinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DFSDM1:CH4DATINR)*/
pub struct CH4DATINRrs;
impl crate::RegisterSpec for CH4DATINRrs {
    type Ux = u32;
}
///`read()` method returns [`ch4datinr::R`](R) reader structure
impl crate::Readable for CH4DATINRrs {}
///`write(|w| ..)` method takes [`ch4datinr::W`](W) writer structure
impl crate::Writable for CH4DATINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH4DATINR to value 0
impl crate::Resettable for CH4DATINRrs {}
