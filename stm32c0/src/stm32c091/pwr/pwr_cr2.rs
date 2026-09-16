///Register `PWR_CR2` reader
pub type R = crate::R<PWR_CR2rs>;
///Register `PWR_CR2` writer
pub type W = crate::W<PWR_CR2rs>;
/**supply voltage monitoring

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PVM_VDDIO2 {
    ///0: Monitoring disabled; IOs in isolation mode
    B0x0 = 0,
    ///1: Monitoring enabled; IOs enabled or in isolation mode according to V<sub>DDIO2</sub> level
    B0x1 = 1,
    ///2: Monitoring bypassed; IOs enabled
    B0x2 = 2,
}
impl From<PVM_VDDIO2> for u8 {
    #[inline(always)]
    fn from(variant: PVM_VDDIO2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PVM_VDDIO2 {
    type Ux = u8;
}
impl crate::IsEnum for PVM_VDDIO2 {}
///Field `PVM_VDDIO2` reader - supply voltage monitoring
pub type PVM_VDDIO2_R = crate::FieldReader<PVM_VDDIO2>;
impl PVM_VDDIO2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PVM_VDDIO2> {
        match self.bits {
            0 => Some(PVM_VDDIO2::B0x0),
            1 => Some(PVM_VDDIO2::B0x1),
            2 => Some(PVM_VDDIO2::B0x2),
            _ => None,
        }
    }
    ///Monitoring disabled; IOs in isolation mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PVM_VDDIO2::B0x0
    }
    ///Monitoring enabled; IOs enabled or in isolation mode according to V<sub>DDIO2</sub> level
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PVM_VDDIO2::B0x1
    }
    ///Monitoring bypassed; IOs enabled
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PVM_VDDIO2::B0x2
    }
}
///Field `PVM_VDDIO2` writer - supply voltage monitoring
pub type PVM_VDDIO2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PVM_VDDIO2>;
impl<'a, REG> PVM_VDDIO2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Monitoring disabled; IOs in isolation mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PVM_VDDIO2::B0x0)
    }
    ///Monitoring enabled; IOs enabled or in isolation mode according to V<sub>DDIO2</sub> level
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PVM_VDDIO2::B0x1)
    }
    ///Monitoring bypassed; IOs enabled
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PVM_VDDIO2::B0x2)
    }
}
impl R {
    ///Bits 8:9 - supply voltage monitoring
    #[inline(always)]
    pub fn pvm_vddio2(&self) -> PVM_VDDIO2_R {
        PVM_VDDIO2_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR2")
            .field("pvm_vddio2", &self.pvm_vddio2())
            .finish()
    }
}
impl W {
    ///Bits 8:9 - supply voltage monitoring
    #[inline(always)]
    pub fn pvm_vddio2(&mut self) -> PVM_VDDIO2_W<'_, PWR_CR2rs> {
        PVM_VDDIO2_W::new(self, 8)
    }
}
/**PWR control register 1

You can [`read`](crate::Reg::read) this register and get [`pwr_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#PWR:PWR_CR2)*/
pub struct PWR_CR2rs;
impl crate::RegisterSpec for PWR_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr2::R`](R) reader structure
impl crate::Readable for PWR_CR2rs {}
///`write(|w| ..)` method takes [`pwr_cr2::W`](W) writer structure
impl crate::Writable for PWR_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR_CR2 to value 0x0100
impl crate::Resettable for PWR_CR2rs {
    const RESET_VALUE: u32 = 0x0100;
}
