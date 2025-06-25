///Register `OTYPER` reader
pub type R = crate::R<OTYPERrs>;
///Register `OTYPER` writer
pub type W = crate::W<OTYPERrs>;
///Field `OT13` reader - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT13_R = crate::BitReader;
///Field `OT13` writer - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT14` reader - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT14_R = crate::BitReader;
///Field `OT14` writer - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OT15` reader - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT15_R = crate::BitReader;
///Field `OT15` writer - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
pub type OT15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 13 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTYPER")
            .field("ot13", &self.ot13())
            .field("ot14", &self.ot14())
            .field("ot15", &self.ot15())
            .finish()
    }
}
impl W {
    ///Bit 13 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot13(&mut self) -> OT13_W<OTYPERrs> {
        OT13_W::new(self, 13)
    }
    ///Bit 14 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot14(&mut self) -> OT14_W<OTYPERrs> {
        OT14_W::new(self, 14)
    }
    ///Bit 15 - Port C configuration I/O pin y These bits are written by software to configure the I/O output type. Access can be protected by GPIOC SECy.
    #[inline(always)]
    pub fn ot15(&mut self) -> OT15_W<OTYPERrs> {
        OT15_W::new(self, 15)
    }
}
/**GPIO port C output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOC:OTYPER)*/
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`otyper::R`](R) reader structure
impl crate::Readable for OTYPERrs {}
///`write(|w| ..)` method takes [`otyper::W`](W) writer structure
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPERrs {}
