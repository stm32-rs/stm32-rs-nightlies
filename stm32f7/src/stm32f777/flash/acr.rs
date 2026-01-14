///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Latency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY {
    ///0: 0 wait states
    Ws0 = 0,
    ///1: 1 wait states
    Ws1 = 1,
    ///2: 2 wait states
    Ws2 = 2,
    ///3: 3 wait states
    Ws3 = 3,
    ///4: 4 wait states
    Ws4 = 4,
    ///5: 5 wait states
    Ws5 = 5,
    ///6: 6 wait states
    Ws6 = 6,
    ///7: 7 wait states
    Ws7 = 7,
    ///8: 8 wait states
    Ws8 = 8,
    ///9: 9 wait states
    Ws9 = 9,
    ///10: 10 wait states
    Ws10 = 10,
    ///11: 11 wait states
    Ws11 = 11,
    ///12: 12 wait states
    Ws12 = 12,
    ///13: 13 wait states
    Ws13 = 13,
    ///14: 14 wait states
    Ws14 = 14,
    ///15: 15 wait states
    Ws15 = 15,
}
impl From<LATENCY> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY {
    type Ux = u8;
}
impl crate::IsEnum for LATENCY {}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LATENCY {
        match self.bits {
            0 => LATENCY::Ws0,
            1 => LATENCY::Ws1,
            2 => LATENCY::Ws2,
            3 => LATENCY::Ws3,
            4 => LATENCY::Ws4,
            5 => LATENCY::Ws5,
            6 => LATENCY::Ws6,
            7 => LATENCY::Ws7,
            8 => LATENCY::Ws8,
            9 => LATENCY::Ws9,
            10 => LATENCY::Ws10,
            11 => LATENCY::Ws11,
            12 => LATENCY::Ws12,
            13 => LATENCY::Ws13,
            14 => LATENCY::Ws14,
            15 => LATENCY::Ws15,
            _ => unreachable!(),
        }
    }
    ///0 wait states
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    ///1 wait states
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
    ///2 wait states
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY::Ws2
    }
    ///3 wait states
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == LATENCY::Ws3
    }
    ///4 wait states
    #[inline(always)]
    pub fn is_ws4(&self) -> bool {
        *self == LATENCY::Ws4
    }
    ///5 wait states
    #[inline(always)]
    pub fn is_ws5(&self) -> bool {
        *self == LATENCY::Ws5
    }
    ///6 wait states
    #[inline(always)]
    pub fn is_ws6(&self) -> bool {
        *self == LATENCY::Ws6
    }
    ///7 wait states
    #[inline(always)]
    pub fn is_ws7(&self) -> bool {
        *self == LATENCY::Ws7
    }
    ///8 wait states
    #[inline(always)]
    pub fn is_ws8(&self) -> bool {
        *self == LATENCY::Ws8
    }
    ///9 wait states
    #[inline(always)]
    pub fn is_ws9(&self) -> bool {
        *self == LATENCY::Ws9
    }
    ///10 wait states
    #[inline(always)]
    pub fn is_ws10(&self) -> bool {
        *self == LATENCY::Ws10
    }
    ///11 wait states
    #[inline(always)]
    pub fn is_ws11(&self) -> bool {
        *self == LATENCY::Ws11
    }
    ///12 wait states
    #[inline(always)]
    pub fn is_ws12(&self) -> bool {
        *self == LATENCY::Ws12
    }
    ///13 wait states
    #[inline(always)]
    pub fn is_ws13(&self) -> bool {
        *self == LATENCY::Ws13
    }
    ///14 wait states
    #[inline(always)]
    pub fn is_ws14(&self) -> bool {
        *self == LATENCY::Ws14
    }
    ///15 wait states
    #[inline(always)]
    pub fn is_ws15(&self) -> bool {
        *self == LATENCY::Ws15
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::FieldWriter<'a, REG, 4, LATENCY, crate::Safe>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///0 wait states
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    ///1 wait states
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
    ///2 wait states
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws2)
    }
    ///3 wait states
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws3)
    }
    ///4 wait states
    #[inline(always)]
    pub fn ws4(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws4)
    }
    ///5 wait states
    #[inline(always)]
    pub fn ws5(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws5)
    }
    ///6 wait states
    #[inline(always)]
    pub fn ws6(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws6)
    }
    ///7 wait states
    #[inline(always)]
    pub fn ws7(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws7)
    }
    ///8 wait states
    #[inline(always)]
    pub fn ws8(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws8)
    }
    ///9 wait states
    #[inline(always)]
    pub fn ws9(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws9)
    }
    ///10 wait states
    #[inline(always)]
    pub fn ws10(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws10)
    }
    ///11 wait states
    #[inline(always)]
    pub fn ws11(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws11)
    }
    ///12 wait states
    #[inline(always)]
    pub fn ws12(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws12)
    }
    ///13 wait states
    #[inline(always)]
    pub fn ws13(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws13)
    }
    ///14 wait states
    #[inline(always)]
    pub fn ws14(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws14)
    }
    ///15 wait states
    #[inline(always)]
    pub fn ws15(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws15)
    }
}
/**Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: Prefetch is disabled
    Disabled = 0,
    ///1: Prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    ///Prefetch is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
/**ART Accelerator Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTEN {
    ///0: ART Accelerator is disabled
    Disabled = 0,
    ///1: ART Accelerator is enabled
    Enabled = 1,
}
impl From<ARTEN> for bool {
    #[inline(always)]
    fn from(variant: ARTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ARTEN` reader - ART Accelerator Enable
pub type ARTEN_R = crate::BitReader<ARTEN>;
impl ARTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARTEN {
        match self.bits {
            false => ARTEN::Disabled,
            true => ARTEN::Enabled,
        }
    }
    ///ART Accelerator is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARTEN::Disabled
    }
    ///ART Accelerator is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARTEN::Enabled
    }
}
///Field `ARTEN` writer - ART Accelerator Enable
pub type ARTEN_W<'a, REG> = crate::BitWriter<'a, REG, ARTEN>;
impl<'a, REG> ARTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ART Accelerator is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARTEN::Disabled)
    }
    ///ART Accelerator is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARTEN::Enabled)
    }
}
/**ART Accelerator reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARTRST {
    ///0: Accelerator is not reset
    NoReset = 0,
    ///1: Accelerator is reset
    Reset = 1,
}
impl From<ARTRST> for bool {
    #[inline(always)]
    fn from(variant: ARTRST) -> Self {
        variant as u8 != 0
    }
}
///Field `ARTRST` reader - ART Accelerator reset
pub type ARTRST_R = crate::BitReader<ARTRST>;
impl ARTRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARTRST {
        match self.bits {
            false => ARTRST::NoReset,
            true => ARTRST::Reset,
        }
    }
    ///Accelerator is not reset
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == ARTRST::NoReset
    }
    ///Accelerator is reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ARTRST::Reset
    }
}
///Field `ARTRST` writer - ART Accelerator reset
pub type ARTRST_W<'a, REG> = crate::BitWriter<'a, REG, ARTRST>;
impl<'a, REG> ARTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Accelerator is not reset
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(ARTRST::NoReset)
    }
    ///Accelerator is reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(ARTRST::Reset)
    }
}
impl R {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ART Accelerator Enable
    #[inline(always)]
    pub fn arten(&self) -> ARTEN_R {
        ARTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - ART Accelerator reset
    #[inline(always)]
    pub fn artrst(&self) -> ARTRST_R {
        ARTRST_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("arten", &self.arten())
            .field("artrst", &self.artrst())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 8)
    }
    ///Bit 9 - ART Accelerator Enable
    #[inline(always)]
    pub fn arten(&mut self) -> ARTEN_W<'_, ACRrs> {
        ARTEN_W::new(self, 9)
    }
    ///Bit 11 - ART Accelerator reset
    #[inline(always)]
    pub fn artrst(&mut self) -> ARTRST_W<'_, ACRrs> {
        ARTRST_W::new(self, 11)
    }
}
/**Flash access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
