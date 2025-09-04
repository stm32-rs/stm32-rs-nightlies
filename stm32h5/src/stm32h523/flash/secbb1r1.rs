///Register `SECBB1R1` reader
pub type R = crate::R<SECBB1R1rs>;
///Register `SECBB1R1` writer
pub type W = crate::W<SECBB1R1rs>;
///Field `SECBB1` reader - Secure/non-secure 8 Kbytes flash Bank1 sector attributes
pub type SECBB1_R = crate::FieldReader<u32>;
///Field `SECBB1` writer - Secure/non-secure 8 Kbytes flash Bank1 sector attributes
pub type SECBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Secure/non-secure 8 Kbytes flash Bank1 sector attributes
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBB1R1")
            .field("secbb1", &self.secbb1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Secure/non-secure 8 Kbytes flash Bank1 sector attributes
    #[inline(always)]
    pub fn secbb1(&mut self) -> SECBB1_W<SECBB1R1rs> {
        SECBB1_W::new(self, 0)
    }
}
/**FLASH secure block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`secbb1r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secbb1r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:SECBB1R1)*/
pub struct SECBB1R1rs;
impl crate::RegisterSpec for SECBB1R1rs {
    type Ux = u32;
}
///`read()` method returns [`secbb1r1::R`](R) reader structure
impl crate::Readable for SECBB1R1rs {}
///`write(|w| ..)` method takes [`secbb1r1::W`](W) writer structure
impl crate::Writable for SECBB1R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECBB1R1 to value 0
impl crate::Resettable for SECBB1R1rs {}
