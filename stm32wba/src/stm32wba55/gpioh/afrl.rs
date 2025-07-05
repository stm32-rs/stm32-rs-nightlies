///Register `AFRL` reader
pub type R = crate::R<AFRLrs>;
///Register `AFRL` writer
pub type W = crate::W<AFRLrs>;
///Field `AFSEL3` reader - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
pub type AFSEL3_R = crate::FieldReader;
///Field `AFSEL3` writer - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 12:15 - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afsel3", &self.afsel3())
            .finish()
    }
}
impl W {
    ///Bits 12:15 - Alternate function selection for port H I/O pin 3 These bits are written by software to configure alternate function I/Os. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W<AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
}
/**GPIO port H alternate function low register

You can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#GPIOH:AFRL)*/
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`afrl::R`](R) reader structure
impl crate::Readable for AFRLrs {}
///`write(|w| ..)` method takes [`afrl::W`](W) writer structure
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AFRL to value 0
impl crate::Resettable for AFRLrs {}
