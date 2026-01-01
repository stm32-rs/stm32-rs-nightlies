///Register `TIM1_DCR` reader
pub type R = crate::R<TIM1_DCRrs>;
///Register `TIM1_DCR` writer
pub type W = crate::W<TIM1_DCRrs>;
/**DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...

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
///Field `DBA` reader - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...
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
///Field `DBA` writer - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...
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
/**DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DBL {
    ///0: 1 transfer
    B0x0 = 0,
    ///1: 2 transfers
    B0x1 = 1,
    ///2: 3 transfers
    B0x2 = 2,
    ///17: 18 transfers
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
///Field `DBL` reader - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.
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
    ///1 transfer
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBL::B0x0
    }
    ///2 transfers
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBL::B0x1
    }
    ///3 transfers
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DBL::B0x2
    }
    ///18 transfers
    #[inline(always)]
    pub fn is_b_0x11(&self) -> bool {
        *self == DBL::B0x11
    }
}
///Field `DBL` writer - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.
pub type DBL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, DBL>;
impl<'a, REG> DBL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 transfer
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x0)
    }
    ///2 transfers
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x1)
    }
    ///3 transfers
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x2)
    }
    ///18 transfers
    #[inline(always)]
    pub fn b_0x11(self) -> &'a mut crate::W<REG> {
        self.variant(DBL::B0x11)
    }
}
impl R {
    ///Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM1_DCR")
            .field("dba", &self.dba())
            .field("dbl", &self.dbl())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DMA base address This 5-bits vector defines the base-address for DMA transfers (when read/write access are done through the TIMx_DMAR address). DBA is defined as an offset starting from the address of the TIMx_CR1 register. Example: ...
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W<'_, TIM1_DCRrs> {
        DBA_W::new(self, 0)
    }
    ///Bits 8:12 - DMA burst length This 5-bit vector defines the length of DMA transfers (the timer recognizes a burst transfer when a read or a write access is done to the TIMx_DMAR address), i.e. the number of transfers. Transfers can be in half-words or in bytes (see example below). ... Example: Let us consider the following transfer: DBL = 7 bytes & DBA = TIMx_CR1. If DBL = 7 bytes and DBA = TIMx_CR1 represents the address of the byte to be transferred, the address of the transfer should be given by the following equation: (TIMx_CR1 address) + DBA + (DMA index), where DMA index = DBL In this example, 7 bytes are added to (TIMx_CR1 address) + DBA, which gives us the address from/to which the data is copied. In this case, the transfer is done to 7 registers starting from the following address: (TIMx_CR1 address) + DBA According to the configuration of the DMA Data Size, several cases may occur: If the DMA Data Size is configured in half-words, 16-bit data is transferred to each of the 7 registers. If the DMA Data Size is configured in bytes, the data is also transferred to 7 registers: the first register contains the first MSB byte, the second register, the first LSB byte and so on. So with the transfer Timer, one also has to specify the size of data transferred by DMA.
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W<'_, TIM1_DCRrs> {
        DBL_W::new(self, 8)
    }
}
/**TIM1 DMA control register

You can [`read`](crate::Reg::read) this register and get [`tim1_dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim1_dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM1:TIM1_DCR)*/
pub struct TIM1_DCRrs;
impl crate::RegisterSpec for TIM1_DCRrs {
    type Ux = u16;
}
///`read()` method returns [`tim1_dcr::R`](R) reader structure
impl crate::Readable for TIM1_DCRrs {}
///`write(|w| ..)` method takes [`tim1_dcr::W`](W) writer structure
impl crate::Writable for TIM1_DCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM1_DCR to value 0
impl crate::Resettable for TIM1_DCRrs {}
