#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN {
    #[doc = "0: Counter disabled"]
    Disabled = 0,
    #[doc = "1: Counter enabled"]
    Enabled = 1,
}
impl From<CEN> for bool {
    #[inline(always)]
    fn from(variant: CEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
pub type CEN_R = crate::BitReader<CEN>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CEN {
        match self.bits {
            false => CEN::Disabled,
            true => CEN::Enabled,
        }
    }
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN::Disabled
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN::Enabled
    }
}
#[doc = "Field `CEN` writer - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG, CEN>;
impl<'a, REG> CEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Disabled)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CEN::Enabled)
    }
}
#[doc = "Update disable This bit is set and cleared by software to enable/disable update interrupt (UEV) event generation. Counter overflow Setting the UG bit. Buffered registers are then loaded with their preload values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDIS {
    #[doc = "0: Update event enabled"]
    Enabled = 0,
    #[doc = "1: Update event disabled"]
    Disabled = 1,
}
impl From<UDIS> for bool {
    #[inline(always)]
    fn from(variant: UDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDIS` reader - Update disable This bit is set and cleared by software to enable/disable update interrupt (UEV) event generation. Counter overflow Setting the UG bit. Buffered registers are then loaded with their preload values."]
pub type UDIS_R = crate::BitReader<UDIS>;
impl UDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDIS {
        match self.bits {
            false => UDIS::Enabled,
            true => UDIS::Disabled,
        }
    }
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDIS::Enabled
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDIS::Disabled
    }
}
#[doc = "Field `UDIS` writer - Update disable This bit is set and cleared by software to enable/disable update interrupt (UEV) event generation. Counter overflow Setting the UG bit. Buffered registers are then loaded with their preload values."]
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG, UDIS>;
impl<'a, REG> UDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDIS::Enabled)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDIS::Disabled)
    }
}
#[doc = "Update request source This bit is set and cleared by software to select the update interrupt (UEV) sources. Counter overflow Setting the UG bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URS {
    #[doc = "0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    AnyEvent = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    CounterOnly = 1,
}
impl From<URS> for bool {
    #[inline(always)]
    fn from(variant: URS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `URS` reader - Update request source This bit is set and cleared by software to select the update interrupt (UEV) sources. Counter overflow Setting the UG bit"]
pub type URS_R = crate::BitReader<URS>;
impl URS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> URS {
        match self.bits {
            false => URS::AnyEvent,
            true => URS::CounterOnly,
        }
    }
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URS::AnyEvent
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == URS::CounterOnly
    }
}
#[doc = "Field `URS` writer - Update request source This bit is set and cleared by software to select the update interrupt (UEV) sources. Counter overflow Setting the UG bit"]
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG, URS>;
impl<'a, REG> URS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut crate::W<REG> {
        self.variant(URS::AnyEvent)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut crate::W<REG> {
        self.variant(URS::CounterOnly)
    }
}
#[doc = "One-pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPM {
    #[doc = "0: Counter is not stopped at update event"]
    Disabled = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    Enabled = 1,
}
impl From<OPM> for bool {
    #[inline(always)]
    fn from(variant: OPM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader<OPM>;
impl OPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OPM {
        match self.bits {
            false => OPM::Disabled,
            true => OPM::Enabled,
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPM::Disabled
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPM::Enabled
    }
}
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG, OPM>;
impl<'a, REG> OPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPM::Disabled)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OPM::Enabled)
    }
}
#[doc = "Auto-reload preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARPE {
    #[doc = "0: TIMx_APRR register is not buffered"]
    Disabled = 0,
    #[doc = "1: TIMx_APRR register is buffered"]
    Enabled = 1,
}
impl From<ARPE> for bool {
    #[inline(always)]
    fn from(variant: ARPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader<ARPE>;
impl ARPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ARPE {
        match self.bits {
            false => ARPE::Disabled,
            true => ARPE::Enabled,
        }
    }
    #[doc = "TIMx_APRR register is not buffered"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPE::Disabled
    }
    #[doc = "TIMx_APRR register is buffered"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPE::Enabled
    }
}
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG, ARPE>;
impl<'a, REG> ARPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMx_APRR register is not buffered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARPE::Disabled)
    }
    #[doc = "TIMx_APRR register is buffered"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ARPE::Enabled)
    }
}
#[doc = "Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and sampling clock used by the digital filters (TIx),\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKD {
    #[doc = "0: t_DTS = t_CK_INT"]
    Div1 = 0,
    #[doc = "1: t_DTS = 2 × t_CK_INT"]
    Div2 = 1,
    #[doc = "2: t_DTS = 4 × t_CK_INT"]
    Div4 = 2,
}
impl From<CKD> for u8 {
    #[inline(always)]
    fn from(variant: CKD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKD {
    type Ux = u8;
}
#[doc = "Field `CKD` reader - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and sampling clock used by the digital filters (TIx),"]
pub type CKD_R = crate::FieldReader<CKD>;
impl CKD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKD> {
        match self.bits {
            0 => Some(CKD::Div1),
            1 => Some(CKD::Div2),
            2 => Some(CKD::Div4),
            _ => None,
        }
    }
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CKD::Div1
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CKD::Div2
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CKD::Div4
    }
}
#[doc = "Field `CKD` writer - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and sampling clock used by the digital filters (TIx),"]
pub type CKD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKD>;
impl<'a, REG> CKD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "t_DTS = t_CK_INT"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CKD::Div1)
    }
    #[doc = "t_DTS = 2 × t_CK_INT"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CKD::Div2)
    }
    #[doc = "t_DTS = 4 × t_CK_INT"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CKD::Div4)
    }
}
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UIFREMAP_R = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UIFREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable update interrupt (UEV) event generation. Counter overflow Setting the UG bit. Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the update interrupt (UEV) sources. Counter overflow Setting the UG bit"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and sampling clock used by the digital filters (TIx),"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable Note: External clock and gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CR1rs> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable This bit is set and cleared by software to enable/disable update interrupt (UEV) event generation. Counter overflow Setting the UG bit. Buffered registers are then loaded with their preload values."]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<CR1rs> {
        UDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source This bit is set and cleared by software to select the update interrupt (UEV) sources. Counter overflow Setting the UG bit"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<CR1rs> {
        URS_W::new(self, 2)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<CR1rs> {
        OPM_W::new(self, 3)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<CR1rs> {
        ARPE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division This bit-field indicates the division ratio between the timer clock (CK_INT) frequency and sampling clock used by the digital filters (TIx),"]
    #[inline(always)]
    #[must_use]
    pub fn ckd(&mut self) -> CKD_W<CR1rs> {
        CKD_W::new(self, 8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<CR1rs> {
        UIFREMAP_W::new(self, 11)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
