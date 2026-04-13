///Register `PWR_CR3` reader
pub type R = crate::R<PWR_CR3rs>;
///Register `PWR_CR3` writer
pub type W = crate::W<PWR_CR3rs>;
///Field `EWUP1` reader - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register.
pub type EWUP1_R = crate::BitReader;
///Field `EWUP1` writer - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register.
pub type EWUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP2` reader - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register.
pub type EWUP2_R = crate::BitReader;
///Field `EWUP2` writer - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register.
pub type EWUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP3` reader - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register.
pub type EWUP3_R = crate::BitReader;
///Field `EWUP3` writer - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register.
pub type EWUP3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP4` reader - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub type EWUP4_R = crate::BitReader;
///Field `EWUP4` writer - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
pub type EWUP4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP5` reader - Enable WKUP5 wakeup pin When this bit is set, the WKUP5 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP5 bit in the PWR_CR4 register.
pub type EWUP5_R = crate::BitReader;
///Field `EWUP5` writer - Enable WKUP5 wakeup pin When this bit is set, the WKUP5 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP5 bit in the PWR_CR4 register.
pub type EWUP5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWUP6` reader - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register.
pub type EWUP6_R = crate::BitReader;
///Field `EWUP6` writer - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register.
pub type EWUP6_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC {
    ///0: Not applied
    B0x0 = 0,
    ///1: Applied
    B0x1 = 1,
}
impl From<APC> for bool {
    #[inline(always)]
    fn from(variant: APC) -> Self {
        variant as u8 != 0
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied.
pub type APC_R = crate::BitReader<APC>;
impl APC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> APC {
        match self.bits {
            false => APC::B0x0,
            true => APC::B0x1,
        }
    }
    ///Not applied
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APC::B0x0
    }
    ///Applied
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APC::B0x1
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied.
pub type APC_W<'a, REG> = crate::BitWriter<'a, REG, APC>;
impl<'a, REG> APC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not applied
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(APC::B0x0)
    }
    ///Applied
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(APC::B0x1)
    }
}
/**Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL {
    ///0: Disable
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<EIWUL> for bool {
    #[inline(always)]
    fn from(variant: EIWUL) -> Self {
        variant as u8 != 0
    }
}
///Field `EIWUL` reader - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event.
pub type EIWUL_R = crate::BitReader<EIWUL>;
impl EIWUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIWUL {
        match self.bits {
            false => EIWUL::B0x0,
            true => EIWUL::B0x1,
        }
    }
    ///Disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EIWUL::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EIWUL::B0x1
    }
}
///Field `EIWUL` writer - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event.
pub type EIWUL_W<'a, REG> = crate::BitWriter<'a, REG, EIWUL>;
impl<'a, REG> EIWUL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EIWUL::B0x1)
    }
}
impl R {
    ///Bit 0 - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Enable WKUP5 wakeup pin When this bit is set, the WKUP5 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied.
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event.
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_CR3")
            .field("ewup1", &self.ewup1())
            .field("ewup2", &self.ewup2())
            .field("ewup3", &self.ewup3())
            .field("ewup4", &self.ewup4())
            .field("ewup5", &self.ewup5())
            .field("ewup6", &self.ewup6())
            .field("apc", &self.apc())
            .field("eiwul", &self.eiwul())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable WKUP1 wakeup pin When this bit is set, the WKUP1 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<'_, PWR_CR3rs> {
        EWUP1_W::new(self, 0)
    }
    ///Bit 1 - Enable WKUP2 wakeup pin When this bit is set, the WKUP2 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<'_, PWR_CR3rs> {
        EWUP2_W::new(self, 1)
    }
    ///Bit 2 - Enable WKUP3 wakeup pin When this bit is set, the WKUP3 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit of the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<'_, PWR_CR3rs> {
        EWUP3_W::new(self, 2)
    }
    ///Bit 3 - Enable WKUP4 wakeup pin When this bit is set, the WKUP4 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup4(&mut self) -> EWUP4_W<'_, PWR_CR3rs> {
        EWUP4_W::new(self, 3)
    }
    ///Bit 4 - Enable WKUP5 wakeup pin When this bit is set, the WKUP5 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP5 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup5(&mut self) -> EWUP5_W<'_, PWR_CR3rs> {
        EWUP5_W::new(self, 4)
    }
    ///Bit 5 - Enable WKUP6 wakeup pin When this bit is set, the WKUP6 external wakeup pin is enabled and triggers a wakeup event when a rising or a falling edge occurs. The active edge is configured through WP6 bit in the PWR_CR4 register.
    #[inline(always)]
    pub fn ewup6(&mut self) -> EWUP6_W<'_, PWR_CR3rs> {
        EWUP6_W::new(self, 5)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied.
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<'_, PWR_CR3rs> {
        APC_W::new(self, 10)
    }
    ///Bit 15 - Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event.
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<'_, PWR_CR3rs> {
        EIWUL_W::new(self, 15)
    }
}
/**PWR control register 3

You can [`read`](crate::Reg::read) this register and get [`pwr_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#PWR:PWR_CR3)*/
pub struct PWR_CR3rs;
impl crate::RegisterSpec for PWR_CR3rs {
    type Ux = u32;
}
///`read()` method returns [`pwr_cr3::R`](R) reader structure
impl crate::Readable for PWR_CR3rs {}
///`write(|w| ..)` method takes [`pwr_cr3::W`](W) writer structure
impl crate::Writable for PWR_CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PWR_CR3 to value 0x8000
impl crate::Resettable for PWR_CR3rs {
    const RESET_VALUE: u32 = 0x8000;
}
