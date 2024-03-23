#[doc = "Register `OPAMP2_TCMR` reader"]
pub type R = crate::R<OPAMP2_TCMRrs>;
#[doc = "Register `OPAMP2_TCMR` writer"]
pub type W = crate::W<OPAMP2_TCMRrs>;
#[doc = "Field `VMS_SEL` reader - VMS_SEL"]
pub type VMS_SEL_R = crate::BitReader;
#[doc = "Field `VMS_SEL` writer - VMS_SEL"]
pub type VMS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "VPS_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPS_SEL {
    #[doc = "0: VINP0 connected to VINP input"]
    Vinp0 = 0,
    #[doc = "1: VINP1 connected to VINP input"]
    Vinp1 = 1,
    #[doc = "2: VINP2 connected to VINP input"]
    Vinp2 = 2,
    #[doc = "3: VINP3 connected to VINP input"]
    Vinp3 = 3,
}
impl From<VPS_SEL> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VPS_SEL {
    type Ux = u8;
}
#[doc = "Field `VPS_SEL` reader - VPS_SEL"]
pub type VPS_SEL_R = crate::FieldReader<VPS_SEL>;
impl VPS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VPS_SEL {
        match self.bits {
            0 => VPS_SEL::Vinp0,
            1 => VPS_SEL::Vinp1,
            2 => VPS_SEL::Vinp2,
            3 => VPS_SEL::Vinp3,
            _ => unreachable!(),
        }
    }
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VPS_SEL::Vinp0
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VPS_SEL::Vinp1
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VPS_SEL::Vinp2
    }
    #[doc = "VINP3 connected to VINP input"]
    #[inline(always)]
    pub fn is_vinp3(&self) -> bool {
        *self == VPS_SEL::Vinp3
    }
}
#[doc = "Field `VPS_SEL` writer - VPS_SEL"]
pub type VPS_SEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VPS_SEL>;
impl<'a, REG> VPS_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Vinp0)
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Vinp1)
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Vinp2)
    }
    #[doc = "VINP3 connected to VINP input"]
    #[inline(always)]
    pub fn vinp3(self) -> &'a mut crate::W<REG> {
        self.variant(VPS_SEL::Vinp3)
    }
}
#[doc = "T1CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1CM_EN {
    #[doc = "0: Automatic input switch triggered by TIM1 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM1 enabled"]
    Enabled = 1,
}
impl From<T1CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T1CM_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T1CM_EN` reader - T1CM_EN"]
pub type T1CM_EN_R = crate::BitReader<T1CM_EN>;
impl T1CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> T1CM_EN {
        match self.bits {
            false => T1CM_EN::Disabled,
            true => T1CM_EN::Enabled,
        }
    }
    #[doc = "Automatic input switch triggered by TIM1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T1CM_EN::Disabled
    }
    #[doc = "Automatic input switch triggered by TIM1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T1CM_EN::Enabled
    }
}
#[doc = "Field `T1CM_EN` writer - T1CM_EN"]
pub type T1CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T1CM_EN>;
impl<'a, REG> T1CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic input switch triggered by TIM1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T1CM_EN::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T1CM_EN::Enabled)
    }
}
#[doc = "T8CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T8CM_EN {
    #[doc = "0: Automatic input switch triggered by TIM8 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM8 enabled"]
    Enabled = 1,
}
impl From<T8CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T8CM_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T8CM_EN` reader - T8CM_EN"]
pub type T8CM_EN_R = crate::BitReader<T8CM_EN>;
impl T8CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> T8CM_EN {
        match self.bits {
            false => T8CM_EN::Disabled,
            true => T8CM_EN::Enabled,
        }
    }
    #[doc = "Automatic input switch triggered by TIM8 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T8CM_EN::Disabled
    }
    #[doc = "Automatic input switch triggered by TIM8 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T8CM_EN::Enabled
    }
}
#[doc = "Field `T8CM_EN` writer - T8CM_EN"]
pub type T8CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T8CM_EN>;
impl<'a, REG> T8CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic input switch triggered by TIM8 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T8CM_EN::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM8 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T8CM_EN::Enabled)
    }
}
#[doc = "T20CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T20CM_EN {
    #[doc = "0: Automatic input switch triggered by TIM20 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM20 enabled"]
    Enabled = 1,
}
impl From<T20CM_EN> for bool {
    #[inline(always)]
    fn from(variant: T20CM_EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T20CM_EN` reader - T20CM_EN"]
pub type T20CM_EN_R = crate::BitReader<T20CM_EN>;
impl T20CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> T20CM_EN {
        match self.bits {
            false => T20CM_EN::Disabled,
            true => T20CM_EN::Enabled,
        }
    }
    #[doc = "Automatic input switch triggered by TIM20 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T20CM_EN::Disabled
    }
    #[doc = "Automatic input switch triggered by TIM20 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T20CM_EN::Enabled
    }
}
#[doc = "Field `T20CM_EN` writer - T20CM_EN"]
pub type T20CM_EN_W<'a, REG> = crate::BitWriter<'a, REG, T20CM_EN>;
impl<'a, REG> T20CM_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic input switch triggered by TIM20 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(T20CM_EN::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM20 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(T20CM_EN::Enabled)
    }
}
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    #[doc = "0: TCMR is read-write"]
    ReadWrite = 0,
    #[doc = "1: TCMR is read-only, can only be cleared by system reset"]
    ReadOnly = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            false => LOCK::ReadWrite,
            true => LOCK::ReadOnly,
        }
    }
    #[doc = "TCMR is read-write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK::ReadWrite
    }
    #[doc = "TCMR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK::ReadOnly
    }
}
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCMR is read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadWrite)
    }
    #[doc = "TCMR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<OPAMP2_TCMRrs> {
        VMS_SEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<OPAMP2_TCMRrs> {
        VPS_SEL_W::new(self, 1)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W<OPAMP2_TCMRrs> {
        T1CM_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W<OPAMP2_TCMRrs> {
        T8CM_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W<OPAMP2_TCMRrs> {
        T20CM_EN_W::new(self, 5)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<OPAMP2_TCMRrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_tcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_tcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPAMP2_TCMRrs;
impl crate::RegisterSpec for OPAMP2_TCMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_tcmr::R`](R) reader structure"]
impl crate::Readable for OPAMP2_TCMRrs {}
#[doc = "`write(|w| ..)` method takes [`opamp2_tcmr::W`](W) writer structure"]
impl crate::Writable for OPAMP2_TCMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPAMP2_TCMR to value 0"]
impl crate::Resettable for OPAMP2_TCMRrs {
    const RESET_VALUE: u32 = 0;
}
