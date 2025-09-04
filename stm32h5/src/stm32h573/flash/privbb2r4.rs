///Register `PRIVBB2R4` reader
pub type R = crate::R<PRIVBB2R4rs>;
///Register `PRIVBB2R4` writer
pub type W = crate::W<PRIVBB2R4rs>;
///Field `PRIVBB2` reader - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
pub type PRIVBB2_R = crate::FieldReader<u32>;
///Field `PRIVBB2` writer - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
pub type PRIVBB2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
    #[inline(always)]
    pub fn privbb2(&self) -> PRIVBB2_R {
        PRIVBB2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVBB2R4")
            .field("privbb2", &self.privbb2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
    #[inline(always)]
    pub fn privbb2(&mut self) -> PRIVBB2_W<PRIVBB2R4rs> {
        PRIVBB2_W::new(self, 0)
    }
}
/**FLASH privilege block-based register for Bank 2

You can [`read`](crate::Reg::read) this register and get [`privbb2r4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb2r4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:PRIVBB2R4)*/
pub struct PRIVBB2R4rs;
impl crate::RegisterSpec for PRIVBB2R4rs {
    type Ux = u32;
}
///`read()` method returns [`privbb2r4::R`](R) reader structure
impl crate::Readable for PRIVBB2R4rs {}
///`write(|w| ..)` method takes [`privbb2r4::W`](W) writer structure
impl crate::Writable for PRIVBB2R4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVBB2R4 to value 0
impl crate::Resettable for PRIVBB2R4rs {}
