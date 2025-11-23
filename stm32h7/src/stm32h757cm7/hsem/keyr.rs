///Register `KEYR` reader
pub type R = crate::R<KEYRrs>;
///Register `KEYR` writer
pub type W = crate::W<KEYRrs>;
///Field `KEY` reader - Semaphore Clear Key
pub type KEY_R = crate::FieldReader<u16>;
///Field `KEY` writer - Semaphore Clear Key
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 16:31 - Semaphore Clear Key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYR").field("key", &self.key()).finish()
    }
}
impl W {
    ///Bits 16:31 - Semaphore Clear Key
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KEYRrs> {
        KEY_W::new(self, 16)
    }
}
/**HSEM Interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`keyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HSEM:KEYR)*/
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
