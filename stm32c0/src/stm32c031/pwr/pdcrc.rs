///Register `PDCRC` reader
pub type R = crate::R<PDCRCrs>;
///Register `PDCRC` writer
pub type W = crate::W<PDCRCrs>;
/**Field `PD6` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD6_R = crate::BitReader;
/**Field `PD6` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD7` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD7_R = crate::BitReader;
/**Field `PD7` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD13` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD13_R = crate::BitReader;
/**Field `PD13` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD13_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD14` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD14_R = crate::BitReader;
/**Field `PD14` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD14_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `PD15` reader - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD15_R = crate::BitReader;
/**Field `PD15` writer - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
I/O. On STM32C011xx, only PD15 and PD14 are available.*/
pub type PD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /**Bit 6 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    /**Bit 7 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    /**Bit 13 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    /**Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    /**Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRC")
            .field("pd6", &self.pd6())
            .field("pd7", &self.pd7())
            .field("pd13", &self.pd13())
            .field("pd14", &self.pd14())
            .field("pd15", &self.pd15())
            .finish()
    }
}
impl W {
    /**Bit 6 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd6(&mut self) -> PD6_W<PDCRCrs> {
        PD6_W::new(self, 6)
    }
    /**Bit 7 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd7(&mut self) -> PD7_W<PDCRCrs> {
        PD7_W::new(self, 7)
    }
    /**Bit 13 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd13(&mut self) -> PD13_W<PDCRCrs> {
        PD13_W::new(self, 13)
    }
    /**Bit 14 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd14(&mut self) -> PD14_W<PDCRCrs> {
        PD14_W::new(self, 14)
    }
    /**Bit 15 - Port C pull-down bit i (i = 15, 14, 13, 7, 6) Setting PDi bit while the APC bit of the PWR_CR3 register is set activates a pull-down device on the PC\[i\]
    I/O. On STM32C011xx, only PD15 and PD14 are available.*/
    #[inline(always)]
    pub fn pd15(&mut self) -> PD15_W<PDCRCrs> {
        PD15_W::new(self, 15)
    }
}
/**PWR Port C pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#PWR:PDCRC)*/
pub struct PDCRCrs;
impl crate::RegisterSpec for PDCRCrs {
    type Ux = u32;
}
///`read()` method returns [`pdcrc::R`](R) reader structure
impl crate::Readable for PDCRCrs {}
///`write(|w| ..)` method takes [`pdcrc::W`](W) writer structure
impl crate::Writable for PDCRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PDCRC to value 0
impl crate::Resettable for PDCRCrs {
    const RESET_VALUE: u32 = 0;
}
