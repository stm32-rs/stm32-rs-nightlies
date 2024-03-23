#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "global interrupt flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1 {
    #[doc = "1: Clear the corresponding CGIFx flag"]
    Clear = 1,
}
impl From<GIF1> for bool {
    #[inline(always)]
    fn from(variant: GIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIF1` writer - global interrupt flag clear for channel 1"]
pub type GIF1_W<'a, REG> = crate::BitWriter<'a, REG, GIF1>;
impl<'a, REG> GIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding CGIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GIF1::Clear)
    }
}
#[doc = "transfer complete flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1 {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<TCIF1> for bool {
    #[inline(always)]
    fn from(variant: TCIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIF1` writer - transfer complete flag clear for channel 1"]
pub type TCIF1_W<'a, REG> = crate::BitWriter<'a, REG, TCIF1>;
impl<'a, REG> TCIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCIF1::Clear)
    }
}
#[doc = "half transfer flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1 {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<HTIF1> for bool {
    #[inline(always)]
    fn from(variant: HTIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIF1` writer - half transfer flag clear for channel 1"]
pub type HTIF1_W<'a, REG> = crate::BitWriter<'a, REG, HTIF1>;
impl<'a, REG> HTIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HTIF1::Clear)
    }
}
#[doc = "transfer error flag clear for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1 {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<TEIF1> for bool {
    #[inline(always)]
    fn from(variant: TEIF1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIF1` writer - transfer error flag clear for channel 1"]
pub type TEIF1_W<'a, REG> = crate::BitWriter<'a, REG, TEIF1>;
impl<'a, REG> TEIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TEIF1::Clear)
    }
}
#[doc = "Field `GIF2` writer - global interrupt flag clear for channel 2"]
pub use GIF1_W as GIF2_W;
#[doc = "Field `GIF3` writer - global interrupt flag clear for channel 3"]
pub use GIF1_W as GIF3_W;
#[doc = "Field `GIF4` writer - global interrupt flag clear for channel 4"]
pub use GIF1_W as GIF4_W;
#[doc = "Field `GIF5` writer - global interrupt flag clear for channel 5"]
pub use GIF1_W as GIF5_W;
#[doc = "Field `GIF6` writer - global interrupt flag clear for channel 6"]
pub use GIF1_W as GIF6_W;
#[doc = "Field `GIF7` writer - global interrupt flag clear for channel 7"]
pub use GIF1_W as GIF7_W;
#[doc = "Field `HTIF2` writer - half transfer flag clear for channel 2"]
pub use HTIF1_W as HTIF2_W;
#[doc = "Field `HTIF3` writer - half transfer flag clear for channel 3"]
pub use HTIF1_W as HTIF3_W;
#[doc = "Field `HTIF4` writer - half transfer flag clear for channel 4"]
pub use HTIF1_W as HTIF4_W;
#[doc = "Field `HTIF5` writer - half transfer flag clear for channel 5"]
pub use HTIF1_W as HTIF5_W;
#[doc = "Field `HTIF6` writer - half transfer flag clear for channel 6"]
pub use HTIF1_W as HTIF6_W;
#[doc = "Field `HTIF7` writer - half transfer flag clear for channel 7"]
pub use HTIF1_W as HTIF7_W;
#[doc = "Field `TCIF2` writer - transfer complete flag clear for channel 2"]
pub use TCIF1_W as TCIF2_W;
#[doc = "Field `TCIF3` writer - transfer complete flag clear for channel 3"]
pub use TCIF1_W as TCIF3_W;
#[doc = "Field `TCIF4` writer - transfer complete flag clear for channel 4"]
pub use TCIF1_W as TCIF4_W;
#[doc = "Field `TCIF5` writer - transfer complete flag clear for channel 5"]
pub use TCIF1_W as TCIF5_W;
#[doc = "Field `TCIF6` writer - transfer complete flag clear for channel 6"]
pub use TCIF1_W as TCIF6_W;
#[doc = "Field `TCIF7` writer - transfer complete flag clear for channel 7"]
pub use TCIF1_W as TCIF7_W;
#[doc = "Field `TEIF2` writer - transfer error flag clear for channel 2"]
pub use TEIF1_W as TEIF2_W;
#[doc = "Field `TEIF3` writer - transfer error flag clear for channel 3"]
pub use TEIF1_W as TEIF3_W;
#[doc = "Field `TEIF4` writer - transfer error flag clear for channel 4"]
pub use TEIF1_W as TEIF4_W;
#[doc = "Field `TEIF5` writer - transfer error flag clear for channel 5"]
pub use TEIF1_W as TEIF5_W;
#[doc = "Field `TEIF6` writer - transfer error flag clear for channel 6"]
pub use TEIF1_W as TEIF6_W;
#[doc = "Field `TEIF7` writer - transfer error flag clear for channel 7"]
pub use TEIF1_W as TEIF7_W;
impl W {
    #[doc = "Bit 0 - global interrupt flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn gif1(&mut self) -> GIF1_W<IFCRrs> {
        GIF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - transfer complete flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn tcif1(&mut self) -> TCIF1_W<IFCRrs> {
        TCIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - half transfer flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn htif1(&mut self) -> HTIF1_W<IFCRrs> {
        HTIF1_W::new(self, 2)
    }
    #[doc = "Bit 3 - transfer error flag clear for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn teif1(&mut self) -> TEIF1_W<IFCRrs> {
        TEIF1_W::new(self, 3)
    }
    #[doc = "Bit 4 - global interrupt flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn gif2(&mut self) -> GIF2_W<IFCRrs> {
        GIF2_W::new(self, 4)
    }
    #[doc = "Bit 5 - transfer complete flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn tcif2(&mut self) -> TCIF2_W<IFCRrs> {
        TCIF2_W::new(self, 5)
    }
    #[doc = "Bit 6 - half transfer flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn htif2(&mut self) -> HTIF2_W<IFCRrs> {
        HTIF2_W::new(self, 6)
    }
    #[doc = "Bit 7 - transfer error flag clear for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn teif2(&mut self) -> TEIF2_W<IFCRrs> {
        TEIF2_W::new(self, 7)
    }
    #[doc = "Bit 8 - global interrupt flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn gif3(&mut self) -> GIF3_W<IFCRrs> {
        GIF3_W::new(self, 8)
    }
    #[doc = "Bit 9 - transfer complete flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn tcif3(&mut self) -> TCIF3_W<IFCRrs> {
        TCIF3_W::new(self, 9)
    }
    #[doc = "Bit 10 - half transfer flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn htif3(&mut self) -> HTIF3_W<IFCRrs> {
        HTIF3_W::new(self, 10)
    }
    #[doc = "Bit 11 - transfer error flag clear for channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn teif3(&mut self) -> TEIF3_W<IFCRrs> {
        TEIF3_W::new(self, 11)
    }
    #[doc = "Bit 12 - global interrupt flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn gif4(&mut self) -> GIF4_W<IFCRrs> {
        GIF4_W::new(self, 12)
    }
    #[doc = "Bit 13 - transfer complete flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn tcif4(&mut self) -> TCIF4_W<IFCRrs> {
        TCIF4_W::new(self, 13)
    }
    #[doc = "Bit 14 - half transfer flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn htif4(&mut self) -> HTIF4_W<IFCRrs> {
        HTIF4_W::new(self, 14)
    }
    #[doc = "Bit 15 - transfer error flag clear for channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn teif4(&mut self) -> TEIF4_W<IFCRrs> {
        TEIF4_W::new(self, 15)
    }
    #[doc = "Bit 16 - global interrupt flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn gif5(&mut self) -> GIF5_W<IFCRrs> {
        GIF5_W::new(self, 16)
    }
    #[doc = "Bit 17 - transfer complete flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn tcif5(&mut self) -> TCIF5_W<IFCRrs> {
        TCIF5_W::new(self, 17)
    }
    #[doc = "Bit 18 - half transfer flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn htif5(&mut self) -> HTIF5_W<IFCRrs> {
        HTIF5_W::new(self, 18)
    }
    #[doc = "Bit 19 - transfer error flag clear for channel 5"]
    #[inline(always)]
    #[must_use]
    pub fn teif5(&mut self) -> TEIF5_W<IFCRrs> {
        TEIF5_W::new(self, 19)
    }
    #[doc = "Bit 20 - global interrupt flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn gif6(&mut self) -> GIF6_W<IFCRrs> {
        GIF6_W::new(self, 20)
    }
    #[doc = "Bit 21 - transfer complete flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn tcif6(&mut self) -> TCIF6_W<IFCRrs> {
        TCIF6_W::new(self, 21)
    }
    #[doc = "Bit 22 - half transfer flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn htif6(&mut self) -> HTIF6_W<IFCRrs> {
        HTIF6_W::new(self, 22)
    }
    #[doc = "Bit 23 - transfer error flag clear for channel 6"]
    #[inline(always)]
    #[must_use]
    pub fn teif6(&mut self) -> TEIF6_W<IFCRrs> {
        TEIF6_W::new(self, 23)
    }
    #[doc = "Bit 24 - global interrupt flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn gif7(&mut self) -> GIF7_W<IFCRrs> {
        GIF7_W::new(self, 24)
    }
    #[doc = "Bit 25 - transfer complete flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn tcif7(&mut self) -> TCIF7_W<IFCRrs> {
        TCIF7_W::new(self, 25)
    }
    #[doc = "Bit 26 - half transfer flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn htif7(&mut self) -> HTIF7_W<IFCRrs> {
        HTIF7_W::new(self, 26)
    }
    #[doc = "Bit 27 - transfer error flag clear for channel 7"]
    #[inline(always)]
    #[must_use]
    pub fn teif7(&mut self) -> TEIF7_W<IFCRrs> {
        TEIF7_W::new(self, 27)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
