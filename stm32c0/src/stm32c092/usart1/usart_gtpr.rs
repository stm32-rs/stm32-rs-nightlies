///Register `USART_GTPR` reader
pub type R = crate::R<USART_GTPRrs>;
///Register `USART_GTPR` writer
pub type W = crate::W<USART_GTPRrs>;
/**Prescaler value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSC {
    ///0: Reserved - do not program this value
    B0x0 = 0,
    ///1: Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)
    B0x1 = 1,
    ///2: Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)
    B0x2 = 2,
    ///3: Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)
    B0x3 = 3,
    ///31: Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)
    B0x1f = 31,
    ///32: Divides the source clock by 32 (IrDA mode)
    B0x20 = 32,
    ///255: Divides the source clock by 255 (IrDA mode)
    B0xFf = 255,
}
impl From<PSC> for u8 {
    #[inline(always)]
    fn from(variant: PSC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSC {
    type Ux = u8;
}
impl crate::IsEnum for PSC {}
///Field `PSC` reader - Prescaler value
pub type PSC_R = crate::FieldReader<PSC>;
impl PSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSC> {
        match self.bits {
            0 => Some(PSC::B0x0),
            1 => Some(PSC::B0x1),
            2 => Some(PSC::B0x2),
            3 => Some(PSC::B0x3),
            31 => Some(PSC::B0x1f),
            32 => Some(PSC::B0x20),
            255 => Some(PSC::B0xFf),
            _ => None,
        }
    }
    ///Reserved - do not program this value
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSC::B0x0
    }
    ///Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSC::B0x1
    }
    ///Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PSC::B0x2
    }
    ///Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PSC::B0x3
    }
    ///Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == PSC::B0x1f
    }
    ///Divides the source clock by 32 (IrDA mode)
    #[inline(always)]
    pub fn is_b_0x20(&self) -> bool {
        *self == PSC::B0x20
    }
    ///Divides the source clock by 255 (IrDA mode)
    #[inline(always)]
    pub fn is_b_0x_ff(&self) -> bool {
        *self == PSC::B0xFf
    }
}
///Field `PSC` writer - Prescaler value
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 8, PSC>;
impl<'a, REG> PSC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reserved - do not program this value
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x0)
    }
    ///Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x1)
    }
    ///Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x2)
    }
    ///Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x3)
    }
    ///Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x1f)
    }
    ///Divides the source clock by 32 (IrDA mode)
    #[inline(always)]
    pub fn b_0x20(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0x20)
    }
    ///Divides the source clock by 255 (IrDA mode)
    #[inline(always)]
    pub fn b_0x_ff(self) -> &'a mut crate::W<REG> {
        self.variant(PSC::B0xFf)
    }
}
///Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type GT_R = crate::FieldReader;
///Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type GT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_GTPR")
            .field("psc", &self.psc())
            .field("gt", &self.gt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Prescaler value
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<'_, USART_GTPRrs> {
        PSC_W::new(self, 0)
    }
    ///Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W<'_, USART_GTPRrs> {
        GT_W::new(self, 8)
    }
}
/**USART guard time and prescaler register

You can [`read`](crate::Reg::read) this register and get [`usart_gtpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_gtpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#USART1:USART_GTPR)*/
pub struct USART_GTPRrs;
impl crate::RegisterSpec for USART_GTPRrs {
    type Ux = u32;
}
///`read()` method returns [`usart_gtpr::R`](R) reader structure
impl crate::Readable for USART_GTPRrs {}
///`write(|w| ..)` method takes [`usart_gtpr::W`](W) writer structure
impl crate::Writable for USART_GTPRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_GTPR to value 0
impl crate::Resettable for USART_GTPRrs {}
