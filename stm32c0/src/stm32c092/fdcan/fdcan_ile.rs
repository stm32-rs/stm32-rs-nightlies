///Register `FDCAN_ILE` reader
pub type R = crate::R<FDCAN_ILErs>;
///Register `FDCAN_ILE` writer
pub type W = crate::W<FDCAN_ILErs>;
/**Enable interrupt line 0

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT0 {
    ///0: Interrupt line fdcan_intr1_it disabled
    B0x0 = 0,
    ///1: Interrupt line fdcan_intr1_it enabled
    B0x1 = 1,
}
impl From<EINT0> for bool {
    #[inline(always)]
    fn from(variant: EINT0) -> Self {
        variant as u8 != 0
    }
}
///Field `EINT0` reader - Enable interrupt line 0
pub type EINT0_R = crate::BitReader<EINT0>;
impl EINT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EINT0 {
        match self.bits {
            false => EINT0::B0x0,
            true => EINT0::B0x1,
        }
    }
    ///Interrupt line fdcan_intr1_it disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT0::B0x0
    }
    ///Interrupt line fdcan_intr1_it enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT0::B0x1
    }
}
///Field `EINT0` writer - Enable interrupt line 0
pub type EINT0_W<'a, REG> = crate::BitWriter<'a, REG, EINT0>;
impl<'a, REG> EINT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt line fdcan_intr1_it disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EINT0::B0x0)
    }
    ///Interrupt line fdcan_intr1_it enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EINT0::B0x1)
    }
}
/**Enable interrupt line 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EINT1 {
    ///0: Interrupt line fdcan_intr0_it disabled
    B0x0 = 0,
    ///1: Interrupt line fdcan_intr0_it enabled
    B0x1 = 1,
}
impl From<EINT1> for bool {
    #[inline(always)]
    fn from(variant: EINT1) -> Self {
        variant as u8 != 0
    }
}
///Field `EINT1` reader - Enable interrupt line 1
pub type EINT1_R = crate::BitReader<EINT1>;
impl EINT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EINT1 {
        match self.bits {
            false => EINT1::B0x0,
            true => EINT1::B0x1,
        }
    }
    ///Interrupt line fdcan_intr0_it disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT1::B0x0
    }
    ///Interrupt line fdcan_intr0_it enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT1::B0x1
    }
}
///Field `EINT1` writer - Enable interrupt line 1
pub type EINT1_W<'a, REG> = crate::BitWriter<'a, REG, EINT1>;
impl<'a, REG> EINT1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt line fdcan_intr0_it disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EINT1::B0x0)
    }
    ///Interrupt line fdcan_intr0_it enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EINT1::B0x1)
    }
}
impl R {
    ///Bit 0 - Enable interrupt line 0
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable interrupt line 1
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_ILE")
            .field("eint0", &self.eint0())
            .field("eint1", &self.eint1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable interrupt line 0
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W<'_, FDCAN_ILErs> {
        EINT0_W::new(self, 0)
    }
    ///Bit 1 - Enable interrupt line 1
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W<'_, FDCAN_ILErs> {
        EINT1_W::new(self, 1)
    }
}
/**FDCAN interrupt line enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_ILE)*/
pub struct FDCAN_ILErs;
impl crate::RegisterSpec for FDCAN_ILErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ile::R`](R) reader structure
impl crate::Readable for FDCAN_ILErs {}
///`write(|w| ..)` method takes [`fdcan_ile::W`](W) writer structure
impl crate::Writable for FDCAN_ILErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_ILE to value 0
impl crate::Resettable for FDCAN_ILErs {}
