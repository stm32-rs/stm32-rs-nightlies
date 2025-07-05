///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEED13` reader - Port C configuration I/O pin 13
pub type OSPEED13_R = crate::FieldReader;
///Field `OSPEED13` writer - Port C configuration I/O pin 13
pub type OSPEED13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEED14` reader - Port C configuration I/O pin 14
pub type OSPEED14_R = crate::FieldReader;
///Field `OSPEED14` writer - Port C configuration I/O pin 14
pub type OSPEED14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OSPEED15` reader - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub type OSPEED15_R = crate::FieldReader;
///Field `OSPEED15` writer - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub type OSPEED15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED13_R {
        OSPEED13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED14_R {
        OSPEED14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED15_R {
        OSPEED15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Bits 26:27 - Port C configuration I/O pin 13
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED13_W<OSPEEDRrs> {
        OSPEED13_W::new(self, 26)
    }
    ///Bits 28:29 - Port C configuration I/O pin 14
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED14_W<OSPEEDRrs> {
        OSPEED14_W::new(self, 28)
    }
    ///Bits 30:31 - Port C configuration I/O pin 15 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOC SEC15. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED15_W<OSPEEDRrs> {
        OSPEED15_W::new(self, 30)
    }
}
/**GPIOC port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOC:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OSPEEDR to value 0
impl crate::Resettable for OSPEEDRrs {}
