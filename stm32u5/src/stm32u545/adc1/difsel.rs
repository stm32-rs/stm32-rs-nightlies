///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
/**DIFSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DIFSEL {
    ///0: ADC analog input channel x is configured in single-ended mode
    SingleEnded = 0,
    ///1: ADC analog input channel x is configured in differential mode
    Differential = 1,
}
impl From<DIFSEL> for u32 {
    #[inline(always)]
    fn from(variant: DIFSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIFSEL {
    type Ux = u32;
}
impl crate::IsEnum for DIFSEL {}
///Field `DIFSEL` reader - DIFSEL
pub type DIFSEL_R = crate::FieldReader<DIFSEL>;
impl DIFSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIFSEL> {
        match self.bits {
            0 => Some(DIFSEL::SingleEnded),
            1 => Some(DIFSEL::Differential),
            _ => None,
        }
    }
    ///ADC analog input channel x is configured in single-ended mode
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == DIFSEL::SingleEnded
    }
    ///ADC analog input channel x is configured in differential mode
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == DIFSEL::Differential
    }
}
///Field `DIFSEL` writer - DIFSEL
pub type DIFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, DIFSEL>;
impl<'a, REG> DIFSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///ADC analog input channel x is configured in single-ended mode
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL::SingleEnded)
    }
    ///ADC analog input channel x is configured in differential mode
    #[inline(always)]
    pub fn differential(self) -> &'a mut crate::W<REG> {
        self.variant(DIFSEL::Differential)
    }
}
impl R {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel", &self.difsel())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    pub fn difsel(&mut self) -> DIFSEL_W<DIFSELrs> {
        DIFSEL_W::new(self, 0)
    }
}
/**ADC differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADC1:DIFSEL)*/
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`difsel::R`](R) reader structure
impl crate::Readable for DIFSELrs {}
///`write(|w| ..)` method takes [`difsel::W`](W) writer structure
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
