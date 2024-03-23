#[doc = "Register `SCSR` reader"]
pub type R = crate::R<SCSRrs>;
#[doc = "Register `SCSR` writer"]
pub type W = crate::W<SCSRrs>;
#[doc = "SRAM2 Erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ER {
    #[doc = "1: Setting this bit starts a hardware SRAM2 erase operation"]
    Erase = 1,
}
impl From<SRAM2ER> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2ER` reader - SRAM2 Erase"]
pub type SRAM2ER_R = crate::BitReader<SRAM2ER>;
impl SRAM2ER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRAM2ER> {
        match self.bits {
            true => Some(SRAM2ER::Erase),
            _ => None,
        }
    }
    #[doc = "Setting this bit starts a hardware SRAM2 erase operation"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ER::Erase
    }
}
#[doc = "Field `SRAM2ER` writer - SRAM2 Erase"]
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2ER>;
impl<'a, REG> SRAM2ER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit starts a hardware SRAM2 erase operation"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2ER::Erase)
    }
}
#[doc = "SRAM2 busy by erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2BS {
    #[doc = "0: No SRAM2 erase operation is on going"]
    NotBusy = 0,
    #[doc = "1: SRAM2 erase operation is on going"]
    Busy = 1,
}
impl From<SRAM2BS> for bool {
    #[inline(always)]
    fn from(variant: SRAM2BS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2BS` reader - SRAM2 busy by erase operation"]
pub type SRAM2BS_R = crate::BitReader<SRAM2BS>;
impl SRAM2BS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2BS {
        match self.bits {
            false => SRAM2BS::NotBusy,
            true => SRAM2BS::Busy,
        }
    }
    #[doc = "No SRAM2 erase operation is on going"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == SRAM2BS::NotBusy
    }
    #[doc = "SRAM2 erase operation is on going"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAM2BS::Busy
    }
}
impl R {
    #[doc = "Bit 0 - SRAM2 Erase"]
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM2 busy by erase operation"]
    #[inline(always)]
    pub fn sram2bs(&self) -> SRAM2BS_R {
        SRAM2BS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM2 Erase"]
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
