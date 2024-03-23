#[doc = "Register `HIFCR` writer"]
pub type W = crate::W<HIFCRrs>;
#[doc = "Stream x clear FIFO error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFEIF4 {
    #[doc = "1: Clear the corresponding CFEIFx flag"]
    Clear = 1,
}
impl From<CFEIF4> for bool {
    #[inline(always)]
    fn from(variant: CFEIF4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEIF4` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CFEIF4>;
impl<'a, REG> CFEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding CFEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CFEIF4::Clear)
    }
}
#[doc = "Stream x clear direct mode error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDMEIF4 {
    #[doc = "1: Clear the corresponding DMEIFx flag"]
    Clear = 1,
}
impl From<CDMEIF4> for bool {
    #[inline(always)]
    fn from(variant: CDMEIF4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDMEIF4` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CDMEIF4>;
impl<'a, REG> CDMEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding DMEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CDMEIF4::Clear)
    }
}
#[doc = "Stream x clear transfer error interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF4 {
    #[doc = "1: Clear the corresponding TEIFx flag"]
    Clear = 1,
}
impl From<CTEIF4> for bool {
    #[inline(always)]
    fn from(variant: CTEIF4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTEIF4` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF4>;
impl<'a, REG> CTEIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TEIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF4::Clear)
    }
}
#[doc = "Stream x clear half transfer interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF4 {
    #[doc = "1: Clear the corresponding HTIFx flag"]
    Clear = 1,
}
impl From<CHTIF4> for bool {
    #[inline(always)]
    fn from(variant: CHTIF4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHTIF4` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG, CHTIF4>;
impl<'a, REG> CHTIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding HTIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CHTIF4::Clear)
    }
}
#[doc = "Stream x clear transfer complete interrupt flag (x = 7..4)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF4 {
    #[doc = "1: Clear the corresponding TCIFx flag"]
    Clear = 1,
}
impl From<CTCIF4> for bool {
    #[inline(always)]
    fn from(variant: CTCIF4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCIF4` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF4>;
impl<'a, REG> CTCIF4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding TCIFx flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF4::Clear)
    }
}
#[doc = "Field `CDMEIF5` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub use CDMEIF4_W as CDMEIF5_W;
#[doc = "Field `CDMEIF6` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub use CDMEIF4_W as CDMEIF6_W;
#[doc = "Field `CDMEIF7` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub use CDMEIF4_W as CDMEIF7_W;
#[doc = "Field `CFEIF5` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub use CFEIF4_W as CFEIF5_W;
#[doc = "Field `CFEIF6` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub use CFEIF4_W as CFEIF6_W;
#[doc = "Field `CFEIF7` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub use CFEIF4_W as CFEIF7_W;
#[doc = "Field `CHTIF5` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub use CHTIF4_W as CHTIF5_W;
#[doc = "Field `CHTIF6` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub use CHTIF4_W as CHTIF6_W;
#[doc = "Field `CHTIF7` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub use CHTIF4_W as CHTIF7_W;
#[doc = "Field `CTCIF5` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub use CTCIF4_W as CTCIF5_W;
#[doc = "Field `CTCIF6` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub use CTCIF4_W as CTCIF6_W;
#[doc = "Field `CTCIF7` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub use CTCIF4_W as CTCIF7_W;
#[doc = "Field `CTEIF5` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub use CTEIF4_W as CTEIF5_W;
#[doc = "Field `CTEIF6` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub use CTEIF4_W as CTEIF6_W;
#[doc = "Field `CTEIF7` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub use CTEIF4_W as CTEIF7_W;
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif4(&mut self) -> CFEIF4_W<HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif5(&mut self) -> CFEIF5_W<HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif6(&mut self) -> CFEIF6_W<HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif7(&mut self) -> CFEIF7_W<HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
#[doc = "high interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIFCRrs;
impl crate::RegisterSpec for HIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hifcr::W`](W) writer structure"]
impl crate::Writable for HIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIFCR to value 0"]
impl crate::Resettable for HIFCRrs {
    const RESET_VALUE: u32 = 0;
}
