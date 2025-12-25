///Register `CH6DATINR` reader
pub type R = crate::R<CH6DATINRrs>;
///Register `CH6DATINR` writer
pub type W = crate::W<CH6DATINRrs>;
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
        f.debug_struct("CH6DATINR")
            .field("indat0", &self.indat0())
            .field("indat1", &self.indat1())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - INDAT0
    #[inline(always)]
    pub fn indat0(&mut self) -> INDAT0_W<'_, CH6DATINRrs> {
        INDAT0_W::new(self, 0)
    }
    ///Bits 16:31 - INDAT1
    #[inline(always)]
    pub fn indat1(&mut self) -> INDAT1_W<'_, CH6DATINRrs> {
        INDAT1_W::new(self, 16)
    }
}
/**This register contains 16-bit input data to be processed by DFSDM filter module.

You can [`read`](crate::Reg::read) this register and get [`ch6datinr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6datinr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:CH6DATINR)*/
pub struct CH6DATINRrs;
impl crate::RegisterSpec for CH6DATINRrs {
    type Ux = u32;
}
///`read()` method returns [`ch6datinr::R`](R) reader structure
impl crate::Readable for CH6DATINRrs {}
///`write(|w| ..)` method takes [`ch6datinr::W`](W) writer structure
impl crate::Writable for CH6DATINRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CH6DATINR to value 0
impl crate::Resettable for CH6DATINRrs {}
