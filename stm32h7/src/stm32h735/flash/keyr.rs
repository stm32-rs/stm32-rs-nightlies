///Register `KEYR` reader
pub type R = crate::R<KEYRrs>;
///Register `KEYR` writer
pub type W = crate::W<KEYRrs>;
///Field `KEYKEYRR` reader - access configuration unlock key
pub type KEYKEYRR_R = crate::FieldReader<u32>;
///Field `KEYKEYRR` writer - access configuration unlock key
pub type KEYKEYRR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - access configuration unlock key
    #[inline(always)]
    pub fn keykeyrr(&self) -> KEYKEYRR_R {
        KEYKEYRR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR")
            .field("keykeyrr", &self.keykeyrr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - access configuration unlock key
    #[inline(always)]
    pub fn keykeyrr(&mut self) -> KEYKEYRR_W<'_, KEYRrs> {
        KEYKEYRR_W::new(self, 0)
    }
}
/**FLASH key register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#FLASH:KEYR)*/
pub struct KEYRrs;
impl crate::RegisterSpec for KEYRrs {
    type Ux = u32;
}
///`read()` method returns [`keyr::R`](R) reader structure
impl crate::Readable for KEYRrs {}
///`write(|w| ..)` method takes [`keyr::W`](W) writer structure
impl crate::Writable for KEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR to value 0
impl crate::Resettable for KEYRrs {}
