///Register `PUCRF` reader
pub type R = crate::R<PUCRFrs>;
///Register `PUCRF` writer
pub type W = crate::W<PUCRFrs>;
/**Field `PU0` reader - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU0_R = crate::BitReader;
/**Field `PU0` writer - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU1` reader - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU1_R = crate::BitReader;
/**Field `PU1` writer - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU2` reader - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU2_R = crate::BitReader;
/**Field `PU2` writer - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU3` reader - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU3_R = crate::BitReader;
/**Field `PU3` writer - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    /**Bit 3 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRF")
            .field("pu0", &self.pu0())
            .field("pu1", &self.pu1())
            .field("pu2", &self.pu2())
            .field("pu3", &self.pu3())
            .finish()
    }
}
impl W {
    /**Bit 0 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<PUCRFrs> {
        PU0_W::new(self, 0)
    }
    /**Bit 1 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<PUCRFrs> {
        PU1_W::new(self, 1)
    }
    /**Bit 2 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<PUCRFrs> {
        PU2_W::new(self, 2)
    }
    /**Bit 3 - Port F pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PF\[i\]
    I/O. On STM32C011xx, only PU2 is available. On STM32C031xx, only PU2 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<PUCRFrs> {
        PU3_W::new(self, 3)
    }
}
/**PWR Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRF)*/
pub struct PUCRFrs;
impl crate::RegisterSpec for PUCRFrs {
    type Ux = u32;
}
///`read()` method returns [`pucrf::R`](R) reader structure
impl crate::Readable for PUCRFrs {}
///`write(|w| ..)` method takes [`pucrf::W`](W) writer structure
impl crate::Writable for PUCRFrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PUCRF to value 0
impl crate::Resettable for PUCRFrs {
    const RESET_VALUE: u32 = 0;
}
