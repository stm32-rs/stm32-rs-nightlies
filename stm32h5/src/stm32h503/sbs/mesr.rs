#[doc = "Register `MESR` reader"]
pub type R = crate::R<MESRrs>;
#[doc = "Register `MESR` writer"]
pub type W = crate::W<MESRrs>;
#[doc = "erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLRR {
    #[doc = "0: Memory erase in progress"]
    EraseInProgress = 0,
    #[doc = "1: Memory erase complete"]
    EraseComplete = 1,
}
impl From<MCLRR> for bool {
    #[inline(always)]
    fn from(variant: MCLRR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLR` reader - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_R = crate::BitReader<MCLRR>;
impl MCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLRR {
        match self.bits {
            false => MCLRR::EraseInProgress,
            true => MCLRR::EraseComplete,
        }
    }
    #[doc = "Memory erase in progress"]
    #[inline(always)]
    pub fn is_erase_in_progress(&self) -> bool {
        *self == MCLRR::EraseInProgress
    }
    #[doc = "Memory erase complete"]
    #[inline(always)]
    pub fn is_erase_complete(&self) -> bool {
        *self == MCLRR::EraseComplete
    }
}
#[doc = "erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLRW {
    #[doc = "1: Clear memory erase status flag"]
    Clear = 1,
}
impl From<MCLRW> for bool {
    #[inline(always)]
    fn from(variant: MCLRW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLR` writer - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
pub type MCLR_W<'a, REG> = crate::BitWriter1C<'a, REG, MCLRW>;
impl<'a, REG> MCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear memory erase status flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MCLRW::Clear)
    }
}
#[doc = "end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPMEER {
    #[doc = "0: ICACHE erase ongoing"]
    EraseInProgress = 0,
    #[doc = "1: ICACHE erase completed"]
    EraseCompleted = 1,
}
impl From<IPMEER> for bool {
    #[inline(always)]
    fn from(variant: IPMEER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPMEE` reader - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_R = crate::BitReader<IPMEER>;
impl IPMEE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IPMEER {
        match self.bits {
            false => IPMEER::EraseInProgress,
            true => IPMEER::EraseCompleted,
        }
    }
    #[doc = "ICACHE erase ongoing"]
    #[inline(always)]
    pub fn is_erase_in_progress(&self) -> bool {
        *self == IPMEER::EraseInProgress
    }
    #[doc = "ICACHE erase completed"]
    #[inline(always)]
    pub fn is_erase_completed(&self) -> bool {
        *self == IPMEER::EraseCompleted
    }
}
#[doc = "end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPMEEW {
    #[doc = "1: Clear ICACHE erase status flag"]
    Clear = 1,
}
impl From<IPMEEW> for bool {
    #[inline(always)]
    fn from(variant: IPMEEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPMEE` writer - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
pub type IPMEE_W<'a, REG> = crate::BitWriter1C<'a, REG, IPMEEW>;
impl<'a, REG> IPMEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear ICACHE erase status flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IPMEEW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn mclr(&mut self) -> MCLR_W<MESRrs> {
        MCLR_W::new(self, 0)
    }
    #[doc = "Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ipmee(&mut self) -> IPMEE_W<MESRrs> {
        IPMEE_W::new(self, 16)
    }
}
#[doc = "SBS memory erase status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mesr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mesr::R`](R) reader structure"]
impl crate::Readable for MESRrs {}
#[doc = "`write(|w| ..)` method takes [`mesr::W`](W) writer structure"]
impl crate::Writable for MESRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0001;
}
#[doc = "`reset()` method sets MESR to value 0"]
impl crate::Resettable for MESRrs {
    const RESET_VALUE: u32 = 0;
}
