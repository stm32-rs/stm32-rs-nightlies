#[doc = "Register `FLASH_NSCR` reader"]
pub type R = crate::R<FLASH_NSCRrs>;
#[doc = "Register `FLASH_NSCR` writer"]
pub type W = crate::W<FLASH_NSCRrs>;
#[doc = "Field `PG` reader - Non-secure programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Non-secure programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PER` reader - Non-secure page erase"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Non-secure page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER1` reader - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
pub type MER1_R = crate::BitReader;
#[doc = "Field `MER1` writer - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNB` reader - Non-secure page number selection These bits select the page to erase. ..."]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Non-secure page number selection These bits select the page to erase. ..."]
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BKER` reader - Non-secure bank selection for page erase"]
pub type BKER_R = crate::BitReader;
#[doc = "Field `BKER` writer - Non-secure bank selection for page erase"]
pub type BKER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWR` reader - Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
pub type BWR_R = crate::BitReader;
#[doc = "Field `BWR` writer - Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
pub type BWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER2` reader - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
pub type MER2_R = crate::BitReader;
#[doc = "Field `MER2` writer - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
pub type MER2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTRT` reader - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
pub type OPTSTRT_R = crate::BitReader;
#[doc = "Field `OPTSTRT` writer - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE` reader - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBL_LAUNCH` reader - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
pub type OBL_LAUNCH_R = crate::BitReader;
#[doc = "Field `OBL_LAUNCH` writer - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTLOCK` reader - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
pub type OPTLOCK_R = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-secure programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-secure page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Non-secure page number selection These bits select the page to erase. ..."]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 11 - Non-secure bank selection for page erase"]
    #[inline(always)]
    pub fn bker(&self) -> BKER_R {
        BKER_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
    #[inline(always)]
    pub fn mer2(&self) -> MER2_R {
        MER2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-secure programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<FLASH_NSCRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Non-secure page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<FLASH_NSCRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
    #[inline(always)]
    #[must_use]
    pub fn mer1(&mut self) -> MER1_W<FLASH_NSCRrs> {
        MER1_W::new(self, 2)
    }
    #[doc = "Bits 3:9 - Non-secure page number selection These bits select the page to erase. ..."]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<FLASH_NSCRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 11 - Non-secure bank selection for page erase"]
    #[inline(always)]
    #[must_use]
    pub fn bker(&mut self) -> BKER_W<FLASH_NSCRrs> {
        BKER_W::new(self, 11)
    }
    #[doc = "Bit 14 - Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
    #[inline(always)]
    #[must_use]
    pub fn bwr(&mut self) -> BWR_W<FLASH_NSCRrs> {
        BWR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
    #[inline(always)]
    #[must_use]
    pub fn mer2(&mut self) -> MER2_W<FLASH_NSCRrs> {
        MER2_W::new(self, 15)
    }
    #[doc = "Bit 16 - Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<FLASH_NSCRrs> {
        STRT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<FLASH_NSCRrs> {
        OPTSTRT_W::new(self, 17)
    }
    #[doc = "Bit 24 - Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<FLASH_NSCRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<FLASH_NSCRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 27 - Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
    #[inline(always)]
    #[must_use]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<FLASH_NSCRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    #[doc = "Bit 30 - Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<FLASH_NSCRrs> {
        OPTLOCK_W::new(self, 30)
    }
    #[doc = "Bit 31 - Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<FLASH_NSCRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "FLASH non-secure control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_nscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_nscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_NSCRrs;
impl crate::RegisterSpec for FLASH_NSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_nscr::R`](R) reader structure"]
impl crate::Readable for FLASH_NSCRrs {}
#[doc = "`write(|w| ..)` method takes [`flash_nscr::W`](W) writer structure"]
impl crate::Writable for FLASH_NSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_NSCR to value 0xc000_0000"]
impl crate::Resettable for FLASH_NSCRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
