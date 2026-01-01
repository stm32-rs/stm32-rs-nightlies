///Register `PRIVBB1R` reader
pub type R = crate::R<PRIVBB1Rrs>;
///Register `PRIVBB1R` writer
pub type W = crate::W<PRIVBB1Rrs>;
///Field `PRIVBB1` reader - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
pub type PRIVBB1_R = crate::FieldReader;
///Field `PRIVBB1` writer - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
pub type PRIVBB1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVBB1R")
            .field("privbb1", &self.privbb1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
    #[inline(always)]
    pub fn privbb1(&mut self) -> PRIVBB1_W<'_, PRIVBB1Rrs> {
        PRIVBB1_W::new(self, 0)
    }
}
/**FLASH privilege register for bank 1

You can [`read`](crate::Reg::read) this register and get [`privbb1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privbb1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#FLASH:PRIVBB1R)*/
pub struct PRIVBB1Rrs;
impl crate::RegisterSpec for PRIVBB1Rrs {
    type Ux = u32;
}
///`read()` method returns [`privbb1r::R`](R) reader structure
impl crate::Readable for PRIVBB1Rrs {}
///`write(|w| ..)` method takes [`privbb1r::W`](W) writer structure
impl crate::Writable for PRIVBB1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVBB1R to value 0
impl crate::Resettable for PRIVBB1Rrs {}
