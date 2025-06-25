///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
///Field `PUPD13` reader - Port C configuration I/O pin 13
pub type PUPD13_R = crate::FieldReader;
///Field `PUPD13` writer - Port C configuration I/O pin 13
pub type PUPD13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPD14` reader - Port C configuration I/O pin 14
pub type PUPD14_R = crate::FieldReader;
///Field `PUPD14` writer - Port C configuration I/O pin 14
pub type PUPD14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PUPD15` reader - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
pub type PUPD15_R = crate::FieldReader;
///Field `PUPD15` writer - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
pub type PUPD15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupd13", &self.pupd13())
            .field("pupd14", &self.pupd14())
            .field("pupd15", &self.pupd15())
            .finish()
    }
}
impl W {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn pupd13(&mut self) -> PUPD13_W<PUPDRrs> {
        PUPD13_W::new(self, 26)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn pupd14(&mut self) -> PUPD14_W<PUPDRrs> {
        PUPD14_W::new(self, 28)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOC SEC15.
    #[inline(always)]
    pub fn pupd15(&mut self) -> PUPD15_W<PUPDRrs> {
        PUPD15_W::new(self, 30)
    }
}
/**GPIO port C pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOC:PUPDR)*/
pub struct PUPDRrs;
impl crate::RegisterSpec for PUPDRrs {
    type Ux = u32;
}
///`read()` method returns [`pupdr::R`](R) reader structure
impl crate::Readable for PUPDRrs {}
///`write(|w| ..)` method takes [`pupdr::W`](W) writer structure
impl crate::Writable for PUPDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDRrs {}
