///Register `ODR` reader
pub type R = crate::R<ODRrs>;
///Register `ODR` writer
pub type W = crate::W<ODRrs>;
///Field `OD13` reader - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD13_R = crate::BitReader;
///Field `OD13` writer - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD14` reader - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD14_R = crate::BitReader;
///Field `OD14` writer - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OD15` reader - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD15_R = crate::BitReader;
///Field `OD15` writer - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
pub type OD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od13(&self) -> OD13_R {
        OD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od14(&self) -> OD14_R {
        OD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od15(&self) -> OD15_R {
        OD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODR")
            .field("od13", &self.od13())
            .field("od14", &self.od14())
            .field("od15", &self.od15())
            .finish()
    }
}
impl W {
    ///Bit 13 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od13(&mut self) -> OD13_W<ODRrs> {
        OD13_W::new(self, 13)
    }
    ///Bit 14 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od14(&mut self) -> OD14_W<ODRrs> {
        OD14_W::new(self, 14)
    }
    ///Bit 15 - Port C output data I/O pin y These bits can be read and written by software. Access can be protected by GPIOC SECy. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOC_BSRR or GPIOC_BRR registers.
    #[inline(always)]
    pub fn od15(&mut self) -> OD15_W<ODRrs> {
        OD15_W::new(self, 15)
    }
}
/**GPIO port C output data register

You can [`read`](crate::Reg::read) this register and get [`odr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOC:ODR)*/
pub struct ODRrs;
impl crate::RegisterSpec for ODRrs {
    type Ux = u32;
}
///`read()` method returns [`odr::R`](R) reader structure
impl crate::Readable for ODRrs {}
///`write(|w| ..)` method takes [`odr::W`](W) writer structure
impl crate::Writable for ODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODRrs {}
