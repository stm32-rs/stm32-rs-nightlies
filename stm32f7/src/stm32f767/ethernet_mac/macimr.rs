///Register `MACIMR` reader
pub type R = crate::R<MACIMRrs>;
///Register `MACIMR` writer
pub type W = crate::W<MACIMRrs>;
/**PMT interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMTIM {
    ///0: PMT Status interrupt generation enabled
    Unmasked = 0,
    ///1: PMT Status interrupt generation disabled
    Masked = 1,
}
impl From<PMTIM> for bool {
    #[inline(always)]
    fn from(variant: PMTIM) -> Self {
        variant as u8 != 0
    }
}
///Field `PMTIM` reader - PMT interrupt mask
pub type PMTIM_R = crate::BitReader<PMTIM>;
impl PMTIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PMTIM {
        match self.bits {
            false => PMTIM::Unmasked,
            true => PMTIM::Masked,
        }
    }
    ///PMT Status interrupt generation enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == PMTIM::Unmasked
    }
    ///PMT Status interrupt generation disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == PMTIM::Masked
    }
}
///Field `PMTIM` writer - PMT interrupt mask
pub type PMTIM_W<'a, REG> = crate::BitWriter<'a, REG, PMTIM>;
impl<'a, REG> PMTIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PMT Status interrupt generation enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(PMTIM::Unmasked)
    }
    ///PMT Status interrupt generation disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(PMTIM::Masked)
    }
}
/**Time stamp trigger interrupt mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTIM {
    ///0: Time stamp interrupt generation enabled
    Unmasked = 0,
    ///1: Time stamp interrupt generation disabled
    Masked = 1,
}
impl From<TSTIM> for bool {
    #[inline(always)]
    fn from(variant: TSTIM) -> Self {
        variant as u8 != 0
    }
}
///Field `TSTIM` reader - Time stamp trigger interrupt mask
pub type TSTIM_R = crate::BitReader<TSTIM>;
impl TSTIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSTIM {
        match self.bits {
            false => TSTIM::Unmasked,
            true => TSTIM::Masked,
        }
    }
    ///Time stamp interrupt generation enabled
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == TSTIM::Unmasked
    }
    ///Time stamp interrupt generation disabled
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TSTIM::Masked
    }
}
///Field `TSTIM` writer - Time stamp trigger interrupt mask
pub type TSTIM_W<'a, REG> = crate::BitWriter<'a, REG, TSTIM>;
impl<'a, REG> TSTIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Time stamp interrupt generation enabled
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(TSTIM::Unmasked)
    }
    ///Time stamp interrupt generation disabled
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TSTIM::Masked)
    }
}
impl R {
    ///Bit 3 - PMT interrupt mask
    #[inline(always)]
    pub fn pmtim(&self) -> PMTIM_R {
        PMTIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 9 - Time stamp trigger interrupt mask
    #[inline(always)]
    pub fn tstim(&self) -> TSTIM_R {
        TSTIM_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACIMR")
            .field("pmtim", &self.pmtim())
            .field("tstim", &self.tstim())
            .finish()
    }
}
impl W {
    ///Bit 3 - PMT interrupt mask
    #[inline(always)]
    pub fn pmtim(&mut self) -> PMTIM_W<'_, MACIMRrs> {
        PMTIM_W::new(self, 3)
    }
    ///Bit 9 - Time stamp trigger interrupt mask
    #[inline(always)]
    pub fn tstim(&mut self) -> TSTIM_W<'_, MACIMRrs> {
        TSTIM_W::new(self, 9)
    }
}
/**Ethernet MAC interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`macimr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macimr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#Ethernet_MAC:MACIMR)*/
pub struct MACIMRrs;
impl crate::RegisterSpec for MACIMRrs {
    type Ux = u32;
}
///`read()` method returns [`macimr::R`](R) reader structure
impl crate::Readable for MACIMRrs {}
///`write(|w| ..)` method takes [`macimr::W`](W) writer structure
impl crate::Writable for MACIMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACIMR to value 0
impl crate::Resettable for MACIMRrs {}
