///Register `EWCR` reader
pub type R = crate::R<EWCRrs>;
///Register `EWCR` writer
pub type W = crate::W<EWCRrs>;
/**Field `EWIT` reader - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]
- 1. EWIT\[11:0\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset.*/
pub type EWIT_R = crate::FieldReader<u16>;
/**Field `EWIT` writer - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]
- 1. EWIT\[11:0\]
must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset.*/
pub type EWIT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
/**Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWICW {
    ///1: Acknowledge early wake-up interrupt
    Acknowledge = 1,
}
impl From<EWICW> for bool {
    #[inline(always)]
    fn from(variant: EWICW) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIC` writer - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.
pub type EWIC_W<'a, REG> = crate::BitWriter<'a, REG, EWICW>;
impl<'a, REG> EWIC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Acknowledge early wake-up interrupt
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut crate::W<REG> {
        self.variant(EWICW::Acknowledge)
    }
}
/**Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIE {
    ///0: Early interrupt is disabled
    Disabled = 0,
    ///1: Early interrupt is enabled
    Enabled = 1,
}
impl From<EWIE> for bool {
    #[inline(always)]
    fn from(variant: EWIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EWIE` reader - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.
pub type EWIE_R = crate::BitReader<EWIE>;
impl EWIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWIE {
        match self.bits {
            false => EWIE::Disabled,
            true => EWIE::Enabled,
        }
    }
    ///Early interrupt is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWIE::Disabled
    }
    ///Early interrupt is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWIE::Enabled
    }
}
///Field `EWIE` writer - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.
pub type EWIE_W<'a, REG> = crate::BitWriter<'a, REG, EWIE>;
impl<'a, REG> EWIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Early interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE::Disabled)
    }
    ///Early interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EWIE::Enabled)
    }
}
impl R {
    /**Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]
    - 1. EWIT\[11:0\]
    must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset.*/
    #[inline(always)]
    pub fn ewit(&self) -> EWIT_R {
        EWIT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.
    #[inline(always)]
    pub fn ewie(&self) -> EWIE_R {
        EWIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EWCR")
            .field("ewit", &self.ewit())
            .field("ewie", &self.ewie())
            .finish()
    }
}
impl W {
    /**Bits 0:11 - Watchdog counter window value These bits are write access protected (see ). They are written by software to define at which position of the IWDCNT down-counter the early wakeup interrupt must be generated. The early interrupt is generated when the IWDCNT is lower or equal to EWIT\[11:0\]
    - 1. EWIT\[11:0\]
    must be bigger than 1. An interrupt is generated only if EWIE = 1. The EWU bit in the must be reset to be able to change the reload value. Note: Reading this register returns the Early wakeup comparator value and the Interrupt enable bit from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing, hence the value read from this register is valid only when the EWU bit in the is reset.*/
    #[inline(always)]
    #[must_use]
    pub fn ewit(&mut self) -> EWIT_W<EWCRrs> {
        EWIT_W::new(self, 0)
    }
    ///Bit 14 - Watchdog early interrupt acknowledge The software must write a 1 into this bit in order to acknowledge the early wakeup interrupt and to clear the EWIF flag. Writing 0 has not effect, reading this flag returns a 0.
    #[inline(always)]
    #[must_use]
    pub fn ewic(&mut self) -> EWIC_W<EWCRrs> {
        EWIC_W::new(self, 14)
    }
    ///Bit 15 - Watchdog early interrupt enable Set and reset by software. The EWU bit in the must be reset to be able to change the value of this bit.
    #[inline(always)]
    #[must_use]
    pub fn ewie(&mut self) -> EWIE_W<EWCRrs> {
        EWIE_W::new(self, 15)
    }
}
/**IWDG early wakeup interrupt register

You can [`read`](crate::Reg::read) this register and get [`ewcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#IWDG:EWCR)*/
pub struct EWCRrs;
impl crate::RegisterSpec for EWCRrs {
    type Ux = u32;
}
///`read()` method returns [`ewcr::R`](R) reader structure
impl crate::Readable for EWCRrs {}
///`write(|w| ..)` method takes [`ewcr::W`](W) writer structure
impl crate::Writable for EWCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EWCR to value 0
impl crate::Resettable for EWCRrs {
    const RESET_VALUE: u32 = 0;
}
