///Register `PUCRC` reader
pub type R = crate::R<PUCRCrs>;
///Register `PUCRC` writer
pub type W = crate::W<PUCRCrs>;
///Field `PU6` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU6_R = crate::BitReader;
///Field `PU6` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU7` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU7_R = crate::BitReader;
///Field `PU7` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU13` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU13_R = crate::BitReader;
///Field `PU13` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU14` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU14_R = crate::BitReader;
///Field `PU14` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU15` reader - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU15_R = crate::BitReader;
///Field `PU15` writer - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
pub type PU15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRC")
            .field("pu6", &self.pu6())
            .field("pu7", &self.pu7())
            .field("pu13", &self.pu13())
            .field("pu14", &self.pu14())
            .field("pu15", &self.pu15())
            .finish()
    }
}
impl W {
    ///Bit 6 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<PUCRCrs> {
        PU6_W::new(self, 6)
    }
    ///Bit 7 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<PUCRCrs> {
        PU7_W::new(self, 7)
    }
    ///Bit 13 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<PUCRCrs> {
        PU13_W::new(self, 13)
    }
    ///Bit 14 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W<PUCRCrs> {
        PU14_W::new(self, 14)
    }
    ///Bit 15 - Port C pull-up bit i (i = 15 to 13, 7 to 6) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PC\[i\] I/O. On STM32C011xx, only PU15 and PU14 are available
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<PUCRCrs> {
        PU15_W::new(self, 15)
    }
}
/**PWR Port C pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PUCRC)*/
pub struct PUCRCrs;
impl crate::RegisterSpec for PUCRCrs {
    type Ux = u32;
}
///`read()` method returns [`pucrc::R`](R) reader structure
impl crate::Readable for PUCRCrs {}
///`write(|w| ..)` method takes [`pucrc::W`](W) writer structure
impl crate::Writable for PUCRCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRC to value 0
impl crate::Resettable for PUCRCrs {}
