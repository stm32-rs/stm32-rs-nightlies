///Register `FDCAN_TSCC` reader
pub type R = crate::R<FDCAN_TSCCrs>;
///Register `FDCAN_TSCC` writer
pub type W = crate::W<FDCAN_TSCCrs>;
/**Timestamp select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSS {
    ///0: Timestamp counter value always 0x0000
    B0x0 = 0,
    ///1: Timestamp counter value incremented according to TCP
    B0x1 = 1,
    ///2: External timestamp counter from TIM3 value (tim3_cnt\[0:15\])
    B0x2 = 2,
    ///3: Same as 00.
    B0x3 = 3,
}
impl From<TSS> for u8 {
    #[inline(always)]
    fn from(variant: TSS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSS {
    type Ux = u8;
}
impl crate::IsEnum for TSS {}
///Field `TSS` reader - Timestamp select
pub type TSS_R = crate::FieldReader<TSS>;
impl TSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSS {
        match self.bits {
            0 => TSS::B0x0,
            1 => TSS::B0x1,
            2 => TSS::B0x2,
            3 => TSS::B0x3,
            _ => unreachable!(),
        }
    }
    ///Timestamp counter value always 0x0000
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSS::B0x0
    }
    ///Timestamp counter value incremented according to TCP
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSS::B0x1
    }
    ///External timestamp counter from TIM3 value (tim3_cnt\[0:15\])
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TSS::B0x2
    }
    ///Same as 00.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TSS::B0x3
    }
}
///Field `TSS` writer - Timestamp select
pub type TSS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TSS, crate::Safe>;
impl<'a, REG> TSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timestamp counter value always 0x0000
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSS::B0x0)
    }
    ///Timestamp counter value incremented according to TCP
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSS::B0x1)
    }
    ///External timestamp counter from TIM3 value (tim3_cnt\[0:15\])
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TSS::B0x2)
    }
    ///Same as 00.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TSS::B0x3)
    }
}
///Field `TCP` reader - Timestamp counter prescaler
pub type TCP_R = crate::FieldReader;
///Field `TCP` writer - Timestamp counter prescaler
pub type TCP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Timestamp select
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:19 - Timestamp counter prescaler
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TSCC")
            .field("tss", &self.tss())
            .field("tcp", &self.tcp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timestamp select
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<'_, FDCAN_TSCCrs> {
        TSS_W::new(self, 0)
    }
    ///Bits 16:19 - Timestamp counter prescaler
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<'_, FDCAN_TSCCrs> {
        TCP_W::new(self, 16)
    }
}
/**FDCAN timestamp counter configuration register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TSCC)*/
pub struct FDCAN_TSCCrs;
impl crate::RegisterSpec for FDCAN_TSCCrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tscc::R`](R) reader structure
impl crate::Readable for FDCAN_TSCCrs {}
///`write(|w| ..)` method takes [`fdcan_tscc::W`](W) writer structure
impl crate::Writable for FDCAN_TSCCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TSCC to value 0
impl crate::Resettable for FDCAN_TSCCrs {}
