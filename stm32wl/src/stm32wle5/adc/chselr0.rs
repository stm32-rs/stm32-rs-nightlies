///Register `CHSELR0` reader
pub type R = crate::R<CHSELR0rs>;
///Register `CHSELR0` writer
pub type W = crate::W<CHSELR0rs>;
/**CHSEL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CHSEL {
    ///0: Input Channel is not selected for conversion
    NotSelected = 0,
    ///1: Input Channel is selected for conversion
    Selected = 1,
}
impl From<CHSEL> for u32 {
    #[inline(always)]
    fn from(variant: CHSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CHSEL {
    type Ux = u32;
}
impl crate::IsEnum for CHSEL {}
///Field `CHSEL` reader - CHSEL
pub type CHSEL_R = crate::FieldReader<CHSEL>;
impl CHSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHSEL> {
        match self.bits {
            0 => Some(CHSEL::NotSelected),
            1 => Some(CHSEL::Selected),
            _ => None,
        }
    }
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == CHSEL::NotSelected
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == CHSEL::Selected
    }
}
///Field `CHSEL` writer - CHSEL
pub type CHSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 18, CHSEL>;
impl<'a, REG> CHSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    ///Input Channel is not selected for conversion
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL::NotSelected)
    }
    ///Input Channel is selected for conversion
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(CHSEL::Selected)
    }
}
impl R {
    ///Bits 0:17 - CHSEL
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(self.bits & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHSELR0")
            .field("chsel", &self.chsel())
            .finish()
    }
}
impl W {
    ///Bits 0:17 - CHSEL
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W<CHSELR0rs> {
        CHSEL_W::new(self, 0)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`chselr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chselr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#ADC:CHSELR0)*/
pub struct CHSELR0rs;
impl crate::RegisterSpec for CHSELR0rs {
    type Ux = u32;
}
///`read()` method returns [`chselr0::R`](R) reader structure
impl crate::Readable for CHSELR0rs {}
///`write(|w| ..)` method takes [`chselr0::W`](W) writer structure
impl crate::Writable for CHSELR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHSELR0 to value 0
impl crate::Resettable for CHSELR0rs {
    const RESET_VALUE: u32 = 0;
}
