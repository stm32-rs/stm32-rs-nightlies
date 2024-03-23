#[doc = "Register `LIFCR` writer"]
pub type W = crate::W<LIFCRrs>;
#[doc = "Stream x clear FIFO error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEIF0 {
    #[doc = "1: Clear the corresponding CFEIFx flag"]
    Clear = 1,
}
impl From<CFEIF0> for bool {
    #[inline(always)]
    fn from(variant: CFEIF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub type CFEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CFEIF0>;
impl<'a, REG> CFEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFEIF0::Clear)
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDMEIF0 {
    #[doc = "1: Clear the corresponding DMEIFx flag"]
    Clear = 1,
}
impl From<CDMEIF0> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub type CDMEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CDMEIF0>;
impl<'a, REG> CDMEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CDMEIF0::Clear)
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF0 {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<CTEIF0> for bool {
    #[inline(always)]
    fn from(variant: CTEIF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF0>;
impl<'a, REG> CTEIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF0::Clear)
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF0 {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<CHTIF0> for bool {
    #[inline(always)]
    fn from(variant: CHTIF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG, CHTIF0>;
impl<'a, REG> CHTIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CHTIF0::Clear)
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 3..0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF0 {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<CTCIF0> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF0>;
impl<'a, REG> CTCIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF0::Clear)
    }
}
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF1_W;
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF2_W;
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub use CDMEIF0_W as CDMEIF3_W;
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF1_W;
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF2_W;
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub use CFEIF0_W as CFEIF3_W;
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF1_W;
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF2_W;
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub use CHTIF0_W as CHTIF3_W;
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF1_W;
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF2_W;
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub use CTCIF0_W as CTCIF3_W;
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF1_W;
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF2_W;
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub use CTEIF0_W as CTEIF3_W;
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif0(&mut self) -> CFEIF0_W<LIFCRrs> {
        CFEIF0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<LIFCRrs> {
        CDMEIF0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif0(&mut self) -> CTEIF0_W<LIFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif0(&mut self) -> CHTIF0_W<LIFCRrs> {
        CHTIF0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif0(&mut self) -> CTCIF0_W<LIFCRrs> {
        CTCIF0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif1(&mut self) -> CFEIF1_W<LIFCRrs> {
        CFEIF1_W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<LIFCRrs> {
        CDMEIF1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<LIFCRrs> {
        CTEIF1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<LIFCRrs> {
        CHTIF1_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<LIFCRrs> {
        CTCIF1_W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif2(&mut self) -> CFEIF2_W<LIFCRrs> {
        CFEIF2_W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<LIFCRrs> {
        CDMEIF2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<LIFCRrs> {
        CTEIF2_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<LIFCRrs> {
        CHTIF2_W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<LIFCRrs> {
        CTCIF2_W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif3(&mut self) -> CFEIF3_W<LIFCRrs> {
        CFEIF3_W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<LIFCRrs> {
        CDMEIF3_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<LIFCRrs> {
        CTEIF3_W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<LIFCRrs> {
        CHTIF3_W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<LIFCRrs> {
        CTCIF3_W::new(self, 27)
    }
}
#[doc = "low interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIFCRrs;
impl crate::RegisterSpec for LIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lifcr::W`](W) writer structure"]
impl crate::Writable for LIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LIFCRrs {
    const RESET_VALUE: u32 = 0;
}
