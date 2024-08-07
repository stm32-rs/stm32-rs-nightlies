///Register `HSEM_KEYR` reader
pub type R = crate::R<HSEM_KEYRrs>;
///Register `HSEM_KEYR` writer
pub type W = crate::W<HSEM_KEYRrs>;
///Field `KEY` reader - KEY
pub type KEY_R = crate::FieldReader<u16>;
///Field `KEY` writer - KEY
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - KEY
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSEM_KEYR")
            .field("key", &self.key())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - KEY
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<HSEM_KEYRrs> {
        KEY_W::new(self, 16)
    }
}
/**HSEM interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`hsem_keyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_keyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:HSEM_KEYR)*/
pub struct HSEM_KEYRrs;
impl crate::RegisterSpec for HSEM_KEYRrs {
    type Ux = u32;
}
///`read()` method returns [`hsem_keyr::R`](R) reader structure
impl crate::Readable for HSEM_KEYRrs {}
///`write(|w| ..)` method takes [`hsem_keyr::W`](W) writer structure
impl crate::Writable for HSEM_KEYRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSEM_KEYR to value 0
impl crate::Resettable for HSEM_KEYRrs {
    const RESET_VALUE: u32 = 0;
}
