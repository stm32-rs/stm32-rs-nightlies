///Register `PUCRB` reader
pub type R = crate::R<PUCRBrs>;
///Register `PUCRB` writer
pub type W = crate::W<PUCRBrs>;
/**Field `PU6` reader - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
I/O. On STM32C011xx, only PU7 and PU6 are available*/
pub type PU6_R = crate::BitReader;
/**Field `PU6` writer - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
I/O. On STM32C011xx, only PU7 and PU6 are available*/
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU7` reader - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
I/O. On STM32C011xx, only PU7 and PU6 are available*/
pub type PU7_R = crate::BitReader;
/**Field `PU7` writer - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
I/O. On STM32C011xx, only PU7 and PU6 are available*/
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 6 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
    I/O. On STM32C011xx, only PU7 and PU6 are available*/
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    /**Bit 7 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
    I/O. On STM32C011xx, only PU7 and PU6 are available*/
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRB")
            .field("pu6", &self.pu6())
            .field("pu7", &self.pu7())
            .finish()
    }
}
impl W {
    /**Bit 6 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
    I/O. On STM32C011xx, only PU7 and PU6 are available*/
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<PUCRBrs> {
        PU6_W::new(self, 6)
    }
    /**Bit 7 - Port B pull-up bit i (i = 15 to 0) Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PB\[i\]
    I/O. On STM32C011xx, only PU7 and PU6 are available*/
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<PUCRBrs> {
        PU7_W::new(self, 7)
    }
}
/**PWR Port B pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#PWR:PUCRB)*/
pub struct PUCRBrs;
impl crate::RegisterSpec for PUCRBrs {
    type Ux = u32;
}
///`read()` method returns [`pucrb::R`](R) reader structure
impl crate::Readable for PUCRBrs {}
///`write(|w| ..)` method takes [`pucrb::W`](W) writer structure
impl crate::Writable for PUCRBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PUCRB to value 0
impl crate::Resettable for PUCRBrs {
    const RESET_VALUE: u32 = 0;
}
