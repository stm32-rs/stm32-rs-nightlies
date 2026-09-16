///Register `EPOCHSELCR` reader
pub type R = crate::R<EPOCHSELCRrs>;
///Register `EPOCHSELCR` writer
pub type W = crate::W<EPOCHSELCRrs>;
///Field `EPOCH_SEL` reader - select EPOCH value to be sent to the SAES
pub type EPOCH_SEL_R = crate::FieldReader;
///Field `EPOCH_SEL` writer - select EPOCH value to be sent to the SAES
pub type EPOCH_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - select EPOCH value to be sent to the SAES
    #[inline(always)]
    pub fn epoch_sel(&self) -> EPOCH_SEL_R {
        EPOCH_SEL_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPOCHSELCR")
            .field("epoch_sel", &self.epoch_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - select EPOCH value to be sent to the SAES
    #[inline(always)]
    pub fn epoch_sel(&mut self) -> EPOCH_SEL_W<'_, EPOCHSELCRrs> {
        EPOCH_SEL_W::new(self, 0)
    }
}
/**SBS EPOCH selection control register

You can [`read`](crate::Reg::read) this register and get [`epochselcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochselcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#SBS:EPOCHSELCR)*/
pub struct EPOCHSELCRrs;
impl crate::RegisterSpec for EPOCHSELCRrs {
    type Ux = u32;
}
///`read()` method returns [`epochselcr::R`](R) reader structure
impl crate::Readable for EPOCHSELCRrs {}
///`write(|w| ..)` method takes [`epochselcr::W`](W) writer structure
impl crate::Writable for EPOCHSELCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EPOCHSELCR to value 0
impl crate::Resettable for EPOCHSELCRrs {}
