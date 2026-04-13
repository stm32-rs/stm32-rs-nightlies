///Register `KEYR` writer
pub type W = crate::W<KEYRrs>;
///Field `KEY1R` writer - Bank 1 access configuration unlock key
pub type KEY1R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<KEYRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Bank 1 access configuration unlock key
    #[inline(always)]
    pub fn key1r(&mut self) -> KEY1R_W<'_, KEYRrs> {
        KEY1R_W::new(self, 0)
    }
}
/**FLASH key register for bank 1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct KEYRrs;
impl crate::RegisterSpec for KEYRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`keyr::W`](W) writer structure
impl crate::Writable for KEYRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KEYR to value 0
impl crate::Resettable for KEYRrs {}
