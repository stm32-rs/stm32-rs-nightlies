///Register `WWDG_CFR` reader
pub type R = crate::R<WWDG_CFRrs>;
///Register `WWDG_CFR` writer
pub type W = crate::W<WWDG_CFRrs>;
///Field `W` reader - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value These bits contain the window value to be compared with the down-counter.
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `EWI` reader - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
pub type EWI_R = crate::BitReader;
///Field `EWI` writer - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Timer base The timebase of the prescaler can be modified as follows:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB {
    ///0: CK counter clock (PCLK div 4096) div 1
    B0x0 = 0,
    ///1: CK counter clock (PCLK div 4096) div 2
    B0x1 = 1,
    ///2: CK counter clock (PCLK div 4096) div 4
    B0x2 = 2,
    ///3: CK counter clock (PCLK div 4096) div 8
    B0x3 = 3,
    ///4: CK counter clock (PCLK div 4096) div 16
    B0x4 = 4,
    ///5: CK counter clock (PCLK div 4096) div 32
    B0x5 = 5,
    ///6: CK counter clock (PCLK div 4096) div 64
    B0x6 = 6,
    ///7: CK counter clock (PCLK div 4096) div 128
    B0x7 = 7,
}
impl From<WDGTB> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDGTB {
    type Ux = u8;
}
impl crate::IsEnum for WDGTB {}
///Field `WDGTB` reader - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_R = crate::FieldReader<WDGTB>;
impl WDGTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB {
        match self.bits {
            0 => WDGTB::B0x0,
            1 => WDGTB::B0x1,
            2 => WDGTB::B0x2,
            3 => WDGTB::B0x3,
            4 => WDGTB::B0x4,
            5 => WDGTB::B0x5,
            6 => WDGTB::B0x6,
            7 => WDGTB::B0x7,
            _ => unreachable!(),
        }
    }
    ///CK counter clock (PCLK div 4096) div 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGTB::B0x0
    }
    ///CK counter clock (PCLK div 4096) div 2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGTB::B0x1
    }
    ///CK counter clock (PCLK div 4096) div 4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == WDGTB::B0x2
    }
    ///CK counter clock (PCLK div 4096) div 8
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == WDGTB::B0x3
    }
    ///CK counter clock (PCLK div 4096) div 16
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == WDGTB::B0x4
    }
    ///CK counter clock (PCLK div 4096) div 32
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == WDGTB::B0x5
    }
    ///CK counter clock (PCLK div 4096) div 64
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == WDGTB::B0x6
    }
    ///CK counter clock (PCLK div 4096) div 128
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == WDGTB::B0x7
    }
}
///Field `WDGTB` writer - Timer base The timebase of the prescaler can be modified as follows:
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WDGTB, crate::Safe>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CK counter clock (PCLK div 4096) div 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x0)
    }
    ///CK counter clock (PCLK div 4096) div 2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x1)
    }
    ///CK counter clock (PCLK div 4096) div 4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x2)
    }
    ///CK counter clock (PCLK div 4096) div 8
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x3)
    }
    ///CK counter clock (PCLK div 4096) div 16
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x4)
    }
    ///CK counter clock (PCLK div 4096) div 32
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x5)
    }
    ///CK counter clock (PCLK div 4096) div 64
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x6)
    }
    ///CK counter clock (PCLK div 4096) div 128
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::B0x7)
    }
}
impl R {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WWDG_CFR")
            .field("w", &self.w())
            .field("ewi", &self.ewi())
            .field("wdgtb", &self.wdgtb())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter.
    #[inline(always)]
    pub fn w(&mut self) -> W_W<'_, WWDG_CFRrs> {
        W_W::new(self, 0)
    }
    ///Bit 9 - Early wake-up interrupt enable Set by software and cleared by hardware after a reset. When set, an interrupt occurs whenever the counter reaches the value 0x40.
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<'_, WWDG_CFRrs> {
        EWI_W::new(self, 9)
    }
    ///Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<'_, WWDG_CFRrs> {
        WDGTB_W::new(self, 11)
    }
}
/**WWDG configuration register

You can [`read`](crate::Reg::read) this register and get [`wwdg_cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdg_cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#WWDG:WWDG_CFR)*/
pub struct WWDG_CFRrs;
impl crate::RegisterSpec for WWDG_CFRrs {
    type Ux = u32;
}
///`read()` method returns [`wwdg_cfr::R`](R) reader structure
impl crate::Readable for WWDG_CFRrs {}
///`write(|w| ..)` method takes [`wwdg_cfr::W`](W) writer structure
impl crate::Writable for WWDG_CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WWDG_CFR to value 0x7f
impl crate::Resettable for WWDG_CFRrs {
    const RESET_VALUE: u32 = 0x7f;
}
