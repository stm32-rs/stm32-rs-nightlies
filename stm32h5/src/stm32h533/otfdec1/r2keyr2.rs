///Register `R2KEYR2` writer
pub type W = crate::W<R2KEYR2rs>;
///Field `REG_KEY` writer - Region key, bits \[95:64\]
pub type REG_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl core::fmt::Debug for crate::generic::Reg<R2KEYR2rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - Region key, bits \[95:64\]
    #[inline(always)]
    pub fn reg_key(&mut self) -> REG_KEY_W<R2KEYR2rs> {
        REG_KEY_W::new(self, 0)
    }
}
/**OTFDEC region 2 key register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r2keyr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#OTFDEC1:R2KEYR2)*/
pub struct R2KEYR2rs;
impl crate::RegisterSpec for R2KEYR2rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`r2keyr2::W`](W) writer structure
impl crate::Writable for R2KEYR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R2KEYR2 to value 0
impl crate::Resettable for R2KEYR2rs {}
