///Register `HIFCR` writer
pub type W = crate::W<HIFCRrs>;
/**Stream x clear FIFO error interrupt flag (x = 7..4)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEIF4 {
    ///1: Clear the corresponding CFEIFx flag
    Clear = 1,
}
impl From<CFEIF4> for bool {
    #[inline(always)]
    fn from(variant: CFEIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `CFEIF4` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CFEIF4>;
impl<'a, REG> CFEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding CFEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFEIF4::Clear)
    }
}
/**Stream x clear direct mode error interrupt flag (x = 7..4)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDMEIF4 {
    ///1: Clear the corresponding DMEIFx flag
    Clear = 1,
}
impl From<CDMEIF4> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `CDMEIF4` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CDMEIF4>;
impl<'a, REG> CDMEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding DMEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CDMEIF4::Clear)
    }
}
/**Stream x clear transfer error interrupt flag (x = 7..4)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF4 {
    ///1: Clear the corresponding TEIFx flag
    Clear = 1,
}
impl From<CTEIF4> for bool {
    #[inline(always)]
    fn from(variant: CTEIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF4` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF4>;
impl<'a, REG> CTEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF4::Clear)
    }
}
/**Stream x clear half transfer interrupt flag (x = 7..4)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF4 {
    ///1: Clear the corresponding HTIFx flag
    Clear = 1,
}
impl From<CHTIF4> for bool {
    #[inline(always)]
    fn from(variant: CHTIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `CHTIF4` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG, CHTIF4>;
impl<'a, REG> CHTIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding HTIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CHTIF4::Clear)
    }
}
/**Stream x clear transfer complete interrupt flag (x = 7..4)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF4 {
    ///1: Clear the corresponding TCIFx flag
    Clear = 1,
}
impl From<CTCIF4> for bool {
    #[inline(always)]
    fn from(variant: CTCIF4) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF4` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF4>;
impl<'a, REG> CTCIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TCIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF4::Clear)
    }
}
///Field `CDMEIF5` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub use CDMEIF4_W as CDMEIF5_W;
///Field `CDMEIF6` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub use CDMEIF4_W as CDMEIF6_W;
///Field `CDMEIF7` writer - Stream x clear direct mode error interrupt flag (x = 7..4)
pub use CDMEIF4_W as CDMEIF7_W;
///Field `CFEIF5` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub use CFEIF4_W as CFEIF5_W;
///Field `CFEIF6` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub use CFEIF4_W as CFEIF6_W;
///Field `CFEIF7` writer - Stream x clear FIFO error interrupt flag (x = 7..4)
pub use CFEIF4_W as CFEIF7_W;
///Field `CHTIF5` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub use CHTIF4_W as CHTIF5_W;
///Field `CHTIF6` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub use CHTIF4_W as CHTIF6_W;
///Field `CHTIF7` writer - Stream x clear half transfer interrupt flag (x = 7..4)
pub use CHTIF4_W as CHTIF7_W;
///Field `CTCIF5` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub use CTCIF4_W as CTCIF5_W;
///Field `CTCIF6` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub use CTCIF4_W as CTCIF6_W;
///Field `CTCIF7` writer - Stream x clear transfer complete interrupt flag (x = 7..4)
pub use CTCIF4_W as CTCIF7_W;
///Field `CTEIF5` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub use CTEIF4_W as CTEIF5_W;
///Field `CTEIF6` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub use CTEIF4_W as CTEIF6_W;
///Field `CTEIF7` writer - Stream x clear transfer error interrupt flag (x = 7..4)
pub use CTEIF4_W as CTEIF7_W;
impl core::fmt::Debug for crate::generic::Reg<HIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W<'_, HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<'_, HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<'_, HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<'_, HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<'_, HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W<'_, HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<'_, HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<'_, HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<'_, HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<'_, HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W<'_, HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<'_, HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<'_, HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<'_, HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<'_, HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W<'_, HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<'_, HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<'_, HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<'_, HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<'_, HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
/**high interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DMA2:HIFCR)*/
pub struct HIFCRrs;
impl crate::RegisterSpec for HIFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`hifcr::W`](W) writer structure
impl crate::Writable for HIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HIFCR to value 0
impl crate::Resettable for HIFCRrs {}
