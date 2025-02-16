///Register `PUCRC` reader
pub type R = crate::R<PUCRCrs>;
///Register `PUCRC` writer
pub type W = crate::W<PUCRCrs>;
/**Field `PU14` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
I/O. On STM32C011xx, only PU15 and PU14 are available*/
pub type PU14_R = crate::BitReader;
/**Field `PU14` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
I/O. On STM32C011xx, only PU15 and PU14 are available*/
pub type PU14_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU15` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
I/O. On STM32C011xx, only PU15 and PU14 are available*/
pub type PU15_R = crate::BitReader;
/**Field `PU15` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
I/O. On STM32C011xx, only PU15 and PU14 are available*/
pub type PU15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    I/O. On STM32C011xx, only PU15 and PU14 are available*/
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    /**Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    I/O. On STM32C011xx, only PU15 and PU14 are available*/
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRC")
            .field("pu14", &self.pu14())
            .field("pu15", &self.pu15())
            .finish()
    }
}
impl W {
    /**Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    I/O. On STM32C011xx, only PU15 and PU14 are available*/
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W<PUCRCrs> {
        PU14_W::new(self, 14)
    }
    /**Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\]
    I/O. On STM32C011xx, only PU15 and PU14 are available*/
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<PUCRCrs> {
        PU15_W::new(self, 15)
    }
}
/**PWR Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#PWR:PUCRC)*/
pub struct PUCRCrs;
impl crate::RegisterSpec for PUCRCrs {
    type Ux = u32;
}
///`read()` method returns [`pucrc::R`](R) reader structure
impl crate::Readable for PUCRCrs {}
///`write(|w| ..)` method takes [`pucrc::W`](W) writer structure
impl crate::Writable for PUCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PUCRC to value 0
impl crate::Resettable for PUCRCrs {
    const RESET_VALUE: u32 = 0;
}
