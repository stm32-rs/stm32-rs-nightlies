#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Clears the Line Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIFW {
    #[doc = "1: Clears the LIF flag in the ISR register"]
    Clear = 1,
}
impl From<CLIFW> for bool {
    #[inline(always)]
    fn from(variant: CLIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLIF` writer - Clears the Line Interrupt Flag"]
pub type CLIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLIFW>;
impl<'a, REG> CLIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the LIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLIFW::Clear)
    }
}
#[doc = "Clears the FIFO Underrun Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFUIFW {
    #[doc = "1: Clears the FUIF flag in the ISR register"]
    Clear = 1,
}
impl From<CFUIFW> for bool {
    #[inline(always)]
    fn from(variant: CFUIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag"]
pub type CFUIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CFUIFW>;
impl<'a, REG> CFUIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the FUIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFUIFW::Clear)
    }
}
#[doc = "Clears the Transfer Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTERRIFW {
    #[doc = "1: Clears the TERRIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTERRIFW> for bool {
    #[inline(always)]
    fn from(variant: CTERRIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag"]
pub type CTERRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CTERRIFW>;
impl<'a, REG> CTERRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TERRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTERRIFW::Clear)
    }
}
#[doc = "Clears Register Reload Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRRIFW {
    #[doc = "1: Clears the RRIF flag in the ISR register"]
    Clear = 1,
}
impl From<CRRIFW> for bool {
    #[inline(always)]
    fn from(variant: CRRIFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRRIF` writer - Clears Register Reload Interrupt Flag"]
pub type CRRIF_W<'a, REG> = crate::BitWriter1C<'a, REG, CRRIFW>;
impl<'a, REG> CRRIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the RRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CRRIFW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the Line Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<ICRrs> {
        CLIF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<ICRrs> {
        CFUIF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clears the Transfer Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<ICRrs> {
        CTERRIF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clears Register Reload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<ICRrs> {
        CRRIF_W::new(self, 3)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
