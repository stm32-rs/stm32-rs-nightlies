#[doc = "Register `SRRVR` reader"]
pub type R = crate::R<SRRVRrs>;
#[doc = "Register `SRRVR` writer"]
pub type W = crate::W<SRRVRrs>;
#[doc = "Field `SBRV` reader - cortex M0 access control register"]
pub type SBRV_R = crate::FieldReader<u32>;
#[doc = "Field `SBRV` writer - cortex M0 access control register"]
pub type SBRV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `SBRSA` reader - Secure backup SRAM2a start address"]
pub type SBRSA_R = crate::FieldReader;
#[doc = "Field `SBRSA` writer - Secure backup SRAM2a start address"]
pub type SBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BRSD` reader - backup SRAM2a security disable"]
pub type BRSD_R = crate::BitReader;
#[doc = "Field `BRSD` writer - backup SRAM2a security disable"]
pub type BRSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNBRSA` reader - Secure non backup SRAM2a start address"]
pub type SNBRSA_R = crate::FieldReader;
#[doc = "Field `SNBRSA` writer - Secure non backup SRAM2a start address"]
pub type SNBRSA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NBRSD` reader - non-backup SRAM2b security disable"]
pub type NBRSD_R = crate::BitReader;
#[doc = "Field `NBRSD` writer - non-backup SRAM2b security disable"]
pub type NBRSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2OPT` reader - CPU2 cortex M0 boot reset vector memory selection"]
pub type C2OPT_R = crate::BitReader;
#[doc = "Field `C2OPT` writer - CPU2 cortex M0 boot reset vector memory selection"]
pub type C2OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    pub fn sbrv(&self) -> SBRV_R {
        SBRV_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    pub fn sbrsa(&self) -> SBRSA_R {
        SBRSA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    pub fn brsd(&self) -> BRSD_R {
        BRSD_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    pub fn snbrsa(&self) -> SNBRSA_R {
        SNBRSA_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    pub fn nbrsd(&self) -> NBRSD_R {
        NBRSD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    pub fn c2opt(&self) -> C2OPT_R {
        C2OPT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - cortex M0 access control register"]
    #[inline(always)]
    #[must_use]
    pub fn sbrv(&mut self) -> SBRV_W<SRRVRrs> {
        SBRV_W::new(self, 0)
    }
    #[doc = "Bits 18:22 - Secure backup SRAM2a start address"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsa(&mut self) -> SBRSA_W<SRRVRrs> {
        SBRSA_W::new(self, 18)
    }
    #[doc = "Bit 23 - backup SRAM2a security disable"]
    #[inline(always)]
    #[must_use]
    pub fn brsd(&mut self) -> BRSD_W<SRRVRrs> {
        BRSD_W::new(self, 23)
    }
    #[doc = "Bits 25:29 - Secure non backup SRAM2a start address"]
    #[inline(always)]
    #[must_use]
    pub fn snbrsa(&mut self) -> SNBRSA_W<SRRVRrs> {
        SNBRSA_W::new(self, 25)
    }
    #[doc = "Bit 30 - non-backup SRAM2b security disable"]
    #[inline(always)]
    #[must_use]
    pub fn nbrsd(&mut self) -> NBRSD_W<SRRVRrs> {
        NBRSD_W::new(self, 30)
    }
    #[doc = "Bit 31 - CPU2 cortex M0 boot reset vector memory selection"]
    #[inline(always)]
    #[must_use]
    pub fn c2opt(&mut self) -> C2OPT_W<SRRVRrs> {
        C2OPT_W::new(self, 31)
    }
}
#[doc = "Secure SRAM2 start address and cortex M0 reset vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srrvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srrvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SRRVR to value 0x0100_0000"]
impl crate::Resettable for SRRVRrs {
    const RESET_VALUE: u32 = 0x0100_0000;
}
