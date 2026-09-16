///Register `LIFCR` writer
pub type W = crate::W<LIFCRrs>;
/**Stream x clear FIFO error interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEIF0 {
    ///1: Clear the corresponding CFEIFx flag
    Clear = 1,
}
impl From<CFEIF0> for bool {
    #[inline(always)]
    fn from(variant: CFEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub type CFEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CFEIF0>;
impl<'a, REG> CFEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding CFEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFEIF0::Clear)
    }
}
/**Stream x clear direct mode error interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDMEIF0 {
    ///1: Clear the corresponding DMEIFx flag
    Clear = 1,
}
impl From<CDMEIF0> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub type CDMEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CDMEIF0>;
impl<'a, REG> CDMEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding DMEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CDMEIF0::Clear)
    }
}
/**Stream x clear transfer error interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF0 {
    ///1: Clear the corresponding TEIFx flag
    Clear = 1,
}
impl From<CTEIF0> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF0>;
impl<'a, REG> CTEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF0::Clear)
    }
}
/**Stream x clear half transfer interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF0 {
    ///1: Clear the corresponding HTIFx flag
    Clear = 1,
}
impl From<CHTIF0> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG, CHTIF0>;
impl<'a, REG> CHTIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding HTIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CHTIF0::Clear)
    }
}
/**Stream x clear transfer complete interrupt flag (x = 3..0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF0 {
    ///1: Clear the corresponding TCIFx flag
    Clear = 1,
}
impl From<CTCIF0> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF0>;
impl<'a, REG> CTCIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TCIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF0::Clear)
    }
}
///Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub use CDMEIF0_W as CDMEIF1_W;
///Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub use CDMEIF0_W as CDMEIF2_W;
///Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)
pub use CDMEIF0_W as CDMEIF3_W;
///Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub use CFEIF0_W as CFEIF1_W;
///Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub use CFEIF0_W as CFEIF2_W;
///Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)
pub use CFEIF0_W as CFEIF3_W;
///Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub use CHTIF0_W as CHTIF1_W;
///Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub use CHTIF0_W as CHTIF2_W;
///Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)
pub use CHTIF0_W as CHTIF3_W;
///Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub use CTCIF0_W as CTCIF1_W;
///Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub use CTCIF0_W as CTCIF2_W;
///Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)
pub use CTCIF0_W as CTCIF3_W;
///Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub use CTEIF0_W as CTEIF1_W;
///Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub use CTEIF0_W as CTEIF2_W;
///Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)
pub use CTEIF0_W as CTEIF3_W;
impl core::fmt::Debug for crate::generic::Reg<LIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W<'_, LIFCRrs> {
        CFEIF0_W::new(self, 0)
    }
    ///Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<'_, LIFCRrs> {
        CDMEIF0_W::new(self, 2)
    }
    ///Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<'_, LIFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    ///Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<'_, LIFCRrs> {
        CHTIF0_W::new(self, 4)
    }
    ///Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<'_, LIFCRrs> {
        CTCIF0_W::new(self, 5)
    }
    ///Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W<'_, LIFCRrs> {
        CFEIF1_W::new(self, 6)
    }
    ///Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<'_, LIFCRrs> {
        CDMEIF1_W::new(self, 8)
    }
    ///Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<'_, LIFCRrs> {
        CTEIF1_W::new(self, 9)
    }
    ///Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<'_, LIFCRrs> {
        CHTIF1_W::new(self, 10)
    }
    ///Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<'_, LIFCRrs> {
        CTCIF1_W::new(self, 11)
    }
    ///Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W<'_, LIFCRrs> {
        CFEIF2_W::new(self, 16)
    }
    ///Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<'_, LIFCRrs> {
        CDMEIF2_W::new(self, 18)
    }
    ///Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<'_, LIFCRrs> {
        CTEIF2_W::new(self, 19)
    }
    ///Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<'_, LIFCRrs> {
        CHTIF2_W::new(self, 20)
    }
    ///Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<'_, LIFCRrs> {
        CTCIF2_W::new(self, 21)
    }
    ///Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W<'_, LIFCRrs> {
        CFEIF3_W::new(self, 22)
    }
    ///Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<'_, LIFCRrs> {
        CDMEIF3_W::new(self, 24)
    }
    ///Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<'_, LIFCRrs> {
        CTEIF3_W::new(self, 25)
    }
    ///Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<'_, LIFCRrs> {
        CHTIF3_W::new(self, 26)
    }
    ///Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<'_, LIFCRrs> {
        CTCIF3_W::new(self, 27)
    }
}
/**low interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#DMA2:LIFCR)*/
pub struct LIFCRrs;
impl crate::RegisterSpec for LIFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lifcr::W`](W) writer structure
impl crate::Writable for LIFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LIFCR to value 0
impl crate::Resettable for LIFCRrs {}
