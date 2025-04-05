///Register `LPCR` reader
pub type R = crate::R<LPCRrs>;
///Register `LPCR` writer
pub type W = crate::W<LPCRrs>;
/**Data enable polarity This bit configures the polarity of data enable pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP {
    ///0: Data enable pin active high (default)
    B0x0 = 0,
    ///1: Data enable pin active low
    B0x1 = 1,
}
impl From<DEP> for bool {
    #[inline(always)]
    fn from(variant: DEP) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - Data enable polarity This bit configures the polarity of data enable pin.
pub type DEP_R = crate::BitReader<DEP>;
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEP {
        match self.bits {
            false => DEP::B0x0,
            true => DEP::B0x1,
        }
    }
    ///Data enable pin active high (default)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DEP::B0x0
    }
    ///Data enable pin active low
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DEP::B0x1
    }
}
///Field `DEP` writer - Data enable polarity This bit configures the polarity of data enable pin.
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data enable pin active high (default)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::B0x0)
    }
    ///Data enable pin active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::B0x1)
    }
}
/**VSYNC polarity This bit configures the polarity of VSYNC pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSP {
    ///0: Shutdown pin active high (default)
    B0x0 = 0,
    ///1: Shutdown pin active low
    B0x1 = 1,
}
impl From<VSP> for bool {
    #[inline(always)]
    fn from(variant: VSP) -> Self {
        variant as u8 != 0
    }
}
///Field `VSP` reader - VSYNC polarity This bit configures the polarity of VSYNC pin.
pub type VSP_R = crate::BitReader<VSP>;
impl VSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VSP {
        match self.bits {
            false => VSP::B0x0,
            true => VSP::B0x1,
        }
    }
    ///Shutdown pin active high (default)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VSP::B0x0
    }
    ///Shutdown pin active low
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VSP::B0x1
    }
}
///Field `VSP` writer - VSYNC polarity This bit configures the polarity of VSYNC pin.
pub type VSP_W<'a, REG> = crate::BitWriter<'a, REG, VSP>;
impl<'a, REG> VSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Shutdown pin active high (default)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(VSP::B0x0)
    }
    ///Shutdown pin active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(VSP::B0x1)
    }
}
/**HSYNC polarity This bit configures the polarity of HSYNC pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSP {
    ///0: HSYNC pin active high (default)
    B0x0 = 0,
    ///1: VSYNC pin active low
    B0x1 = 1,
}
impl From<HSP> for bool {
    #[inline(always)]
    fn from(variant: HSP) -> Self {
        variant as u8 != 0
    }
}
///Field `HSP` reader - HSYNC polarity This bit configures the polarity of HSYNC pin.
pub type HSP_R = crate::BitReader<HSP>;
impl HSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSP {
        match self.bits {
            false => HSP::B0x0,
            true => HSP::B0x1,
        }
    }
    ///HSYNC pin active high (default)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSP::B0x0
    }
    ///VSYNC pin active low
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSP::B0x1
    }
}
///Field `HSP` writer - HSYNC polarity This bit configures the polarity of HSYNC pin.
pub type HSP_W<'a, REG> = crate::BitWriter<'a, REG, HSP>;
impl<'a, REG> HSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HSYNC pin active high (default)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSP::B0x0)
    }
    ///VSYNC pin active low
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSP::B0x1)
    }
}
impl R {
    ///Bit 0 - Data enable polarity This bit configures the polarity of data enable pin.
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSYNC polarity This bit configures the polarity of VSYNC pin.
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSYNC polarity This bit configures the polarity of HSYNC pin.
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCR")
            .field("dep", &self.dep())
            .field("vsp", &self.vsp())
            .field("hsp", &self.hsp())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data enable polarity This bit configures the polarity of data enable pin.
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<LPCRrs> {
        DEP_W::new(self, 0)
    }
    ///Bit 1 - VSYNC polarity This bit configures the polarity of VSYNC pin.
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W<LPCRrs> {
        VSP_W::new(self, 1)
    }
    ///Bit 2 - HSYNC polarity This bit configures the polarity of HSYNC pin.
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W<LPCRrs> {
        HSP_W::new(self, 2)
    }
}
/**DSI Host LTDC polarity configuration register

You can [`read`](crate::Reg::read) this register and get [`lpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:LPCR)*/
pub struct LPCRrs;
impl crate::RegisterSpec for LPCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpcr::R`](R) reader structure
impl crate::Readable for LPCRrs {}
///`write(|w| ..)` method takes [`lpcr::W`](W) writer structure
impl crate::Writable for LPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPCR to value 0
impl crate::Resettable for LPCRrs {}
