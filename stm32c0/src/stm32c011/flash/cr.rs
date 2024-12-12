///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `PG` reader - Flash memory programming enable
pub type PG_R = crate::BitReader;
///Field `PG` writer - Flash memory programming enable
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PER` reader - Page erase enable
pub type PER_R = crate::BitReader;
///Field `PER` writer - Page erase enable
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MER1` reader - Mass erase When set, this bit triggers the mass erase, that is, all user pages.
pub type MER1_R = crate::BitReader;
///Field `MER1` writer - Mass erase When set, this bit triggers the mass erase, that is, all user pages.
pub type MER1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PNB` reader - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the Main memory are not allowed. See Table 6 and Table 7.
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the Main memory are not allowed. See Table 6 and Table 7.
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STRT` reader - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero.
pub type STRT_R = crate::BitReader;
///Field `STRT` writer - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero.
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTSTRT` reader - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR.
pub type OPTSTRT_R = crate::BitReader;
///Field `OPTSTRT` writer - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR.
pub type OPTSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSTPG` reader - Fast programming enable
pub type FSTPG_R = crate::BitReader;
///Field `FSTPG` writer - Fast programming enable
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPIE` reader - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register.
pub type EOPIE_R = crate::BitReader;
///Field `EOPIE` writer - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register.
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register.
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register.
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDERRIE` reader - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register.
pub type RDERRIE_R = crate::BitReader;
///Field `RDERRIE` writer - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register.
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OBL_LAUNCH` reader - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set.
pub type OBL_LAUNCH_R = crate::BitReader;
///Field `OBL_LAUNCH` writer - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set.
pub type OBL_LAUNCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC_PROT` reader - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\[4:0\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset.
pub type SEC_PROT_R = crate::BitReader;
///Field `SEC_PROT` writer - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\[4:0\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset.
pub type SEC_PROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OPTLOCK` reader - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_R = crate::BitReader;
///Field `OPTLOCK` writer - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
pub type OPTLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Flash memory programming enable
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page erase enable
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass erase When set, this bit triggers the mass erase, that is, all user pages.
    #[inline(always)]
    pub fn mer1(&self) -> MER1_R {
        MER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the Main memory are not allowed. See Table 6 and Table 7.
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    ///Bit 16 - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero.
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR.
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast programming enable
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register.
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register.
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register.
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set.
    #[inline(always)]
    pub fn obl_launch(&self) -> OBL_LAUNCH_R {
        OBL_LAUNCH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\[4:0\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset.
    #[inline(always)]
    pub fn sec_prot(&self) -> SEC_PROT_R {
        SEC_PROT_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer1", &self.mer1())
            .field("pnb", &self.pnb())
            .field("strt", &self.strt())
            .field("optstrt", &self.optstrt())
            .field("fstpg", &self.fstpg())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .field("obl_launch", &self.obl_launch())
            .field("sec_prot", &self.sec_prot())
            .field("optlock", &self.optlock())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flash memory programming enable
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page erase enable
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Mass erase When set, this bit triggers the mass erase, that is, all user pages.
    #[inline(always)]
    pub fn mer1(&mut self) -> MER1_W<CRrs> {
        MER1_W::new(self, 2)
    }
    ///Bits 3:6 - Page number selection These bits select the page to erase: ... Note: Values corresponding to addresses outside the Main memory are not allowed. See Table 6 and Table 7.
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 16 - Start erase operation This bit triggers an erase operation when set. This bit is possible to set only by software and to clear only by hardware. The hardware clears it when one of BSY1 and BSY2 flags in the FLASH_SR register transits to zero.
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<CRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 17 - Start of modification of option bytes This bit triggers an options operation when set. This bit is set only by software, and is cleared when the BSY1 bit is cleared in FLASH_SR.
    #[inline(always)]
    pub fn optstrt(&mut self) -> OPTSTRT_W<CRrs> {
        OPTSTRT_W::new(self, 17)
    }
    ///Bit 18 - Fast programming enable
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<CRrs> {
        FSTPG_W::new(self, 18)
    }
    ///Bit 24 - End-of-operation interrupt enable This bit enables the interrupt generation upon setting the EOP flag in the FLASH_SR register.
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<CRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Error interrupt enable This bit enables the interrupt generation upon setting the OPERR flag in the FLASH_SR register.
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<CRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - PCROP read error interrupt enable This bit enables the interrupt generation upon setting the RDERR flag in the FLASH_SR register.
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<CRrs> {
        RDERRIE_W::new(self, 26)
    }
    ///Bit 27 - Option byte load launch When set, this bit triggers the load of option bytes into option registers. It is automatically cleared upon the completion of the load. The high state of the bit indicates pending option byte load. The bit cannot be cleared by software. It cannot be written as long as OPTLOCK is set.
    #[inline(always)]
    pub fn obl_launch(&mut self) -> OBL_LAUNCH_W<CRrs> {
        OBL_LAUNCH_W::new(self, 27)
    }
    ///Bit 28 - Securable memory area protection enable This bit enables the protection on securable area, provided that a non-null securable memory area size (SEC_SIZE\[4:0\]) is defined in option bytes. This bit is possible to set only by software and to clear only through a system reset.
    #[inline(always)]
    pub fn sec_prot(&mut self) -> SEC_PROT_W<CRrs> {
        SEC_PROT_W::new(self, 28)
    }
    ///Bit 30 - Options Lock This bit is set only. When set, all bits concerning user option in FLASH_CR register and so option page are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset.
    #[inline(always)]
    pub fn optlock(&mut self) -> OPTLOCK_W<CRrs> {
        OPTLOCK_W::new(self, 30)
    }
    ///Bit 31 - FLASH_CR Lock This bit is set only. When set, the FLASH_CR register is locked. It is cleared by hardware after detecting the unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next system reset.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<CRrs> {
        LOCK_W::new(self, 31)
    }
}
/**FLASH control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#FLASH:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CR to value 0xc000_0000
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
