///Register `MESR` reader
pub type R = crate::R<MESRrs>;
///Register `MESR` writer
pub type W = crate::W<MESRrs>;
/**erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLRR {
    ///0: Memory erase in progress
    EraseInProgress = 0,
    ///1: Memory erase complete
    EraseComplete = 1,
}
impl From<MCLRR> for bool {
    #[inline(always)]
    fn from(variant: MCLRR) -> Self {
        variant as u8 != 0
    }
}
///Field `MCLR` reader - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software
pub type MCLR_R = crate::BitReader<MCLRR>;
impl MCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MCLRR {
        match self.bits {
            false => MCLRR::EraseInProgress,
            true => MCLRR::EraseComplete,
        }
    }
    ///Memory erase in progress
    #[inline(always)]
    pub fn is_erase_in_progress(&self) -> bool {
        *self == MCLRR::EraseInProgress
    }
    ///Memory erase complete
    #[inline(always)]
    pub fn is_erase_complete(&self) -> bool {
        *self == MCLRR::EraseComplete
    }
}
/**erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCLRW {
    ///1: Clear memory erase status flag
    Clear = 1,
}
impl From<MCLRW> for bool {
    #[inline(always)]
    fn from(variant: MCLRW) -> Self {
        variant as u8 != 0
    }
}
///Field `MCLR` writer - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software
pub type MCLR_W<'a, REG> = crate::BitWriter1C<'a, REG, MCLRW>;
impl<'a, REG> MCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear memory erase status flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(MCLRW::Clear)
    }
}
/**end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPMEER {
    ///0: ICACHE erase ongoing
    EraseInProgress = 0,
    ///1: ICACHE erase completed
    EraseCompleted = 1,
}
impl From<IPMEER> for bool {
    #[inline(always)]
    fn from(variant: IPMEER) -> Self {
        variant as u8 != 0
    }
}
///Field `IPMEE` reader - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.
pub type IPMEE_R = crate::BitReader<IPMEER>;
impl IPMEE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPMEER {
        match self.bits {
            false => IPMEER::EraseInProgress,
            true => IPMEER::EraseCompleted,
        }
    }
    ///ICACHE erase ongoing
    #[inline(always)]
    pub fn is_erase_in_progress(&self) -> bool {
        *self == IPMEER::EraseInProgress
    }
    ///ICACHE erase completed
    #[inline(always)]
    pub fn is_erase_completed(&self) -> bool {
        *self == IPMEER::EraseCompleted
    }
}
/**end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPMEEW {
    ///1: Clear ICACHE erase status flag
    Clear = 1,
}
impl From<IPMEEW> for bool {
    #[inline(always)]
    fn from(variant: IPMEEW) -> Self {
        variant as u8 != 0
    }
}
///Field `IPMEE` writer - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.
pub type IPMEE_W<'a, REG> = crate::BitWriter1C<'a, REG, IPMEEW>;
impl<'a, REG> IPMEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear ICACHE erase status flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IPMEEW::Clear)
    }
}
impl R {
    ///Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MESR")
            .field("mclr", &self.mclr())
            .field("ipmee", &self.ipmee())
            .finish()
    }
}
impl W {
    ///Bit 0 - erase after reset status This bit shows the status of the protection for SRAM2, BKPRAM, ICACHE, ICACHE. It is set by hardware and reset by software
    #[inline(always)]
    pub fn mclr(&mut self) -> MCLR_W<'_, MESRrs> {
        MCLR_W::new(self, 0)
    }
    ///Bit 16 - end-of-erase status for ICACHE This bit shows the status of the protection for ICACHE. It is set by hardware and reset by software.
    #[inline(always)]
    pub fn ipmee(&mut self) -> IPMEE_W<'_, MESRrs> {
        IPMEE_W::new(self, 16)
    }
}
/**SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:MESR)*/
pub struct MESRrs;
impl crate::RegisterSpec for MESRrs {
    type Ux = u32;
}
///`read()` method returns [`mesr::R`](R) reader structure
impl crate::Readable for MESRrs {}
///`write(|w| ..)` method takes [`mesr::W`](W) writer structure
impl crate::Writable for MESRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0001;
}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MESRrs {}
