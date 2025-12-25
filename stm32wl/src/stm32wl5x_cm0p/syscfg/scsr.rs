///Register `SCSR` reader
pub type R = crate::R<SCSRrs>;
///Register `SCSR` writer
pub type W = crate::W<SCSRrs>;
/**SRAM2 erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM2ERW {
    ///1: Start SRAM2 erase operation
    Erase = 1,
}
impl From<SRAM2ERW> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ERW) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAM2ER` reader - SRAM2 erase
pub type SRAM2ER_R = crate::BitReader<SRAM2ERW>;
impl SRAM2ER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRAM2ERW> {
        match self.bits {
            true => Some(SRAM2ERW::Erase),
            _ => None,
        }
    }
    ///Start SRAM2 erase operation
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == SRAM2ERW::Erase
    }
}
///Field `SRAM2ER` writer - SRAM2 erase
pub type SRAM2ER_W<'a, REG> = crate::BitWriter<'a, REG, SRAM2ERW>;
impl<'a, REG> SRAM2ER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Start SRAM2 erase operation
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(SRAM2ERW::Erase)
    }
}
/**SRAM1, SRAM2 and PKA SRAM busy by erase operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAMBSY {
    ///0: No SRAM1 or SRAM2 erase operation is ongoing
    Idle = 0,
    ///1: SRAM1 or SRAM2 erase operation is ongoing
    Busy = 1,
}
impl From<SRAMBSY> for bool {
    #[inline(always)]
    fn from(variant: SRAMBSY) -> Self {
        variant as u8 != 0
    }
}
///Field `SRAMBSY` reader - SRAM1, SRAM2 and PKA SRAM busy by erase operation
pub type SRAMBSY_R = crate::BitReader<SRAMBSY>;
impl SRAMBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRAMBSY {
        match self.bits {
            false => SRAMBSY::Idle,
            true => SRAMBSY::Busy,
        }
    }
    ///No SRAM1 or SRAM2 erase operation is ongoing
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SRAMBSY::Idle
    }
    ///SRAM1 or SRAM2 erase operation is ongoing
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SRAMBSY::Busy
    }
}
/**PKA SRAM busy by erase operation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKASRAMBSY {
    ///0: No PKA SRAM erase operation is ongoing
    Idle = 0,
    ///1: PKA SRAM erase operation is ongoing
    Busy = 1,
}
impl From<PKASRAMBSY> for bool {
    #[inline(always)]
    fn from(variant: PKASRAMBSY) -> Self {
        variant as u8 != 0
    }
}
///Field `PKASRAMBSY` reader - PKA SRAM busy by erase operation
pub type PKASRAMBSY_R = crate::BitReader<PKASRAMBSY>;
impl PKASRAMBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PKASRAMBSY {
        match self.bits {
            false => PKASRAMBSY::Idle,
            true => PKASRAMBSY::Busy,
        }
    }
    ///No PKA SRAM erase operation is ongoing
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PKASRAMBSY::Idle
    }
    ///PKA SRAM erase operation is ongoing
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == PKASRAMBSY::Busy
    }
}
impl R {
    ///Bit 0 - SRAM2 erase
    #[inline(always)]
    pub fn sram2er(&self) -> SRAM2ER_R {
        SRAM2ER_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM1, SRAM2 and PKA SRAM busy by erase operation
    #[inline(always)]
    pub fn srambsy(&self) -> SRAMBSY_R {
        SRAMBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - PKA SRAM busy by erase operation
    #[inline(always)]
    pub fn pkasrambsy(&self) -> PKASRAMBSY_R {
        PKASRAMBSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCSR")
            .field("pkasrambsy", &self.pkasrambsy())
            .field("srambsy", &self.srambsy())
            .field("sram2er", &self.sram2er())
            .finish()
    }
}
impl W {
    ///Bit 0 - SRAM2 erase
    #[inline(always)]
    pub fn sram2er(&mut self) -> SRAM2ER_W<'_, SCSRrs> {
        SRAM2ER_W::new(self, 0)
    }
}
/**SCSR

You can [`read`](crate::Reg::read) this register and get [`scsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#SYSCFG:SCSR)*/
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
