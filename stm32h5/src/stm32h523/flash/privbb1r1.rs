///Register `PRIVBB1R1` reader
pub type R = crate::R<PRIVBB1R1rs>;
///Register `PRIVBB1R1` writer
pub type W = crate::W<PRIVBB1R1rs>;
///Field `PRIVBB1` reader - Privileged/unprivileged 8-Kbyte flash Bank1 sector attribute
pub type PRIVBB1_R = crate::FieldReader<u32>;
///Field `PRIVBB1` writer - Privileged/unprivileged 8-Kbyte flash Bank1 sector attribute
pub type PRIVBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank1 sector attribute
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVBB1R1")
            .field("privbb1", &self.privbb1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Privileged/unprivileged 8-Kbyte flash Bank1 sector attribute
    #[inline(always)]
    pub fn privbb1(&mut self) -> PRIVBB1_W<'_, PRIVBB1R1rs> {
        PRIVBB1_W::new(self, 0)
    }
}
/**FLASH privilege block based register for Bank1

You can [`read`](crate::Reg::read) this register and get [`privbb1r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FLASH:PRIVBB1R1)*/
pub struct PRIVBB1R1rs;
impl crate::RegisterSpec for PRIVBB1R1rs {
    type Ux = u32;
}
///`read()` method returns [`privbb1r1::R`](R) reader structure
impl crate::Readable for PRIVBB1R1rs {}
///`write(|w| ..)` method takes [`privbb1r1::W`](W) writer structure
impl crate::Writable for PRIVBB1R1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVBB1R1 to value 0
impl crate::Resettable for PRIVBB1R1rs {}
