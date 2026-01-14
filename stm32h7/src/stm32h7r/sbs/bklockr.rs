///Register `BKLOCKR` reader
pub type R = crate::R<BKLOCKRrs>;
///Register `BKLOCKR` writer
pub type W = crate::W<BKLOCKRrs>;
///Field `PVD_BL` reader - PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\[2:0\] bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset.
pub type PVD_BL_R = crate::BitReader;
///Field `PVD_BL` writer - PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\[2:0\] bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset.
pub type PVD_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHECC_BL` reader - Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type FLASHECC_BL_R = crate::BitReader;
///Field `FLASHECC_BL` writer - Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type FLASHECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CM7LCKUP_BL` reader - Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type CM7LCKUP_BL_R = crate::BitReader;
///Field `CM7LCKUP_BL` writer - Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type CM7LCKUP_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKRAMECC_BL` reader - Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type BKRAMECC_BL_R = crate::BitReader;
///Field `BKRAMECC_BL` writer - Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type BKRAMECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTCMECC_BL` reader - DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC
pub type DTCMECC_BL_R = crate::BitReader;
///Field `DTCMECC_BL` writer - DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC
pub type DTCMECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCMECC_BL` reader - ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type ITCMECC_BL_R = crate::BitReader;
///Field `ITCMECC_BL` writer - ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type ITCMECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAM3ECC_BL` reader - AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset.
pub type ARAM3ECC_BL_R = crate::BitReader;
///Field `ARAM3ECC_BL` writer - AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset.
pub type ARAM3ECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAM1ECC_BL` reader - AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type ARAM1ECC_BL_R = crate::BitReader;
///Field `ARAM1ECC_BL` writer - AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
pub type ARAM1ECC_BL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\[2:0\] bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn pvd_bl(&self) -> PVD_BL_R {
        PVD_BL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn flashecc_bl(&self) -> FLASHECC_BL_R {
        FLASHECC_BL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn cm7lckup_bl(&self) -> CM7LCKUP_BL_R {
        CM7LCKUP_BL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn bkramecc_bl(&self) -> BKRAMECC_BL_R {
        BKRAMECC_BL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC
    #[inline(always)]
    pub fn dtcmecc_bl(&self) -> DTCMECC_BL_R {
        DTCMECC_BL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn itcmecc_bl(&self) -> ITCMECC_BL_R {
        ITCMECC_BL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 21 - AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn aram3ecc_bl(&self) -> ARAM3ECC_BL_R {
        ARAM3ECC_BL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn aram1ecc_bl(&self) -> ARAM1ECC_BL_R {
        ARAM1ECC_BL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKLOCKR")
            .field("pvd_bl", &self.pvd_bl())
            .field("flashecc_bl", &self.flashecc_bl())
            .field("cm7lckup_bl", &self.cm7lckup_bl())
            .field("bkramecc_bl", &self.bkramecc_bl())
            .field("dtcmecc_bl", &self.dtcmecc_bl())
            .field("itcmecc_bl", &self.itcmecc_bl())
            .field("aram3ecc_bl", &self.aram3ecc_bl())
            .field("aram1ecc_bl", &self.aram1ecc_bl())
            .finish()
    }
}
impl W {
    ///Bit 2 - PVD break lock This bit is set by SW and cleared only by a system reset. it can be used to enable and lock the connection to TIM1/8/15/16/17Break input as well as the PVDE and PLS\[2:0\] bitfields in the PWR_CR1 register. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn pvd_bl(&mut self) -> PVD_BL_W<'_, BKLOCKRrs> {
        PVD_BL_W::new(self, 2)
    }
    ///Bit 3 - Flash ECC error break lock Set this bit to enable and lock the connection between embedded flash memory ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn flashecc_bl(&mut self) -> FLASHECC_BL_W<'_, BKLOCKRrs> {
        FLASHECC_BL_W::new(self, 3)
    }
    ///Bit 6 - Cortex-M7 lockup break lock Set this bit to enable and lock the connection between the Cortex-M7 lockup (HardFault) output and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn cm7lckup_bl(&mut self) -> CM7LCKUP_BL_W<'_, BKLOCKRrs> {
        CM7LCKUP_BL_W::new(self, 6)
    }
    ///Bit 7 - Backup RAM ECC error break lock Set this bit to enable and lock the connection between backup RAM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn bkramecc_bl(&mut self) -> BKRAMECC_BL_W<'_, BKLOCKRrs> {
        BKRAMECC_BL_W::new(self, 7)
    }
    ///Bit 13 - DTCM ECC error break lock Set this bit to enable and lock the connection between DTCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset. Note: The DTCM0 and DTCM1 are Ored to give DTCMECC
    #[inline(always)]
    pub fn dtcmecc_bl(&mut self) -> DTCMECC_BL_W<'_, BKLOCKRrs> {
        DTCMECC_BL_W::new(self, 13)
    }
    ///Bit 14 - ITCM ECC error break lock Set this bit to enable and lock the connection between ITCM ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn itcmecc_bl(&mut self) -> ITCMECC_BL_W<'_, BKLOCKRrs> {
        ITCMECC_BL_W::new(self, 14)
    }
    ///Bit 21 - AXIRAM3 ECC error break lock Set this bit to enable and lock the connection between AXIRAM3 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn aram3ecc_bl(&mut self) -> ARAM3ECC_BL_W<'_, BKLOCKRrs> {
        ARAM3ECC_BL_W::new(self, 21)
    }
    ///Bit 23 - AXIRAM1 ECC error break lock Set this bit to enable and lock the connection between AXIRAM1 ECC double error detection flag and break inputs of TIM1/15/16/17 peripherals. Once set, this bit is cleared only by a system reset.
    #[inline(always)]
    pub fn aram1ecc_bl(&mut self) -> ARAM1ECC_BL_W<'_, BKLOCKRrs> {
        ARAM1ECC_BL_W::new(self, 23)
    }
}
/**SBS break lockup register

You can [`read`](crate::Reg::read) this register and get [`bklockr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bklockr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SBS:BKLOCKR)*/
pub struct BKLOCKRrs;
impl crate::RegisterSpec for BKLOCKRrs {
    type Ux = u32;
}
///`read()` method returns [`bklockr::R`](R) reader structure
impl crate::Readable for BKLOCKRrs {}
///`write(|w| ..)` method takes [`bklockr::W`](W) writer structure
impl crate::Writable for BKLOCKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKLOCKR to value 0x88
impl crate::Resettable for BKLOCKRrs {
    const RESET_VALUE: u32 = 0x88;
}
