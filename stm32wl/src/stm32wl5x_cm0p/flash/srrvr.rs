///Register `SRRVR` reader
pub type R = crate::R<SRRVRrs>;
///Register `SRRVR` writer
pub type W = crate::W<SRRVRrs>;
///Field `SBRV` reader - CPU2 boot reset vector
pub type SBRV_R = crate::FieldReader<u16>;
///Field `SBRV` writer - CPU2 boot reset vector
pub type SBRV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
///Field `SBRSA` reader - Secure backup SRAM2 start address
pub type SBRSA_R = crate::FieldReader;
///Field `SBRSA` writer - Secure backup SRAM2 start address
pub type SBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**backup SRAM2 security disable

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSD {
    ///0: SRAM2 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area
    Secure = 0,
    ///1: SRAM2 is non-secure
    NonSecure = 1,
}
impl From<BRSD> for bool {
    #[inline(always)]
    fn from(variant: BRSD) -> Self {
        variant as u8 != 0
    }
}
///Field `BRSD` reader - backup SRAM2 security disable
pub type BRSD_R = crate::BitReader<BRSD>;
impl BRSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRSD {
        match self.bits {
            false => BRSD::Secure,
            true => BRSD::NonSecure,
        }
    }
    ///SRAM2 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == BRSD::Secure
    }
    ///SRAM2 is non-secure
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == BRSD::NonSecure
    }
}
///Field `BRSD` writer - backup SRAM2 security disable
pub type BRSD_W<'a, REG> = crate::BitWriter<'a, REG, BRSD>;
impl<'a, REG> BRSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM2 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(BRSD::Secure)
    }
    ///SRAM2 is non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(BRSD::NonSecure)
    }
}
///Field `SNBRSA` reader - Secure non-backup SRAM1 start address
pub type SNBRSA_R = crate::FieldReader;
///Field `SNBRSA` writer - Secure non-backup SRAM1 start address
pub type SNBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**NBRSD

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBRSD {
    ///0: SRAM1 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area
    Secure = 0,
    ///1: SRAM1 is non-secure
    NonSecure = 1,
}
impl From<NBRSD> for bool {
    #[inline(always)]
    fn from(variant: NBRSD) -> Self {
        variant as u8 != 0
    }
}
///Field `NBRSD` reader - NBRSD
pub type NBRSD_R = crate::BitReader<NBRSD>;
impl NBRSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NBRSD {
        match self.bits {
            false => NBRSD::Secure,
            true => NBRSD::NonSecure,
        }
    }
    ///SRAM1 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == NBRSD::Secure
    }
    ///SRAM1 is non-secure
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == NBRSD::NonSecure
    }
}
///Field `NBRSD` writer - NBRSD
pub type NBRSD_W<'a, REG> = crate::BitWriter<'a, REG, NBRSD>;
impl<'a, REG> NBRSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SRAM1 is secure. SNBRSA\[4:0\] contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(NBRSD::Secure)
    }
    ///SRAM1 is non-secure
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(NBRSD::NonSecure)
    }
}
/**C2OPT

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2OPT {
    ///0: SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV
    Sram = 0,
    ///1: SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV
    Flash = 1,
}
impl From<C2OPT> for bool {
    #[inline(always)]
    fn from(variant: C2OPT) -> Self {
        variant as u8 != 0
    }
}
///Field `C2OPT` reader - C2OPT
pub type C2OPT_R = crate::BitReader<C2OPT>;
impl C2OPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> C2OPT {
        match self.bits {
            false => C2OPT::Sram,
            true => C2OPT::Flash,
        }
    }
    ///SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == C2OPT::Sram
    }
    ///SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == C2OPT::Flash
    }
}
///Field `C2OPT` writer - C2OPT
pub type C2OPT_W<'a, REG> = crate::BitWriter<'a, REG, C2OPT>;
impl<'a, REG> C2OPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(C2OPT::Sram)
    }
    ///SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(C2OPT::Flash)
    }
}
impl R {
    ///Bits 0:15 - CPU2 boot reset vector
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 18:22 - Secure backup SRAM2 start address
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - backup SRAM2 security disable
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:29 - Secure non-backup SRAM1 start address
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    ///Bit 30 - NBRSD
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - C2OPT
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRRVR")
            .field("sbrv", &self.sbrv())
            .field("sbrsa", &self.sbrsa())
            .field("brsd", &self.brsd())
            .field("snbrsa", &self.snbrsa())
            .field("nbrsd", &self.nbrsd())
            .field("c2opt", &self.c2opt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - CPU2 boot reset vector
    #[inline(always)]
    pub fn sbrv(&mut self) -> SBRV_W<'_, SRRVRrs> {
        SBRV_W::new(self, 0)
    }
    ///Bits 18:22 - Secure backup SRAM2 start address
    #[inline(always)]
    pub fn sbrsa(&mut self) -> SBRSA_W<'_, SRRVRrs> {
        SBRSA_W::new(self, 18)
    }
    ///Bit 23 - backup SRAM2 security disable
    #[inline(always)]
    pub fn brsd(&mut self) -> BRSD_W<'_, SRRVRrs> {
        BRSD_W::new(self, 23)
    }
    ///Bits 25:29 - Secure non-backup SRAM1 start address
    #[inline(always)]
    pub fn snbrsa(&mut self) -> SNBRSA_W<'_, SRRVRrs> {
        SNBRSA_W::new(self, 25)
    }
    ///Bit 30 - NBRSD
    #[inline(always)]
    pub fn nbrsd(&mut self) -> NBRSD_W<'_, SRRVRrs> {
        NBRSD_W::new(self, 30)
    }
    ///Bit 31 - C2OPT
    #[inline(always)]
    pub fn c2opt(&mut self) -> C2OPT_W<'_, SRRVRrs> {
        C2OPT_W::new(self, 31)
    }
}
/**Flash secure SRAM start address and CPU2 reset vector register

You can [`read`](crate::Reg::read) this register and get [`srrvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srrvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#FLASH:SRRVR)*/
pub struct SRRVRrs;
impl crate::RegisterSpec for SRRVRrs {
    type Ux = u32;
}
///`read()` method returns [`srrvr::R`](R) reader structure
impl crate::Readable for SRRVRrs {}
///`write(|w| ..)` method takes [`srrvr::W`](W) writer structure
impl crate::Writable for SRRVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRRVR to value 0xffff_8000
impl crate::Resettable for SRRVRrs {
    const RESET_VALUE: u32 = 0xffff_8000;
}
