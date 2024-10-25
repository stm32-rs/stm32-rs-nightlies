///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
/**global interrupt flag clear for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GIF1 {
    ///1: Clear the corresponding CGIFx flag
    Clear = 1,
}
impl From<GIF1> for bool {
    #[inline(always)]
    fn from(variant: GIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `GIF1` writer - global interrupt flag clear for channel 1
pub type GIF1_W<'a, REG> = crate::BitWriter<'a, REG, GIF1>;
impl<'a, REG> GIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding CGIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(GIF1::Clear)
    }
}
/**transfer complete flag clear for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIF1 {
    ///1: Clear the corresponding TCIFx flag
    Clear = 1,
}
impl From<TCIF1> for bool {
    #[inline(always)]
    fn from(variant: TCIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIF1` writer - transfer complete flag clear for channel 1
pub type TCIF1_W<'a, REG> = crate::BitWriter<'a, REG, TCIF1>;
impl<'a, REG> TCIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TCIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TCIF1::Clear)
    }
}
/**half transfer flag clear for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIF1 {
    ///1: Clear the corresponding HTIFx flag
    Clear = 1,
}
impl From<HTIF1> for bool {
    #[inline(always)]
    fn from(variant: HTIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIF1` writer - half transfer flag clear for channel 1
pub type HTIF1_W<'a, REG> = crate::BitWriter<'a, REG, HTIF1>;
impl<'a, REG> HTIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding HTIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(HTIF1::Clear)
    }
}
/**transfer error flag clear for channel 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIF1 {
    ///1: Clear the corresponding TEIFx flag
    Clear = 1,
}
impl From<TEIF1> for bool {
    #[inline(always)]
    fn from(variant: TEIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIF1` writer - transfer error flag clear for channel 1
pub type TEIF1_W<'a, REG> = crate::BitWriter<'a, REG, TEIF1>;
impl<'a, REG> TEIF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TEIF1::Clear)
    }
}
///Field `GIF2` writer - global interrupt flag clear for channel 2
pub use GIF1_W as GIF2_W;
///Field `GIF3` writer - global interrupt flag clear for channel 3
pub use GIF1_W as GIF3_W;
///Field `GIF4` writer - global interrupt flag clear for channel 4
pub use GIF1_W as GIF4_W;
///Field `GIF5` writer - global interrupt flag clear for channel 5
pub use GIF1_W as GIF5_W;
///Field `GIF6` writer - global interrupt flag clear for channel 6
pub use GIF1_W as GIF6_W;
///Field `GIF7` writer - global interrupt flag clear for channel 7
pub use GIF1_W as GIF7_W;
///Field `HTIF2` writer - half transfer flag clear for channel 2
pub use HTIF1_W as HTIF2_W;
///Field `HTIF3` writer - half transfer flag clear for channel 3
pub use HTIF1_W as HTIF3_W;
///Field `HTIF4` writer - half transfer flag clear for channel 4
pub use HTIF1_W as HTIF4_W;
///Field `HTIF5` writer - half transfer flag clear for channel 5
pub use HTIF1_W as HTIF5_W;
///Field `HTIF6` writer - half transfer flag clear for channel 6
pub use HTIF1_W as HTIF6_W;
///Field `HTIF7` writer - half transfer flag clear for channel 7
pub use HTIF1_W as HTIF7_W;
///Field `TCIF2` writer - transfer complete flag clear for channel 2
pub use TCIF1_W as TCIF2_W;
///Field `TCIF3` writer - transfer complete flag clear for channel 3
pub use TCIF1_W as TCIF3_W;
///Field `TCIF4` writer - transfer complete flag clear for channel 4
pub use TCIF1_W as TCIF4_W;
///Field `TCIF5` writer - transfer complete flag clear for channel 5
pub use TCIF1_W as TCIF5_W;
///Field `TCIF6` writer - transfer complete flag clear for channel 6
pub use TCIF1_W as TCIF6_W;
///Field `TCIF7` writer - transfer complete flag clear for channel 7
pub use TCIF1_W as TCIF7_W;
///Field `TEIF2` writer - transfer error flag clear for channel 2
pub use TEIF1_W as TEIF2_W;
///Field `TEIF3` writer - transfer error flag clear for channel 3
pub use TEIF1_W as TEIF3_W;
///Field `TEIF4` writer - transfer error flag clear for channel 4
pub use TEIF1_W as TEIF4_W;
///Field `TEIF5` writer - transfer error flag clear for channel 5
pub use TEIF1_W as TEIF5_W;
///Field `TEIF6` writer - transfer error flag clear for channel 6
pub use TEIF1_W as TEIF6_W;
///Field `TEIF7` writer - transfer error flag clear for channel 7
pub use TEIF1_W as TEIF7_W;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - global interrupt flag clear for channel 1
    #[inline(always)]
    #[must_use]
    pub fn gif1(&mut self) -> GIF1_W<IFCRrs> {
        GIF1_W::new(self, 0)
    }
    ///Bit 1 - transfer complete flag clear for channel 1
    #[inline(always)]
    #[must_use]
    pub fn tcif1(&mut self) -> TCIF1_W<IFCRrs> {
        TCIF1_W::new(self, 1)
    }
    ///Bit 2 - half transfer flag clear for channel 1
    #[inline(always)]
    #[must_use]
    pub fn htif1(&mut self) -> HTIF1_W<IFCRrs> {
        HTIF1_W::new(self, 2)
    }
    ///Bit 3 - transfer error flag clear for channel 1
    #[inline(always)]
    #[must_use]
    pub fn teif1(&mut self) -> TEIF1_W<IFCRrs> {
        TEIF1_W::new(self, 3)
    }
    ///Bit 4 - global interrupt flag clear for channel 2
    #[inline(always)]
    #[must_use]
    pub fn gif2(&mut self) -> GIF2_W<IFCRrs> {
        GIF2_W::new(self, 4)
    }
    ///Bit 5 - transfer complete flag clear for channel 2
    #[inline(always)]
    #[must_use]
    pub fn tcif2(&mut self) -> TCIF2_W<IFCRrs> {
        TCIF2_W::new(self, 5)
    }
    ///Bit 6 - half transfer flag clear for channel 2
    #[inline(always)]
    #[must_use]
    pub fn htif2(&mut self) -> HTIF2_W<IFCRrs> {
        HTIF2_W::new(self, 6)
    }
    ///Bit 7 - transfer error flag clear for channel 2
    #[inline(always)]
    #[must_use]
    pub fn teif2(&mut self) -> TEIF2_W<IFCRrs> {
        TEIF2_W::new(self, 7)
    }
    ///Bit 8 - global interrupt flag clear for channel 3
    #[inline(always)]
    #[must_use]
    pub fn gif3(&mut self) -> GIF3_W<IFCRrs> {
        GIF3_W::new(self, 8)
    }
    ///Bit 9 - transfer complete flag clear for channel 3
    #[inline(always)]
    #[must_use]
    pub fn tcif3(&mut self) -> TCIF3_W<IFCRrs> {
        TCIF3_W::new(self, 9)
    }
    ///Bit 10 - half transfer flag clear for channel 3
    #[inline(always)]
    #[must_use]
    pub fn htif3(&mut self) -> HTIF3_W<IFCRrs> {
        HTIF3_W::new(self, 10)
    }
    ///Bit 11 - transfer error flag clear for channel 3
    #[inline(always)]
    #[must_use]
    pub fn teif3(&mut self) -> TEIF3_W<IFCRrs> {
        TEIF3_W::new(self, 11)
    }
    ///Bit 12 - global interrupt flag clear for channel 4
    #[inline(always)]
    #[must_use]
    pub fn gif4(&mut self) -> GIF4_W<IFCRrs> {
        GIF4_W::new(self, 12)
    }
    ///Bit 13 - transfer complete flag clear for channel 4
    #[inline(always)]
    #[must_use]
    pub fn tcif4(&mut self) -> TCIF4_W<IFCRrs> {
        TCIF4_W::new(self, 13)
    }
    ///Bit 14 - half transfer flag clear for channel 4
    #[inline(always)]
    #[must_use]
    pub fn htif4(&mut self) -> HTIF4_W<IFCRrs> {
        HTIF4_W::new(self, 14)
    }
    ///Bit 15 - transfer error flag clear for channel 4
    #[inline(always)]
    #[must_use]
    pub fn teif4(&mut self) -> TEIF4_W<IFCRrs> {
        TEIF4_W::new(self, 15)
    }
    ///Bit 16 - global interrupt flag clear for channel 5
    #[inline(always)]
    #[must_use]
    pub fn gif5(&mut self) -> GIF5_W<IFCRrs> {
        GIF5_W::new(self, 16)
    }
    ///Bit 17 - transfer complete flag clear for channel 5
    #[inline(always)]
    #[must_use]
    pub fn tcif5(&mut self) -> TCIF5_W<IFCRrs> {
        TCIF5_W::new(self, 17)
    }
    ///Bit 18 - half transfer flag clear for channel 5
    #[inline(always)]
    #[must_use]
    pub fn htif5(&mut self) -> HTIF5_W<IFCRrs> {
        HTIF5_W::new(self, 18)
    }
    ///Bit 19 - transfer error flag clear for channel 5
    #[inline(always)]
    #[must_use]
    pub fn teif5(&mut self) -> TEIF5_W<IFCRrs> {
        TEIF5_W::new(self, 19)
    }
    ///Bit 20 - global interrupt flag clear for channel 6
    #[inline(always)]
    #[must_use]
    pub fn gif6(&mut self) -> GIF6_W<IFCRrs> {
        GIF6_W::new(self, 20)
    }
    ///Bit 21 - transfer complete flag clear for channel 6
    #[inline(always)]
    #[must_use]
    pub fn tcif6(&mut self) -> TCIF6_W<IFCRrs> {
        TCIF6_W::new(self, 21)
    }
    ///Bit 22 - half transfer flag clear for channel 6
    #[inline(always)]
    #[must_use]
    pub fn htif6(&mut self) -> HTIF6_W<IFCRrs> {
        HTIF6_W::new(self, 22)
    }
    ///Bit 23 - transfer error flag clear for channel 6
    #[inline(always)]
    #[must_use]
    pub fn teif6(&mut self) -> TEIF6_W<IFCRrs> {
        TEIF6_W::new(self, 23)
    }
    ///Bit 24 - global interrupt flag clear for channel 7
    #[inline(always)]
    #[must_use]
    pub fn gif7(&mut self) -> GIF7_W<IFCRrs> {
        GIF7_W::new(self, 24)
    }
    ///Bit 25 - transfer complete flag clear for channel 7
    #[inline(always)]
    #[must_use]
    pub fn tcif7(&mut self) -> TCIF7_W<IFCRrs> {
        TCIF7_W::new(self, 25)
    }
    ///Bit 26 - half transfer flag clear for channel 7
    #[inline(always)]
    #[must_use]
    pub fn htif7(&mut self) -> HTIF7_W<IFCRrs> {
        HTIF7_W::new(self, 26)
    }
    ///Bit 27 - transfer error flag clear for channel 7
    #[inline(always)]
    #[must_use]
    pub fn teif7(&mut self) -> TEIF7_W<IFCRrs> {
        TEIF7_W::new(self, 27)
    }
}
/**interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DMA1:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
