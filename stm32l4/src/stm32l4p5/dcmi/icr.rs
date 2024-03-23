#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Capture complete interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_ISC {
    #[doc = "1: Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<FRAME_ISC> for bool {
    #[inline(always)]
    fn from(variant: FRAME_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME_ISC` writer - Capture complete interrupt status clear"]
pub type FRAME_ISC_W<'a, REG> = crate::BitWriter<'a, REG, FRAME_ISC>;
impl<'a, REG> FRAME_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(FRAME_ISC::Clear)
    }
}
#[doc = "Overrun interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_ISC {
    #[doc = "1: Setting this bit clears the OVR_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<OVR_ISC> for bool {
    #[inline(always)]
    fn from(variant: OVR_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_ISC` writer - Overrun interrupt status clear"]
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, OVR_ISC>;
impl<'a, REG> OVR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the OVR_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVR_ISC::Clear)
    }
}
#[doc = "Synchronization error interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_ISC {
    #[doc = "1: Setting this bit clears the ERR_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<ERR_ISC> for bool {
    #[inline(always)]
    fn from(variant: ERR_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_ISC` writer - Synchronization error interrupt status clear"]
pub type ERR_ISC_W<'a, REG> = crate::BitWriter<'a, REG, ERR_ISC>;
impl<'a, REG> ERR_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the ERR_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ERR_ISC::Clear)
    }
}
#[doc = "Vertical synch interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_ISC {
    #[doc = "1: Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<VSYNC_ISC> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_ISC` writer - Vertical synch interrupt status clear"]
pub type VSYNC_ISC_W<'a, REG> = crate::BitWriter<'a, REG, VSYNC_ISC>;
impl<'a, REG> VSYNC_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VSYNC_ISC::Clear)
    }
}
#[doc = "line interrupt status clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_ISC {
    #[doc = "1: Setting this bit clears the LINE_RIS flag in the DCMI_RIS register"]
    Clear = 1,
}
impl From<LINE_ISC> for bool {
    #[inline(always)]
    fn from(variant: LINE_ISC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINE_ISC` writer - line interrupt status clear"]
pub type LINE_ISC_W<'a, REG> = crate::BitWriter<'a, REG, LINE_ISC>;
impl<'a, REG> LINE_ISC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Setting this bit clears the LINE_RIS flag in the DCMI_RIS register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LINE_ISC::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<ICRrs> {
        FRAME_ISC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn err_isc(&mut self) -> ERR_ISC_W<ICRrs> {
        ERR_ISC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synch interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<ICRrs> {
        VSYNC_ISC_W::new(self, 3)
    }
    #[doc = "Bit 4 - line interrupt status clear"]
    #[inline(always)]
    #[must_use]
    pub fn line_isc(&mut self) -> LINE_ISC_W<ICRrs> {
        LINE_ISC_W::new(self, 4)
    }
}
#[doc = "interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
