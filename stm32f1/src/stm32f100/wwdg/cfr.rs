///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `W` reader - 7-bit window value
pub type W_R = crate::FieldReader;
///Field `W` writer - 7-bit window value
pub type W_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Timer Base

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDGTB {
    ///0: Counter clock (PCLK1 div 4096) div 1
    Div1 = 0,
    ///1: Counter clock (PCLK1 div 4096) div 2
    Div2 = 1,
    ///2: Counter clock (PCLK1 div 4096) div 4
    Div4 = 2,
    ///3: Counter clock (PCLK1 div 4096) div 8
    Div8 = 3,
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
///Field `WDGTB` reader - Timer Base
pub type WDGTB_R = crate::FieldReader<WDGTB>;
impl WDGTB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDGTB {
        match self.bits {
            0 => WDGTB::Div1,
            1 => WDGTB::Div2,
            2 => WDGTB::Div4,
            3 => WDGTB::Div8,
            _ => unreachable!(),
        }
    }
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == WDGTB::Div1
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WDGTB::Div2
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WDGTB::Div4
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WDGTB::Div8
    }
}
///Field `WDGTB` writer - Timer Base
pub type WDGTB_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WDGTB, crate::Safe>;
impl<'a, REG> WDGTB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Counter clock (PCLK1 div 4096) div 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div1)
    }
    ///Counter clock (PCLK1 div 4096) div 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div2)
    }
    ///Counter clock (PCLK1 div 4096) div 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div4)
    }
    ///Counter clock (PCLK1 div 4096) div 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(WDGTB::Div8)
    }
}
/**Early Wakeup Interrupt

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWIW {
    ///1: interrupt occurs whenever the counter reaches the value 0x40
    Enable = 1,
}
impl From<EWIW> for bool {
    #[inline(always)]
    fn from(variant: EWIW) -> Self {
        variant as u8 != 0
    }
}
///Field `EWI` reader - Early Wakeup Interrupt
pub type EWI_R = crate::BitReader<EWIW>;
impl EWI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EWIW> {
        match self.bits {
            true => Some(EWIW::Enable),
            _ => None,
        }
    }
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EWIW::Enable
    }
}
///Field `EWI` writer - Early Wakeup Interrupt
pub type EWI_W<'a, REG> = crate::BitWriter<'a, REG, EWIW>;
impl<'a, REG> EWI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///interrupt occurs whenever the counter reaches the value 0x40
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EWIW::Enable)
    }
}
impl R {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:8 - Timer Base
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Early Wakeup Interrupt
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR")
            .field("w", &self.w())
            .field("wdgtb", &self.wdgtb())
            .field("ewi", &self.ewi())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - 7-bit window value
    #[inline(always)]
    pub fn w(&mut self) -> W_W<'_, CFRrs> {
        W_W::new(self, 0)
    }
    ///Bits 7:8 - Timer Base
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W<'_, CFRrs> {
        WDGTB_W::new(self, 7)
    }
    ///Bit 9 - Early Wakeup Interrupt
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W<'_, CFRrs> {
        EWI_W::new(self, 9)
    }
}
/**Configuration register (WWDG_CFR)

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F100.html#WWDG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u16;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0x7f
impl crate::Resettable for CFRrs {
    const RESET_VALUE: u16 = 0x7f;
}
