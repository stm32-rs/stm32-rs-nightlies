///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
///Field `OSPEED3` reader - Port H configuration I/O pin 3 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOH SEC3. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub type OSPEED3_R = crate::FieldReader;
///Field `OSPEED3` writer - Port H configuration I/O pin 3 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOH SEC3. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
pub type OSPEED3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOH SEC3. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED3_R {
        OSPEED3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed3", &self.ospeed3())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O output speed. Access can be protected by GPIOH SEC3. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED3_W<OSPEEDRrs> {
        OSPEED3_W::new(self, 6)
    }
}
/**GPIO port H output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:OSPEEDR)*/
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
