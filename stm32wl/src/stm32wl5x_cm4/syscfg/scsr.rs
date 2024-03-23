#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSRrs>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSRrs>;
#[doc = "SRAM2 erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ERW {
    #[doc = "1: Start SRAM2 erase operation"]
    Erase = 1,
}
impl From<SRAM2ERW> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ERW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2ER` reader - SRAM2 erase"]
pub type SRAM2ER_R = crate::BitReader<SRAM2ERW>;
impl SRAM2ER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRAM2ERW> {
        match self.bits {
            true => Some(SRAM2ERW::Erase),
            _ => None,
        }
    }
    #[doc = "Start SRAM2 erase operation"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ERW::Erase
    }
}
#[doc = "Field `SRAM2ER` writer - SRAM2 erase"]
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2ERW>;
impl<'a, REG> SRAM2ER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start SRAM2 erase operation"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2ERW::Erase)
    }
}
#[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMBSY {
    #[doc = "0: No SRAM1 or SRAM2 erase operation is ongoing"]
    Idle = 0,
    #[doc = "1: SRAM1 or SRAM2 erase operation is ongoing"]
    Busy = 1,
}
impl From<SRAMBSY> for bool {
    #[inline(always)]
    fn from(variant: SRAMBSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMBSY` reader - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
pub type SRAMBSY_R = crate::BitReader<SRAMBSY>;
impl SRAMBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAMBSY {
        match self.bits {
            false => SRAMBSY::Idle,
            true => SRAMBSY::Busy,
        }
    }
    #[doc = "No SRAM1 or SRAM2 erase operation is ongoing"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SRAMBSY::Idle
    }
    #[doc = "SRAM1 or SRAM2 erase operation is ongoing"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAMBSY::Busy
    }
}
#[doc = "PKA SRAM busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASRAMBSY {
    #[doc = "0: No PKA SRAM erase operation is ongoing"]
    Idle = 0,
    #[doc = "1: PKA SRAM erase operation is ongoing"]
    Busy = 1,
}
impl From<PKASRAMBSY> for bool {
    #[inline(always)]
    fn from(variant: PKASRAMBSY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PKASRAMBSY` reader - PKA SRAM busy by erase operation"]
pub type PKASRAMBSY_R = crate::BitReader<PKASRAMBSY>;
impl PKASRAMBSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PKASRAMBSY {
        match self.bits {
            false => PKASRAMBSY::Idle,
            true => PKASRAMBSY::Busy,
        }
    }
    #[doc = "No PKA SRAM erase operation is ongoing"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PKASRAMBSY::Idle
    }
    #[doc = "PKA SRAM erase operation is ongoing"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PKASRAMBSY::Busy
    }
}
impl R {
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn srambsy(&self) -> SRAMBSY_R {
        SRAMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - PKA SRAM busy by erase operation"]
    #[inline(always)]
    pub fn pkasrambsy(&self) -> PKASRAMBSY_R {
        PKASRAMBSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 erase"]
    #[inline(always)]
    #[must_use]
    pub fn sram2er(&mut self) -> SRAM2ER_W<SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
#[doc = "SCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scsr::R`](R) reader structure"]
impl crate::Readable for SCSRrs {}
#[doc = "`write(|w| ..)` method takes [`scsr::W`](W) writer structure"]
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCSR to value 0"]
impl crate::Resettable for SCSRrs {
    const RESET_VALUE: u32 = 0;
}
