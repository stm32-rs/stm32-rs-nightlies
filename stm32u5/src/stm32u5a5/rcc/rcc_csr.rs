///Register `RCC_CSR` reader
pub type R = crate::R<RCC_CSRrs>;
///Register `RCC_CSR` writer
pub type W = crate::W<RCC_CSRrs>;
///Field `MSIKSRANGE` reader - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
pub type MSIKSRANGE_R = crate::FieldReader;
///Field `MSIKSRANGE` writer - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
pub type MSIKSRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MSISSRANGE` reader - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
pub type MSISSRANGE_R = crate::FieldReader;
///Field `MSISSRANGE` writer - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
pub type MSISSRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RMVF` reader - Remove reset flag This bit is set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader;
///Field `RMVF` writer - Remove reset flag This bit is set by software to clear the reset flags.
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBLRSTF` reader - Option-byte loader reset flag This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared by�writing to the RMVF bit.
pub type OBLRSTF_R = crate::BitReader;
///Field `PINRSTF` reader - NRST pin reset flag This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing to�the RMVF bit.
pub type PINRSTF_R = crate::BitReader;
///Field `BORRSTF` reader - Brownout reset or an exit from Shutdown mode reset flag This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset occurs. It is cleared by writing to the RMVF bit.
pub type BORRSTF_R = crate::BitReader;
///Field `SFTRSTF` reader - Software reset flag This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.
pub type SFTRSTF_R = crate::BitReader;
///Field `IWDGRSTF` reader - Independent watchdog reset flag This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared by writing to the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader;
///Field `WWDGRSTF` reader - Window watchdog reset flag This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing to�the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader;
///Field `LPWRRSTF` reader - Low-power reset flag This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is cleared. This bit is cleared by writing to the RMVF bit.
pub type LPWRRSTF_R = crate::BitReader;
impl R {
    ///Bits 8:11 - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
    #[inline(always)]
    pub fn msiksrange(&self) -> MSIKSRANGE_R {
        MSIKSRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
    #[inline(always)]
    pub fn msissrange(&self) -> MSISSRANGE_R {
        MSISSRANGE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 23 - Remove reset flag This bit is set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option-byte loader reset flag This bit is set by hardware when a reset from the option-byte loading occurs. It is cleared by�writing to the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - NRST pin reset flag This bit is set by hardware when a reset from the NRST pin occurs. It is cleared by writing to�the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Brownout reset or an exit from Shutdown mode reset flag This bit is set by hardware when a brownout reset or an exit from Shutdown mode reset occurs. It is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag This bit is set by hardware when a software reset occurs. It is cleared by writing to RMVF.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag This bit is set by hardware when an independent watchdog reset domain occurs. It is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag This bit is set by hardware when a window watchdog reset occurs. It is cleared by writing to�the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag This bit is set by hardware when a reset occurs due to a Stop, Standby, or Shutdown mode entry, whereas the corresponding NRST_STOP, NRST_STBY, or NRST_SHDW option bit is cleared. This bit is cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSR")
            .field("msiksrange", &self.msiksrange())
            .field("msissrange", &self.msissrange())
            .field("rmvf", &self.rmvf())
            .field("oblrstf", &self.oblrstf())
            .field("pinrstf", &self.pinrstf())
            .field("borrstf", &self.borrstf())
            .field("sftrstf", &self.sftrstf())
            .field("iwdgrstf", &self.iwdgrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("lpwrrstf", &self.lpwrrstf())
            .finish()
    }
}
impl W {
    ///Bits 8:11 - MSIK range after Standby mode This bit is set by software to chose the MSIK frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIK frequency.
    #[inline(always)]
    #[must_use]
    pub fn msiksrange(&mut self) -> MSIKSRANGE_W<RCC_CSRrs> {
        MSIKSRANGE_W::new(self, 8)
    }
    ///Bits 12:15 - MSIS range after Standby mode This bitfield is set by software to chose the MSIS frequency at startup. It is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4�MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing this bitfield does not change the current MSIS frequency.
    #[inline(always)]
    #[must_use]
    pub fn msissrange(&mut self) -> MSISSRANGE_W<RCC_CSRrs> {
        MSISSRANGE_W::new(self, 12)
    }
    ///Bit 23 - Remove reset flag This bit is set by software to clear the reset flags.
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<RCC_CSRrs> {
        RMVF_W::new(self, 23)
    }
}
/**RCC control/status register

You can [`read`](crate::Reg::read) this register and get [`rcc_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#RCC:RCC_CSR)*/
pub struct RCC_CSRrs;
impl crate::RegisterSpec for RCC_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_csr::R`](R) reader structure
impl crate::Readable for RCC_CSRrs {}
///`write(|w| ..)` method takes [`rcc_csr::W`](W) writer structure
impl crate::Writable for RCC_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CSR to value 0x0c00_4400
impl crate::Resettable for RCC_CSRrs {
    const RESET_VALUE: u32 = 0x0c00_4400;
}
