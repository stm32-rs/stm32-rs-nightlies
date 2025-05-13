///Register `WRPCR` reader
pub type R = crate::R<WRPCRrs>;
///Register `WRPCR` writer
pub type W = crate::W<WRPCRrs>;
/**PLL enable This bit enables the D-PHY PLL.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLEN {
    ///0: PLL disabled
    B0x0 = 0,
    ///1: PLL enabled
    B0x1 = 1,
}
impl From<PLLEN> for bool {
    #[inline(always)]
    fn from(variant: PLLEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLEN` reader - PLL enable This bit enables the D-PHY PLL.
pub type PLLEN_R = crate::BitReader<PLLEN>;
impl PLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLEN {
        match self.bits {
            false => PLLEN::B0x0,
            true => PLLEN::B0x1,
        }
    }
    ///PLL disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLEN::B0x0
    }
    ///PLL enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLEN::B0x1
    }
}
///Field `PLLEN` writer - PLL enable This bit enables the D-PHY PLL.
pub type PLLEN_W<'a, REG> = crate::BitWriter<'a, REG, PLLEN>;
impl<'a, REG> PLLEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PLL disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLEN::B0x0)
    }
    ///PLL enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLEN::B0x1)
    }
}
///Field `NDIV` reader - PLL loop division factor This field configures the PLL loop division factor. 10 to 125: Allowed loop division factor values Others: Reserved
pub type NDIV_R = crate::FieldReader;
///Field `NDIV` writer - PLL loop division factor This field configures the PLL loop division factor. 10 to 125: Allowed loop division factor values Others: Reserved
pub type NDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**PLL input division factor This field configures the PLL input division factor.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDF {
    ///0: PLL input divided by 1
    B0x0 = 0,
    ///1: PLL input divided by 1
    B0x1 = 1,
    ///2: PLL input divided by 2
    B0x2 = 2,
    ///3: PLL input divided by 3
    B0x3 = 3,
    ///4: PLL input divided by 4
    B0x4 = 4,
    ///5: PLL input divided by 5
    B0x5 = 5,
    ///6: PLL input divided by 6
    B0x6 = 6,
    ///7: PLL input divided by 7
    B0x7 = 7,
}
impl From<IDF> for u8 {
    #[inline(always)]
    fn from(variant: IDF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IDF {
    type Ux = u8;
}
impl crate::IsEnum for IDF {}
///Field `IDF` reader - PLL input division factor This field configures the PLL input division factor.
pub type IDF_R = crate::FieldReader<IDF>;
impl IDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IDF> {
        match self.bits {
            0 => Some(IDF::B0x0),
            1 => Some(IDF::B0x1),
            2 => Some(IDF::B0x2),
            3 => Some(IDF::B0x3),
            4 => Some(IDF::B0x4),
            5 => Some(IDF::B0x5),
            6 => Some(IDF::B0x6),
            7 => Some(IDF::B0x7),
            _ => None,
        }
    }
    ///PLL input divided by 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IDF::B0x0
    }
    ///PLL input divided by 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IDF::B0x1
    }
    ///PLL input divided by 2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IDF::B0x2
    }
    ///PLL input divided by 3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == IDF::B0x3
    }
    ///PLL input divided by 4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == IDF::B0x4
    }
    ///PLL input divided by 5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == IDF::B0x5
    }
    ///PLL input divided by 6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == IDF::B0x6
    }
    ///PLL input divided by 7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == IDF::B0x7
    }
}
///Field `IDF` writer - PLL input division factor This field configures the PLL input division factor.
pub type IDF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, IDF>;
impl<'a, REG> IDF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL input divided by 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x0)
    }
    ///PLL input divided by 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x1)
    }
    ///PLL input divided by 2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x2)
    }
    ///PLL input divided by 3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x3)
    }
    ///PLL input divided by 4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x4)
    }
    ///PLL input divided by 5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x5)
    }
    ///PLL input divided by 6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x6)
    }
    ///PLL input divided by 7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(IDF::B0x7)
    }
}
/**PLL output division factor This field configures the PLL output division factor.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ODF {
    ///0: PLL output divided by 1
    B0x0 = 0,
    ///1: PLL output divided by 2
    B0x1 = 1,
    ///2: PLL output divided by 4
    B0x2 = 2,
    ///3: PLL output divided by 8
    B0x3 = 3,
}
impl From<ODF> for u8 {
    #[inline(always)]
    fn from(variant: ODF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ODF {
    type Ux = u8;
}
impl crate::IsEnum for ODF {}
///Field `ODF` reader - PLL output division factor This field configures the PLL output division factor.
pub type ODF_R = crate::FieldReader<ODF>;
impl ODF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ODF {
        match self.bits {
            0 => ODF::B0x0,
            1 => ODF::B0x1,
            2 => ODF::B0x2,
            3 => ODF::B0x3,
            _ => unreachable!(),
        }
    }
    ///PLL output divided by 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ODF::B0x0
    }
    ///PLL output divided by 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ODF::B0x1
    }
    ///PLL output divided by 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ODF::B0x2
    }
    ///PLL output divided by 8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ODF::B0x3
    }
}
///Field `ODF` writer - PLL output division factor This field configures the PLL output division factor.
pub type ODF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ODF, crate::Safe>;
impl<'a, REG> ODF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PLL output divided by 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ODF::B0x0)
    }
    ///PLL output divided by 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ODF::B0x1)
    }
    ///PLL output divided by 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(ODF::B0x2)
    }
    ///PLL output divided by 8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(ODF::B0x3)
    }
}
/**Regulator enable This bit enables the DPHY regulator.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGEN {
    ///0: regulator disabled
    B0x0 = 0,
    ///1: regulator enabled
    B0x1 = 1,
}
impl From<REGEN> for bool {
    #[inline(always)]
    fn from(variant: REGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `REGEN` reader - Regulator enable This bit enables the DPHY regulator.
pub type REGEN_R = crate::BitReader<REGEN>;
impl REGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REGEN {
        match self.bits {
            false => REGEN::B0x0,
            true => REGEN::B0x1,
        }
    }
    ///regulator disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REGEN::B0x0
    }
    ///regulator enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REGEN::B0x1
    }
}
///Field `REGEN` writer - Regulator enable This bit enables the DPHY regulator.
pub type REGEN_W<'a, REG> = crate::BitWriter<'a, REG, REGEN>;
impl<'a, REG> REGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///regulator disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REGEN::B0x0)
    }
    ///regulator enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REGEN::B0x1)
    }
}
impl R {
    ///Bit 0 - PLL enable This bit enables the D-PHY PLL.
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:8 - PLL loop division factor This field configures the PLL loop division factor. 10 to 125: Allowed loop division factor values Others: Reserved
    #[inline(always)]
    pub fn ndiv(&self) -> NDIV_R {
        NDIV_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    ///Bits 11:14 - PLL input division factor This field configures the PLL input division factor.
    #[inline(always)]
    pub fn idf(&self) -> IDF_R {
        IDF_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 16:17 - PLL output division factor This field configures the PLL output division factor.
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Regulator enable This bit enables the DPHY regulator.
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPCR")
            .field("pllen", &self.pllen())
            .field("ndiv", &self.ndiv())
            .field("idf", &self.idf())
            .field("odf", &self.odf())
            .field("regen", &self.regen())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL enable This bit enables the D-PHY PLL.
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W<WRPCRrs> {
        PLLEN_W::new(self, 0)
    }
    ///Bits 2:8 - PLL loop division factor This field configures the PLL loop division factor. 10 to 125: Allowed loop division factor values Others: Reserved
    #[inline(always)]
    pub fn ndiv(&mut self) -> NDIV_W<WRPCRrs> {
        NDIV_W::new(self, 2)
    }
    ///Bits 11:14 - PLL input division factor This field configures the PLL input division factor.
    #[inline(always)]
    pub fn idf(&mut self) -> IDF_W<WRPCRrs> {
        IDF_W::new(self, 11)
    }
    ///Bits 16:17 - PLL output division factor This field configures the PLL output division factor.
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W<WRPCRrs> {
        ODF_W::new(self, 16)
    }
    ///Bit 24 - Regulator enable This bit enables the DPHY regulator.
    #[inline(always)]
    pub fn regen(&mut self) -> REGEN_W<WRPCRrs> {
        REGEN_W::new(self, 24)
    }
}
/**DSI Wrapper regulator and PLL control register

You can [`read`](crate::Reg::read) this register and get [`wrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WRPCR)*/
pub struct WRPCRrs;
impl crate::RegisterSpec for WRPCRrs {
    type Ux = u32;
}
///`read()` method returns [`wrpcr::R`](R) reader structure
impl crate::Readable for WRPCRrs {}
///`write(|w| ..)` method takes [`wrpcr::W`](W) writer structure
impl crate::Writable for WRPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRPCR to value 0
impl crate::Resettable for WRPCRrs {}
