///Register `CFGR2` reader
pub type R = crate::R<CFGR2rs>;
///Register `CFGR2` writer
pub type W = crate::W<CFGR2rs>;
///Field `CCL` reader - Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.
pub type CCL_R = crate::BitReader;
///Field `CCL` writer - Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.
pub type CCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPL` reader - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.
pub type SPL_R = crate::BitReader;
///Field `SPL` writer - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.
pub type SPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PVDL` reader - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\[2:0\] in the PWR_CR register.
pub type PVDL_R = crate::BitReader;
///Field `PVDL` writer - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\[2:0\] in the PWR_CR register.
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCL` reader - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.
pub type ECCL_R = crate::BitReader;
///Field `ECCL` writer - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.
pub type ECCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPL` reader - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.
pub type BKPL_R = crate::BitReader;
///Field `BKPL` writer - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.
pub type BKPL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPF` reader - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.
pub type BKPF_R = crate::BitReader;
///Field `BKPF` writer - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.
pub type BKPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPF` reader - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.
pub type SPF_R = crate::BitReader;
///Field `SPF` writer - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.
pub type SPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn ccl(&self) -> CCL_R {
        CCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\[2:0\] in the PWR_CR register.
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn eccl(&self) -> ECCL_R {
        ECCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn bkpl(&self) -> BKPL_R {
        BKPL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.
    #[inline(always)]
    pub fn bkpf(&self) -> BKPF_R {
        BKPF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.
    #[inline(always)]
    pub fn spf(&self) -> SPF_R {
        SPF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR2")
            .field("ccl", &self.ccl())
            .field("spl", &self.spl())
            .field("pvdl", &self.pvdl())
            .field("eccl", &self.eccl())
            .field("bkpl", &self.bkpl())
            .field("bkpf", &self.bkpf())
            .field("spf", &self.spf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn ccl(&mut self) -> CCL_W<'_, CFGR2rs> {
        CCL_W::new(self, 0)
    }
    ///Bit 1 - SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<'_, CFGR2rs> {
        SPL_W::new(self, 1)
    }
    ///Bit 2 - PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\[2:0\] in the PWR_CR register.
    #[inline(always)]
    pub fn pvdl(&mut self) -> PVDL_W<'_, CFGR2rs> {
        PVDL_W::new(self, 2)
    }
    ///Bit 3 - ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn eccl(&mut self) -> ECCL_W<'_, CFGR2rs> {
        ECCL_W::new(self, 3)
    }
    ///Bit 4 - Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input.
    #[inline(always)]
    pub fn bkpl(&mut self) -> BKPL_W<'_, CFGR2rs> {
        BKPL_W::new(self, 4)
    }
    ///Bit 7 - Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1.
    #[inline(always)]
    pub fn bkpf(&mut self) -> BKPF_W<'_, CFGR2rs> {
        BKPF_W::new(self, 7)
    }
    ///Bit 8 - SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1.
    #[inline(always)]
    pub fn spf(&mut self) -> SPF_W<'_, CFGR2rs> {
        SPF_W::new(self, 8)
    }
}
/**SYSCFG configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:CFGR2)*/
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr2::R`](R) reader structure
impl crate::Readable for CFGR2rs {}
///`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2rs {}
