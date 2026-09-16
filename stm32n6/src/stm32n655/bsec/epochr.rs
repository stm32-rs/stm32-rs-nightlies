///Register `EPOCHR%s` reader
pub type R = crate::R<EPOCHRrs>;
///Register `EPOCHR%s` writer
pub type W = crate::W<EPOCHRrs>;
///Field `EPOCH` reader - epoch
pub type EPOCH_R = crate::FieldReader<u32>;
///Field `EPOCH` writer - epoch
pub type EPOCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - epoch
    #[inline(always)]
    pub fn epoch(&self) -> EPOCH_R {
        EPOCH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPOCHR")
            .field("epoch", &self.epoch())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - epoch
    #[inline(always)]
    pub fn epoch(&mut self) -> EPOCH_W<'_, EPOCHRrs> {
        EPOCH_W::new(self, 0)
    }
}
/**BSEC epoch register

You can [`read`](crate::Reg::read) this register and get [`epochr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#BSEC:EPOCHR[0])*/
pub struct EPOCHRrs;
impl crate::RegisterSpec for EPOCHRrs {
    type Ux = u32;
}
///`read()` method returns [`epochr::R`](R) reader structure
impl crate::Readable for EPOCHRrs {}
///`write(|w| ..)` method takes [`epochr::W`](W) writer structure
impl crate::Writable for EPOCHRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EPOCHR%s to value 0
impl crate::Resettable for EPOCHRrs {}
