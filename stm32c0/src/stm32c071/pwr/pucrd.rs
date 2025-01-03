///Register `PUCRD` reader
pub type R = crate::R<PUCRDrs>;
///Register `PUCRD` writer
pub type W = crate::W<PUCRDrs>;
/**Field `PU0` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU0_R = crate::BitReader;
/**Field `PU0` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU1` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU1_R = crate::BitReader;
/**Field `PU1` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU2` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU2_R = crate::BitReader;
/**Field `PU2` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU3` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU3_R = crate::BitReader;
/**Field `PU3` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU4` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU4_R = crate::BitReader;
/**Field `PU4` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU5` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU5_R = crate::BitReader;
/**Field `PU5` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU5_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU6` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU6_R = crate::BitReader;
/**Field `PU6` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU8` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU8_R = crate::BitReader;
/**Field `PU8` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU8_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PU9` reader - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU9_R = crate::BitReader;
/**Field `PU9` writer - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
pub type PU9_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 0 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    /**Bit 3 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    /**Bit 4 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    /**Bit 5 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    /**Bit 6 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    /**Bit 8 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    /**Bit 9 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRD")
            .field("pu0", &self.pu0())
            .field("pu1", &self.pu1())
            .field("pu2", &self.pu2())
            .field("pu3", &self.pu3())
            .field("pu4", &self.pu4())
            .field("pu5", &self.pu5())
            .field("pu6", &self.pu6())
            .field("pu8", &self.pu8())
            .field("pu9", &self.pu9())
            .finish()
    }
}
impl W {
    /**Bit 0 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<PUCRDrs> {
        PU0_W::new(self, 0)
    }
    /**Bit 1 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<PUCRDrs> {
        PU1_W::new(self, 1)
    }
    /**Bit 2 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<PUCRDrs> {
        PU2_W::new(self, 2)
    }
    /**Bit 3 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<PUCRDrs> {
        PU3_W::new(self, 3)
    }
    /**Bit 4 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<PUCRDrs> {
        PU4_W::new(self, 4)
    }
    /**Bit 5 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<PUCRDrs> {
        PU5_W::new(self, 5)
    }
    /**Bit 6 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Not available on STM32C011xx. On STM32C031xx, only PU3 to PU0 are available. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<PUCRDrs> {
        PU6_W::new(self, 6)
    }
    /**Bit 8 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<PUCRDrs> {
        PU8_W::new(self, 8)
    }
    /**Bit 9 - Port D pull-up bit i Setting PUi bit while the corresponding PDi bit is zero and the APC bit of the PWR_CR3 register is set activates a pull-up device on the PD\[i\]
    I/O. Only available on STM32C071xx. Note: For the same pin, this pull-up device must not be activated when a pull-down device is set through the GPIOx_PUPDR register.*/
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<PUCRDrs> {
        PU9_W::new(self, 9)
    }
}
/**PWR Port D pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#PWR:PUCRD)*/
pub struct PUCRDrs;
impl crate::RegisterSpec for PUCRDrs {
    type Ux = u32;
}
///`read()` method returns [`pucrd::R`](R) reader structure
impl crate::Readable for PUCRDrs {}
///`write(|w| ..)` method takes [`pucrd::W`](W) writer structure
impl crate::Writable for PUCRDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PUCRD to value 0
impl crate::Resettable for PUCRDrs {
    const RESET_VALUE: u32 = 0;
}
