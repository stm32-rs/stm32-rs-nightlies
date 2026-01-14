///Register `NSCR1` reader
pub type R = crate::R<NSCR1rs>;
///Register `NSCR1` writer
pub type W = crate::W<NSCR1rs>;
///Field `PG` reader - Non-secure programming
pub type PG_R = crate::BitReader;
///Field `PG` writer - Non-secure programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Non-secure page erase
pub type PER_R = crate::BitReader;
///Field `PER` writer - Non-secure page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER` reader - Non-secure Flash mass erase This bit triggers the Flash non-secure mass erase (all Flash user pages) when set.
pub type MER_R = crate::BitReader;
///Field `MER` writer - Non-secure Flash mass erase This bit triggers the Flash non-secure mass erase (all Flash user pages) when set.
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices.
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices.
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BWR` reader - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_R = crate::BitReader;
///Field `BWR` writer - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
pub type BWR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STRT` reader - Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBL_LAUNCH` reader - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH.
pub type OBL_LAUNCH_R = crate::BitReader;
///Field `OBL_LAUNCH` writer - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH.
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTLOCK` reader - Option lock This bit is set only. When set, the FLASH_NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_OPTKEYR. The FLASH_NSCR1.LOCK bit must be cleared before doing the FLASH_OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Option lock This bit is set only. When set, the FLASH_NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_OPTKEYR. The FLASH_NSCR1.LOCK bit must be cleared before doing the FLASH_OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Non-secure lock This bit is set only. When set, the FLASH_NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - Non-secure lock This bit is set only. When set, the FLASH_NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Non-secure Flash mass erase This bit triggers the Flash non-secure mass erase (all Flash user pages) when set.
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices.
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH.
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Option lock This bit is set only. When set, the FLASH_NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_OPTKEYR. The FLASH_NSCR1.LOCK bit must be cleared before doing the FLASH_OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSCR1")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("bwr", &self.bwr())
            .field("strt", &self.strt())
            .field("optstrt", &self.optstrt())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("obl_launch", &self.obl_launch())
            .field("optlock", &self.optlock())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Non-secure programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, NSCR1rs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Non-secure page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, NSCR1rs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Non-secure Flash mass erase This bit triggers the Flash non-secure mass erase (all Flash user pages) when set.
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, NSCR1rs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:9 - Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices.
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, NSCR1rs> {
        PNB_W::new(self, 3)
    }
    ///Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode.
    #[inline(always)]
    pub fn bwr(&mut self) -> BWR_W<'_, NSCR1rs> {
        BWR_W::new(self, 14)
    }
    ///Bit 16 - Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, NSCR1rs> {
        STRT_W::new(self, 16)
    }
    ///Bit 17 - Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR.
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<'_, NSCR1rs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, NSCR1rs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1.
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, NSCR1rs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH.
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<'_, NSCR1rs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 30 - Option lock This bit is set only. When set, the FLASH_NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_OPTKEYR. The FLASH_NSCR1.LOCK bit must be cleared before doing the FLASH_OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<'_, NSCR1rs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, NSCR1rs> {
        LOCK_W::new(self, 31)
    }
}
/**FLASH control register

You can [`read`](crate::Reg::read) this register and get [`nscr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nscr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#FLASH:NSCR1)*/
pub struct NSCR1rs;
impl crate::RegisterSpec for NSCR1rs {
    type Ux = u32;
}
///`read()` method returns [`nscr1::R`](R) reader structure
impl crate::Readable for NSCR1rs {}
///`write(|w| ..)` method takes [`nscr1::W`](W) writer structure
impl crate::Writable for NSCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSCR1 to value 0xc000_0000
impl crate::Resettable for NSCR1rs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
