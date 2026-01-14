///Register `TIM15_DCR` reader
pub type R = crate::R<TIM15_DCRrs>;
///Register `TIM15_DCR` writer
pub type W = crate::W<TIM15_DCRrs>;
/**DMA base address

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBA {
    ///0: TIMx_CR1,
    B0x0 = 0,
    ///1: TIMx_CR2,
    B0x1 = 1,
    ///2: TIMx_SMCR,
    B0x2 = 2,
}
impl From<DBA> for u8 {
    #[inline(always)]
    fn from(variant: DBA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBA {
    type Ux = u8;
}
impl crate::IsEnum for DBA {}
///Field `DBA` reader - DMA base address
pub type DBA_R = crate::FieldReader<DBA>;
impl DBA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBA> {
        match self.bits {
            0 => Some(DBA::B0x0),
            1 => Some(DBA::B0x1),
            2 => Some(DBA::B0x2),
            _ => None,
        }
    }
    ///TIMx_CR1,
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBA::B0x0
    }
    ///TIMx_CR2,
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBA::B0x1
    }
    ///TIMx_SMCR,
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBA::B0x2
    }
}
///Field `DBA` writer - DMA base address
pub type DBA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBA>;
impl<'a, REG> DBA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIMx_CR1,
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBA::B0x0)
    }
    ///TIMx_CR2,
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBA::B0x1)
    }
    ///TIMx_SMCR,
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBA::B0x2)
    }
}
/**DMA burst length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBL {
    ///0: 1 transfer,
    B0x0 = 0,
    ///1: 2 transfers,
    B0x1 = 1,
    ///2: 3 transfers,
    B0x2 = 2,
    ///17: 18 transfers.
    B0x11 = 17,
}
impl From<DBL> for u8 {
    #[inline(always)]
    fn from(variant: DBL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DBL {
    type Ux = u8;
}
impl crate::IsEnum for DBL {}
///Field `DBL` reader - DMA burst length
pub type DBL_R = crate::FieldReader<DBL>;
impl DBL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DBL> {
        match self.bits {
            0 => Some(DBL::B0x0),
            1 => Some(DBL::B0x1),
            2 => Some(DBL::B0x2),
            17 => Some(DBL::B0x11),
            _ => None,
        }
    }
    ///1 transfer,
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBL::B0x0
    }
    ///2 transfers,
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBL::B0x1
    }
    ///3 transfers,
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBL::B0x2
    }
    ///18 transfers.
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == DBL::B0x11
    }
}
///Field `DBL` writer - DMA burst length
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBL>;
impl<'a, REG> DBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 transfer,
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x0)
    }
    ///2 transfers,
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x1)
    }
    ///3 transfers,
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x2)
    }
    ///18 transfers.
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x11)
    }
}
impl R {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA base address
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<'_, TIM15_DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DMA burst length
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<'_, TIM15_DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM15 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim15_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_DCR)*/
pub struct TIM15_DCRrs;
impl crate::RegisterSpec for TIM15_DCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_dcr::R`](R) reader structure
impl crate::Readable for TIM15_DCRrs {}
///`write(|w| ..)` method takes [`tim15_dcr::W`](W) writer structure
impl crate::Writable for TIM15_DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_DCR to value 0
impl crate::Resettable for TIM15_DCRrs {}
