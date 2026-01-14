///Register `EPOCHSRP` reader
pub type R = crate::R<EPOCHSRPrs>;
///Register `EPOCHSRP` writer
pub type W = crate::W<EPOCHSRPrs>;
///Field `EPOCH` reader - Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register.
pub type EPOCH_R = crate::FieldReader<u32>;
///Field `EPOCH` writer - Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register.
pub type EPOCH_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register.
    #[inline(always)]
    pub fn epoch(&self) -> EPOCH_R {
        EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPOCHSRP")
            .field("epoch", &self.epoch())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - Epoch programming Write to change corresponding bits in FLASH_EPOCHSR register.
    #[inline(always)]
    pub fn epoch(&mut self) -> EPOCH_W<'_, EPOCHSRPrs> {
        EPOCH_W::new(self, 0)
    }
}
/**FLASH RoT status register programming

You can [`read`](crate::Reg::read) this register and get [`epochsrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochsrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:EPOCHSRP)*/
pub struct EPOCHSRPrs;
impl crate::RegisterSpec for EPOCHSRPrs {
    type Ux = u32;
}
///`read()` method returns [`epochsrp::R`](R) reader structure
impl crate::Readable for EPOCHSRPrs {}
///`write(|w| ..)` method takes [`epochsrp::W`](W) writer structure
impl crate::Writable for EPOCHSRPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EPOCHSRP to value 0
impl crate::Resettable for EPOCHSRPrs {}
