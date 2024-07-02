///Register `IFCR` reader
pub type R = crate::R<IFCRrs>;
/**Channel %s Global interrupt clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGIF1 {
    ///1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    Clear = 1,
}
impl From<CGIF1> for bool {
    #[inline(always)]
    fn from(variant: CGIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CGIF(1-5)` reader - Channel %s Global interrupt clear
pub type CGIF_R = crate::BitReader<CGIF1>;
impl CGIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CGIF1> {
        match self.bits {
            true => Some(CGIF1::Clear),
            _ => None,
        }
    }
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CGIF1::Clear
    }
}
/**Channel %s Transfer Complete clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF1 {
    ///1: Clears the TCIF flag in the ISR register
    Clear = 1,
}
impl From<CTCIF1> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF(1-5)` reader - Channel %s Transfer Complete clear
pub type CTCIF_R = crate::BitReader<CTCIF1>;
impl CTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTCIF1> {
        match self.bits {
            true => Some(CTCIF1::Clear),
            _ => None,
        }
    }
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF1::Clear
    }
}
/**Channel %s Half Transfer clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF1 {
    ///1: Clears the HTIF flag in the ISR register
    Clear = 1,
}
impl From<CHTIF1> for bool {
    #[inline(always)]
    fn from(variant: CHTIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CHTIF(1-5)` reader - Channel %s Half Transfer clear
pub type CHTIF_R = crate::BitReader<CHTIF1>;
impl CHTIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CHTIF1> {
        match self.bits {
            true => Some(CHTIF1::Clear),
            _ => None,
        }
    }
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CHTIF1::Clear
    }
}
/**Channel %s Transfer Error clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF1 {
    ///1: Clears the TEIF flag in the ISR register
    Clear = 1,
}
impl From<CTEIF1> for bool {
    #[inline(always)]
    fn from(variant: CTEIF1) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF(1-5)` reader - Channel %s Transfer Error clear
pub type CTEIF_R = crate::BitReader<CTEIF1>;
impl CTEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTEIF1> {
        match self.bits {
            true => Some(CTEIF1::Clear),
            _ => None,
        }
    }
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF1::Clear
    }
}
impl R {
    ///Channel (1-5) Global interrupt clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CGIF1` field.</div>
    #[inline(always)]
    pub fn cgif(&self, n: u8) -> CGIF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        CGIF_R::new(((self.bits >> (n * 4)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (1-5) Global interrupt clear
    #[inline(always)]
    pub fn cgif_iter(&self) -> impl Iterator<Item = CGIF_R> + '_ {
        (0..5).map(move |n| CGIF_R::new(((self.bits >> (n * 4)) & 1) != 0))
    }
    ///Bit 0 - Channel 1 Global interrupt clear
    #[inline(always)]
    pub fn cgif1(&self) -> CGIF_R {
        CGIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - Channel 2 Global interrupt clear
    #[inline(always)]
    pub fn cgif2(&self) -> CGIF_R {
        CGIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - Channel 3 Global interrupt clear
    #[inline(always)]
    pub fn cgif3(&self) -> CGIF_R {
        CGIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - Channel 4 Global interrupt clear
    #[inline(always)]
    pub fn cgif4(&self) -> CGIF_R {
        CGIF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Channel 5 Global interrupt clear
    #[inline(always)]
    pub fn cgif5(&self) -> CGIF_R {
        CGIF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Channel (1-5) Transfer Complete clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CTCIF1` field.</div>
    #[inline(always)]
    pub fn ctcif(&self, n: u8) -> CTCIF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        CTCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (1-5) Transfer Complete clear
    #[inline(always)]
    pub fn ctcif_iter(&self) -> impl Iterator<Item = CTCIF_R> + '_ {
        (0..5).map(move |n| CTCIF_R::new(((self.bits >> (n * 4 + 1)) & 1) != 0))
    }
    ///Bit 1 - Channel 1 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - Channel 2 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - Channel 3 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - Channel 4 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 17 - Channel 5 Transfer Complete clear
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Channel (1-5) Half Transfer clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CHTIF1` field.</div>
    #[inline(always)]
    pub fn chtif(&self, n: u8) -> CHTIF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        CHTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (1-5) Half Transfer clear
    #[inline(always)]
    pub fn chtif_iter(&self) -> impl Iterator<Item = CHTIF_R> + '_ {
        (0..5).map(move |n| CHTIF_R::new(((self.bits >> (n * 4 + 2)) & 1) != 0))
    }
    ///Bit 2 - Channel 1 Half Transfer clear
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF_R {
        CHTIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Channel 2 Half Transfer clear
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF_R {
        CHTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Channel 3 Half Transfer clear
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF_R {
        CHTIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Channel 4 Half Transfer clear
    #[inline(always)]
    pub fn chtif4(&self) -> CHTIF_R {
        CHTIF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - Channel 5 Half Transfer clear
    #[inline(always)]
    pub fn chtif5(&self) -> CHTIF_R {
        CHTIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Channel (1-5) Transfer Error clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CTEIF1` field.</div>
    #[inline(always)]
    pub fn cteif(&self, n: u8) -> CTEIF_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        CTEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Channel (1-5) Transfer Error clear
    #[inline(always)]
    pub fn cteif_iter(&self) -> impl Iterator<Item = CTEIF_R> + '_ {
        (0..5).map(move |n| CTEIF_R::new(((self.bits >> (n * 4 + 3)) & 1) != 0))
    }
    ///Bit 3 - Channel 1 Transfer Error clear
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF_R {
        CTEIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Channel 2 Transfer Error clear
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF_R {
        CTEIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - Channel 3 Transfer Error clear
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF_R {
        CTEIF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Channel 4 Transfer Error clear
    #[inline(always)]
    pub fn cteif4(&self) -> CTEIF_R {
        CTEIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - Channel 5 Transfer Error clear
    #[inline(always)]
    pub fn cteif5(&self) -> CTEIF_R {
        CTEIF_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFCR")
            .field("cgif1", &self.cgif1())
            .field("cgif2", &self.cgif2())
            .field("cgif3", &self.cgif3())
            .field("cgif4", &self.cgif4())
            .field("cgif5", &self.cgif5())
            .field("ctcif1", &self.ctcif1())
            .field("ctcif2", &self.ctcif2())
            .field("ctcif3", &self.ctcif3())
            .field("ctcif4", &self.ctcif4())
            .field("ctcif5", &self.ctcif5())
            .field("chtif1", &self.chtif1())
            .field("chtif2", &self.chtif2())
            .field("chtif3", &self.chtif3())
            .field("chtif4", &self.chtif4())
            .field("chtif5", &self.chtif5())
            .field("cteif1", &self.cteif1())
            .field("cteif2", &self.cteif2())
            .field("cteif3", &self.cteif3())
            .field("cteif4", &self.cteif4())
            .field("cteif5", &self.cteif5())
            .finish()
    }
}
/**DMA interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#DMA2:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ifcr::R`](R) reader structure
impl crate::Readable for IFCRrs {}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
