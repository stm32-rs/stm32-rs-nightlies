///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
/**Channel %s Global interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGIF0 {
    ///1: Clear the corresponding CGIFx flag
    Clear = 1,
}
impl From<CGIF0> for bool {
    #[inline(always)]
    fn from(variant: CGIF0) -> Self {
        variant as u8 != 0
    }
}
///Field `CGIF(0-7)` writer - Channel %s Global interrupt clear
pub type CGIF_W<'a, REG> = crate::BitWriter<'a, REG, CGIF0>;
impl<'a, REG> CGIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding CGIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CGIF0::Clear)
    }
}
/**Channel %s Transfer Complete clear

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
///Field `CTCIF(0-7)` writer - Channel %s Transfer Complete clear
pub type CTCIF_W<'a, REG> = crate::BitWriter<'a, REG, CTCIF0>;
impl<'a, REG> CTCIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TCIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTCIF0::Clear)
    }
}
/**Channel %s Half Transfer clear

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
///Field `CHTIF(0-7)` writer - Channel %s Half Transfer clear
pub type CHTIF_W<'a, REG> = crate::BitWriter<'a, REG, CHTIF0>;
impl<'a, REG> CHTIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding HTIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CHTIF0::Clear)
    }
}
/**Channel %s Transfer Error clear

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
///Field `CTEIF(0-7)` writer - Channel %s Transfer Error clear
pub type CTEIF_W<'a, REG> = crate::BitWriter<'a, REG, CTEIF0>;
impl<'a, REG> CTEIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the corresponding TEIFx flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CTEIF0::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Channel (0-7) Global interrupt clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CGIF0` field.</div>
    #[inline(always)]
    pub fn cgif(&mut self, n: u8) -> CGIF_W<'_, IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CGIF_W::new(self, n * 4)
    }
    ///Bit 0 - Channel 0 Global interrupt clear
    #[inline(always)]
    pub fn cgif0(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 0)
    }
    ///Bit 4 - Channel 1 Global interrupt clear
    #[inline(always)]
    pub fn cgif1(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 4)
    }
    ///Bit 8 - Channel 2 Global interrupt clear
    #[inline(always)]
    pub fn cgif2(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 8)
    }
    ///Bit 12 - Channel 3 Global interrupt clear
    #[inline(always)]
    pub fn cgif3(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 12)
    }
    ///Bit 16 - Channel 4 Global interrupt clear
    #[inline(always)]
    pub fn cgif4(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 16)
    }
    ///Bit 20 - Channel 5 Global interrupt clear
    #[inline(always)]
    pub fn cgif5(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 20)
    }
    ///Bit 24 - Channel 6 Global interrupt clear
    #[inline(always)]
    pub fn cgif6(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 24)
    }
    ///Bit 28 - Channel 7 Global interrupt clear
    #[inline(always)]
    pub fn cgif7(&mut self) -> CGIF_W<'_, IFCRrs> {
        CGIF_W::new(self, 28)
    }
    ///Channel (0-7) Transfer Complete clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CTCIF0` field.</div>
    #[inline(always)]
    pub fn ctcif(&mut self, n: u8) -> CTCIF_W<'_, IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CTCIF_W::new(self, n * 4 + 1)
    }
    ///Bit 1 - Channel 0 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 1)
    }
    ///Bit 5 - Channel 1 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 5)
    }
    ///Bit 9 - Channel 2 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 9)
    }
    ///Bit 13 - Channel 3 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 13)
    }
    ///Bit 17 - Channel 4 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 17)
    }
    ///Bit 21 - Channel 5 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 21)
    }
    ///Bit 25 - Channel 6 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 25)
    }
    ///Bit 29 - Channel 7 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF_W<'_, IFCRrs> {
        CTCIF_W::new(self, 29)
    }
    ///Channel (0-7) Half Transfer clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHTIF0` field.</div>
    #[inline(always)]
    pub fn chtif(&mut self, n: u8) -> CHTIF_W<'_, IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CHTIF_W::new(self, n * 4 + 2)
    }
    ///Bit 2 - Channel 0 Half Transfer clear
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 2)
    }
    ///Bit 6 - Channel 1 Half Transfer clear
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 6)
    }
    ///Bit 10 - Channel 2 Half Transfer clear
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 10)
    }
    ///Bit 14 - Channel 3 Half Transfer clear
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 14)
    }
    ///Bit 18 - Channel 4 Half Transfer clear
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 18)
    }
    ///Bit 22 - Channel 5 Half Transfer clear
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 22)
    }
    ///Bit 26 - Channel 6 Half Transfer clear
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 26)
    }
    ///Bit 30 - Channel 7 Half Transfer clear
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF_W<'_, IFCRrs> {
        CHTIF_W::new(self, 30)
    }
    ///Channel (0-7) Transfer Error clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CTEIF0` field.</div>
    #[inline(always)]
    pub fn cteif(&mut self, n: u8) -> CTEIF_W<'_, IFCRrs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CTEIF_W::new(self, n * 4 + 3)
    }
    ///Bit 3 - Channel 0 Transfer Error clear
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 3)
    }
    ///Bit 7 - Channel 1 Transfer Error clear
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 7)
    }
    ///Bit 11 - Channel 2 Transfer Error clear
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 11)
    }
    ///Bit 15 - Channel 3 Transfer Error clear
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 15)
    }
    ///Bit 19 - Channel 4 Transfer Error clear
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 19)
    }
    ///Bit 23 - Channel 5 Transfer Error clear
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 23)
    }
    ///Bit 27 - Channel 6 Transfer Error clear
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 27)
    }
    ///Bit 31 - Channel 7 Transfer Error clear
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF_W<'_, IFCRrs> {
        CTEIF_W::new(self, 31)
    }
}
/**BDMA interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#BDMA:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
