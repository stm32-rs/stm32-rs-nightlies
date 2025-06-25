///Register `PUPDR` reader
pub type R = crate::R<PUPDRrs>;
///Register `PUPDR` writer
pub type W = crate::W<PUPDRrs>;
///Field `PUPD3` reader - Port H configuration I/O pin 3 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOH SEC3.
pub type PUPD3_R = crate::FieldReader;
///Field `PUPD3` writer - Port H configuration I/O pin 3 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOH SEC3.
pub type PUPD3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUPDR")
            .field("pupd3", &self.pupd3())
            .finish()
    }
}
impl W {
    ///Bits 6:7 - Port H configuration I/O pin 3 These bits are written by software to configure the I/O pull-up or pull-down Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn pupd3(&mut self) -> PUPD3_W<PUPDRrs> {
        PUPD3_W::new(self, 6)
    }
}
/**GPIO port H pull-up/pull-down register

You can [`read`](crate::Reg::read) this register and get [`pupdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pupdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOH:PUPDR)*/
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
