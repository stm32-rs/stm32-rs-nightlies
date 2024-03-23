#[doc = "Register `CR4` reader"]
pub type R = crate::R<CR4rs>;
#[doc = "Register `CR4` writer"]
pub type W = crate::W<CR4rs>;
#[doc = "Wakeup pin WKUP1 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1 {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP1> for bool {
    #[inline(always)]
    fn from(variant: WP1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP1` reader - Wakeup pin WKUP1 polarity"]
pub type WP1_R = crate::BitReader<WP1>;
impl WP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP1 {
        match self.bits {
            false => WP1::RisingEdge,
            true => WP1::FallingEdge,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP1::RisingEdge
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP1::FallingEdge
    }
}
#[doc = "Field `WP1` writer - Wakeup pin WKUP1 polarity"]
pub type WP1_W<'a, REG> = crate::BitWriter<'a, REG, WP1>;
impl<'a, REG> WP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP1::FallingEdge)
    }
}
#[doc = "Wakeup pin WKUP2 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2 {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP2> for bool {
    #[inline(always)]
    fn from(variant: WP2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub type WP2_R = crate::BitReader<WP2>;
impl WP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP2 {
        match self.bits {
            false => WP2::RisingEdge,
            true => WP2::FallingEdge,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP2::RisingEdge
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP2::FallingEdge
    }
}
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub type WP2_W<'a, REG> = crate::BitWriter<'a, REG, WP2>;
impl<'a, REG> WP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP2::FallingEdge)
    }
}
#[doc = "Wakeup pin WKUP3 polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3 {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WP3> for bool {
    #[inline(always)]
    fn from(variant: WP3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub type WP3_R = crate::BitReader<WP3>;
impl WP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WP3 {
        match self.bits {
            false => WP3::RisingEdge,
            true => WP3::FallingEdge,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WP3::RisingEdge
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WP3::FallingEdge
    }
}
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub type WP3_W<'a, REG> = crate::BitWriter<'a, REG, WP3>;
impl<'a, REG> WP3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WP3::FallingEdge)
    }
}
#[doc = "VBAT battery charging enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    #[doc = "0: VBAT battery charging disabled"]
    Disabled = 0,
    #[doc = "1: VBAT battery charging enabled"]
    Enabled = 1,
}
impl From<VBE> for bool {
    #[inline(always)]
    fn from(variant: VBE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBE` reader - VBAT battery charging enable"]
pub type VBE_R = crate::BitReader<VBE>;
impl VBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBE {
        match self.bits {
            false => VBE::Disabled,
            true => VBE::Enabled,
        }
    }
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBE::Enabled
    }
}
#[doc = "Field `VBE` writer - VBAT battery charging enable"]
pub type VBE_W<'a, REG> = crate::BitWriter<'a, REG, VBE>;
impl<'a, REG> VBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT battery charging disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    #[doc = "VBAT battery charging enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
#[doc = "VBAT battery charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    #[doc = "0: VBAT charging through a 5 kΩ resistor"]
    R5k = 0,
    #[doc = "1: VBAT charging through a 1.5 kΩ resistor"]
    R1_5k = 1,
}
impl From<VBRS> for bool {
    #[inline(always)]
    fn from(variant: VBRS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBRS` reader - VBAT battery charging resistor selection"]
pub type VBRS_R = crate::BitReader<VBRS>;
impl VBRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBRS {
        match self.bits {
            false => VBRS::R5k,
            true => VBRS::R1_5k,
        }
    }
    #[doc = "VBAT charging through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == VBRS::R5k
    }
    #[doc = "VBAT charging through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn is_r1_5k(&self) -> bool {
        *self == VBRS::R1_5k
    }
}
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT charging through a 5 kΩ resistor"]
    #[inline(always)]
    pub fn r5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R5k)
    }
    #[doc = "VBAT charging through a 1.5 kΩ resistor"]
    #[inline(always)]
    pub fn r1_5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R1_5k)
    }
}
#[doc = "Wakeup Radio BUSY polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRFBUSYP {
    #[doc = "0: Detection on high level (rising edge)"]
    RisingEdge = 0,
    #[doc = "1: Detection on low level (falling edge)"]
    FallingEdge = 1,
}
impl From<WRFBUSYP> for bool {
    #[inline(always)]
    fn from(variant: WRFBUSYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRFBUSYP` reader - Wakeup Radio BUSY polarity"]
pub type WRFBUSYP_R = crate::BitReader<WRFBUSYP>;
impl WRFBUSYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WRFBUSYP {
        match self.bits {
            false => WRFBUSYP::RisingEdge,
            true => WRFBUSYP::FallingEdge,
        }
    }
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == WRFBUSYP::RisingEdge
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == WRFBUSYP::FallingEdge
    }
}
#[doc = "Field `WRFBUSYP` writer - Wakeup Radio BUSY polarity"]
pub type WRFBUSYP_W<'a, REG> = crate::BitWriter<'a, REG, WRFBUSYP>;
impl<'a, REG> WRFBUSYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Detection on high level (rising edge)"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WRFBUSYP::RisingEdge)
    }
    #[doc = "Detection on low level (falling edge)"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(WRFBUSYP::FallingEdge)
    }
}
#[doc = "Field `C2BOOT` reader - oot CPU2 after reset or wakeup from Stop or Standby modes."]
pub type C2BOOT_R = crate::BitReader;
#[doc = "Field `C2BOOT` writer - oot CPU2 after reset or wakeup from Stop or Standby modes."]
pub type C2BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    pub fn wrfbusyp(&self) -> WRFBUSYP_R {
        WRFBUSYP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - oot CPU2 after reset or wakeup from Stop or Standby modes."]
    #[inline(always)]
    pub fn c2boot(&self) -> C2BOOT_R {
        C2BOOT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUP1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<CR4rs> {
        WP1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUP2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<CR4rs> {
        WP2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUP3 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<CR4rs> {
        WP3_W::new(self, 2)
    }
    #[doc = "Bit 8 - VBAT battery charging enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<CR4rs> {
        VBE_W::new(self, 8)
    }
    #[doc = "Bit 9 - VBAT battery charging resistor selection"]
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<CR4rs> {
        VBRS_W::new(self, 9)
    }
    #[doc = "Bit 11 - Wakeup Radio BUSY polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wrfbusyp(&mut self) -> WRFBUSYP_W<CR4rs> {
        WRFBUSYP_W::new(self, 11)
    }
    #[doc = "Bit 15 - oot CPU2 after reset or wakeup from Stop or Standby modes."]
    #[inline(always)]
    #[must_use]
    pub fn c2boot(&mut self) -> C2BOOT_W<CR4rs> {
        C2BOOT_W::new(self, 15)
    }
}
#[doc = "Power control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR4rs;
impl crate::RegisterSpec for CR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for CR4rs {}
#[doc = "`write(|w| ..)` method takes [`cr4::W`](W) writer structure"]
impl crate::Writable for CR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for CR4rs {
    const RESET_VALUE: u32 = 0;
}
