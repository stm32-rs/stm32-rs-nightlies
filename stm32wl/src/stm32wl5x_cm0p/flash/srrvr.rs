#[doc = "Register `SRRVR` reader"]
pub type R = crate::R<SRRVRrs>;
#[doc = "Register `SRRVR` writer"]
pub type W = crate::W<SRRVRrs>;
#[doc = "Field `SBRV` reader - CPU2 boot reset vector"]
pub type SBRV_R = crate::FieldReader<u16>;
#[doc = "Field `SBRV` writer - CPU2 boot reset vector"]
pub type SBRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
#[doc = "Field `SBRSA` reader - Secure backup SRAM2 start address"]
pub type SBRSA_R = crate::FieldReader;
#[doc = "Field `SBRSA` writer - Secure backup SRAM2 start address"]
pub type SBRSA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "backup SRAM2 security disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRSD {
    #[doc = "0: SRAM2 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area"]
    Secure = 0,
    #[doc = "1: SRAM2 is non-secure"]
    NonSecure = 1,
}
impl From<BRSD> for bool {
    #[inline(always)]
    fn from(variant: BRSD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSD` reader - backup SRAM2 security disable"]
pub type BRSD_R = crate::BitReader<BRSD>;
impl BRSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRSD {
        match self.bits {
            false => BRSD::Secure,
            true => BRSD::NonSecure,
        }
    }
    #[doc = "SRAM2 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == BRSD::Secure
    }
    #[doc = "SRAM2 is non-secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == BRSD::NonSecure
    }
}
#[doc = "Field `BRSD` writer - backup SRAM2 security disable"]
pub type BRSD_W<'a, REG> = crate::BitWriter<'a, REG, BRSD>;
impl<'a, REG> BRSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM2 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure backup SRAM2 area"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(BRSD::Secure)
    }
    #[doc = "SRAM2 is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(BRSD::NonSecure)
    }
}
#[doc = "Field `SNBRSA` reader - Secure non-backup SRAM1 start address"]
pub type SNBRSA_R = crate::FieldReader;
#[doc = "Field `SNBRSA` writer - Secure non-backup SRAM1 start address"]
pub type SNBRSA_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "NBRSD\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NBRSD {
    #[doc = "0: SRAM1 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area"]
    Secure = 0,
    #[doc = "1: SRAM1 is non-secure"]
    NonSecure = 1,
}
impl From<NBRSD> for bool {
    #[inline(always)]
    fn from(variant: NBRSD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NBRSD` reader - NBRSD"]
pub type NBRSD_R = crate::BitReader<NBRSD>;
impl NBRSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NBRSD {
        match self.bits {
            false => NBRSD::Secure,
            true => NBRSD::NonSecure,
        }
    }
    #[doc = "SRAM1 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == NBRSD::Secure
    }
    #[doc = "SRAM1 is non-secure"]
    #[inline(always)]
    pub fn is_non_secure(&self) -> bool {
        *self == NBRSD::NonSecure
    }
}
#[doc = "Field `NBRSD` writer - NBRSD"]
pub type NBRSD_W<'a, REG> = crate::BitWriter<'a, REG, NBRSD>;
impl<'a, REG> NBRSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRAM1 is secure. SNBRSA\\[4:0\\]
contains the start address of the first 1-Kbyte page of the secure non-backup SRAM1 area"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(NBRSD::Secure)
    }
    #[doc = "SRAM1 is non-secure"]
    #[inline(always)]
    pub fn non_secure(self) -> &'a mut crate::W<REG> {
        self.variant(NBRSD::NonSecure)
    }
}
#[doc = "C2OPT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2OPT {
    #[doc = "0: SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV"]
    Sram = 0,
    #[doc = "1: SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV"]
    Flash = 1,
}
impl From<C2OPT> for bool {
    #[inline(always)]
    fn from(variant: C2OPT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C2OPT` reader - C2OPT"]
pub type C2OPT_R = crate::BitReader<C2OPT>;
impl C2OPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C2OPT {
        match self.bits {
            false => C2OPT::Sram,
            true => C2OPT::Flash,
        }
    }
    #[doc = "SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == C2OPT::Sram
    }
    #[doc = "SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == C2OPT::Flash
    }
}
#[doc = "Field `C2OPT` writer - C2OPT"]
pub type C2OPT_W<'a, REG> = crate::BitWriter<'a, REG, C2OPT>;
impl<'a, REG> C2OPT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SBRV offset addresses SRAM1 or SRAM2, from start address 0x2000_0000 + SBRV"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut crate::W<REG> {
        self.variant(C2OPT::Sram)
    }
    #[doc = "SBRV offset addresses the Flash memory, from start address 0x0800_0000 + SBRV"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut crate::W<REG> {
        self.variant(C2OPT::Flash)
    }
}
impl R {
    #[doc = "Bits 0:15 - CPU2 boot reset vector"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2 start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2 security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Secure non-backup SRAM1 start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - NBRSD"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - C2OPT"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU2 boot reset vector"]
    #[inline(always)]
    #[must_use]
    pub fn sbrv(&mut self) -> SBRV_W<SRRVRrs> {
        SBRV_W::new(self, 0)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2 start address"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsa(&mut self) -> SBRSA_W<SRRVRrs> {
        SBRSA_W::new(self, 18)
    }
    #[doc = "Bit 23 - backup SRAM2 security disable"]
    #[inline(always)]
    #[must_use]
    pub fn brsd(&mut self) -> BRSD_W<SRRVRrs> {
        BRSD_W::new(self, 23)
    }
    #[doc = "Bits 25:29 - Secure non-backup SRAM1 start address"]
    #[inline(always)]
    #[must_use]
    pub fn snbrsa(&mut self) -> SNBRSA_W<SRRVRrs> {
        SNBRSA_W::new(self, 25)
    }
    #[doc = "Bit 30 - NBRSD"]
    #[inline(always)]
    #[must_use]
    pub fn nbrsd(&mut self) -> NBRSD_W<SRRVRrs> {
        NBRSD_W::new(self, 30)
    }
    #[doc = "Bit 31 - C2OPT"]
    #[inline(always)]
    #[must_use]
    pub fn c2opt(&mut self) -> C2OPT_W<SRRVRrs> {
        C2OPT_W::new(self, 31)
    }
}
#[doc = "Flash secure SRAM start address and CPU2 reset vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srrvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srrvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRRVRrs;
impl crate::RegisterSpec for SRRVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srrvr::R`](R) reader structure"]
impl crate::Readable for SRRVRrs {}
#[doc = "`write(|w| ..)` method takes [`srrvr::W`](W) writer structure"]
impl crate::Writable for SRRVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRRVR to value 0xffff_8000"]
impl crate::Resettable for SRRVRrs {
    const RESET_VALUE: u32 = 0xffff_8000;
}
