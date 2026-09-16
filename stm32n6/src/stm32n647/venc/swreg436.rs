///Register `SWREG436` reader
pub type R = crate::R<SWREG436rs>;
///Register `SWREG436` writer
pub type W = crate::W<SWREG436rs>;
///Field `SWREG_FIELD` reader - high 32 bits of Base address for input picture cb (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - high 32 bits of Base address for input picture cb (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - high 32 bits of Base address for input picture cb (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG436")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - high 32 bits of Base address for input picture cb (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG436rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC high 32 bits of base address for input picture cb register

You can [`read`](crate::Reg::read) this register and get [`swreg436::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg436::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG436)*/
pub struct SWREG436rs;
impl crate::RegisterSpec for SWREG436rs {
    type Ux = u32;
}
///`read()` method returns [`swreg436::R`](R) reader structure
impl crate::Readable for SWREG436rs {}
///`write(|w| ..)` method takes [`swreg436::W`](W) writer structure
impl crate::Writable for SWREG436rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG436 to value 0
impl crate::Resettable for SWREG436rs {}
