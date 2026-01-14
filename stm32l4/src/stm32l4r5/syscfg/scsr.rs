///Register `SCSR` reader
pub type R = crate::R<SCSRrs>;
///Register `SCSR` writer
pub type W = crate::W<SCSRrs>;
/**SRAM2 Erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ER {
    ///1: Setting this bit starts a hardware SRAM2 erase operation
    Erase = 1,
}
impl From<SRAM2ER> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ER) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2ER` reader - SRAM2 Erase
pub type SRAM2ER_R = crate::BitReader<SRAM2ER>;
impl SRAM2ER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRAM2ER> {
        match self.bits {
            true => Some(SRAM2ER::Erase),
            _ => None,
        }
    }
    ///Setting this bit starts a hardware SRAM2 erase operation
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ER::Erase
    }
}
///Field `SRAM2ER` writer - SRAM2 Erase
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2ER>;
impl<'a, REG> SRAM2ER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Setting this bit starts a hardware SRAM2 erase operation
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2ER::Erase)
    }
}
/**SRAM2 busy by erase operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2BS {
    ///0: No SRAM2 erase operation is on going
    NotBusy = 0,
    ///1: SRAM2 erase operation is on going
    Busy = 1,
}
impl From<SRAM2BS> for bool {
    #[inline(always)]
    fn from(variant: SRAM2BS) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2BS` reader - SRAM2 busy by erase operation
pub type SRAM2BS_R = crate::BitReader<SRAM2BS>;
impl SRAM2BS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAM2BS {
        match self.bits {
            false => SRAM2BS::NotBusy,
            true => SRAM2BS::Busy,
        }
    }
    ///No SRAM2 erase operation is on going
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == SRAM2BS::NotBusy
    }
    ///SRAM2 erase operation is on going
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAM2BS::Busy
    }
}
impl R {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 busy by erase operation
    #[inline(always)]
    pub fn sram2bs(&self) -> SRAM2BS_R {
        SRAM2BS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("sram2bs", &self.sram2bs())
            .field("sram2er", &self.sram2er())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 Erase
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<'_, SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
/**SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#SYSCFG:SCSR)*/
pub struct SCSRrs;
impl crate::RegisterSpec for SCSRrs {
    type Ux = u32;
}
///`read()` method returns [`scsr::R`](R) reader structure
impl crate::Readable for SCSRrs {}
///`write(|w| ..)` method takes [`scsr::W`](W) writer structure
impl crate::Writable for SCSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCSR to value 0
impl crate::Resettable for SCSRrs {}
