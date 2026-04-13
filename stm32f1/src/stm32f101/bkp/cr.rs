///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Tamper pin enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPE {
    ///0: The TAMPER pin is free for general purpose I/O
    General = 0,
    ///1: Tamper alternate I/O function is activated
    Alternate = 1,
}
impl From<TPE> for bool {
    #[inline(always)]
    fn from(variant: TPE) -> Self {
        variant as u8 != 0
    }
}
///Field `TPE` reader - Tamper pin enable
pub type TPE_R = crate::BitReader<TPE>;
impl TPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPE {
        match self.bits {
            false => TPE::General,
            true => TPE::Alternate,
        }
    }
    ///The TAMPER pin is free for general purpose I/O
    #[inline(always)]
    pub fn is_general(&self) -> bool {
        *self == TPE::General
    }
    ///Tamper alternate I/O function is activated
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TPE::Alternate
    }
}
///Field `TPE` writer - Tamper pin enable
pub type TPE_W<'a, REG> = crate::BitWriter<'a, REG, TPE>;
impl<'a, REG> TPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The TAMPER pin is free for general purpose I/O
    #[inline(always)]
    pub fn general(self) -> &'a mut crate::W<REG> {
        self.variant(TPE::General)
    }
    ///Tamper alternate I/O function is activated
    #[inline(always)]
    pub fn alternate(self) -> &'a mut crate::W<REG> {
        self.variant(TPE::Alternate)
    }
}
/**Tamper pin active level

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPAL {
    ///0: A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    High = 0,
    ///1: A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    Low = 1,
}
impl From<TPAL> for bool {
    #[inline(always)]
    fn from(variant: TPAL) -> Self {
        variant as u8 != 0
    }
}
///Field `TPAL` reader - Tamper pin active level
pub type TPAL_R = crate::BitReader<TPAL>;
impl TPAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TPAL {
        match self.bits {
            false => TPAL::High,
            true => TPAL::Low,
        }
    }
    ///A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TPAL::High
    }
    ///A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TPAL::Low
    }
}
///Field `TPAL` writer - Tamper pin active level
pub type TPAL_W<'a, REG> = crate::BitWriter<'a, REG, TPAL>;
impl<'a, REG> TPAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A high level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(TPAL::High)
    }
    ///A low level on the TAMPER pin resets all data backup registers (if TPE bit is set)
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(TPAL::Low)
    }
}
impl R {
    ///Bit 0 - Tamper pin enable
    #[inline(always)]
    pub fn tpe(&self) -> TPE_R {
        TPE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper pin active level
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("tpe", &self.tpe())
            .field("tpal", &self.tpal())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper pin enable
    #[inline(always)]
    pub fn tpe(&mut self) -> TPE_W<'_, CRrs> {
        TPE_W::new(self, 0)
    }
    ///Bit 1 - Tamper pin active level
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W<'_, CRrs> {
        TPAL_W::new(self, 1)
    }
}
/**Backup control register (BKP_CR)

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#BKP:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
