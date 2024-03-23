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
#[doc = "Field `WP2` reader - Wakeup pin WKUP2 polarity"]
pub use WP1_R as WP2_R;
#[doc = "Field `WP3` reader - Wakeup pin WKUP3 polarity"]
pub use WP1_R as WP3_R;
#[doc = "Field `WP4` reader - Wakeup pin WKUP4 polarity"]
pub use WP1_R as WP4_R;
#[doc = "Field `WP5` reader - Wakeup pin WKUP5 polarity"]
pub use WP1_R as WP5_R;
#[doc = "Field `WP2` writer - Wakeup pin WKUP2 polarity"]
pub use WP1_W as WP2_W;
#[doc = "Field `WP3` writer - Wakeup pin WKUP3 polarity"]
pub use WP1_W as WP3_W;
#[doc = "Field `WP4` writer - Wakeup pin WKUP4 polarity"]
pub use WP1_W as WP4_W;
#[doc = "Field `WP5` writer - Wakeup pin WKUP5 polarity"]
pub use WP1_W as WP5_W;
#[doc = "VBAT battery charging enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBE {
    #[doc = "0: VBAT battery charging disable"]
    Disabled = 0,
    #[doc = "1: VBAT battery charging enable"]
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
    #[doc = "VBAT battery charging disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBE::Disabled
    }
    #[doc = "VBAT battery charging enable"]
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
    #[doc = "VBAT battery charging disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Disabled)
    }
    #[doc = "VBAT battery charging enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBE::Enabled)
    }
}
#[doc = "VBAT battery charging resistor selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBRS {
    #[doc = "0: Charge VBAT through a 5 kOhms resistor"]
    R5k = 0,
    #[doc = "1: Charge VBAT through a 1.5 kOhms resistor"]
    R1k5 = 1,
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
            true => VBRS::R1k5,
        }
    }
    #[doc = "Charge VBAT through a 5 kOhms resistor"]
    #[inline(always)]
    pub fn is_r5k(&self) -> bool {
        *self == VBRS::R5k
    }
    #[doc = "Charge VBAT through a 1.5 kOhms resistor"]
    #[inline(always)]
    pub fn is_r1k5(&self) -> bool {
        *self == VBRS::R1k5
    }
}
#[doc = "Field `VBRS` writer - VBAT battery charging resistor selection"]
pub type VBRS_W<'a, REG> = crate::BitWriter<'a, REG, VBRS>;
impl<'a, REG> VBRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Charge VBAT through a 5 kOhms resistor"]
    #[inline(always)]
    pub fn r5k(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R5k)
    }
    #[doc = "Charge VBAT through a 1.5 kOhms resistor"]
    #[inline(always)]
    pub fn r1k5(self) -> &'a mut crate::W<REG> {
        self.variant(VBRS::R1k5)
    }
}
#[doc = "External SMPS on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_SMPS_ON {
    #[doc = "0: The external SMPS switch is open"]
    Disabled = 0,
    #[doc = "1: The external SMPS switch is closed, internal regulator output is set to 0.95 V"]
    Enabled = 1,
}
impl From<EXT_SMPS_ON> for bool {
    #[inline(always)]
    fn from(variant: EXT_SMPS_ON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXT_SMPS_ON` reader - External SMPS on"]
pub type EXT_SMPS_ON_R = crate::BitReader<EXT_SMPS_ON>;
impl EXT_SMPS_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EXT_SMPS_ON {
        match self.bits {
            false => EXT_SMPS_ON::Disabled,
            true => EXT_SMPS_ON::Enabled,
        }
    }
    #[doc = "The external SMPS switch is open"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXT_SMPS_ON::Disabled
    }
    #[doc = "The external SMPS switch is closed, internal regulator output is set to 0.95 V"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXT_SMPS_ON::Enabled
    }
}
#[doc = "Field `EXT_SMPS_ON` writer - External SMPS on"]
pub type EXT_SMPS_ON_W<'a, REG> = crate::BitWriter<'a, REG, EXT_SMPS_ON>;
impl<'a, REG> EXT_SMPS_ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The external SMPS switch is open"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_SMPS_ON::Disabled)
    }
    #[doc = "The external SMPS switch is closed, internal regulator output is set to 0.95 V"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_SMPS_ON::Enabled)
    }
}
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
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 13 - External SMPS on"]
    #[inline(always)]
    pub fn ext_smps_on(&self) -> EXT_SMPS_ON_R {
        EXT_SMPS_ON_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 3 - Wakeup pin WKUP4 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<CR4rs> {
        WP4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUP5 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> WP5_W<CR4rs> {
        WP5_W::new(self, 4)
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
    #[doc = "Bit 13 - External SMPS on"]
    #[inline(always)]
    #[must_use]
    pub fn ext_smps_on(&mut self) -> EXT_SMPS_ON_W<CR4rs> {
        EXT_SMPS_ON_W::new(self, 13)
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
