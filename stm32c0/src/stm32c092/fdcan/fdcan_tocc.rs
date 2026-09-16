///Register `FDCAN_TOCC` reader
pub type R = crate::R<FDCAN_TOCCrs>;
///Register `FDCAN_TOCC` writer
pub type W = crate::W<FDCAN_TOCCrs>;
/**Timeout counter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETOC {
    ///0: Timeout counter disabled
    B0x0 = 0,
    ///1: Timeout counter enabled
    B0x1 = 1,
}
impl From<ETOC> for bool {
    #[inline(always)]
    fn from(variant: ETOC) -> Self {
        variant as u8 != 0
    }
}
///Field `ETOC` reader - Timeout counter enable
pub type ETOC_R = crate::BitReader<ETOC>;
impl ETOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETOC {
        match self.bits {
            false => ETOC::B0x0,
            true => ETOC::B0x1,
        }
    }
    ///Timeout counter disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETOC::B0x0
    }
    ///Timeout counter enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETOC::B0x1
    }
}
///Field `ETOC` writer - Timeout counter enable
pub type ETOC_W<'a, REG> = crate::BitWriter<'a, REG, ETOC>;
impl<'a, REG> ETOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timeout counter disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ETOC::B0x0)
    }
    ///Timeout counter enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ETOC::B0x1)
    }
}
/**Timeout select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOS {
    ///0: Continuous operation
    B0x0 = 0,
    ///1: Timeout controlled by Tx event FIFO
    B0x1 = 1,
    ///2: Timeout controlled by Rx FIFO 0
    B0x2 = 2,
    ///3: Timeout controlled by Rx FIFO 1
    B0x3 = 3,
}
impl From<TOS> for u8 {
    #[inline(always)]
    fn from(variant: TOS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TOS {
    type Ux = u8;
}
impl crate::IsEnum for TOS {}
///Field `TOS` reader - Timeout select
pub type TOS_R = crate::FieldReader<TOS>;
impl TOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOS {
        match self.bits {
            0 => TOS::B0x0,
            1 => TOS::B0x1,
            2 => TOS::B0x2,
            3 => TOS::B0x3,
            _ => unreachable!(),
        }
    }
    ///Continuous operation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOS::B0x0
    }
    ///Timeout controlled by Tx event FIFO
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOS::B0x1
    }
    ///Timeout controlled by Rx FIFO 0
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TOS::B0x2
    }
    ///Timeout controlled by Rx FIFO 1
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TOS::B0x3
    }
}
///Field `TOS` writer - Timeout select
pub type TOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TOS, crate::Safe>;
impl<'a, REG> TOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Continuous operation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOS::B0x0)
    }
    ///Timeout controlled by Tx event FIFO
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOS::B0x1)
    }
    ///Timeout controlled by Rx FIFO 0
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TOS::B0x2)
    }
    ///Timeout controlled by Rx FIFO 1
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TOS::B0x3)
    }
}
///Field `TOP` reader - Timeout period
pub type TOP_R = crate::FieldReader<u16>;
///Field `TOP` writer - Timeout period
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bit 0 - Timeout counter enable
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Timeout select
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 16:31 - Timeout period
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCC")
            .field("etoc", &self.etoc())
            .field("tos", &self.tos())
            .field("top", &self.top())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout counter enable
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W<'_, FDCAN_TOCCrs> {
        ETOC_W::new(self, 0)
    }
    ///Bits 1:2 - Timeout select
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W<'_, FDCAN_TOCCrs> {
        TOS_W::new(self, 1)
    }
    ///Bits 16:31 - Timeout period
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W<'_, FDCAN_TOCCrs> {
        TOP_W::new(self, 16)
    }
}
/**FDCAN timeout counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TOCC)*/
pub struct FDCAN_TOCCrs;
impl crate::RegisterSpec for FDCAN_TOCCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tocc::R`](R) reader structure
impl crate::Readable for FDCAN_TOCCrs {}
///`write(|w| ..)` method takes [`fdcan_tocc::W`](W) writer structure
impl crate::Writable for FDCAN_TOCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TOCC to value 0xffff_0000
impl crate::Resettable for FDCAN_TOCCrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
